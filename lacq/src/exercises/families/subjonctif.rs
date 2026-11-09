//! Family: subjonctif after triggers (cloze + error-spot).

use crate::exercises::{ClozeItem, ErrorSpotItem, ExerciseItem};

use super::shared::slugify;

struct Trigger {
    expr: &'static str,
    kind: &'static str,
}

const TRIGGERS: &[Trigger] = &[
    Trigger { expr: "Je suis content que", kind: "a feeling" },
    Trigger { expr: "Nous sommes désolés que", kind: "a feeling" },
    Trigger { expr: "Elle a peur que", kind: "a feeling" },
    Trigger { expr: "C'est dommage que", kind: "a feeling" },
    Trigger { expr: "Je doute que", kind: "doubt" },
    Trigger { expr: "Je ne pense pas que", kind: "doubt" },
    Trigger { expr: "Il est possible que", kind: "doubt" },
    Trigger { expr: "Il faut que", kind: "necessity" },
];

struct SubjVerb {
    inf: &'static str,
    /// (subjunctive, indicative) for tu, il/elle, nous, vous, ils/elles
    forms: [(&'static str, &'static str); 5],
    /// Complement used in error-spot sentences (with final period).
    es_complement: &'static str,
}

const SUBJ_VERBS: &[SubjVerb] = &[
    SubjVerb {
        inf: "avoir",
        forms: [("aies", "as"), ("ait", "a"), ("ayons", "avons"), ("ayez", "avez"), ("aient", "ont")],
        es_complement: "raison.",
    },
    SubjVerb {
        inf: "être",
        forms: [("sois", "es"), ("soit", "est"), ("soyons", "sommes"), ("soyez", "êtes"), ("soient", "sont")],
        es_complement: "malade.",
    },
    SubjVerb {
        inf: "venir",
        forms: [("viennes", "viens"), ("vienne", "vient"), ("venions", "venons"), ("veniez", "venez"), ("viennent", "viennent")],
        es_complement: "demain.",
    },
    SubjVerb {
        inf: "faire",
        forms: [("fasses", "fais"), ("fasse", "fait"), ("fassions", "faisons"), ("fassiez", "faites"), ("fassent", "font")],
        es_complement: "ses devoirs.",
    },
    SubjVerb {
        inf: "aller",
        forms: [("ailles", "vas"), ("aille", "va"), ("allions", "allons"), ("alliez", "allez"), ("aillent", "vont")],
        es_complement: "au cinéma.",
    },
];

const SUBJ_PRONS: &[&str] = &["tu", "il", "nous", "vous", "ils"];

pub(crate) fn items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for t in TRIGGERS {
        for v in SUBJ_VERBS {
            for (pi, pr) in SUBJ_PRONS.iter().enumerate() {
                let (subj_form, indic_form) = v.forms[pi];
                let trig_slug = slugify(&t.expr.to_lowercase());
                out.push(ExerciseItem::Cloze(ClozeItem {
                    id: format!("subj:{}:{}:{}:cloze", trig_slug, v.inf, pr),
                    level: "B1",
                    topic: "Subjonctif after feelings and doubt",
                    topic_slug: Some("subjonctif".into()),
                    task: vec!["Put the verb in the right form.".into()],
                    head: format!("{} {} ", t.expr, pr),
                    tail: ".".into(),
                    infinitive: v.inf,
                    answer: subj_form.to_string(),
                    rule: vec![
                        format!("{} expresses {} — the clause that follows takes the subjonctif.", t.expr, t.kind),
                    ],
                }));
                // Error-spot: indicative after the trigger.
                let mut words: Vec<String> =
                    t.expr.split(' ').map(|w| w.to_string()).collect();
                words.push(pr.to_string());
                words.push(indic_form.to_string());
                let comp: Vec<String> =
                    v.es_complement.split(' ').map(|w| w.to_string()).collect();
                let wrong = words.len() - 1;
                words.extend(comp);
                out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
                    id: format!("subj:{}:{}:{}:es", trig_slug, v.inf, pr),
                    level: "B1",
                    topic: "Subjonctif after feelings and doubt",
                    topic_slug: Some("subjonctif".into()),
                    task: vec!["One word is wrong. Tap it.".into()],
                    words,
                    wrong,
                    fix: subj_form.to_string(),
                    fix_task: "Now write the correct form.",
                    rule: vec![
                        format!("{} expresses {} — the clause takes", t.expr, t.kind),
                        format!("the subjonctif: qu'{} {}.", pr, subj_form),
                    ],
                }));
            }
        }
    }
    out
}
