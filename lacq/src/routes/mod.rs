use axum::{extract::State, routing::get, Router, http::StatusCode};
use std::sync::Arc;

use crate::{AppState, ArticleResponse};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health))
        .route("/api/article-of-the-day", get(article_of_the_day))
}

async fn health() -> &'static str {
    "OK"
}

async fn article_of_the_day(
    State(state): State<Arc<AppState>>,
) -> Result<axum::Json<ArticleResponse>, StatusCode> {
    let cached = state.cached_article.read().await;
    match &*cached {
        Some(c) => Ok(axum::Json(c.response.clone())),
        None => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}
