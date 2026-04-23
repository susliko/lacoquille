use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let _config = lacq::config::Config::from_env();

    let state = Arc::new(lacq::AppState::new(
        reqwest::Client::new(),
        None,
        "data".to_string(),
    ));

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/article-of-the-day", get(lacq::routes::article::article_of_the_day))
        .route("/api/stories", get(lacq::routes::stories::list_stories))
        .route("/api/stories/:id", get(lacq::routes::stories::get_story))
        .nest_service("/", ServeDir::new("dist"))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "OK"
}
