use crate::AppState;
use axum::{extract::State, Json, http::StatusCode, response::IntoResponse};
use sha2::{Sha256, Digest};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug)]
pub enum TtsError {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
    ApiError { status: reqwest::StatusCode, body: String },
}

impl From<reqwest::Error> for TtsError {
    fn from(e: reqwest::Error) -> Self { TtsError::Reqwest(e) }
}

impl From<std::io::Error> for TtsError {
    fn from(e: std::io::Error) -> Self { TtsError::Io(e) }
}

/// Generate TTS audio for the given text and cache it.
/// Returns the bytes of the audio file. If already cached, returns cached bytes.
/// Does not return an HTTP response — just the audio bytes.
pub async fn generate_tts(
    http: &reqwest::Client,
    config: &crate::config::Config,
    cache_dir: &Path,
    text: &str,
) -> Result<Vec<u8>, TtsError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(TtsError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, "empty text")));
    }

    let hash = Sha256::digest(text);
    let hash_hex = hex::encode(hash);
    let cache_path = cache_dir.join(format!("{}.mp3", hash_hex));

    if cache_path.exists() {
        tracing::debug!("TTS cache hit for text hash {}", hash_hex);
        return tokio::fs::read(&cache_path).await.map_err(Into::into);
    }

    tokio::fs::create_dir_all(cache_dir).await?;

    #[derive(serde::Serialize)]
    struct MinimaxTtsRequest {
        model: String,
        text: String,
        stream: bool,
        voice_setting: serde_json::Value,
    }

    let request_body = MinimaxTtsRequest {
        model: "speech-02-hd".to_string(),
        text: text.to_string(),
        stream: false,
        voice_setting: serde_json::json!({
            "voice_id": "male-qn-qingse",
            "speed": 0.8,
            "pitch": 0,
            "volume": 0,
            "output_format": "mp3"
        }),
    };

    let resp = http
        .post(format!("{}/t2a_v2", config.minimax_base_url.clone()))
        .header("Authorization", format!("Bearer {}", config.minimax_api_key.clone()))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        tracing::error!("TTS API returned error {}: {}", status, body);
        return Err(TtsError::ApiError { status, body });
    }

    let bytes = resp.bytes().await?;

    tokio::fs::write(&cache_path, &bytes).await?;
    tracing::info!("Generated and cached TTS audio for hash {}", hash_hex);

    Ok(bytes.to_vec())
}

#[derive(serde::Deserialize)]
pub struct TtsRequest {
    pub text: String,
}

pub async fn text_to_speech(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TtsRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let cache_dir = Path::new(&state.data_dir).join("tts_cache");
    let bytes = generate_tts(&state.http, &state.config, &cache_dir, &payload.text)
        .await
        .map_err(|e| {
            tracing::error!("TTS generation failed: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((
        [(axum::http::header::CONTENT_TYPE, "audio/mpeg")],
        bytes,
    ))
}
