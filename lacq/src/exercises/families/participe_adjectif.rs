//! Family: participle vs verbal adjective — the present participle
//! describes the thing that acts (un film amusant = it amuses), the
//! past participle / adjective describes the thing affected (les
//! enfants amusés = they are amused). Pairs: amusant/amusé,
//! fatigant/fatigué, étonnant/étonné, ennuyeux/ennuyé, inquiétant/
//! inquiet.

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem};

use super::shared::punctuate;

const LEVEL: &str = "B2";

/// (sentence with ____, correct adjective, wrong, cue, rule)
const PVA_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Ce film est vraiment ____.", "amusant", "amusé", "it amuses people", "amusant = qui amuse (the thing that acts)."),
    ("Les enfants sont ____ par le film.", "amusés", "amusants", "they are amused by it", "amusés = qu'on amuse (the people affected)."),
    ("Ce travail de nuit est ____.", "fatigant", "fatigué", "it tires people", "fatigant = qui fatigue (note the single t)."),
    ("Je suis ____ après ce travail de nuit.", "fatigué", "fatigant", "I am tired", "fatigué = qui est fatigué (the person affected, -gué)."),
    ("Une nouvelle ____ vient d'arriver.", "étonnante", "étonnée", "it astonishes people", "étonnant = qui étonne."),
    ("Je suis ____ de ton silence.", "étonné", "étonnant", "I am astonished", "étonné = qui est étonné."),
    ("Un voisin ____ a déménagé.", "ennuyeux", "ennuyé", "he bores people", "ennuyeux = qui ennuie."),
    ("Je suis ____ de t'attendre.", "ennuyé", "ennuyeux", "I am bored", "ennuyé = qui s'ennuie."),
    ("Un bruit ____ vient du sous-sol.", "inquiétant", "inquiet", "it worries people", "inquiétant = qui inquiète."),
    ("Elle est ____ pour son fils.", "inquiète", "inquiétante", "she worries about him", "inquiet/inquiète = qui est inquiet."),
    ("Des étudiants ____ ont posé des questions.", "intéressés", "intéressants", "they are interested", "intéressés = qui s'intéressent."),
    ("Un livre ____ se vend bien.", "intéressant", "intéressé", "it interests readers", "intéressant = qui intéresse."),
];

fn pva_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in PVA_ROWS {
        let key = super::shared::slugify(&fr.chars().take(24).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("pva:{}:ch", key),
            level: LEVEL,
            topic: "Participle vs verbal adjective",
            topic_slug: Some("participe-ou-adjectif".into()),
            task: vec![format!("{} — choose the adjective.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("pva:{}:cl", key),
            level: LEVEL,
            topic: "Participle vs verbal adjective",
            topic_slug: Some("participe-ou-adjectif".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "adjective",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Error-spot: fatigant vs fatigué confusion.
    out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
        id: "pva:fatigue:es".into(),
        level: LEVEL,
        topic: "Participle vs verbal adjective",
        topic_slug: Some("participe-ou-adjectif".into()),
        task: vec!["One word is wrong. Tap it.".into()],
        words: punctuate(vec![
            "Je".into(),
            "suis".into(),
            "fatigant".into(),
            "aujourd'hui".into(),
        ]),
        wrong: 2,
        fix: "fatigué".into(),
        fix_task: "Now write the correct form.",
        rule: vec![
            "Je suis fatigué aujourd'hui.".into(),
            "The person who is tired is fatigué; the thing that tires is fatigant.".into(),
        ],
    }));
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "participe-ou-adjectif" => pva_items(),
        _ => Vec::new(),
    }
}
