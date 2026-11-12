//! Family: articles — the noun-group cluster, four topics. Gender of
//! nouns (un/une choice + error-spot), plural of nouns (computed
//! plurals: -s, -x, -aux, invariable), definite ↔ indefinite swaps
//! (with l' elision), and partitive articles (du/de la/de l'/des, and
//! the negative rule: pas de).

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem, TransformItem};

use super::shared::{SUBJ_CAP, punctuate, starts_vowel};

const LEVEL: &str = "A1";

/* ── gender of nouns ────────────────────────────────────────────── */

const GENDER_NOUNS: &[(&str, bool)] = &[
    ("problème", false),
    ("livre", false),
    ("chien", false),
    ("jardin", false),
    ("hôtel", false),
    ("film", false),
    ("voyage", false),
    ("musée", false),
    ("restaurant", false),
    ("week-end", false),
    ("table", true),
    ("maison", true),
    ("idée", true),
    ("école", true),
    ("soirée", true),
    ("question", true),
    ("pharmacie", true),
    ("clé", true),
    ("montre", true),
    ("voiture", true),
];

fn gender_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(noun, feminine) in GENDER_NOUNS {
        let (right, wrong) = if feminine { ("une", "un") } else { ("un", "une") };
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("gen:{}:ch", noun),
            level: LEVEL,
            topic: "Gender of nouns",
            topic_slug: Some("gender-of-nouns".into()),
            task: vec!["Choose the right article.".into()],
            sentence: format!("____ {}.", noun),
            options: vec![right.into(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("{} is {} — {} {}.", noun, if feminine { "feminine" } else { "masculine" }, right, noun),
                "The gender is a property of the word — learn it with the noun.".into(),
            ],
        }));
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("gen:{}:es", noun),
            level: LEVEL,
            topic: "Gender of nouns",
            topic_slug: Some("gender-of-nouns".into()),
            task: vec!["One word is wrong. Tap it.".into()],
            words: punctuate(vec![wrong.to_string(), noun.to_string()]),
            wrong: 0,
            fix: right.into(),
            fix_task: "Now write the correct form.",
            rule: vec![format!("{} {} — the article agrees with the gender.", right, noun)],
        }));
    }
    out
}

/* ── plural of nouns ────────────────────────────────────────────── */

/// (singular, plural) — the classic patterns: -s, -x, -aux, invariable.
const PLURAL_NOUNS: &[(&str, &str)] = &[
    ("livre", "livres"),
    ("clé", "clés"),
    ("table", "tables"),
    ("chapeau", "chapeaux"),
    ("cadeau", "cadeaux"),
    ("château", "châteaux"),
    ("cheval", "chevaux"),
    ("journal", "journaux"),
    ("animal", "animaux"),
    ("bijou", "bijoux"),
    ("genou", "genoux"),
    ("bus", "bus"),
    ("choix", "choix"),
    ("mois", "mois"),
    ("prix", "prix"),
    ("nez", "nez"),
];

fn plural_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(sg, pl) in PLURAL_NOUNS {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("plur:{}:tf", sg),
            level: LEVEL,
            topic: "Plural of nouns",
            topic_slug: Some("plural-of-nouns".into()),
            task: vec!["Write it with « deux » — make the noun plural.".into()],
            source: format!("un {}", sg),
            answer: format!("deux {}", pl),
            rule: vec![format!("un {} → deux {}.", sg, pl), plural_rule_note(sg, pl).into()],
        }));
        // Choice on every noun — the non-trivial patterns get a real
        // distractor, the -s nouns get the un-marked singular.
        let (right, wrong) = if pl == sg {
            (pl.to_string(), format!("{}s", sg))
        } else if pl.ends_with("aux") {
            (pl.to_string(), format!("{}als", sg))
        } else if pl.ends_with('x') {
            (pl.to_string(), format!("{}s", sg))
        } else {
            (pl.to_string(), sg.to_string())
        };
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("plur:{}:ch", sg),
            level: LEVEL,
            topic: "Plural of nouns",
            topic_slug: Some("plural-of-nouns".into()),
            task: vec!["Choose the right plural.".into()],
            sentence: "deux ____.".into(),
            options: vec![right, wrong],
            correct: 0, // shuffled in `generate`
            rule: vec![format!("deux {}.", pl), plural_rule_note(sg, pl).into()],
        }));
    }
    out
}

fn plural_rule_note(sg: &str, pl: &str) -> &'static str {
    if pl == sg {
        "Words ending in -s, -x, -z don't change: the plural is invisible."
    } else if pl.ends_with("aux") {
        "-al becomes -aux in the plural."
    } else if pl.ends_with('x') && sg.ends_with("ou") {
        "-ou takes -x (bijou, genou, caillou… — the famous seven)."
    } else if pl.ends_with('x') {
        "-eau takes -x in the plural."
    } else {
        "Most words just take -s."
    }
}

/* ── definite ↔ indefinite articles ─────────────────────────────── */

