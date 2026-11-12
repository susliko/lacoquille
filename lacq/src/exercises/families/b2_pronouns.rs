//! Family: interrogative lequel — lequel / laquelle / lesquels /
//! lesquelles as a question word, agreeing with the thing asked about
//! (laquelle de ces robes préfères-tu ?).

use crate::exercises::{ChoiceItem, ClozeItem, ExerciseItem, ErrorSpotItem};

use super::shared::punctuate;

const LEVEL: &str = "B2";

/// (question with ____, correct form, wrong form, cue, rule)
const LQ_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("____ de ces voitures est la tienne ?", "Laquelle", "Lequel", "une voiture", "laquelle agrees with « ces voitures » (feminine plural)."),
    ("____ de ces livres veux-tu ?", "Lequel", "Laquelle", "un livre", "lequel agrees with « ces livres » (masculine plural)."),
    ("____ de tes amis viennent ?", "Lesquels", "Lesquelles", "des amis (male)", "lesquels for masculine plurals."),
    ("____ de ces fleurs cueilles-tu ?", "Lesquelles", "Lesquels", "des fleurs", "lesquelles for feminine plurals."),
    ("Voici deux clés. ____ ouvre le garage ?", "Laquelle", "Lequel", "une clé", "Back-reference too: la clé → laquelle."),
    ("Des plats régionaux : ____ préfères-tu ?", "Lesquels", "Lesquelles", "des plats", "le plat → lesquels."),
    ("____ de tes sœurs ressemble à ta mère ?", "Laquelle", "Lequel", "une sœur", "la sœur → laquelle."),
    ("Trois chemins mènent au lac. ____ prends-tu ?", "Lequel", "Laquelle", "un chemin", "le chemin → lequel."),
];

fn lq_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in LQ_ROWS {
        let key = super::shared::slugify(&fr.chars().take(24).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("lq:{}:ch", key),
            level: LEVEL,
            topic: "Interrogative lequel",
            topic_slug: Some("lequel-interrogatif".into()),
            task: vec![format!("{} — choose the form.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("lq:{}:cl", key),
            level: LEVEL,
            topic: "Interrogative lequel",
            topic_slug: Some("lequel-interrogatif".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "lequel",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Error-spot: agreement flipped.
    out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
        id: "lq:robes:es".into(),
        level: LEVEL,
        topic: "Interrogative lequel",
        topic_slug: Some("lequel-interrogatif".into()),
        task: vec!["One word is wrong. Tap it.".into()],
        words: punctuate(vec![
            "Lequel".into(),
            "de".into(),
            "ces".into(),
            "robes".into(),
            "préfères-tu".into(),
        ]),
        wrong: 0,
        fix: "Laquelle".into(),
        fix_task: "Now write the correct form.",
        rule: vec![
            "Laquelle de ces robes préfères-tu ?".into(),
            "lequel agrees with the noun after « de »: les robes → laquelle (asking about one).".into(),
        ],
    }));
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "lequel-interrogatif" => lq_items(),
        _ => Vec::new(),
    }
}
