//! The coach — decides what to drill next.
//!
//! Domain contract, deliberately narrow:
//!   * the client sends **only** the learner's own context (level
//!     estimate, per-topic history, recent activity);
//!   * everything pedagogical — the topic catalog, how topics are
//!     described, which levels exist — is attached here, server-side;
//!   * the decision comes back as a `DrillPlan` (topic, level,
//!     confidence).
//!
//! How the decision is made is an implementation detail: an external
//! decision model when a key is configured, a built-in heuristic when
//! it is not, fails, or answers outside the catalog. The route and the
//! client cannot tell which produced the plan beyond `source`.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::exercises::topics::{self, Topic};

/// Exactly what the client sends — its own state, nothing else.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LearnerContext {
    /// The client's current CEFR estimate for the learner.
    pub level: Option<String>,
    /// Per-topic aggregates, keyed by catalog slug.
    #[serde(default)]
    pub topics: Vec<TopicStat>,
    /// Topics drilled in the recent past (slugs), most recent last.
    #[serde(default)]
    pub recent_topics: Vec<String>,
    /// Recently served item ids (any length; capped by the client).
    #[serde(default)]
    pub recent_items: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicStat {
    pub slug: String,
    pub attempts: u32,
    pub correct: u32,
    /// Unix seconds of the last attempt on this topic.
    pub last_at: i64,
}

/// What to drill next.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DrillPlan {
    pub topic: String,
    pub topic_title: String,
    pub level: String,
    /// 0..1 — how settled the decision was.
    pub confidence: f64,
    /// `coach` (decision model) or `built-in` (heuristic fallback).
    pub source: &'static str,
}

pub const SOURCE_COACH: &str = "coach";
pub const SOURCE_BUILTIN: &str = "built-in";

impl LearnerContext {
    /// Known-topic stats, ignoring slugs outside the catalog.
    fn known_stats(&self) -> Vec<&TopicStat> {
        self.topics
            .iter()
            .filter(|t| topics::by_slug(&t.slug).is_some())
            .collect()
    }
}

/* ── Built-in fallback: weakest first, spaced, no repeats ───────── */

/// Pick the next topic without any external model: the topic with the
/// lowest recent accuracy among those attempted, tie-broken by longest
/// time since last drill; unseen topics rotate in when few have been
/// attempted; whatever was just drilled is excluded.
pub fn builtin_plan(ctx: &LearnerContext) -> DrillPlan {
    let recent: HashSet<&str> =
        ctx.recent_topics.iter().map(|s| s.as_str()).collect();
    let known = ctx.known_stats();

    let mut candidates: Vec<&Topic> = topics::TOPICS
        .iter()
        .filter(|t| !recent.contains(t.slug))
        .collect();
    if candidates.is_empty() {
        // Everything was just drilled — catalog order is fine.
        candidates = topics::TOPICS.iter().collect();
    }

    // Few topics attempted → rotate in unseen ones first.
    if known.len() < 2 {
        let fresh = candidates
            .iter()
            .find(|t| !known.iter().any(|s| s.slug == t.slug))
            .copied();
        if let Some(t) = fresh {
            return plan_for(t, ctx, 0.0);
        }
    }

    // Weakest accuracy (with attempts) wins; tie → longest untouched.
    let mut weakest: Option<(&TopicStat, f64)> = None;
    for stat in &known {
        if recent.contains(stat.slug.as_str()) {
            continue;
        }
        let accuracy = if stat.attempts == 0 {
            1.0
        } else {
            stat.correct as f64 / stat.attempts as f64
        };
        // Slight recency nudge: older drills are more urgent.
        let score = accuracy - recency_bonus(stat.last_at);
        if weakest.map(|(_, s)| score < s).unwrap_or(true) {
            weakest = Some((stat, score));
        }
    }
    match weakest {
        Some((stat, _)) => plan_for(topics::by_slug(&stat.slug).unwrap(), ctx, 0.0),
        None => plan_for(candidates[0], ctx, 0.0),
    }
}

/// A tiny freshness bonus so untouched topics edge out fresh ones.
fn recency_bonus(last_at: i64) -> f64 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let days = ((now - last_at) / 86_400).max(0) as f64;
    (days / 14.0).min(1.0) * 0.1
}

fn plan_for(topic: &Topic, ctx: &LearnerContext, confidence: f64) -> DrillPlan {
    let level = ctx
        .level
        .as_deref()
        .filter(|l| topic.levels.contains(l))
        .unwrap_or(topic.levels[0]);
    DrillPlan {
        topic: topic.slug.to_string(),
        topic_title: topic.title.to_string(),
        level: level.to_string(),
        confidence,
        source: SOURCE_BUILTIN,
    }
}

/* ── External decision model (implementation detail) ────────────── */

