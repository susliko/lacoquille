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

    let config = lacq::config::Config::from_env();
    let api_key = config.minimax_api_key.clone();
    let base_url = config.minimax_base_url.clone();

    let http = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("reqwest client");

    let llm = if api_key != "mock-key" && !api_key.is_empty() {
        Some(Arc::new(lacq::llm::MinimaxProvider::new(
            api_key,
            base_url,
        )) as Arc<dyn lacq::llm::LlmProvider>)
    } else {
        None
    };

    let data_dir = config.data_dir.clone();
    let state = Arc::new(lacq::AppState::new(http, llm, data_dir, config));

    let app = lacq::routes::routes()
        .with_state(state)
        .nest_service("/", ServeDir::new("dist").append_index_html_on_directories(true));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}