/// (indefinite, definite) — elision included.
const DET_SWAPS: &[(&str, &str)] = &[
    ("un livre", "le livre"),
    ("une table", "la table"),
    ("un ami", "l'ami"),
    ("une école", "l'école"),
    ("un hôtel", "l'hôtel"),
    ("une idée", "l'idée"),
    ("un chien", "le chien"),
    ("une montre", "la montre"),
    ("un voyage", "le voyage"),
    ("une question", "la question"),
    ("un musée", "le musée"),
    ("une pharmacie", "la pharmacie"),
    ("des livres", "les livres"),
    ("des clés", "les clés"),
    ("des amis", "les amis"),
    ("des enfants", "les enfants"),
];

fn det_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(indef, def) in DET_SWAPS {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("det:{}:tf", super::shared::slugify(indef)),
            level: LEVEL,
            topic: "Definite & indefinite articles",
            topic_slug: Some("definite-indefinite-articles".into()),
            task: vec!["Rewrite it with the definite article.".into()],
            source: format!("{}", indef),
            answer: format!("{}", def),
            rule: vec![format!("{} → {}.", indef, def), det_rule_note(def).into()],
        }));
        // Choice: the same noun with a possessor — definite wins.
        let noun = def.trim_start_matches("le ")
            .trim_start_matches("la ")
            .trim_start_matches("les ")
            .trim_start_matches("l'");
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("det:{}:ch", super::shared::slugify(indef)),
            level: LEVEL,
            topic: "Definite & indefinite articles",
            topic_slug: Some("definite-indefinite-articles".into()),
            task: vec!["Choose the right article.".into()],
            sentence: format!("____ {} de Paul est là.", noun),
            options: vec![def_article(def).into(), indef.split(' ').next().unwrap().into()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                "With « de + someone », the thing is specific → definite.".into(),
                format!("{} {} de Paul.", def_article(def), noun),
            ],
        }));
    }
    out
}

/// The definite article alone (le / la / les / l').
fn def_article(def: &str) -> &str {
    let article = def.split(' ').next().unwrap();
    if article.starts_with('l') && article != "les" && article != "la" && article != "le" {
        "L'"
    } else if article == "le" {
        "Le"
    } else if article == "la" {
        "La"
    } else {
        "Les"
    }
}

fn det_rule_note(def: &str) -> &'static str {
    if def.starts_with("l'") {
        "Before a vowel, le/la becomes l'."
    } else if def.starts_with("les") {
        "Plural: both un/une and des become les."
    } else {
        "un/une → le/la — the article follows the gender."
    }
}

/* ── partitive articles ─────────────────────────────────────────── */

/// (food, partitive article) — du / de la / de l'.
const FOODS: &[(&str, &str)] = &[
    ("pain", "du"),
    ("beurre", "du"),
    ("chocolat", "du"),
    ("fromage", "du"),
    ("riz", "du"),
    ("lait", "du"),
    ("confiture", "de la"),
    ("salade", "de la"),
    ("viande", "de la"),
    ("soupe", "de la"),
    ("eau", "de l'"),
    ("eau minérale", "de l'"),
];

fn partitive_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(food, art) in FOODS {
        for &(subj, subj_c, _fi) in &SUBJ_CAP[0..4] {
            let subject = if subj == "je" { "Je" } else { subj_c };
            // Cloze: article + noun as the answer chunk.
            out.push(ExerciseItem::Cloze(ClozeItem {
                id: format!("part:{}:{}:cl", food, subj),
                level: LEVEL,
                topic: "Partitive articles",
                topic_slug: Some("partitive-articles".into()),
                task: vec!["Complete with the partitive article.".into()],
                head: format!("{} mange ", subject),
                tail: ".".into(),
                infinitive: "manger",
                answer: format!("{} {}", art, food),
                rule: vec![
                    format!("{} mange {} {}.", subject, art, food),
                    "du/de la/de l' = some (an unspecified amount).".into(),
                ],
            }));
            // Transform: the negative takes plain de (d' before a vowel).
            let neg_art = if starts_vowel(food) || food.starts_with('h') { "d'" } else { "de" };
            out.push(ExerciseItem::Transform(TransformItem {
                id: format!("part:{}:{}:tf", food, subj),
                level: LEVEL,
                topic: "Partitive articles",
                topic_slug: Some("partitive-articles".into()),
                task: vec!["Make it negative — watch the article.".into()],
                source: format!("{} mange {} {}.", subject, art, food),
                answer: format!("{} ne mange pas {}{}.", subject, neg_art, food),
                rule: vec![
                    "In the negative, du/de la/de l' become de (d' before a vowel).".into(),
                    format!("{} ne mange pas {}{}.", subject, neg_art, food),
                ],
            }));
        }
    }
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "gender-of-nouns" => gender_items(),
        "plural-of-nouns" => plural_items(),
        "definite-indefinite-articles" => det_items(),
        "partitive-articles" => partitive_items(),
        _ => Vec::new(),
    }
}
