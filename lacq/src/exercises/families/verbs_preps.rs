//! Family: verbs with prepositions — which of à / de joins a
//! conjugated verb to its infinitive (aider à, essayer de, oublier
//! de…). Cloze hides the preposition + infinitive chunk, choice
//! drills the à/de pick alone.

use crate::exercises::{ChoiceItem, ClozeItem, ExerciseItem};

use super::shared::{SUBJ_CAP, starts_vowel};

const LEVEL: &str = "A2";

/// (verb, forms je..ils, à?)
const PREP_VERBS: &[(&str, [&str; 6], bool)] = &[
    ("aider", ["aide", "aides", "aide", "aidons", "aidez", "aident"], true),
    ("apprendre", ["apprends", "apprends", "apprend", "apprenons", "apprenez", "apprennent"], true),
    ("réussir", ["réussis", "réussis", "réussit", "réussissons", "réussissez", "réussissent"], true),
    ("penser", ["pense", "penses", "pense", "pensons", "pensez", "pensent"], true),
    ("commencer", ["commence", "commences", "commence", "commençons", "commencez", "commencent"], true),
    ("essayer", ["essaie", "essaies", "essaie", "essayons", "essayez", "essaient"], false),
    ("décider", ["décide", "décides", "décide", "décidons", "décidez", "décident"], false),
    ("oublier", ["oublie", "oublies", "oublie", "oublions", "oubliez", "oublient"], false),
    ("finir", ["finis", "finis", "finit", "finissons", "finissez", "finissent"], false),
    ("refuser", ["refuse", "refuses", "refuse", "refusons", "refusez", "refusent"], false),
];

const SECOND_VERBS: &[&str] = &[
    "partir",
    "manger",
    "dormir",
    "venir",
    "travailler",
    "lire",
];

fn subj_aff<'a>(subj: &'a str, subj_c: &'a str, form: &str) -> &'a str {
    if subj == "je" && starts_vowel(form) {
        "J'"
    } else {
        subj_c
    }
}

pub(crate) fn items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(inf, forms, takes_a) in PREP_VERBS {
        let prep = if takes_a { "à" } else { "de" };
        for second in SECOND_VERBS {
            for &(subj, subj_c, fi) in SUBJ_CAP {
                let form = forms[fi];
                let head = subj_aff(subj, subj_c, form);
                // Cloze: the preposition is the answer, infinitive in tail.
                out.push(ExerciseItem::Cloze(ClozeItem {
                    id: format!("vp:{}:{}:{}:cl", inf, second, subj),
                    level: LEVEL,
                    topic: "Verbs with prepositions",
                    topic_slug: Some("verbs-prepositions".into()),
                    task: vec!["Complete with the right preposition.".into()],
                    head: format!("{} ", head),
                    tail: format!(" {}.", second),
                    infinitive: inf,
                    answer: prep.to_string(),
                    rule: vec![
                        format!("{} {} {} {}.", head, form, prep, second),
                        format!("{} takes {} before the infinitive.", inf, prep),
                    ],
                }));
            }
            // Choice: just the preposition, fixed subject (il).
            let wrong = if takes_a { "de" } else { "à" };
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("vp:{}:{}:ch", inf, second),
                level: LEVEL,
                topic: "Verbs with prepositions",
                topic_slug: Some("verbs-prepositions".into()),
                task: vec!["Choose the right preposition.".into()],
                sentence: format!("Il {} ____ {}.", forms[2], second),
                options: vec![prep.into(), wrong.into()],
                correct: 0, // shuffled in `generate`
                rule: vec![
                    format!("{} {} {} {}.", "Il", forms[2], prep, second),
                    format!("{} + {} + infinitive.", inf, prep),
                ],
            }));
        }
    }
    out
}
