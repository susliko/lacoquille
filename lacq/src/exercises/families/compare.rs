//! Family: comparison — two topics. The comparative (plus / moins /
//! aussi … que, driven by clear real-world facts) and the superlative
//! (le/la/les plus …, article agrees, with bon → meilleur and
//! mauvais → pire).

use crate::exercises::{ChoiceItem, ClozeItem, ExerciseItem};

use super::shared::starts_vowel;

const LEVEL: &str = "A2";

/* ── comparative ────────────────────────────────────────────────── */

/// (A, B, adjective (agreed with A), direction) — direction "plus"
/// means A > B; "moins" means A < B; facts are safe common knowledge.
const COMP_FACTS: &[(&str, &str, &str, &str)] = &[
    ("Un cheval", "un chat", "rapide", "plus"),
    ("Une fusée", "une voiture", "rapide", "plus"),
    ("La Tour Eiffel", "mon immeuble", "haute", "plus"),
    ("L'été", "l'hiver", "chaud", "plus"),
    ("Un éléphant", "une souris", "lourd", "plus"),
    ("Le TGV", "le vélo", "rapide", "plus"),
    ("L'or", "l'argent", "cher", "plus"),
    ("Une fourmi", "un chien", "petite", "moins"),
    ("Une souris", "un éléphant", "lourde", "moins"),
    ("Le vélo", "le TGV", "rapide", "moins"),
    ("L'hiver", "l'été", "chaud", "moins"),
    ("Une tartine", "un festin", "cher", "moins"),
    ("Mon frère", "moi", "grand", "plus"),
    ("Ma sœur", "notre mère", "grand", "moins"),
    ("Le métro", "le taxi", "cher", "moins"),
    ("La glace", "le thé", "froide", "plus"),
];

fn comparative_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(a, b, adj, dir) in COMP_FACTS {
        // Cloze: the comparison word is the gap.
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("cmp:{}:{}:cl", slug2(a), slug2(b)),
            level: LEVEL,
            topic: "Comparative",
            topic_slug: Some("comparative".into()),
            task: vec!["Complete with plus, moins or aussi.".into()],
            head: format!("{} est ", a),
            tail: format!(" {} {}.", adj, que_join(b)),
            infinitive: "plus / moins / aussi",
            answer: dir.to_string(),
            rule: vec![
                format!("{} est {} {}{}.", a, dir, adj, que_join(b)),
                "plus … que = more … than; moins … que = less … than.".into(),
            ],
        }));
        // Choice: the comparison word against its natural opposite.
        let wrong = match dir {
            "plus" => "moins",
            "moins" => "plus",
            _ => "plus",
        };
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("cmp:{}:{}:ch", slug2(a), slug2(b)),
            level: LEVEL,
            topic: "Comparative",
            topic_slug: Some("comparative".into()),
            task: vec!["Choose the right word.".into()],
            sentence: format!("{} est ____ {} {}.", a, adj, que_join(b)),
            options: vec![dir.into(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![format!("{} est {} {}{}.", a, dir, adj, que_join(b))],
        }));
        // Neutral pair: aussi … que on the same subject.
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("cmp:{}:{}:aussi", slug2(a), slug2(b)),
            level: LEVEL,
            topic: "Comparative",
            topic_slug: Some("comparative".into()),
            task: vec!["They are the same — choose the right word.".into()],
            sentence: format!("{} est ____ {} {}.", a, adj, que_join(b)),
            options: vec!["aussi".into(), "plus".into()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                "aussi … que = as … as (no difference).".into(),
                format!("{} est aussi {}{}.", a, adj, que_join(b)),
            ],
        }));
    }
    out
}

/// "que une voiture" → "qu'une voiture" (elision before a vowel).
fn que_join(b: &str) -> String {
    if starts_vowel(b) {
        format!("qu'{}", b)
    } else {
        format!("que {}", b)
    }
}

/* ── superlative ────────────────────────────────────────────────── */

/// (subject noun phrase, adjective agreed, superlative chunk)
const SUP_FACTS: &[(&str, &str, &str)] = &[
    ("Paris", "belle", "la plus belle"),
    ("L'Everest", "haut", "le plus haut"),
    ("Le TGV", "rapide", "le plus rapide"),
    ("Ma sœur", "gentille", "la plus gentille"),
    ("Cet examen", "difficile", "le plus difficile"),
    ("Cette ville", "bruyante", "la plus bruyante"),
    ("Mon chat", "paresseux", "le plus paresseux"),
    ("Cette voiture", "chère", "la plus chère"),
    ("Ce restaurant", "bon", "le meilleur"),
    ("Cette pâtisserie", "bon", "la meilleure"),
    ("Ce film", "mauvais", "le pire"),
    ("Cette chanson", "mauvais", "la pire"),
    ("Mes amis", "sympathique", "les plus sympathiques"),
    ("Ces jardins", "joli", "les plus jolis"),
    ("Cet hôtel", "cher", "le plus cher"),
    ("Cette plage", "sauvage", "la plus sauvage"),
];

fn superlative_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(subj, adj, sup) in SUP_FACTS {
        let sup_art = sup.split(' ').next().unwrap(); // le / la / les
        let verb = if sup.starts_with("les ") { "sont" } else { "est" };
        // Cloze: article + plus + adjective as the chunk.
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("sup:{}:cl", slug2(subj)),
            level: LEVEL,
            topic: "Superlative",
            topic_slug: Some("superlative".into()),
            task: vec!["Complete with the superlative.".into()],
            head: format!("{} {} ", subj, verb),
            tail: " de tous.".into(),
            infinitive: adj,
            answer: sup.to_string(),
            rule: vec![
                format!("{} {} {} de tous.", subj, verb, sup),
                "le/la/les plus + adjective — the article agrees with the noun.".into(),
            ],
        }));
        // Choice: the article before plus (or the irregular chunk).
        let wrong_chunk = if let Some(rest) = sup.strip_prefix("la ") {
            format!("le {}", rest)
        } else if let Some(rest) = sup.strip_prefix("les ") {
            format!("le {}", rest)
        } else {
            format!("la {}", sup.strip_prefix("le ").unwrap())
        };
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("sup:{}:ch", slug2(subj)),
            level: LEVEL,
            topic: "Superlative",
            topic_slug: Some("superlative".into()),
            task: vec!["Choose the right superlative.".into()],
            sentence: format!("{} {} ____ de tous.", subj, verb),
            options: vec![sup.to_string(), wrong_chunk],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("{} {} {} de tous.", subj, verb, sup),
                if sup_art == "le" {
                    "Masculine singular → le plus…".into()
                } else if sup_art == "la" {
                    "Feminine singular → la plus…".into()
                } else {
                    "Plural → les plus…".into()
                },
            ],
        }));
    }
    out
}

fn slug2(s: &str) -> String {
    super::shared::slugify(s)
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "comparative" => comparative_items(),
        "superlative" => superlative_items(),
        _ => Vec::new(),
    }
}
