use crate::{AppState, EnglishContent, FrenchContent, StoryMeta};
use axum::{extract::Path, extract::State, Json};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct StoryListItem {
    pub id: String,
    pub title: String,
    pub source: String,
    pub published_year: i32,
    pub level: String,
}

#[derive(Serialize)]
pub struct StoryResponse {
    pub story: StoryMeta,
    pub french: FrenchContent,
    pub english: Option<EnglishContent>,
}

pub async fn list_stories(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<StoryListItem>> {
    let stories = state.curated_books.iter().map(|book| {
        StoryListItem {
            id: book.gutenberg_id.to_string(),
            title: book.title.clone(),
            source: book.collection.clone(),
            published_year: book.published_year,
            level: "B1".to_string(),
        }
    }).collect();
    Json(stories)
}

pub async fn get_story(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<StoryResponse>, axum::http::StatusCode> {
    let gutenberg_id: u64 = id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    let book = state.curated_books.iter()
        .find(|b| b.gutenberg_id == gutenberg_id)
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    let french_text = state.get_french_text(book.gutenberg_id).await
        .map_err(|e| {
            tracing::error!("Failed to fetch French text for book {}: {}", book.gutenberg_id, e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let story_text = crate::gutenberg::first_story_content(&french_text);
    let paragraphs_raw = crate::gutenberg::split_paragraphs(&story_text);
    let excerpt_paras = crate::gutenberg::extract_excerpt(&paragraphs_raw, 350);
    let excerpt_strings: Vec<String> = excerpt_paras.iter().map(|s| s.to_string()).collect();

    let vocab: Vec<&str> = excerpt_paras.iter().map(|s| s.as_ref()).collect();
    let vocab = crate::vocab::extract_vocab_words(&vocab);

    let english = if let Some(eng_id) = book.english_gutenberg_id {
        if let Some(text) = state.get_english_text(eng_id).await {
            let eng_paragraphs = crate::gutenberg::split_paragraphs(&text);
            let eng_excerpt = crate::gutenberg::extract_excerpt(&eng_paragraphs, 350);
            let eng_strings: Vec<String> = eng_excerpt.iter().map(|s| s.to_string()).collect();
            Some(EnglishContent {
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
            let eng_content = EnglishContent {
                paragraphs: translated.split("\n\n").map(|s| s.to_string()).collect(),
                source: "llm".to_string(),
            };
            return Ok(Json(StoryResponse {
                story: StoryMeta {
                    id: book.gutenberg_id.to_string(),
                    title: book.title.clone(),
                    source: book.collection.clone(),
                    published_year: book.published_year,
                },
                french: FrenchContent {
                    paragraphs: excerpt_strings,
                    vocab_highlights: vocab,
                },
                english: Some(eng_content),
            }));
        }
    }

    Ok(Json(StoryResponse {
        story: StoryMeta {
            id: book.gutenberg_id.to_string(),
            title: book.title.clone(),
            source: book.collection.clone(),
            published_year: book.published_year,
        },
        french: FrenchContent {
            paragraphs: excerpt_strings,
            vocab_highlights: vocab,
        },
        english,
    }))
}