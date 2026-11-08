//! Seeded exercise generation. Deterministic for a given seed — the same
//! request replays the same session, which keeps the endpoint stateless
//! while letting clients reload "today's" batch if they pass their own
//! seed. The RNG is a tiny SplitMix64 (no `rand` dependency).

use super::corpus as c;
use super::{
    ChoiceItem, ClozeItem, DictationItem, ErrorSpotItem, ExerciseItem, TilesItem, TransformItem,
};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Cloze,
    Choice,
    Tiles,
    Transform,
    ErrorSpot,
    Dictation,
}

pub const ALL_TYPES: &[ItemType] = &[
    ItemType::Cloze,
    ItemType::Choice,
    ItemType::Tiles,
    ItemType::Transform,
    ItemType::ErrorSpot,
    ItemType::Dictation,
];

impl ItemType {
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "cloze" => Some(ItemType::Cloze),
            "choice" => Some(ItemType::Choice),
            "tiles" => Some(ItemType::Tiles),
            "transform" => Some(ItemType::Transform),
            "error-spot" | "error_spot" | "errorspot" => Some(ItemType::ErrorSpot),
            "dictation" => Some(ItemType::Dictation),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct GenRequest {
    /// 1..=12.
    pub count: usize,
    pub types: Vec<ItemType>,
    /// One of A1/A2/B1/B2, or None for mixed.
    pub level: Option<String>,
    pub seed: u64,
}

impl Default for GenRequest {
    fn default() -> Self {
        Self {
            count: 6,
            types: ALL_TYPES.to_vec(),
            level: None,
            seed: 1234,
        }
    }
}

pub fn time_seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(42)
}

/* ── RNG (SplitMix64 + Fisher–Yates) ────────────────────────────── */

pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed)
    }

    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.next() % n as u64) as usize
        }
    }

    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.below(i + 1);
            v.swap(i, j);
        }
    }

    /// Random element (cloned) of a slice.
    pub fn pick<'a, T>(&mut self, v: &'a [T]) -> &'a T {
        &v[self.below(v.len())]
    }
}

/* ── Generation ─────────────────────────────────────────────────── */

