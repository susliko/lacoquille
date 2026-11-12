//! Parameterized exercise families — the endless tier. Each family owns
//! tagged word banks and composes items from (bank × bank × bank) slot
//! combinations, so a topic yields far more distinct items than any
//! fixed corpus. Items keep the exact shapes the client's views render.
//!
//! Ids are derived from the slot values (stable across requests), which
//! is what lets the feed avoid immediate repeats via `exclude`.
//!
//! One file per engine cluster; shared banks and helpers live in
//! `shared`. To add a family: a bank + an `items()` fn in a new module,
//! a `Family::` variant in `topics`, an arm in `family_pool` — then
//! flip `family:` on its topic.

mod accord_antepose;
mod adjectives;
mod articles;
mod cest_vs_il_est;
mod imparfait;
mod imppc;
mod modals;
mod mood_a1;
mod negation;
mod pc_aux;
mod pc_avoir;
mod pc_etre;
mod periphrases;
mod present;
mod pronoun_y;
mod pronouns;
mod pronouns_basic;
mod questions;
mod sentence_a1;
mod shared;
mod subjonctif;

use std::collections::HashSet;

use super::gen::{self, Rng};
use super::topics::{Family, Topic};
use super::ExerciseItem;

/* ── Family pool dispatch ───────────────────────────────────────── */

/// Every distinct item the topic's family can produce — the pool the
/// sampler draws a shuffled, exclusion-aware `count` from. Empty for
/// `Family::Curated` (callers fall back to the curated corpus).
pub fn family_pool(topic: &Topic) -> Vec<ExerciseItem> {
    match topic.family {
        Family::PcAux => pc_aux::items(),
        Family::PcEtre => pc_etre::items(),
        Family::Pronouns => pronouns::items(),
        Family::Subjonctif => subjonctif::items(),
        Family::ImpVsPc => imppc::items(),
        Family::Negation => negation::items(),
        Family::CestVsIlEst => cest_vs_il_est::items(),
        Family::PronounY => pronoun_y::items(),
        Family::AccordAntepose => accord_antepose::items(),
        Family::Imparfait => imparfait::items(),
        Family::PcAvoir => pc_avoir::items(),
        Family::Present => present::items(topic.slug),
        Family::Periphrases => periphrases::items(topic.slug),
        Family::Modals => modals::items(topic.slug),
        Family::Articles => articles::items(topic.slug),
        Family::Adjectives => adjectives::items(topic.slug),
        Family::PronounBasics => pronouns_basic::items(topic.slug),
        Family::Questions => questions::items(topic.slug),
        Family::SentencesA1 => sentence_a1::items(topic.slug),
        Family::MoodsA1 => mood_a1::items(topic.slug),
        Family::Curated => Vec::new(),
    }
}

/* ── Coverage gate ──────────────────────────────────────────────── */

/// Minimum distinct items per topic family. Families are endless by
/// design; a pool below its floor means a bank shrank or a slot loop
/// got disconnected. Unlisted slugs fall back to [`COVERAGE_FLOOR`].
fn pool_floor(slug: &str) -> usize {
    match slug {
        "pc-etre" => 40,
        "pc-aux" => 60,
        "pronouns-cod-coi" => 200,
        "subjonctif" => 300,
        "imparfait-vs-pc" => 80,
        "negation" => 300,
        "cest-vs-il-est" => 40,
        "pronoun-y" => 100,
        "accord-antepose" => 250,
        "imparfait" => 250,
        "pc-avoir" => 120,
        "present-etre-avoir" => 100,
        "present-er-verbs" => 90,
        "present-irregulars" => 90,
        "present-pronominal" => 60,
        "etre-en-train-de" => 70,
        "futur-proche" => 70,
        "passe-recent" => 70,
        "modal-verbs-infinitive" => 200,
        "il-faut-infinitive" => 30,
        "pour-infinitive" => 80,
        "gender-of-nouns" => 30,
        "plural-of-nouns" => 30,
        "definite-indefinite-articles" => 30,
        "partitive-articles" => 70,
        "adjective-agreement" => 70,
        "adjective-position" => 30,
        "possessive-adjectives" => 70,
        "demonstrative-adjectives" => 30,
        "quantity-adverbs" => 30,
        "subject-pronouns" => 150,
        "stressed-pronouns" => 30,
        "three-question-forms" => 30,
        "question-words" => 30,
        "il-ya-cest-voila" => 30,
        "dans-depuis-il-ya" => 30,
        "prepositions-place" => 30,
        "prepositions-time" => 30,
        "connectors-basic" => 30,
        "imperative" => 40,
        "polite-conditionnel" => 70,
        _ => COVERAGE_FLOOR,
    }
}

