//! Family: auxiliary choice (choice + error-spot).

use crate::exercises::{ChoiceItem, ErrorSpotItem, ExerciseItem};

use super::shared::{AVOIR_VERBS, ETRE_VERBS, PRONS, cap, punctuate};

const LEVEL: &str = "A2";

pub(crate) fn items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    // Être verbs: the trap is avoir.
    for v in ETRE_VERBS {
        for pr in PRONS {
            let agreed = pr.participle(v.p);
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("pc-aux:{}:{}:choice", v.inf, pr.p),
                level: LEVEL,
                topic: "Auxiliary choice",
                topic_slug: Some("pc-aux".into()),
                task: vec!["Choose the right form.".into()],
                sentence: format!("Hier, {} ____ {}.", pr.p, v.complement),
                options: vec![
                    format!("{} {}", pr.etre, agreed),
                    format!("{} {}", pr.avoir, agreed),
                ],
                correct: 0, // shuffled in `generate`
                rule: vec![
                    format!("{} takes être in the passé composé.", cap(v.inf)),
                    "Arrival and movement verbs conjugate with être, not avoir.".into(),
                ],
            }));

            let mut words = vec![cap(pr.p), pr.avoir.to_string(), agreed.clone()];
            words.extend(v.complement.split(' ').map(|w| w.to_string()));
            let words = punctuate(words);
            out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
                id: format!("pc-aux:{}:{}:es", v.inf, pr.p),
                level: LEVEL,
                topic: "Auxiliary choice",
                topic_slug: Some("pc-aux".into()),
                task: vec!["One word is wrong. Tap it.".into()],
                words,
                wrong: 1,
                fix: pr.etre.to_string(),
                fix_task: "Now write the correct form.",
                rule: vec![
                    format!("{} takes être: {} {} {}.", v.inf, pr.p, pr.etre, agreed),
                    "Its passé composé never uses avoir.".into(),
                ],
            }));
        }
    }
    // Avoir verbs: the trap is être.
    for v in AVOIR_VERBS {
        for pr in PRONS {
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("pc-aux:{}:{}:choice", v.inf, pr.p),
                level: LEVEL,
                topic: "Auxiliary choice",
                topic_slug: Some("pc-aux".into()),
                task: vec!["Choose the right form.".into()],
                sentence: format!("Hier, {} ____ {}.", pr.p, v.complement),
                options: vec![
                    format!("{} {}", pr.avoir, v.p),
                    format!("{} {}", pr.etre, v.p),
                ],
                correct: 0,
                rule: vec![
                    format!("{} takes avoir in the passé composé.", cap(v.inf)),
                    "Most verbs form the passé composé with avoir.".into(),
                ],
            }));
        }
    }
    out
}
