use crate::{AppState, ArticleResponse, FrenchContent, StoryMeta};
use axum::{extract::State, Json};
use std::sync::Arc;

pub async fn article_of_the_day(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ArticleResponse>, axum::http::StatusCode> {
    let book = crate::BookMeta::for_today();

    let french_text = state.get_french_text(book.gutenberg_id).await
        .map_err(|e| {
            tracing::error!("Failed to fetch French text for book {}: {}", book.gutenberg_id, e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let paragraphs_raw = crate::gutenberg::split_paragraphs(&french_text);
    let excerpt_paras = crate::gutenberg::extract_excerpt(&paragraphs_raw, 350);
    let excerpt_strings: Vec<String> = excerpt_paras.iter().map(|s| s.to_string()).collect();

    let vocab: Vec<&str> = excerpt_paras.iter().map(|s| s.as_ref()).collect();
    let vocab = crate::vocab::extract_vocab_words(&vocab);

    let english = if let Some(eng_id) = book.english_gutenberg_id {
        if let Some(text) = state.get_english_text(eng_id).await {
            let eng_paragraphs = crate::gutenberg::split_paragraphs(&text);
            let eng_excerpt = crate::gutenberg::extract_excerpt(&eng_paragraphs, 350);
            let eng_strings: Vec<String> = eng_excerpt.iter().map(|s| s.to_string()).collect();
            Some(crate::EnglishContent {
                paragraphs: eng_strings,
                source: format!("gutenberg:{}", eng_id),
            })
        } else {
            None
        }
    } else {
        None
    };

    if english.is_none() {
        if let Some(llm) = &state.llm {
            let text = excerpt_strings.join("\n\n");
            let translated = llm.translate(&text).await
                .map_err(|e| {
                    tracing::error!("LLM translation failed: {}", e);
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR
                })?;
            let eng_content = crate::EnglishContent {
                paragraphs: translated.split("\n\n").map(|s| s.to_string()).collect(),
                source: "llm".to_string(),
            };
            return Ok(Json(ArticleResponse {
                story: StoryMeta {
                    id: book.gutenberg_id.to_string(),
                    title: book.title,
                    source: book.collection,
                    published_year: book.published_year,
                },
                french: FrenchContent {
                    paragraphs: excerpt_strings,
                    vocab_highlights: vocab,
                },
                english: Some(eng_content),
                date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            }));
        }
    }

    Ok(Json(ArticleResponse {
        story: StoryMeta {
            id: book.gutenberg_id.to_string(),
            title: book.title,
            source: book.collection,
            published_year: book.published_year,
        },
        french: FrenchContent {
            paragraphs: excerpt_strings,
            vocab_highlights: vocab,
        },
        english,
        date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
    }))
}
