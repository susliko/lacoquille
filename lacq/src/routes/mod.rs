use axum::{extract::State, routing::get, Router};
use std::sync::Arc;

use crate::{AppState, ArticleResponse, BookMeta, EnToken, FrToken, TokenizedPayload};

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
) -> Result<axum::Json<ArticleResponse>, axum::http::StatusCode> {
    let book = BookMeta::for_today();

    let text = state.get_french_text(book.gutenberg_id).await.map_err(|e| {
        tracing::error!("Failed to fetch text for book {}: {}", book.gutenberg_id, e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let story_text = crate::gutenberg::first_story_content(&text);
    let paragraphs_raw = crate::gutenberg::split_paragraphs(&story_text);
    let paragraphs = crate::gutenberg::extract_excerpt(&paragraphs_raw, 10); // ~10 words, Groq has 6000 TPM limit
    
    // Additional safety: if combined text is too long, truncate to ~300 chars
    let mut combined_text = paragraphs.join("\n\n");
    if combined_text.len() > 500 {
        combined_text.truncate(300);
    }

    // Tokenize with fallback chain
    let (tokenized, tokenization_error) = match state.get_tokenized(book.gutenberg_id, &combined_text).await {
        Ok(t) => {
            tracing::info!(
                "Tokenized {} French tokens, {} English tokens",
                t.fr_tokens.len(),
                t.en_tokens.len()
            );
            let provider_used = t.provider_used.clone();
            let payload = TokenizedPayload {
                fr_tokens: t.fr_tokens.into_iter().map(|ft| FrToken {
                    text: ft.text,
                    trailing_punct: if ft.trailing_punct.is_empty() { None } else { Some(ft.trailing_punct) },
                    leading_punct: if ft.leading_punct.is_empty() { None } else { Some(ft.leading_punct) },
                    translation: ft.translation,
                    spans: ft.spans.into_iter().map(|s| s.into()).collect(),
                    en_indices: ft.en_indices,
                }).collect(),
                en_tokens: t.en_tokens.into_iter().map(|et| EnToken {
                    text: et.text,
                    index: et.index,
                }).collect(),
                provider: provider_used,
            };
            if let Some(ref provider) = payload.provider {
                tracing::info!("Translation provider: {}", provider);
            }
            (Some(payload), None)
        }
        Err(e) => {
            tracing::error!("Tokenization failed after all providers: {}", e);
            (None, Some(format!("Translation temporarily unavailable. Showing French text only.")))
        }
    };

    Ok(axum::Json(ArticleResponse {
        title: book.title,
        source: book.collection,
        published_year: book.published_year,
        paragraphs,
        tokenized,
        tokenization_error,
    }))
}