fn strs(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

fn level_ok(want: &Option<String>, level: &str) -> bool {
    match want {
        Some(w) => w == level,
        None => true,
    }
}

fn build_choice(rng: &mut Rng, t: &c::ChoiceTemplate) -> ChoiceItem {
    let mut opts: Vec<(String, bool)> = t
        .options
        .iter()
        .enumerate()
        .map(|(i, o)| (o.to_string(), i == t.correct))
        .collect();
    rng.shuffle(&mut opts);
    let correct = opts.iter().position(|(_, is)| *is).unwrap_or(0);
    ChoiceItem {
        id: t.id.to_string(),
        level: t.level,
        topic: t.topic,
        topic_slug: crate::exercises::topics::slug_for_title(t.topic).map(|s| s.to_string()),
        task: strs(t.task),
        sentence: t.sentence.to_string(),
        options: opts.into_iter().map(|(o, _)| o).collect(),
        correct,
        rule: strs(t.rule),
    }
}

fn build_tiles(rng: &mut Rng, t: &c::TilesTemplate) -> TilesItem {
    let mut bank: Vec<String> =
        t.target.iter().chain(t.distractors.iter()).map(|s| s.to_string()).collect();
    rng.shuffle(&mut bank);
    TilesItem {
        id: t.id.to_string(),
        level: t.level,
        topic: t.topic,
        topic_slug: crate::exercises::topics::slug_for_title(t.topic).map(|s| s.to_string()),
        task: strs(t.task),
        prompt: t.prompt,
        target: strs(t.target),
        bank,
        rule: strs(t.rule),
    }
}

/// Pick a random unused index from `remaining`; when the pool is
/// exhausted, refill it from the (level-filtered) pool so items repeat
/// rather than error out — never reaching outside the filter.
fn take_index(rng: &mut Rng, remaining: &mut Vec<usize>, pool: &[usize]) -> usize {
    if remaining.is_empty() {
        *remaining = pool.to_vec();
        rng.shuffle(remaining);
    }
    remaining.swap_remove(rng.below(remaining.len()))
}

pub fn generate(req: &GenRequest) -> Result<Vec<ExerciseItem>, String> {
    if req.count == 0 {
        return Err("count must be at least 1".into());
    }
    let count = req.count.min(12);
    if req.types.is_empty() {
        return Err("no valid exercise types requested".into());
    }

    // Level-filtered pools per requested type. Types whose pool is empty
    // for this level are dropped; if nothing survives, that's an error.
    struct Pool {
        kind: ItemType,
        /// For cloze: (template index, person index) pairs.
        cloze: Vec<(usize, usize)>,
        /// For non-cloze types: template indices.
        plain: Vec<usize>,
    }

    let mut pools: Vec<Pool> = Vec::new();
    for kind in &req.types {
        match kind {
            ItemType::Cloze => {
                let mut cloze = Vec::new();
                for (ti, t) in c::CLOZE.iter().enumerate() {
                    if !level_ok(&req.level, t.level) {
                        continue;
                    }
                    for pi in 0..t.persons.len() {
                        cloze.push((ti, pi));
                    }
                }
                if !cloze.is_empty() {
                    pools.push(Pool { kind: *kind, cloze, plain: Vec::new() });
                }
            }
            ItemType::Choice => {
                let plain: Vec<usize> = c::CHOICE
                    .iter()
                    .enumerate()
                    .filter(|(_, t)| level_ok(&req.level, t.level))
                    .map(|(i, _)| i)
                    .collect();
                if !plain.is_empty() {
                    pools.push(Pool { kind: *kind, cloze: Vec::new(), plain });
                }
            }
            ItemType::Tiles => {
                let plain: Vec<usize> = c::TILES
                    .iter()
                    .enumerate()
                    .filter(|(_, t)| level_ok(&req.level, t.level))
                    .map(|(i, _)| i)
                    .collect();
                if !plain.is_empty() {
                    pools.push(Pool { kind: *kind, cloze: Vec::new(), plain });
                }
            }
            ItemType::Transform => {
                let plain: Vec<usize> = c::TRANSFORM
                    .iter()
                    .enumerate()
                    .filter(|(_, t)| level_ok(&req.level, t.level))
                    .map(|(i, _)| i)
                    .collect();
                if !plain.is_empty() {
                    pools.push(Pool { kind: *kind, cloze: Vec::new(), plain });
                }
            }
            ItemType::ErrorSpot => {
                let plain: Vec<usize> = c::ERROR_SPOT
                    .iter()
                    .enumerate()
                    .filter(|(_, t)| level_ok(&req.level, t.level))
                    .map(|(i, _)| i)
                    .collect();
                if !plain.is_empty() {
                    pools.push(Pool { kind: *kind, cloze: Vec::new(), plain });
                }
            }
            ItemType::Dictation => {
                let plain: Vec<usize> = c::DICTATION
                    .iter()
                    .enumerate()
                    .filter(|(_, t)| level_ok(&req.level, t.level))
                    .map(|(i, _)| i)
                    .collect();
                if !plain.is_empty() {
                    pools.push(Pool { kind: *kind, cloze: Vec::new(), plain });
                }
            }
        }
    }

    if pools.is_empty() {
        return Err(format!(
            "no exercises available for level {:?}",
            req.level.as_deref().unwrap_or_default()
        ));
    }

    let mut rng = Rng::new(req.seed);

    // Round-robin over the type pools (in a shuffled order) so a session
    // mixes types before repeating any.
    let mut order: Vec<usize> = (0..pools.len()).collect();
    rng.shuffle(&mut order);
    // Per-pool "unused" trackers (cloze variant pairs / plain indices).
    let mut cloze_left: Vec<Vec<(usize, usize)>> = pools
        .iter()
        .map(|p| {
            let mut v = p.cloze.clone();
            rng.shuffle(&mut v);
            v
        })
        .collect();
    let mut plain_left: Vec<Vec<usize>> = pools
        .iter()
        .map(|p| {
            let mut v = p.plain.clone();
            rng.shuffle(&mut v);
            v
        })
        .collect();

    let mut items = Vec::with_capacity(count);
    let mut next_pool = 0usize;
    while items.len() < count {
        let ord = order[next_pool % order.len()];
        next_pool += 1;
        let pool = &pools[ord];
        let item = match pool.kind {
            ItemType::Cloze => {
                if pool.cloze.is_empty() {
                    continue;
                }
                let (ti, pi) = take_index_pair(&mut rng, &mut cloze_left[ord], &pool.cloze);
                let t = &c::CLOZE[ti];
                ExerciseItem::Cloze(ClozeItem::from_template(t, &t.persons[pi]))
            }
            ItemType::Choice => {
                let ti = take_index(&mut rng, &mut plain_left[ord], &pool.plain);
                ExerciseItem::Choice(build_choice(&mut rng, &c::CHOICE[ti]))
            }
            ItemType::Tiles => {
                let ti = take_index(&mut rng, &mut plain_left[ord], &pool.plain);
                ExerciseItem::Tiles(build_tiles(&mut rng, &c::TILES[ti]))
            }
            ItemType::Transform => {
                let ti = take_index(&mut rng, &mut plain_left[ord], &pool.plain);
                let t = &c::TRANSFORM[ti];
                ExerciseItem::Transform(TransformItem {
                    id: t.id.to_string(),
                    level: t.level,
                    topic: t.topic,
        topic_slug: crate::exercises::topics::slug_for_title(t.topic).map(|s| s.to_string()),
                    task: strs(t.task),
                    source: t.source.to_string(),
                    answer: t.answer.to_string(),
                    rule: strs(t.rule),
                })
            }
            ItemType::ErrorSpot => {
                let ti = take_index(&mut rng, &mut plain_left[ord], &pool.plain);
                let t = &c::ERROR_SPOT[ti];
                ExerciseItem::ErrorSpot(ErrorSpotItem {
                    id: t.id.to_string(),
                    level: t.level,
                    topic: t.topic,
        topic_slug: crate::exercises::topics::slug_for_title(t.topic).map(|s| s.to_string()),
                    task: strs(t.task),
                    words: strs(t.words),
                    wrong: t.wrong,
                    fix: t.fix.to_string(),
                    fix_task: t.fix_task,
                    rule: strs(t.rule),
                })
            }
            ItemType::Dictation => {
                let ti = take_index(&mut rng, &mut plain_left[ord], &pool.plain);
                let t = &c::DICTATION[ti];
                ExerciseItem::Dictation(DictationItem {
                    id: t.id.to_string(),
                    level: t.level,
                    topic: t.topic,
        topic_slug: crate::exercises::topics::slug_for_title(t.topic).map(|s| s.to_string()),
                    task: strs(t.task),
                    speech: t.speech,
                    answer: t.answer,
                    rule: strs(t.rule),
                })
            }
        };
        items.push(item);
    }

    Ok(items)
}

/// Like `take_index` but for (template, person) pairs.
fn take_index_pair(
    rng: &mut Rng,
    remaining: &mut Vec<(usize, usize)>,
    pool: &[(usize, usize)],
) -> (usize, usize) {
    if remaining.is_empty() {
        *remaining = pool.to_vec();
        rng.shuffle(remaining);
    }
    remaining.swap_remove(rng.below(remaining.len()))
}

/// Curated items whose topic matches `title` — the fallback for topics
/// that have no generator family yet. Level-agnostic; shuffled.
pub fn curated_for_topic(title: &str, count: usize, seed: u64) -> Vec<ExerciseItem> {
    let mut rng = Rng::new(seed);
    let mut pool: Vec<ExerciseItem> = Vec::new();

    for t in c::CLOZE.iter().filter(|t| t.topic == title) {
        for p in t.persons.iter() {
            pool.push(ExerciseItem::Cloze(ClozeItem::from_template(t, p)));
        }
    }
    for t in c::CHOICE.iter().filter(|t| t.topic == title) {
        pool.push(ExerciseItem::Choice(build_choice(&mut rng, t)));
    }
    for t in c::TILES.iter().filter(|t| t.topic == title) {
        pool.push(ExerciseItem::Tiles(build_tiles(&mut rng, t)));
    }
    for t in c::TRANSFORM.iter().filter(|t| t.topic == title) {
        pool.push(ExerciseItem::Transform(TransformItem {
            id: t.id.to_string(),
            level: t.level,
            topic: t.topic,
            topic_slug: crate::exercises::topics::slug_for_title(t.topic).map(|s| s.to_string()),
            task: strs(t.task),
            source: t.source.to_string(),
            answer: t.answer.to_string(),
            rule: strs(t.rule),
        }));
    }
    for t in c::ERROR_SPOT.iter().filter(|t| t.topic == title) {
        pool.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: t.id.to_string(),
            level: t.level,
            topic: t.topic,
            topic_slug: crate::exercises::topics::slug_for_title(t.topic).map(|s| s.to_string()),
            task: strs(t.task),
            words: strs(t.words),
            wrong: t.wrong,
            fix: t.fix.to_string(),
            fix_task: t.fix_task,
            rule: strs(t.rule),
        }));
    }
    for t in c::DICTATION.iter().filter(|t| t.topic == title) {
        pool.push(ExerciseItem::Dictation(DictationItem {
            id: t.id.to_string(),
            level: t.level,
            topic: t.topic,
            topic_slug: crate::exercises::topics::slug_for_title(t.topic).map(|s| s.to_string()),
            task: strs(t.task),
            speech: t.speech,
            answer: t.answer,
            rule: strs(t.rule),
        }));
    }

    rng.shuffle(&mut pool);
    pool.truncate(count);
    pool
}

