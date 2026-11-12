//! Family: nominalisation — turning verbs into nouns (décider → la
//! décision) with their gender. Cloze picks the noun, choice drills
//! the article/gender, transform rebuilds the sentence around it.

use crate::exercises::{ChoiceItem, ClozeItem, ExerciseItem, TransformItem};

const LEVEL: &str = "B1";

/// (verb phrase stem, conjugated 3pl passé composé, article, noun, feminine)
const NOM_ROWS: &[(&str, &str, &str, &str, bool)] = &[
    ("décider", "ont décidé", "la", "décision", true),
    ("expliquer", "ont expliqué", "l'", "explication", true),
    ("choisir", "ont choisi", "le", "choix", false),
    ("réussir", "ont réussi", "la", "réussite", true),
    ("échouer", "ont échoué", "l'", "échec", false),
    ("construire", "ont construit", "la", "construction", true),
    ("détruire", "ont détruit", "la", "destruction", true),
    ("produire", "ont produit", "la", "production", true),
    ("vendre", "ont vendu", "la", "vente", true),
    ("développer", "ont développé", "le", "développement", false),
    ("améliorer", "ont amélioré", "l'", "amélioration", true),
    ("organiser", "ont organisé", "l'", "organisation", true),
    ("participer", "ont participé", "la", "participation", true),
    ("grandir", "ont grandi", "la", "croissance", true),
];

/// « l'explication » or « la décision »
fn article_noun(art: &str, noun: &str) -> String {
    if art == "l'" {
        format!("l'{}", noun)
    } else {
        format!("{} {}", art, noun)
    }
}

fn nom_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(verb, pc, art, noun, fem) in NOM_ROWS {
        let noun_full = article_noun(art, noun);
        // Cloze: the noun in a natural frame.
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("nom:{}:cl", verb),
            level: LEVEL,
            topic: "Nominalization",
            topic_slug: Some("nominalization".into()),
            task: vec!["Put the verb into its noun form.".into()],
            head: "Ils ont annoncé ".into(),
            tail: " du projet.".into(),
            infinitive: verb,
            answer: noun_full.clone(),
            rule: vec![
                format!("Ils ont annoncé {} du projet.", noun_full),
                format!("{} {} — learn the gender with the noun.", art, noun),
            ],
        }));
        // Choice: the article/gender.
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("nom:{}:ch", verb),
            level: LEVEL,
            topic: "Nominalization",
            topic_slug: Some("nominalization".into()),
            task: vec!["Choose the noun with its article.".into()],
            sentence: "Ils ont annoncé ____ du projet.".into(),
            options: vec![
                noun_full.clone(),
                article_noun(if fem { "le" } else { "la" }, noun),
            ],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("Ils ont annoncé {} du projet.", noun_full),
                format!("{} {} — the gender has to be learned with the noun.", art, noun),
            ],
        }));
        // Transform: verb sentence → noun sentence (participle agrees).
        let announced = if fem { "annoncée" } else { "annoncé" };
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("nom:{}:tf", verb),
            level: LEVEL,
            topic: "Nominalization",
            topic_slug: Some("nominalization".into()),
            task: vec!["Say it with the noun form.".into()],
            source: format!("Ils {} ce projet.", pc),
            answer: format!("{} du projet a été {}.", cap_noun(&noun_full), announced),
            rule: vec![
                format!("{} du projet a été {}.", cap_noun(&noun_full), announced),
                "Verb → noun: the sentence reorganises around it.".into(),
            ],
        }));
    }
    out
}

/// capitalize the noun, keeping the elided article attached
fn cap_noun(noun_full: &str) -> String {
    if let Some(rest) = noun_full.strip_prefix("l'") {
        format!("L'{}", rest)
    } else if let Some(rest) = noun_full.strip_prefix("la ") {
        format!("La {}", rest)
    } else if let Some(rest) = noun_full.strip_prefix("le ") {
        format!("Le {}", rest)
    } else {
        noun_full.to_string()
    }
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "nominalization" => nom_items(),
        _ => Vec::new(),
    }
}
