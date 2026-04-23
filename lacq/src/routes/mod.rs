use axum::{routing::get, Router};
use std::sync::Arc;

pub mod article;
pub mod stories;

use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health))
        .route("/api/article-of-the-day", get(article::article_of_the_day))
        .route("/api/stories", get(stories::list_stories))
        .route("/api/stories/:id", get(stories::get_story))
}

async fn health() -> &'static str {
    "OK"
}