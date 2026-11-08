//! Exercise generation — implements the exercise item shapes consumed by
//! the frontend (`src/components/exercise/data.ts`). The endpoint is a
//! pure, stateless generator: no sessions, no users, no writes. All
//! learner state lives in the client (localStorage).
//!
//! - `corpus` — the curated French template pools
//! - `gen`    — seeded selection + per-type assembly

pub mod corpus;
pub mod gen;

use serde::Serialize;

/// A single exercise item. Serialized with a `kind` tag so the client's
/// discriminated union parses it directly (`camelCase` field names).
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ExerciseItem {
    Cloze(ClozeItem),
    Choice(ChoiceItem),
    Tiles(TilesItem),
    Transform(TransformItem),
    ErrorSpot(ErrorSpotItem),
    Dictation(DictationItem),
}

impl ExerciseItem {
    pub fn id(&self) -> &str {
        match self {
            ExerciseItem::Cloze(x) => &x.id,
            ExerciseItem::Choice(x) => &x.id,
            ExerciseItem::Tiles(x) => &x.id,
            ExerciseItem::Transform(x) => &x.id,
            ExerciseItem::ErrorSpot(x) => &x.id,
            ExerciseItem::Dictation(x) => &x.id,
        }
    }
}

fn strs(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClozeItem {
    pub id: String,
    pub level: &'static str,
    pub topic: &'static str,
    pub task: Vec<String>,
    pub head: String,
    pub tail: String,
    pub infinitive: &'static str,
    pub answer: String,
    pub rule: Vec<String>,
}

impl ClozeItem {
    pub(crate) fn from_template(
        t: &corpus::ClozeTemplate,
        p: &corpus::ClozePerson,
    ) -> Self {
        Self {
            id: format!("{}-{}", t.id, p.key),
            level: t.level,
            topic: t.topic,
            task: strs(t.task),
            head: p.head.to_string(),
            tail: p.tail.to_string(),
            infinitive: t.infinitive,
            answer: p.answer.to_string(),
            rule: strs(t.rule),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChoiceItem {
    pub id: String,
    pub level: &'static str,
    pub topic: &'static str,
    pub task: Vec<String>,
    pub sentence: &'static str,
    pub options: Vec<String>,
    pub correct: usize,
    pub rule: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TilesItem {
    pub id: String,
    pub level: &'static str,
    pub topic: &'static str,
    pub task: Vec<String>,
    pub prompt: &'static str,
    pub target: Vec<String>,
    pub bank: Vec<String>,
    pub rule: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformItem {
    pub id: String,
    pub level: &'static str,
    pub topic: &'static str,
    pub task: Vec<String>,
    pub source: &'static str,
    pub answer: &'static str,
    pub rule: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorSpotItem {
    pub id: String,
    pub level: &'static str,
    pub topic: &'static str,
    pub task: Vec<String>,
    pub words: Vec<String>,
    pub wrong: usize,
    pub fix: &'static str,
    pub fix_task: &'static str,
    pub rule: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DictationItem {
    pub id: String,
    pub level: &'static str,
    pub topic: &'static str,
    pub task: Vec<String>,
    pub speech: &'static str,
    pub answer: &'static str,
    pub rule: Vec<String>,
}

/// Response envelope for `GET /api/exercises`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExercisesResponse {
    pub items: Vec<ExerciseItem>,
    pub seed: u64,
    pub count: usize,
}
