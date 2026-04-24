use axum::{routing::{get, post}, Router};
use std::sync::Arc;

pub mod article;
pub mod stories;
pub mod tts;

use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health))
        .route("/api/article-of-the-day", get(article::article_of_the_day))
        .route("/api/stories", get(stories::list_stories))
        .route("/api/stories/:id", get(stories::get_story))
        .route("/api/tts", post(tts::text_to_speech))
}

async fn health() -> &'static str {
    "OK"
}