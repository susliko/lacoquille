pub mod config;
pub mod gutendex;
pub mod gutenberg;
pub mod llm;
pub mod vocab;
pub mod translation;
pub mod routes;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::llm::LlmProvider;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Book {
    pub id: u64,
    pub title: String,
    pub authors: Vec<String>,
    pub languages: Vec<String>,
    pub download_count: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Story {
    pub id: String,
    pub title: String,
    pub source: String,
    pub book_id: u64,
    pub published_year: i32,
    pub level: String,
    pub paragraphs: Vec<String>,
    pub excerpt_paragraphs: Vec<usize>,
    pub has_english_translation: bool,
    pub english_book_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StoryListItem {
    pub id: String,
    pub title: String,
    pub source: String,
    pub published_year: i32,
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleResponse {
    pub story: StoryMeta,
    pub french: FrenchContent,
    pub english: Option<EnglishContent>,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryMeta {
    pub id: String,
    pub title: String,
    pub source: String,
    pub published_year: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrenchContent {
    pub paragraphs: Vec<String>,
    pub vocab_highlights: Vec<VocabHighlight>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabHighlight {
    pub word: String,
    pub paragraph_index: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

impl VocabHighlight {
    pub fn new(word: String, paragraph_index: usize, start_offset: usize, end_offset: usize) -> Self {
        Self { word, paragraph_index, start_offset, end_offset }
    }
}

#[derive(Clone)]
pub struct BookMeta {
    pub gutenberg_id: u64,
    pub title: String,
    pub collection: String,
    pub published_year: i32,
    pub english_gutenberg_id: Option<u64>,
}

pub struct CachedText {
    pub text: String,
    pub fetched_at: std::time::Instant,
}

impl BookMeta {
    pub fn curated() -> Vec<Self> {
        vec![
            BookMeta {
                gutenberg_id: 10775, // Le Horla
                title: "Le Horla".to_string(),
                collection: "Contes du jour et de la nuit".to_string(),
                published_year: 1887,
                english_gutenberg_id: None,
            },
            BookMeta {
                gutenberg_id: 14790, // Contes du jour et de la nuit
                title: "Contes du jour et de la nuit".to_string(),
                collection: "Contes du jour et de la nuit".to_string(),
                published_year: 1884,
                english_gutenberg_id: None,
            },
            BookMeta {
                gutenberg_id: 10746, // Boule de Suif
                title: "Boule de Suif".to_string(),
                collection: "Les Rougon-Macquart".to_string(),
                published_year: 1880,
                english_gutenberg_id: None,
            },
            BookMeta {
                gutenberg_id: 12011, // Monsieur Parent
                title: "Monsieur Parent".to_string(),
                collection: "Nouvelles".to_string(),
                published_year: 1885,
                english_gutenberg_id: None,
            },
            BookMeta {
                gutenberg_id: 11131, // Pierre et Jean
                title: "Pierre et Jean".to_string(),
                collection: "Roman".to_string(),
                published_year: 1888,
                english_gutenberg_id: None,
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

#[derive(Clone)]
pub struct AppState {
    pub curated_books: Vec<BookMeta>,
    pub french_cache: std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<u64, CachedText>>>,
    pub http: reqwest::Client,
    pub llm: Option<std::sync::Arc<dyn LlmProvider>>,
    pub data_dir: String,
}

impl AppState {
    pub fn new(http: reqwest::Client, llm: Option<Arc<dyn LlmProvider>>, data_dir: String) -> Self {
        Self {
            curated_books: BookMeta::curated(),
            french_cache: std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
            http,
            llm,
            data_dir,
        }
    }

    pub async fn get_french_text(&self, book_id: u64) -> Result<String, String> {
        {
            let cache = self.french_cache.lock().await;
            if let Some(cached) = cache.get(&book_id) {
                let age = cached.fetched_at.elapsed().as_secs();
                if age < 3600 {
                    return Ok(cached.text.clone());
                }
            }
        }

        let url = format!("https://www.gutenberg.org/files/{}/{}-0.txt", book_id, book_id);
        let text = self.http.get(&url).send().await
            .map_err(|e| format!("failed to fetch {}: {}", url, e))?
            .text().await
            .map_err(|e| format!("failed to read response: {}", e))?;

        let cleaned = crate::gutenberg::clean_text(&text);
        let cleaned = cleaned.trim().to_string();

        {
            let mut cache = self.french_cache.lock().await;
            cache.insert(book_id, CachedText {
                text: cleaned.clone(),
                fetched_at: std::time::Instant::now(),
            });
        }

        Ok(cleaned)
    }

    pub async fn get_english_text(&self, book_id: u64) -> Option<String> {
        let url = format!("https://www.gutenberg.org/files/{}/{}-0.txt", book_id, book_id);
        let text = self.http.get(&url).send().await.ok()?.text().await.ok()?;
        Some(crate::gutenberg::clean_text(&text).trim().to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnglishContent {
    pub paragraphs: Vec<String>,
    pub source: String,
}
