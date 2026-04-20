use axum::{extract::Path, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct StoryListItem {
    pub id: String,
    pub title: String,
    pub source: String,
    pub published_year: i32,
    pub level: String,
}

pub async fn list_stories() -> Json<Vec<StoryListItem>> {
    Json(vec![])
}

pub async fn get_story(Path(id): Path<String>) -> Json<serde_json::Value> {
    let _id = id;
    Json(serde_json::json!({ "error": "not implemented" }))
}
