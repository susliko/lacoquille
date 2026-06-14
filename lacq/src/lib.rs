pub mod config;
pub mod routes;
pub mod gutenberg;
pub mod tokenizer;

pub use config::Config;

use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};
use tokio::time::Duration;

#[derive(Clone)]
pub struct BookMeta {
    pub gutenberg_id: u64,
    pub title: String,
    pub collection: String,
    pub published_year: i32,
}

impl BookMeta {
    pub fn curated() -> Vec<Self> {
        vec![
            BookMeta {
                gutenberg_id: 10775, // Le Horla
                title: "Le Horla".to_string(),
                collection: "Contes du jour et de la nuit".to_string(),
                published_year: 1887,
            },
            BookMeta {
                gutenberg_id: 14790, // Contes du jour et de la nuit
                title: "Contes du jour et de la nuit".to_string(),
                collection: "Contes du jour et de la nuit".to_string(),
                published_year: 1884,
            },
            BookMeta {
                gutenberg_id: 10746, // Boule de Suif
                title: "Boule de Suif".to_string(),
                collection: "Les Rougon-Macquart".to_string(),
                published_year: 1880,
            },
            BookMeta {
                gutenberg_id: 12011, // Monsieur Parent
                title: "Monsieur Parent".to_string(),
                collection: "Nouvelles".to_string(),
                published_year: 1885,
            },
            BookMeta {
                gutenberg_id: 11131, // Pierre et Jean
                title: "Pierre et Jean".to_string(),
                collection: "Roman".to_string(),
                published_year: 1888,
            },
        ]
    }

    pub fn days_since_epoch() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() / 86400
    }

    pub fn for_today() -> Self {
        let books = Self::curated();
        let idx = (Self::days_since_epoch() % books.len() as u64) as usize;
        books[idx].clone()
    }
}

pub struct CachedText {
    pub text: String,
    pub fetched_at: std::time::Instant,
}

pub struct CachedArticle {
    pub response: ArticleResponse,
    pub computed_at: std::time::Instant,
}

#[derive(Clone)]
pub struct AppState {
    pub curated_books: Vec<BookMeta>,
    pub text_cache: std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<u64, CachedText>>>,
    pub tokenization_cache: std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<String, Arc<OnceCell<tokenizer::TokenizedText>>>>>,
    pub http: reqwest::Client,
    pub data_dir: String,
    pub config: Config,
    /// Pre-computed article-of-the-day, refreshed daily.
    /// The Option is Some once initial computation completes.
    pub cached_article: Arc<RwLock<Option<CachedArticle>>>,
}

impl AppState {
    pub fn new(http: reqwest::Client, data_dir: String, config: Config) -> Self {
        Self {
            curated_books: BookMeta::curated(),
            text_cache: std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
            tokenization_cache: std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
            http,
            data_dir,
            config,
            cached_article: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn get_french_text(&self, book_id: u64) -> Result<String, String> {
        {
            let cache = self.text_cache.lock().await;
            if let Some(cached) = cache.get(&book_id) {
                let age_secs = cached.fetched_at.elapsed().as_secs();
                if age_secs < 86400 {
                    return Ok(cached.text.clone());
                }
            }
        }

        let url = format!("https://www.gutenberg.org/files/{}/{}-0.txt", book_id, book_id);
        let resp = self.http.get(&url).send().await
            .map_err(|e| format!("failed to fetch {}: {}", url, e))?;
        let resp = resp.error_for_status().map_err(|e| format!("HTTP error: {}", e))?;
        let text = resp.text().await
            .map_err(|e| format!("failed to read response: {}", e))?;

        let cleaned = crate::gutenberg::clean_text(&text);
        let cleaned = cleaned.trim().to_string();

        {
            let mut cache = self.text_cache.lock().await;
            cache.insert(book_id, CachedText {
                text: cleaned.clone(),
                fetched_at: std::time::Instant::now(),
            });
        }

        Ok(cleaned)
    }

    /// Simple non-crypto hash of input text, used to differentiate
    /// excerpts of the same book.
    fn text_hash(s: &str) -> u64 {
        s.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64))
    }

    /// Tokenize with fallback through the provider chain.
    pub async fn get_tokenized(&self, book_id: u64, french_text: &str) -> Result<tokenizer::TokenizedText, String> {
        let cache_key = format!("{}-{}", book_id, Self::text_hash(french_text));
        let cell: std::sync::Arc<tokio::sync::OnceCell<tokenizer::TokenizedText>> = {
            let mut cache = self.tokenization_cache.lock().await;
            cache.entry(cache_key).or_default().clone()
        };

        cell.get_or_try_init(|| async {
            if std::env::var("MOCK_TOKENIZED").unwrap_or_default() == "true" {
                tracing::info!("Using mock tokenized data");
                return tokenizer::mock_tokenize_async(french_text).await;
            }

            let chain = self.config.translation_chain();
            if chain.is_empty() {
                return Err("No translation providers configured (set OPENROUTER_API_KEY, GROQ_API_KEY, or GEMINI_API_KEY)".to_string());
            }

            let mut last_error = String::new();

            for (provider, api_key) in &chain {
                for attempt in 0..self.config.retries() {
                    match tokenizer::tokenize(&self.http, api_key, french_text, provider).await {
                        Ok(result) => {
                            tracing::info!("Tokenization succeeded with {}", provider.display());
                            return Ok(result);
                        }
                        Err(e) => {
                            last_error = e.clone();
                            tracing::warn!("Attempt {} failed for {}: {}", attempt, provider.display(), e);
                        }
                    }
                }
            }

            Err(format!("All providers failed. Last error: {}", last_error))
        }).await.cloned()
    }

