use axum::{routing::{get, post}, Router};
use std::sync::Arc;

pub mod stories;
pub mod tts;

use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health))
        .route("/api/stories", get(stories::list_stories))
        .route("/api/stories/:id", get(stories::get_story))
        .route("/api/stories/:id/tts-cache", post(stories::tts_cache))
        .route("/api/tts", post(tts::text_to_speech))
}

async fn health() -> &'static str {
    "OK"
}