//! Family: passé composé with être (cloze + error-spot).

use crate::exercises::{ClozeItem, ErrorSpotItem, ExerciseItem};

use super::shared::{ETRE_VERBS, PRONS, cap, punctuate};

const LEVEL: &str = "A2";

pub(crate) fn items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in ETRE_VERBS {
        for pr in PRONS {
            let agreed = pr.participle(v.p);
            out.push(ExerciseItem::Cloze(ClozeItem {
                id: format!("pc-etre:{}:{}:cloze", v.inf, pr.p),
                level: LEVEL,
                topic: "Passé composé with être",
                topic_slug: Some("pc-etre".into()),
                task: vec!["Put the verb in the passé composé.".into()],
                head: format!("Hier soir, {} ", pr.p),
                tail: format!(" {}.", v.complement),
                infinitive: v.inf,
                answer: format!("{} {}", pr.etre, agreed),
                rule: vec![
                    "With être, the participle agrees with the".into(),
                    format!("subject: {} → {} ({}).", pr.p, agreed, pr.agree_note()),
                ],
            }));

            // Agreement error is only visible when the participle changes.
            if agreed != v.p {
                let mut words = vec![
                    cap(pr.p),
                    pr.etre.to_string(),
                    v.p.to_string(),
                ];
                words.extend(v.complement.split(' ').map(|w| w.to_string()));
                let words = punctuate(words);
                out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
                    id: format!("pc-etre:{}:{}:es", v.inf, pr.p),
                    level: LEVEL,
                    topic: "Passé composé with être",
                    topic_slug: Some("pc-etre".into()),
                    task: vec!["One word is wrong. Tap it.".into()],
                    words,
                    wrong: 2,
                    fix: agreed.clone(),
                    fix_task: "Now write the correct form.",
                    rule: vec![
                        "With être, the participle agrees with the".into(),
                        format!("subject: {} → {}.", pr.p, agreed),
                    ],
                }));
            }
        }
    }
    out
}
