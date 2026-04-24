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

    let http = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("reqwest client");

    let llm = if config.minimax_api_key != "mock-key" && !config.minimax_api_key.is_empty() {
        Some(Arc::new(lacq::llm::MinimaxProvider::new(
            config.minimax_api_key,
            config.minimax_base_url,
        )) as Arc<dyn lacq::llm::LlmProvider>)
    } else {
        None
    };

    let state = Arc::new(lacq::AppState::new(http, llm, config.data_dir));

    let app = lacq::routes::routes()
        .with_state(state)
        .nest_service("/", ServeDir::new("dist").append_index_html_on_directories(true));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}