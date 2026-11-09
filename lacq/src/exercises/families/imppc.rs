//! Family: imparfait vs passé composé, marker-driven (choice).

use crate::exercises::{ChoiceItem, ExerciseItem};

struct ImpPcVerb {
    inf: &'static str,
    /// (imparfait, passé composé) for il, nous, elles
    il: (&'static str, &'static str),
    nous: (&'static str, &'static str),
    elles: (&'static str, &'static str),
    complement: &'static str,
    il_only: bool,
}

const IMPPC_VERBS: &[ImpPcVerb] = &[
    ImpPcVerb {
        inf: "aller",
        il: ("allait", "est allé"),
        nous: ("allions", "sommes allés"),
        elles: ("allaient", "sont allées"),
        complement: "à la plage",
        il_only: false,
    },
    ImpPcVerb {
        inf: "dormir",
        il: ("dormait", "a dormi"),
        nous: ("dormions", "avons dormi"),
        elles: ("dormaient", "ont dormi"),
        complement: "toute la journée",
        il_only: false,
    },
    ImpPcVerb {
        inf: "parler",
        il: ("parlait", "a parlé"),
        nous: ("parlions", "avons parlé"),
        elles: ("parlaient", "ont parlé"),
        complement: "au téléphone",
        il_only: false,
    },
    ImpPcVerb {
        inf: "regarder",
        il: ("regardait", "a regardé"),
        nous: ("regardions", "avons regardé"),
        elles: ("regardaient", "ont regardé"),
        complement: "la télévision",
        il_only: false,
    },
    ImpPcVerb {
        inf: "faire",
        il: ("faisait", "a fait"),
        nous: ("faisions", "avons fait"),
        elles: ("faisaient", "ont fait"),
        complement: "ses devoirs",
        il_only: false,
    },
    ImpPcVerb {
        inf: "marcher",
        il: ("marchait", "a marché"),
        nous: ("marchions", "avons marché"),
        elles: ("marchaient", "ont marché"),
        complement: "au parc",
        il_only: false,
    },
    ImpPcVerb {
        inf: "pleuvoir",
        il: ("pleuvait", "a plu"),
        nous: ("", ""),
        elles: ("", ""),
        complement: "",
        il_only: true,
    },
];

const IMP_MARKERS: &[&str] = &[
    "Chaque été,",
    "Tous les jours,",
    "Le dimanche,",
    "Pendant des années,",
    "Tous les matins,",
];

const PC_MARKERS: &[&str] = &[
    "Soudain,",
    "Tout à coup,",
    "Un matin,",
    "Hier soir,",
    "À ce moment-là,",
];

fn rule(is_habit: bool) -> &'static str {
    if is_habit {
        "A repeated habit or ongoing background takes the imparfait."
    } else {
        "A sudden, completed event takes the passé composé."
    }
}

pub(crate) fn items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in IMPPC_VERBS {
        let subjects: &[(&str, &(&'static str, &'static str))] = if v.il_only {
            &[("il", &v.il)]
        } else {
            &[("il", &v.il), ("nous", &v.nous), ("elles", &v.elles)]
        };
        for (subj, forms) in subjects {
            for (mi, marker) in IMP_MARKERS.iter().chain(PC_MARKERS.iter()).enumerate() {
                let is_habit = mi < IMP_MARKERS.len();
                let (right, wrong) = if is_habit {
                    (forms.0, forms.1)
                } else {
                    (forms.1, forms.0)
                };
                let sentence = if v.complement.is_empty() {
                    format!("{} {} ____.", marker, subj)
                } else {
                    format!("{} {} ____ {}.", marker, subj, v.complement)
                };
                out.push(ExerciseItem::Choice(ChoiceItem {
                    id: format!("impvspc:{}:{}:{}", mi, v.inf, subj),
                    level: "A2",
                    topic: "Imparfait vs passé composé",
                    topic_slug: Some("imparfait-vs-pc".into()),
                    task: vec!["Choose the right form.".into()],
                    sentence,
                    options: vec![right.to_string(), wrong.to_string()],
                    correct: 0, // shuffled in `generate`
                    rule: vec![rule(is_habit).into()],
                }));
            }
        }
    }
    out
}
