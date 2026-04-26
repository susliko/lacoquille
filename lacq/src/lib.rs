pub mod config;
pub mod routes;
pub mod gutenberg;

pub use config::Config;

use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

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

#[derive(Clone)]
pub struct AppState {
    pub curated_books: Vec<BookMeta>,
    pub text_cache: std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<u64, CachedText>>>,
    pub http: reqwest::Client,
    pub data_dir: String,
    pub config: Config,
}

impl AppState {
    pub fn new(http: reqwest::Client, data_dir: String, config: Config) -> Self {
        Self {
            curated_books: BookMeta::curated(),
            text_cache: std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
            http,
            data_dir,
            config,
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
        let text = self.http.get(&url).send().await
            .map_err(|e| format!("failed to fetch {}: {}", url, e))?
            .text().await
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
}

#[derive(Debug, Clone, Serialize)]
pub struct ArticleResponse {
    pub title: String,
    pub source: String,
    pub published_year: i32,
    pub paragraphs: Vec<String>,
}
