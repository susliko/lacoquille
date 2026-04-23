use axum::{routing::get, Router};
use std::sync::Arc;

pub mod article;
pub mod stories;

use crate::AppState;

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/article-of-the-day", get(article::article_of_the_day))
        .route("/api/stories", get(stories::list_stories))
        .route("/api/stories/:id", get(stories::get_story))
        .with_state(Arc::new(AppState::new(
            reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("reqwest client"),
            None,
            std::env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string()),
        )))
}

async fn health() -> &'static str {
    "OK"
}

async fn health() -> &'static str {
    "OK"
}