/// The floor for curated-only topics: enough that a coach-picked topic
/// never re-serves the same handful of items run after run.
const COVERAGE_FLOOR: usize = 30;

/// Generate `count` items for a topic from its family. `exclude` holds
/// recently served item ids; excluded combos are skipped unless the
/// family can't otherwise fill the count. Choice options come back
/// shuffled with `correct` re-indexed. Returns empty for topics with
/// no family yet (callers fall back to the curated corpus).
pub fn generate(
    topic: &Topic,
    level: Option<&str>,
    count: usize,
    exclude: &HashSet<String>,
    seed: u64,
) -> Vec<ExerciseItem> {
    let mut pool = family_pool(topic);
    if let Some(l) = level {
        pool.retain(|i| match i {
            ExerciseItem::Cloze(x) => x.level == l,
            ExerciseItem::Choice(x) => x.level == l,
            ExerciseItem::Tiles(x) => x.level == l,
            ExerciseItem::Transform(x) => x.level == l,
            ExerciseItem::ErrorSpot(x) => x.level == l,
            ExerciseItem::Dictation(x) => x.level == l,
        });
    }
    let mut rng = Rng::new(seed);
    rng.shuffle(&mut pool);
    // Shuffle choice options and re-index `correct`.
    for item in &mut pool {
        if let ExerciseItem::Choice(c) = item {
            let mut opts: Vec<(usize, String)> =
                c.options.iter().enumerate().map(|(i, o)| (i, o.clone())).collect();
            rng.shuffle(&mut opts);
            c.correct = opts.iter().position(|(i, _)| *i == c.correct).unwrap_or(0);
            c.options = opts.into_iter().map(|(_, o)| o).collect();
        }
    }
    let mut picked: Vec<ExerciseItem> = Vec::with_capacity(count);
    let mut fallback: Vec<ExerciseItem> = Vec::new();
    for item in pool {
        if picked.len() == count {
            break;
        }
        if exclude.contains(item.id()) {
            fallback.push(item);
        } else {
            picked.push(item);
        }
    }
    for item in fallback {
        if picked.len() == count {
            break;
        }
        picked.push(item);
    }
    picked
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exercises::topics;

    fn gen(slug: &str, level: Option<&str>, count: usize, seed: u64) -> Vec<ExerciseItem> {
        topics::by_slug(slug)
            .map(|t| generate(t, level, count, &HashSet::new(), seed))
            .unwrap_or_default()
    }

    #[test]
    fn every_family_topic_produces_items() {
        for t in topics::TOPICS {
            if t.family != Family::Curated {
                let items = gen(t.slug, None, 5, 1);
                assert!(items.len() == 5, "{} produced {}", t.slug, items.len());
                assert!(items.iter().all(|i| i.id().starts_with(|p: char| p.is_ascii_alphabetic())));
            }
        }
    }

    #[test]
    fn coverage_gate() {
        // Every catalog topic must clear its pool floor: family topics
        // via their family pool, curated topics via distinct ids in a
        // large curated sample. Fails when a bank shrinks, a slot loop
        // disconnects, or a new topic ships without enough content.
        for t in topics::TOPICS {
            let items = match t.family {
                Family::Curated => gen::curated_for_topic(t.title, 100, 1),
                _ => family_pool(t),
            };
            let floor = match t.family {
                Family::Curated => COVERAGE_FLOOR,
                _ => pool_floor(t.slug),
            };
            let mut ids: Vec<&str> = items.iter().map(|i| i.id()).collect();
            ids.sort_unstable();
            let distinct = ids.len();
            ids.dedup();
            assert_eq!(
                ids.len(),
                distinct,
                "topic {} has duplicate item ids",
                t.slug
            );
            assert!(
                distinct >= floor,
                "topic {} covers only {} distinct items (floor {})",
                t.slug,
                distinct,
                floor
            );
            println!("coverage: {:>18}  pool {:>4}  floor {:>3}  ok", t.slug, distinct, floor);
        }
    }

    #[test]
    fn choice_answers_stay_valid_after_shuffle() {
        for seed in 0..25 {
            for item in gen("pc-aux", None, 6, seed) {
                if let ExerciseItem::Choice(c) = item {
                    assert!(c.correct < c.options.len());
                    // The être form must be flagged correct for être verbs.
                    if c.id.contains("aller") || c.id.contains("partir") {
                        assert!(c.options[c.correct].starts_with("est")
                            || c.options[c.correct].starts_with("sont")
                            || c.options[c.correct].starts_with("sommes")
                            || c.options[c.correct].starts_with("êtes"));
                    }
                }
            }
        }
    }

    #[test]
    fn level_filter_respected() {
        let items = gen("pronouns-cod-coi", Some("A2"), 10, 3);
        assert!(!items.is_empty());
        assert!(items.iter().all(|i| match i {
            ExerciseItem::Transform(t) => t.level == "A2",
            _ => true,
        }));
        // B1 pairs (visible agreement) are excluded at A2.
        assert!(!items.iter().any(|i| i.id().contains("fleurs") || i.id().contains("clés")));
    }

    #[test]
    fn ids_are_distinct_within_pool() {
        let items = gen("subjonctif", None, 400, 7);
        let mut ids: Vec<&str> = items.iter().map(|i| i.id()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total);
    }

    #[test]
    fn exclusion_avoids_recent_items() {
        let t = topics::by_slug("pc-etre").unwrap();
        let first = generate(t, None, 5, &HashSet::new(), 11);
        let exclude: HashSet<String> = first.iter().map(|i| i.id().to_string()).collect();
        let second = generate(t, None, 5, &exclude, 12);
        assert_eq!(second.len(), 5);
        let overlap = second.iter().filter(|i| exclude.contains(i.id())).count();
        // Pool is large enough that a fresh 5 should avoid all excluded ids.
        assert_eq!(overlap, 0, "overlapping ids: {:?}", second.iter().map(|i| i.id()).collect::<Vec<_>>());
    }

    #[test]
    fn curated_family_yields_nothing() {
        // A Curated topic never gets family output — the route falls
        // back to the curated corpus instead.
        let t = Topic {
            slug: "synthetic",
            title: "Synthetic",
            blurb: "",
            levels: &["A1"],
            family: Family::Curated,
            section: crate::exercises::topics::Section::Sentence,
        };
        assert!(family_pool(&t).is_empty());
        assert!(generate(&t, None, 5, &HashSet::new(), 1).is_empty());
    }

    #[test]
    fn error_spot_indices_valid() {
        let mut items = Vec::new();
        for slug in ["subjonctif", "pc-aux", "pc-etre", "imparfait", "pc-avoir"] {
            items.extend(gen(slug, None, 200, 5));
        }
        for item in items {
            if let ExerciseItem::ErrorSpot(e) = item {
                assert!(e.wrong < e.words.len(), "{:?}", e.id);
                assert!(!e.fix.is_empty());
            }
        }
    }
}
