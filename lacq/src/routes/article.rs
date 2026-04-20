use crate::{ArticleResponse, EnglishContent, FrenchContent, StoryMeta};
use axum::Json;

pub async fn article_of_the_day() -> Json<ArticleResponse> {
    Json(ArticleResponse {
        story: StoryMeta {
            id: "mock-001".to_string(),
            title: "La Maison Tellier".to_string(),
            source: "Contes du jour et de la nuit".to_string(),
            published_year: 1885,
        },
        french: FrenchContent {
            paragraphs: vec![
                "Mock paragraph 1 in French".to_string(),
                "Mock paragraph 2 in French".to_string(),
            ],
            vocab_highlights: vec![],
        },
        english: Some(EnglishContent {
            paragraphs: vec!["[MOCK TRANSLATION]".to_string()],
            source: "mock".to_string(),
        }),
        date: "2026-04-21".to_string(),
    })
}
