//! Family: adjectives — the describing cluster, five topics.
//! Agreement (masculine → feminine transforms + feminine choice),
//! position (pre vs post noun order), possessive adjectives (agree
//! with the THING owned, not the owner), demonstrative adjectives
//! (ce/cet/cette/ces with elision), and quantity adverbs (très +
//! adjective vs beaucoup de + noun).

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem, TransformItem};

use super::shared::{punctuate, slugify, starts_vowel};

const LEVEL: &str = "A1";

fn vowel(noun: &str) -> bool {
    starts_vowel(noun) || noun.starts_with('h')
}

/* ── adjective agreement ────────────────────────────────────────── */

/// (masculine singular, feminine singular)
const AGREEMENT_ADJ: &[(&str, &str)] = &[
    ("grand", "grande"),
    ("petit", "petite"),
    ("vert", "verte"),
    ("noir", "noire"),
    ("bleu", "bleue"),
    ("français", "française"),
    ("bon", "bonne"),
    ("content", "contente"),
    ("sportif", "sportive"),
    ("gros", "grosse"),
    ("blanc", "blanche"),
    ("intelligent", "intelligente"),
    ("canadien", "canadienne"),
    ("roux", "rousse"),
];

/// (masculine subject, feminine subject)
const GENDER_PAIRS: &[(&str, &str)] = &[
    ("Le garçon", "La fille"),
    ("L'homme", "La femme"),
    ("Le chien", "La chienne"),
    ("Le cousin", "La cousine"),
    ("L'ami", "L'amie"),
    ("Le chat", "La chatte"),
];

fn agreement_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(m, f) in AGREEMENT_ADJ {
        for &(masc, fem) in GENDER_PAIRS {
            out.push(ExerciseItem::Transform(TransformItem {
                id: format!("aga:{}:{}:tf", m, slugify(fem)),
                level: LEVEL,
                topic: "Adjective agreement",
                topic_slug: Some("adjective-agreement".into()),
                task: vec![
                    "Say the same thing about the other person —".into(),
                    "the adjective must agree.".into(),
                ],
                source: format!("{} est {}.", masc, m),
                answer: format!("{} est {}.", fem, f),
                rule: vec![
                    format!("{} (masculine) → {} (feminine).", m, f),
                    format!("{} est {}.", fem, f),
                ],
            }));
        }
        // Choice: the feminine after « une ».
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("aga:{}:ch", m),
            level: LEVEL,
            topic: "Adjective agreement",
            topic_slug: Some("adjective-agreement".into()),
            task: vec!["Choose the right form.".into()],
            sentence: "C'est une voiture ____.".into(),
            options: vec![f.to_string(), m.to_string()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("voiture is feminine → {}.", f),
                format!("une voiture {}.", f),
            ],
        }));
    }
    out
}

/* ── adjective position ─────────────────────────────────────────── */

/// (wrong order, right order) — BAGS adjectives go before the noun.
const POSITION_PAIRS: &[(&str, &str)] = &[
    ("une maison grande", "une grande maison"),
    ("un jardin grand", "un grand jardin"),
    ("une maison petite", "une petite maison"),
    ("un chien petit", "un petit chien"),
    ("un jardin beau", "un beau jardin"),
    ("un garçon beau", "un beau garçon"),
    ("une fleur jolie", "une jolie fleur"),
    ("une femme jolie", "une jolie femme"),
    ("un film long", "un long film"),
    ("un voyage long", "un long voyage"),
    ("un livre nouveau", "un nouveau livre"),
    ("un château vieux", "un vieux château"),
    ("un café bon", "un bon café"),
    ("un restaurant bon", "un bon restaurant"),
    ("une femme jeune", "une jeune femme"),
    ("un chat gros", "un gros chat"),
];

/// Post-position adjectives (the default in French).
const POST_ADJ: &[(&str, &str)] = &[
    ("rouge", "voiture"),
    ("intéressant", "film"),
    ("facile", "question"),
    ("difficile", "exercice"),
    ("noir", "chat"),
    ("content", "enfant"),
];

