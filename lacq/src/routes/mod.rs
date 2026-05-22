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
    let excerpt_paras = crate::gutenberg::extract_excerpt(&paragraphs_raw, 150);
    let paragraphs: Vec<String> = excerpt_paras.iter().map(|s| s.to_string()).collect();

    // Combine paragraphs into one text for tokenization
    let combined_text = paragraphs.join("\n\n");

    // Tokenize via Gemini — fail fast so we see the real error
    let tokenized = match state.get_tokenized(book.gutenberg_id, &combined_text).await {
        Ok(t) => {
            tracing::info!(
                "Tokenized {} French tokens, {} English tokens",
                t.fr_tokens.len(),
                t.en_tokens.len()
            );
            Some(TokenizedPayload {
                fr_tokens: t.fr_tokens.into_iter().map(|ft| FrToken {
                    text: ft.text,
                    translation: ft.translation,
                    spans: ft.spans.into_iter().map(|s| s.into()).collect(),
                    en_indices: ft.en_indices,
                }).collect(),
                en_tokens: t.en_tokens.into_iter().map(|et| EnToken {
                    text: et.text,
                    index: et.index,
                }).collect(),
            })
        }
        Err(e) => {
            tracing::error!("Tokenization failed: {}", e);
            None
        }
    };

    Ok(axum::Json(ArticleResponse {
        title: book.title,
        source: book.collection,
        published_year: book.published_year,
        paragraphs,
        tokenized,
    }))
}
