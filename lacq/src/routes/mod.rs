use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::sync::Arc;

use crate::{AppState, ArticleResponse};
use crate::coach::{self, LearnerContext};
use crate::exercises::gen::{self, ItemType, ALL_TYPES};
use crate::exercises::{corpus, families, topics, ExercisesResponse};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health))
        .route("/api/article-of-the-day", get(article_of_the_day))
        .route("/api/exercises", get(exercises))
        .route("/api/exercises/topics", get(list_topics))
        .route("/api/exercises/feed", post(feed))
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
            if !corpus::ALL_LEVELS.contains(&l.as_str()) {
                return Err(bad(format!(
                    "invalid level {:?} (expected one of {:?})",
                    l, corpus::ALL_LEVELS
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

/* ── Progress grid: the topic catalog, learner-facing ───────────── */

#[derive(serde::Serialize)]
struct TopicEntry {
    slug: &'static str,
    title: &'static str,
    blurb: &'static str,
    levels: &'static [&'static str],
    section: topics::Section,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct TopicsResponse {
    topics: Vec<TopicEntry>,
    /// Curated corpus title → catalog slug, so the client's fixed-session
    /// history (keyed by curated title) rolls up under catalog topics.
    curated_titles: std::collections::BTreeMap<&'static str, &'static str>,
}

/// The topic catalog with its grid placement. Server-owned pedagogy:
/// sections, level spans and the title roll-up all come from here.
async fn list_topics() -> axum::Json<TopicsResponse> {
    axum::Json(TopicsResponse {
        topics: topics::TOPICS
            .iter()
            .map(|t| TopicEntry {
                slug: t.slug,
                title: t.title,
                blurb: t.blurb,
                levels: t.levels,
                section: t.section,
            })
            .collect(),
        curated_titles: topics::CURATED_TITLES.iter().copied().collect(),
    })
}

/* ── Endless feed: coach decides the topic, generators fill the run ── */

/// Items per coach decision — one run drills one topic.
const FEED_RUN_LEN: usize = 5;
/// Cap on the client's recent-item list we honor for dedupe.
const FEED_RECENT_CAP: usize = 40;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct FeedResponse {
    plan: coach::DrillPlan,
    items: Vec<crate::exercises::ExerciseItem>,
    run_length: usize,
}

async fn feed(
    State(state): State<Arc<AppState>>,
    axum::Json(ctx): axum::Json<LearnerContext>,
) -> Result<axum::Json<FeedResponse>, (StatusCode, axum::Json<ErrorResponse>)> {
    let plan = coach::decide(state.config.typesafe_api_key.as_deref(), &ctx).await;

    let topic = topics::by_slug(&plan.topic)
        .ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(ErrorResponse {
            error: format!("plan picked unknown topic {}", plan.topic),
        })))?;

    let recent: std::collections::HashSet<String> = ctx
        .recent_items
        .iter()
        .take(FEED_RECENT_CAP)
        .cloned()
        .collect();

    let seed = gen::time_seed();
    // Family first; curated corpus covers topics without one; anything
    // at all keeps the feed moving rather than erroring mid-session.
    let items = families::generate(topic, Some(&plan.level), FEED_RUN_LEN, &recent, seed);
    let items = if items.is_empty() {
        gen::curated_for_topic(topic.title, FEED_RUN_LEN, seed)
    } else {
        items
    };

    Ok(axum::Json(FeedResponse {
        plan,
        run_length: items.len(),
        items,
    }))
}