const DECISION_MODEL: &str = "jev-latest";
const DECISION_URL: &str = "https://api.typesafe.ai/v1/systemone";
const DECISION_TIMEOUT_SECS: u64 = 6;

/// Ask the configured decision model which topic to drill. All the
/// pedagogy context — catalog descriptions, level taxonomy, what to
/// consider — is attached here, never sent by the client.
pub async fn decide(api_key: Option<&str>, ctx: &LearnerContext) -> DrillPlan {
    let key = match api_key {
        Some(k) if !k.is_empty() => k,
        _ => return builtin_plan(ctx),
    };
    match ask_decision_model(key, ctx).await {
        Ok(plan) => plan,
        Err(_) => builtin_plan(ctx),
    }
}

async fn ask_decision_model(
    key: &str,
    ctx: &LearnerContext,
) -> Result<DrillPlan, String> {
    let body = build_request(ctx);
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(DECISION_TIMEOUT_SECS))
        .build()
        .map_err(|e| e.to_string())?;
    let res = client
        .post(DECISION_URL)
        .bearer_auth(key)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !res.status().is_success() {
        return Err(format!("decision model returned {}", res.status()));
    }
    let parsed: DecisionResponse = res.json().await.map_err(|e| e.to_string())?;
    parse_response(parsed, ctx)
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
struct DecisionRequest {
    model: &'static str,
    state: serde_json::Value,
    questions: serde_json::Value,
}

fn build_request(ctx: &LearnerContext) -> DecisionRequest {
    // The learner's own history, verbatim.
    let learner = serde_json::json!({
        "level_estimate": ctx.level,
        "topic_history": ctx.topics
            .iter()
            .filter(|t| topics::by_slug(&t.slug).is_some())
            .map(|t| {
                serde_json::json!({
                    "topic": t.slug,
                    "attempts": t.attempts,
                    "correct": t.correct,
                    "last_practiced_unix": t.last_at,
                })
            })
            .collect::<Vec<_>>(),
        "topics_drilled_recently": ctx.recent_topics,
    });

    // Server-attached pedagogy: what each topic is, what this app wants.
    let state = serde_json::json!({
        "learner": learner,
        "coaching_goals": [
            "Drill the learner's weakest topics more often than their strong ones.",
            "Space topics out: avoid drilling the same topic back to back.",
            "Introduce new topics gradually once recent ones look solid.",
            "Keep difficulty near the learner's level estimate."
        ],
        "available_topics": topics::TOPICS
            .iter()
            .map(|t| {
                serde_json::json!({
                    "topic": t.slug,
                    "what_it_drills": t.blurb,
                    "levels": t.levels,
                })
            })
            .collect::<Vec<_>>(),
    });

    let mut criteria = serde_json::Map::new();
    for t in topics::TOPICS {
        criteria.insert(
            t.slug.to_string(),
            serde_json::json!({ "what": t.blurb, "levels": t.levels }),
        );
    }

    let questions = serde_json::json!({
        "next_topic": {
            "type": "choice",
            "instructions": "Which grammar topic should this learner drill next? \
Consider their accuracy per topic, how long since each was practiced, the topics \
drilled recently, and their level estimate. Prefer a weak or stale topic at their \
level; introduce an untouched topic when recent ones are solid.",
            "criteria": criteria,
        },
        "level": {
            "type": "choice",
            "instructions": "At which CEFR level should this drill be pitched? \
Stay at the learner's estimate unless their recent accuracy on this topic clearly \
says otherwise.",
            "criteria": {
                "A1": "Beginner: single present-tense sentences, basic vocabulary",
                "A2": "Elementary: passé composé, imparfait habits, simple pronouns",
                "B1": "Intermediate: subjonctif, combined pronouns, tense contrasts",
                "B2": "Upper intermediate: nuanced tense and register choices"
            },
        },
    });

    DecisionRequest { model: DECISION_MODEL, state, questions }
}

#[derive(Debug, Deserialize)]
struct DecisionResponse {
    answers: serde_json::Value,
}