/* ── Tests ──────────────────────────────────────────────────────── */

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(seed: u64) -> Vec<String> {
        let items = generate(&GenRequest { count: 12, types: ALL_TYPES.to_vec(), level: None, seed }).unwrap();
        items
            .iter()
            .map(|i| match i {
                ExerciseItem::Cloze(_) => "cloze",
                ExerciseItem::Choice(_) => "choice",
                ExerciseItem::Tiles(_) => "tiles",
                ExerciseItem::Transform(_) => "transform",
                ExerciseItem::ErrorSpot(_) => "error-spot",
                ExerciseItem::Dictation(_) => "dictation",
            })
            .map(String::from)
            .collect()
    }

    #[test]
    fn deterministic_for_seed() {
        let a = generate(&GenRequest::default()).unwrap();
        let b = generate(&GenRequest::default()).unwrap();
        let a_ids: Vec<String> = a.iter().map(|i| i.id().to_string()).collect();
        let b_ids: Vec<String> = b.iter().map(|i| i.id().to_string()).collect();
        assert_eq!(a_ids, b_ids);
    }

    #[test]
    fn different_seeds_differ() {
        let a = generate(&GenRequest { seed: 1, ..Default::default() }).unwrap();
        let b = generate(&GenRequest { seed: 2, ..Default::default() }).unwrap();
        let a_ids: Vec<&str> = a.iter().map(|i| i.id()).collect();
        let b_ids: Vec<&str> = b.iter().map(|i| i.id()).collect();
        assert_ne!(a_ids, b_ids);
    }

    #[test]
    fn mixes_all_types_when_requested() {
        // With 12 slots across 6 pools, every type must appear.
        let got = kinds(99);
        for k in ["cloze", "choice", "tiles", "transform", "error-spot", "dictation"] {
            assert!(got.iter().any(|g| g == k), "missing kind {} in {:?}", k, got);
        }
    }

    #[test]
    fn respects_count_and_single_type() {
        let items = generate(&GenRequest {
            count: 3,
            types: vec![ItemType::Choice],
            level: None,
            seed: 7,
        })
        .unwrap();
        assert_eq!(items.len(), 3);
        assert!(items.iter().all(|i| matches!(i, ExerciseItem::Choice(_))));
    }

    #[test]
    fn level_filters() {
        let items = generate(&GenRequest {
            count: 5,
            types: vec![ItemType::Dictation],
            level: Some("A1".into()),
            seed: 5,
        })
        .unwrap();
        assert!(items
            .iter()
            .all(|i| matches!(i, ExerciseItem::Dictation(d) if d.level == "A1")));

        assert!(generate(&GenRequest {
            count: 5,
            types: vec![ItemType::Dictation],
            level: Some("C2".into()),
            seed: 5,
        })
        .is_err());
    }

    #[test]
    fn choice_shuffle_keeps_answer_valid() {
        for seed in 0..50 {
            let items = generate(&GenRequest {
                count: 6,
                types: vec![ItemType::Choice],
                level: None,
                seed,
            })
            .unwrap();
            for i in items {
                if let ExerciseItem::Choice(c) = i {
                    assert!(c.correct < c.options.len());
                    assert!(c.options.iter().any(|o| !o.is_empty()));
                }
            }
        }
    }

    #[test]
    fn tiles_bank_contains_target() {
        for seed in 0..50 {
            let items = generate(&GenRequest {
                count: 6,
                types: vec![ItemType::Tiles],
                level: None,
                seed,
            })
            .unwrap();
            for i in items {
                if let ExerciseItem::Tiles(t) = i {
                    assert!(t.bank.len() >= t.target.len());
                    for w in &t.target {
                        assert!(
                            t.bank.contains(w) || t.bank.iter().any(|b| b.eq_ignore_ascii_case(w)),
                            "target word {} missing from bank {:?}",
                            w,
                            t.bank
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn cloze_ids_carry_person_key() {
        let items = generate(&GenRequest {
            count: 4,
            types: vec![ItemType::Cloze],
            level: None,
            seed: 11,
        })
        .unwrap();
        assert!(items.iter().all(|i| {
            matches!(i, ExerciseItem::Cloze(c) if c.id.contains('-') && c.id.split('-').last().unwrap().len() >= 2)
        }));
    }

    #[test]
    fn count_is_clamped() {
        let items = generate(&GenRequest {
            count: 99,
            types: ALL_TYPES.to_vec(),
            level: None,
            seed: 3,
        })
        .unwrap();
        assert_eq!(items.len(), 12);
    }

    #[test]
    fn zero_count_is_error() {
        assert!(generate(&GenRequest { count: 0, ..Default::default() }).is_err());
    }
}
