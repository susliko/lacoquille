use crate::AppState;
use axum::{extract::State, Json, http::StatusCode, response::IntoResponse};
use sha2::{Sha256, Digest};
use std::path::Path;
use std::sync::Arc;

#[derive(serde::Deserialize)]
pub struct TtsRequest {
    pub text: String,
}

pub async fn text_to_speech(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TtsRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let text = payload.text.trim();
    if text.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let cache_dir = Path::new(&state.data_dir).join("tts_cache");
    let hash =Sha256::digest(text);
    let hash_hex = hex::encode(hash);
    let cache_path = cache_dir.join(format!("{}.mp3", hash_hex));

    // Return cached file if it exists
    if cache_path.exists() {
        tracing::debug!("TTS cache hit for text hash {}", hash_hex);
        let data = tokio::fs::read(&cache_path).await
            .map_err(|e| {
                tracing::error!("Failed to read cached TTS file: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        return Ok((
            [(axum::http::header::CONTENT_TYPE, "audio/mpeg")],
            data,
        ));
    }

    // Create cache directory if it doesn't exist
    tokio::fs::create_dir_all(&cache_dir).await
        .map_err(|e| {
            tracing::error!("Failed to create TTS cache directory: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Call Minimax TTS API
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

    let minimax_base_url = state.config.minimax_base_url.clone();
    let api_key = state.config.minimax_api_key.clone();

    let resp = state.http
        .post(format!("{}/t2a_v2", minimax_base_url))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("TTS API request failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        tracing::error!("TTS API returned error {}: {}", status, body);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let bytes = resp.bytes().await
        .map_err(|e| {
            tracing::error!("Failed to read TTS response bytes: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Cache the audio file
    tokio::fs::write(&cache_path, &bytes).await
        .map_err(|e| {
            tracing::error!("Failed to write TTS cache file: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    tracing::info!("Generated and cached TTS audio for hash {}", hash_hex);

    Ok((
        [(axum::http::header::CONTENT_TYPE, "audio/mpeg")],
        bytes.to_vec(),
    ))
}