fn position_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(wrong_order, right_order) in POSITION_PAIRS {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("apos:{}:tf", slugify(right_order)),
            level: LEVEL,
            topic: "Adjective position",
            topic_slug: Some("adjective-position".into()),
            task: vec![
                "Rewrite it correctly —".into(),
                "this adjective goes before the noun.".into(),
            ],
            source: format!("{}.", wrong_order),
            answer: format!("{}.", right_order),
            rule: vec![
                "Beauty, age, goodness, size → before the noun (BAGS).".into(),
                format!("{}.", right_order),
            ],
        }));
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("apos:{}:ch", slugify(right_order)),
            level: LEVEL,
            topic: "Adjective position",
            topic_slug: Some("adjective-position".into()),
            task: vec!["Choose the right order.".into()],
            sentence: "C'est ____.".into(),
            options: vec![right_order.to_string(), wrong_order.to_string()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("Before the noun: {}.", right_order),
                "Beauty, age, goodness, size (BAGS) go first.".into(),
            ],
        }));
    }
    for &(adj, noun) in POST_ADJ {
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("apos:{}:{}:ch", adj, noun),
            level: LEVEL,
            topic: "Adjective position",
            topic_slug: Some("adjective-position".into()),
            task: vec!["Choose the right order.".into()],
            sentence: "C'est une ____.".into(),
            options: vec![
                format!("{} {}", noun, adj),
                format!("{} {}", adj, noun),
            ],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("Most adjectives — like {} — follow the noun.", adj),
                format!("une {} {}.", noun, adj),
            ],
        }));
    }
    out
}

/* ── possessive adjectives ──────────────────────────────────────── */

/// (noun, feminine?) — the THING owned sets the agreement.
const POSS_NOUNS: &[(&str, bool)] = &[
    ("frère", false),
    ("père", false),
    ("oncle", false),
    ("professeur", false),
    ("voisin", false),
    ("livre", false),
    ("sœur", true),
    ("mère", true),
    ("tante", true),
    ("voisine", true),
    ("copine", true),
    ("voiture", true),
    ("amie", true),
    ("idée", true),
];

/// (possessor, masculine, feminine, plural)
const OWNERS: &[(&str, &str, &str, &str)] = &[
    ("je", "mon", "ma", "mes"),
    ("tu", "ton", "ta", "tes"),
    ("il", "son", "sa", "ses"),
    ("nous", "notre", "notre", "nos"),
    ("vous", "votre", "votre", "vos"),
    ("ils", "leur", "leur", "leurs"),
];

fn possessive_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(noun, feminine) in POSS_NOUNS {
        let plural = noun.ends_with('s');
        for &(owner, m, f, p) in OWNERS {
            let right = if plural { p } else if feminine { f } else { m };
            let wrong = if plural { m } else if feminine { m } else { f };
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("poss:{}:{}:ch", owner, noun),
                level: LEVEL,
                topic: "Possessive adjectives",
                topic_slug: Some("possessive-adjectives".into()),
                task: vec!["Choose the right possessive.".into()],
                sentence: format!("C'est ____ {}. (owner: {})", noun, owner),
                options: vec![right.to_string(), wrong.to_string()],
                correct: 0, // shuffled in `generate`
                rule: vec![
                    format!("The possessive agrees with « {} », not with the owner.", noun),
                    format!("{} → {} {}.", owner, right, noun),
                ],
            }));
        }
        // Transform: « le X de Paul » → son X — son stays son even for
        // a woman's car.
        let poss = if plural { "ses" } else if feminine { "sa" } else { "son" };
        let article = if vowel(noun) { "l'" } else if feminine { "la" } else { "le" };
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("poss:{}:tf", noun),
            level: LEVEL,
            topic: "Possessive adjectives",
            topic_slug: Some("possessive-adjectives".into()),
            task: vec!["Rewrite it with a possessive (owner: Paul).".into()],
            source: format!("C'est {}{} de Paul.", article, noun),
            answer: format!("C'est {} {}.", poss, noun),
            rule: vec![
                format!("son/sa/ses agrees with « {} »:", noun),
                format!("son {} — son, even when Paul is a woman!", noun),
            ],
        }));
    }
    out
}

/* ── demonstrative adjectives ───────────────────────────────────── */

/// (noun, feminine?) — plural detection by the final s.
const DEMO_NOUNS: &[(&str, bool)] = &[
    ("chien", false),
    ("livre", false),
    ("train", false),
    ("jardin", false),
    ("arbre", false),
    ("hôtel", false),
    ("ami", false),
    ("table", true),
    ("maison", true),
    ("idée", true),
    ("école", true),
    ("voiture", true),
    ("chiens", false),
    ("livres", false),
    ("tables", true),
    ("idées", true),
];

