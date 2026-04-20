use axum::{routing::get, Router};

pub mod article;
pub mod stories;

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/article-of-the-day", get(article::article_of_the_day))
        .route("/api/stories", get(stories::list_stories))
        .route("/api/stories/:id", get(stories::get_story))
}

async fn health() -> &'static str {
    "OK"
}
