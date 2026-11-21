use std::net::SocketAddr;
use std::sync::Arc;
use axum::{extract::Request, http::header::{CACHE_CONTROL, HeaderValue}, middleware::{self, Next}, response::Response};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Cache policy for static files served from dist/.
///
/// Pages inline their CSS, so a stale cached page is a stale layout — HTML
/// must always revalidate (no-cache = store + conditional GET; ServeDir
/// answers with 304 via Last-Modified). Hashed assets (/_astro/, /fonts/)
/// are content-addressed: cache them forever. API responses are untouched.
async fn cache_policy(req: Request, next: Next) -> Response {
    let path = req.uri().path().to_owned();
    let mut res = next.run(req).await;
    if !path.starts_with("/api/") && path != "/health" {
        let immutable = path.starts_with("/_astro/") || path.starts_with("/fonts/");
        let value = if immutable {
            "public, max-age=31536000, immutable"
        } else {
            "no-cache"
        };
        res.headers_mut().insert(CACHE_CONTROL, HeaderValue::from_static(value));
    }
    res
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = lacq::config::Config::from_env();

    let http = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout_secs()))
        .build()
        .expect("reqwest client");

    let data_dir = config.data_dir.clone();
    let state = Arc::new(lacq::AppState::new(http, data_dir, config));

    let app = lacq::routes::routes()
        .with_state(Arc::clone(&state))
        .nest_service("/", ServeDir::new("dist").append_index_html_on_directories(true))
        .layer(middleware::from_fn(cache_policy));

    // Start background workers for article-of-the-day computation
    state.start_background_workers();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