fn parse_response(res: DecisionResponse, ctx: &LearnerContext) -> Result<DrillPlan, String> {
    let answers = res
        .answers
        .as_object()
        .ok_or("answers is not an object")?;

    let topic_ans = answers
        .get("next_topic")
        .and_then(|a| a.get("choice"))
        .and_then(|c| c.as_str())
        .ok_or("missing next_topic.choice")?;
    let topic = topics::by_slug(topic_ans).ok_or_else(|| format!("unknown topic {topic_ans}"))?;

    let level = answers
        .get("level")
        .and_then(|a| a.get("choice"))
        .and_then(|c| c.as_str())
        .map(str::to_string);

    let confidence = answers
        .get("next_topic")
        .and_then(|a| a.get("confidence"))
        .and_then(|c| c.as_f64())
        .unwrap_or(0.0);

    let level = match level {
        Some(l) => topics::clamp_level(topic, &l).to_string(),
        None => {
            let mut p = plan_for(topic, ctx, confidence);
            p.source = SOURCE_COACH;
            return Ok(p);
        }
    };

    Ok(DrillPlan {
        topic: topic.slug.to_string(),
        topic_title: topic.title.to_string(),
        level,
        confidence: confidence.clamp(0.0, 1.0),
        source: SOURCE_COACH,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(stats: &[(&str, u32, u32, i64)], recent: &[&str]) -> LearnerContext {
        LearnerContext {
            level: Some("A2".into()),
            topics: stats
                .iter()
                .map(|(s, a, c, t)| TopicStat {
                    slug: s.to_string(),
                    attempts: *a,
                    correct: *c,
                    last_at: *t,
                })
                .collect(),
            recent_topics: recent.iter().map(|s| s.to_string()).collect(),
            recent_items: Vec::new(),
        }
    }

    #[test]
    fn context_parses_camel_case() {
        let raw = serde_json::json!({
            "level": "A2",
            "topics": [{"slug": "pc-etre", "attempts": 2, "correct": 1, "lastAt": 123}],
            "recentTopics": ["pc-etre"],
            "recentItems": ["pc-etre:aller:nous:cloze"]
        });
        let ctx: LearnerContext = serde_json::from_value(raw).unwrap();
        assert_eq!(ctx.topics[0].last_at, 123);
        assert_eq!(ctx.recent_topics, vec!["pc-etre".to_string()]);
        assert_eq!(ctx.recent_items.len(), 1);
    }

    #[test]
    fn fresh_learner_gets_unseen_topic() {
        let plan = builtin_plan(&Default::default());
        assert_eq!(plan.source, SOURCE_BUILTIN);
        assert!(topics::by_slug(&plan.topic).is_some());
        assert_eq!(plan.confidence, 0.0);
    }

    #[test]
    fn weakest_topic_wins() {
        let now = 1_700_000_000;
        let plan = builtin_plan(&ctx(
            &[("pc-etre", 10, 10, now), ("subjonctif", 10, 2, now)],
            &[],
        ));
        assert_eq!(plan.topic, "subjonctif");
    }

    #[test]
    fn recently_drilled_topic_is_skipped() {
        let now = 1_700_000_000;
        let plan = builtin_plan(&ctx(
            &[("pc-etre", 10, 10, now), ("subjonctif", 10, 2, now)],
            &["subjonctif"],
        ));
        assert_ne!(plan.topic, "subjonctif");
    }

    #[test]
    fn level_clamped_into_topic_range() {
        // A B1-only topic at an A1 estimate clamps to B1.
        let now = 1_700_000_000;
        let plan = builtin_plan(&ctx(
            &[("pc-etre", 10, 10, now), ("subjonctif", 5, 1, 0)],
            &[],
        ));
        assert_eq!(plan.topic, "subjonctif");
        assert_eq!(plan.level, "B1");
    }

    #[test]
    fn unknown_client_slugs_are_ignored() {
        let plan = builtin_plan(&ctx(&[("not-a-topic", 9, 0, 1)], &[]));
        assert!(topics::by_slug(&plan.topic).is_some());
    }

    #[test]
    fn decision_response_parses_and_validates() {
        let raw = serde_json::json!({
            "answers": {
                "next_topic": {
                    "type": "choice",
                    "choice": "pc-etre",
                    "confidence": 0.82,
                    "probabilities": {}
                },
                "level": { "type": "choice", "choice": "B2", "confidence": 0.5, "probabilities": {} }
            }
        });
        let res: DecisionResponse = serde_json::from_value(raw).unwrap();
        let plan = parse_response(res, &Default::default()).unwrap();
        assert_eq!(plan.topic, "pc-etre");
        assert_eq!(plan.source, SOURCE_COACH);
        assert_eq!(plan.confidence, 0.82);
        // B2 clamps into pc-etre's range → A2.
        assert_eq!(plan.level, "A2");
    }

    #[test]
    fn decision_response_with_unknown_topic_falls_back() {
        let raw = serde_json::json!({
            "answers": {
                "next_topic": { "choice": "quantum-physics", "confidence": 0.9 }
            }
        });
        let res: DecisionResponse = serde_json::from_value(raw).unwrap();
        assert!(parse_response(res, &Default::default()).is_err());
    }

    #[test]
    fn request_state_carries_catalog_not_just_client_data() {
        let req = build_request(&Default::default());
        let state = req.state.as_object().unwrap();
        assert!(state.contains_key("available_topics"));
        assert!(state.contains_key("coaching_goals"));
        let questions = req.questions.as_object().unwrap();
        let next = questions.get("next_topic").and_then(|q| q.get("criteria"))
            .and_then(|c| c.as_object()).unwrap();
        // Every catalog topic is an option.
        assert_eq!(next.len(), topics::TOPICS.len());
    }
}
