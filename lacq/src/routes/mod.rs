use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Router,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::sync::Arc;

use crate::{AppState, ArticleResponse};
use crate::exercises::gen::{self, ItemType, ALL_TYPES};
use crate::exercises::ExercisesResponse;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health))
        .route("/api/article-of-the-day", get(article_of_the_day))
        .route("/api/exercises", get(exercises))
}

async fn health() -> &'static str {
    "OK"
}

async fn article_of_the_day(
    State(state): State<Arc<AppState>>,
) -> Result<axum::Json<ArticleResponse>, StatusCode> {
    let cached = state.cached_article.read().await;
    match &*cached {
        Some(c) => Ok(axum::Json(c.response.clone())),
        None => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

/* ── Exercise generation (stateless) ────────────────────────────── */

#[derive(Deserialize)]
struct ExerciseQuery {
    count: Option<String>,
    types: Option<String>,
    level: Option<String>,
    seed: Option<String>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

/// Parse a numeric query param without axum's rejection handling, so a
/// bad value becomes a 400 with a message instead of a blanket 400.
fn parse_num<T: DeserializeOwned + std::str::FromStr>(raw: &str, name: &str) -> Result<T, String> {
    serde_json::from_str(raw).or_else(|_| {
        raw.parse::<T>()
            .map_err(|_| format!("invalid {} parameter: {:?}", name, raw))
    })
}

async fn exercises(
    Query(q): Query<ExerciseQuery>,
) -> Result<axum::Json<ExercisesResponse>, (StatusCode, axum::Json<ErrorResponse>)> {
    let bad = |msg: String| {
        (
            StatusCode::BAD_REQUEST,
            axum::Json(ErrorResponse { error: msg }),
        )
    };

    let count = match &q.count {
        Some(raw) => parse_num::<usize>(raw, "count").map_err(bad)?.max(1),
        None => 6,
    };

    let types: Vec<ItemType> = match &q.types {
        Some(raw) => raw
            .split(',')
            .filter_map(ItemType::parse)
            .collect::<Vec<_>>(),
        None => Vec::new(),
    };
    let types = if types.is_empty() { ALL_TYPES.to_vec() } else { types };

    let level = match &q.level {
        Some(l) if !l.is_empty() => {
            let l = l.trim().to_ascii_uppercase();
            if !exercises::corpus::ALL_LEVELS.contains(&l.as_str()) {
                return Err(bad(format!(
                    "invalid level {:?} (expected one of {:?})",
                    l, exercises::corpus::ALL_LEVELS
                )));
            }
            Some(l)
        }
        _ => None,
    };

    let seed = match &q.seed {
        Some(raw) => parse_num::<u64>(raw, "seed").map_err(bad)?,
        None => gen::time_seed(),
    };

    let req = gen::GenRequest { count, types, level, seed };
    match gen::generate(&req) {
        Ok(items) => {
            let count = items.len();
            Ok(axum::Json(ExercisesResponse { items, seed, count }))
        }
        Err(e) => Err(bad(e)),
    }
}

use crate::exercises;