fn demonstrative_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(noun, feminine) in DEMO_NOUNS {
        let plural = noun.ends_with('s');
        let (right, wrong) = if plural {
            ("ces", if feminine { "cettes" } else { "cet" })
        } else if feminine {
            ("cette", "ce")
        } else if vowel(noun) {
            ("cet", "ce")
        } else {
            ("ce", "cette")
        };
        let note = if plural {
            "ces for every plural — no gender."
        } else if feminine {
            "cette for feminine singular."
        } else if vowel(noun) {
            "ce becomes cet before a vowel."
        } else {
            "ce for masculine singular."
        };
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("demo:{}:ch", noun),
            level: LEVEL,
            topic: "Demonstrative adjectives",
            topic_slug: Some("demonstrative-adjectives".into()),
            task: vec!["Choose the right demonstrative.".into()],
            sentence: format!("____ {} est là.", noun),
            options: vec![right.into(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![format!("{} {}.", right, noun), note.into()],
        }));
        // Error-spot on the classic slips (cet chien / ce arbre), and
        // the fake plural (cettes / cet + plural noun).
        let wrong_es = if plural {
            if feminine { "cettes" } else { "cet" }
        } else {
            wrong
        };
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("demo:{}:es", noun),
            level: LEVEL,
            topic: "Demonstrative adjectives",
            topic_slug: Some("demonstrative-adjectives".into()),
            task: vec!["One word is wrong. Tap it.".into()],
            words: punctuate(vec![wrong_es.to_string(), noun.to_string()]),
            wrong: 0,
            fix: right.into(),
            fix_task: "Now write the correct form.",
            rule: vec![format!("{} {}.", right, noun), note.into()],
        }));
    }
    out
}

/* ── quantity & intensity adverbs ───────────────────────────────── */

/// (adjective, sentence lead)
const INTENSITY: &[(&str, &str)] = &[
    ("long", "Ce film est"),
    ("gentil", "Ce monsieur est"),
    ("fatigué", "Il est"),
    ("content", "Elle est"),
    ("difficile", "Cet exercice est"),
    ("cher", "Ce sac est"),
    ("grand", "Ce jardin est"),
    ("simple", "Cette recette est"),
    ("poli", "Ce serveur est"),
    ("froid", "Il fait"),
    ("prêt", "Elle est"),
    ("lourd", "Ce sac est"),
];

/// (noun, elision piece after beaucoup)
const QUANTITY: &[(&str, &str)] = &[
    ("amis", "d'"),
    ("travail", "de"),
    ("livres", "de"),
    ("argent", "d'"),
    ("chances", "de"),
    ("temps", "de"),
    ("voitures", "de"),
    ("idées", "d'"),
    ("questions", "de"),
    ("sucre", "de"),
    ("dettes", "de"),
    ("histoires", "d'"),
];

fn quantity_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(adj, lead) in INTENSITY {
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("qua:{}:ch", slugify(adj)),
            level: LEVEL,
            topic: "Quantity & intensity adverbs",
            topic_slug: Some("quantity-adverbs".into()),
            task: vec!["Choose the right adverb.".into()],
            sentence: format!("{} ____ {}.", lead, adj),
            options: vec!["très".into(), "beaucoup".into()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                "très + adjective — beaucoup never touches an adjective.".into(),
                format!("{} très {}.", lead, adj),
            ],
        }));
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("qua:{}:es", slugify(adj)),
            level: LEVEL,
            topic: "Quantity & intensity adverbs",
            topic_slug: Some("quantity-adverbs".into()),
            task: vec!["One word is wrong. Tap it.".into()],
            words: punctuate(vec![lead.to_string(), "beaucoup".to_string(), adj.to_string()]),
            wrong: 1,
            fix: "très".into(),
            fix_task: "Now write the correct form.",
            rule: vec![format!("{} très {}.", lead, adj)],
        }));
    }
    for &(noun, el) in QUANTITY {
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("qua:{}:cl", noun),
            level: LEVEL,
            topic: "Quantity & intensity adverbs",
            topic_slug: Some("quantity-adverbs".into()),
            task: vec!["Complete with « beaucoup ».".into()],
            head: "Elle a ".into(),
            tail: format!(" {}.", noun),
            infinitive: "beaucoup",
            answer: format!("beaucoup {}{}", el, noun),
            rule: vec![
                format!("beaucoup {}{} — beaucoup takes de (d' before a vowel).", el, noun),
                format!("Elle a beaucoup {}{}.", el, noun),
            ],
        }));
    }
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "adjective-agreement" => agreement_items(),
        "adjective-position" => position_items(),
        "possessive-adjectives" => possessive_items(),
        "demonstrative-adjectives" => demonstrative_items(),
        "quantity-adverbs" => quantity_items(),
        _ => Vec::new(),
    }
}