    /// Spawns background workers:
    /// 1. Immediate computation of today's article
    /// 2. Daily refresh at midnight (using tokio::time::interval)
    pub fn start_background_workers(self: &Arc<Self>) {
        let state = Arc::clone(self);
        tokio::spawn(async move {
            // Initial computation
            compute_article_of_the_day(&state).await;

            // Schedule refresh at next midnight
            let now = chrono::Local::now();
            let next_midnight = (now.date_naive() + chrono::Duration::days(1))
                .and_hms_opt(0, 0, 0)
                .unwrap();
            let secs_until_midnight = (next_midnight - now.naive_local()).num_seconds().max(1) as u64;
            tracing::info!("Article background worker: scheduling first refresh in {} seconds (at midnight)", secs_until_midnight);
            tokio::time::sleep(Duration::from_secs(secs_until_midnight)).await;

            // Then refresh every 24 hours
            let mut interval = tokio::time::interval(Duration::from_secs(86400));
            loop {
                interval.tick().await;
                tracing::info!("Article background worker: refreshing article-of-the-day");
                compute_article_of_the_day(&state).await;
            }
        });
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ArticleResponse {
    pub title: String,
    pub source: String,
    pub published_year: i32,
    pub paragraphs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokenized: Option<TokenizedPayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokenization_error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenizedPayload {
    pub fr_tokens: Vec<FrToken>,
    pub en_tokens: Vec<EnToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FrToken {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_punct: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leading_punct: Option<String>,
    pub translation: String,
    pub spans: Vec<[usize; 2]>,
    pub en_indices: Vec<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EnToken {
    pub text: String,
    pub index: usize,
}

/// Background job: fetch, translate, and cache the article-of-the-day.
pub async fn compute_article_of_the_day(state: &Arc<AppState>) {
    let book = BookMeta::for_today();

    match state.get_french_text(book.gutenberg_id).await {
        Ok(text) => {
            let story_text = crate::gutenberg::first_story_content(&text);
            let paragraphs_raw = crate::gutenberg::split_paragraphs(&story_text);
            let paragraphs = crate::gutenberg::extract_excerpt(&paragraphs_raw, 250);

            // Keep only as many *whole* paragraphs as fit within the character
            // budget. This avoids exceeding token limits while ensuring the
            // paragraphs we return to the client stay perfectly aligned with the
            // text we tokenize (token char-spans must index into this same text).
            // Note: slicing `combined_text[..MAX_CHARS]` directly would panic if
            // the MAX_CHARSth byte fell inside a multi-byte UTF-8 character.
            const MAX_CHARS: usize = 2500;
            let paragraphs: Vec<String> = {
                let mut kept: Vec<String> = Vec::new();
                let mut used = 0usize;
                for p in paragraphs {
                    let sep = if kept.is_empty() { 0 } else { 2 }; // "\n\n"
                    let plen = p.chars().count();
                    if !kept.is_empty() && used + sep + plen > MAX_CHARS {
                        break;
                    }
                    used += sep + plen;
                    kept.push(p);
                }
                kept
            };
            // Guard: nothing usable to show.
            if paragraphs.is_empty() {
                tracing::error!("No paragraphs extracted for book {}", book.gutenberg_id);
                return;
            }
            let combined_text = paragraphs.join("\n\n");

            let (tokenized, tokenization_error) = match state.get_tokenized(book.gutenberg_id, &combined_text).await {
                Ok(t) => {
                    tracing::info!(
                        "Tokenized {} French tokens, {} English tokens",
                        t.fr_tokens.len(),
                        t.en_tokens.len()
                    );
                    let provider_used = t.provider_used.clone();
                    let payload = TokenizedPayload {
                        fr_tokens: t.fr_tokens.into_iter().map(|ft| FrToken {
                            text: ft.text,
                            trailing_punct: if ft.trailing_punct.is_empty() { None } else { Some(ft.trailing_punct) },
                            leading_punct: if ft.leading_punct.is_empty() { None } else { Some(ft.leading_punct) },
                            translation: ft.translation,
                            spans: ft.spans.into_iter().map(|s| s.into()).collect(),
                            en_indices: ft.en_indices,
                        }).collect(),
                        en_tokens: t.en_tokens.into_iter().map(|et| EnToken {
                            text: et.text,
                            index: et.index,
                        }).collect(),
                        provider: provider_used,
                    };
                    (Some(payload), None)
                }
                Err(e) => {
                    tracing::error!("Tokenization failed after all providers: {}", e);
                    (None, Some(format!("Translation temporarily unavailable. Showing French text only.")))
                }
            };

            let book_title = book.title.clone();

            let response = ArticleResponse {
                title: book_title.clone(),
                source: book.collection,
                published_year: book.published_year,
                paragraphs,
                tokenized,
                tokenization_error,
            };

            {
                let mut cached = state.cached_article.write().await;
                *cached = Some(CachedArticle {
                    response,
                    computed_at: std::time::Instant::now(),
                });
            }

            tracing::info!("Article-of-the-day cached: {}", book_title);
        }
        Err(e) => {
            tracing::error!("Failed to fetch French text for {}: {}", book.gutenberg_id, e);
        }
    }
}