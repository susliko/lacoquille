//! Family: negation — ne … pas / ne … jamais wrapping the conjugated
//! verb. Transform drills the wrapping (affirmative → negative), the
//! cloze hides the whole negation chunk (n'…pas included), and the
//! choice set drills the n'-before-a-vowel elision.

use crate::exercises::{ChoiceItem, ClozeItem, ExerciseItem, TransformItem};

use super::shared::{SUBJ_CAP, cap};

const LEVEL: &str = "A1";

struct NegVerb {
    inf: &'static str,
    /// present forms for je, tu, il/elle, nous, vous, ils/elles
    forms: [&'static str; 6],
    complement: &'static str,
    /// Verb form starts with a vowel → ne → n', je → j'.
    elides: bool,
}

const NEG_VERBS: &[NegVerb] = &[
    NegVerb { inf: "parler", forms: ["parle", "parles", "parle", "parlons", "parlez", "parlent"], complement: "anglais", elides: false },
    NegVerb { inf: "manger", forms: ["mange", "manges", "mange", "mangeons", "mangez", "mangent"], complement: "de la viande", elides: false },
    NegVerb { inf: "regarder", forms: ["regarde", "regardes", "regarde", "regardons", "regardez", "regardent"], complement: "la télévision", elides: false },
    NegVerb { inf: "travailler", forms: ["travaille", "travailles", "travaille", "travaillons", "travaillez", "travaillent"], complement: "le samedi", elides: false },
    NegVerb { inf: "jouer", forms: ["joue", "joues", "joue", "jouons", "jouez", "jouent"], complement: "au tennis", elides: false },
    NegVerb { inf: "finir", forms: ["finis", "finis", "finit", "finissons", "finissez", "finissent"], complement: "ses devoirs", elides: false },
    NegVerb { inf: "sortir", forms: ["sors", "sors", "sort", "sortons", "sortez", "sortent"], complement: "le soir", elides: false },
    NegVerb { inf: "comprendre", forms: ["comprends", "comprends", "comprend", "comprenons", "comprenez", "comprennent"], complement: "la question", elides: false },
    NegVerb { inf: "aimer", forms: ["aime", "aimes", "aime", "aimons", "aimez", "aiment"], complement: "le café", elides: true },
    NegVerb { inf: "oublier", forms: ["oublie", "oublies", "oublie", "oublions", "oubliez", "oublient"], complement: "ses clés", elides: true },
];

/// Negation words drilled, in order of strength.
const NEG_WORDS: &[&str] = &["pas", "jamais"];

/// Full negative clause: subject + negated verb + marker, with the
/// elision already joined (Je n'aime pas / Tu ne parles pas).
fn neg_clause(subj: &str, elides: bool, form: &str, neg: &str) -> String {
    if subj == "je" && elides {
        format!("Je n'{} {}", form, neg)
    } else if elides {
        format!("{} n'{} {}", cap(subj), form, neg)
    } else {
        format!("{} ne {} {}", cap(subj), form, neg)
    }
}

/// Affirmative subject display — j'aime, not Je aime.
fn subj_aff<'a>(subj: &'a str, subj_c: &'a str, elides: bool) -> &'a str {
    if subj == "je" && elides {
        "J'"
    } else {
        subj_c
    }
}

pub(crate) fn items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in NEG_VERBS {
        for (subj, subj_c, fi) in SUBJ_CAP {
            let form = v.forms[*fi];
            for neg in NEG_WORDS {
                // Transform: affirmative → negative.
                let source = format!("{} {} {}.", subj_aff(subj, subj_c, v.elides), form, v.complement);
                let answer = format!("{} {}.", neg_clause(subj, v.elides, form, neg), v.complement);
                out.push(ExerciseItem::Transform(TransformItem {
                    id: format!("neg:{}:{}:{}:tf", v.inf, subj, neg),
                    level: LEVEL,
                    topic: "Negation",
                    topic_slug: Some("negation".into()),
                    task: vec![format!("Make it negative with « {} ».", neg)],
                    source,
                    answer,
                    rule: if neg == &"jamais" {
                        vec![
                            "ne … jamais = never — stronger than".into(),
                            "ne … pas. It wraps the verb the same way.".into(),
                        ]
                    } else {
                        vec![
                            "ne goes before the verb, pas after it:".into(),
                            format!("{} {}.", neg_clause(subj, v.elides, form, neg), v.complement),
                        ]
                    },
                }));

                // Cloze: the whole negation chunk is the answer.
                let is_je = *subj == "je";
                let chunk = if v.elides {
                    format!("n'{} {}", form, neg)
                } else {
                    format!("ne {} {}", form, neg)
                };
                out.push(ExerciseItem::Cloze(ClozeItem {
                    id: format!("neg:{}:{}:{}:cl", v.inf, subj, neg),
                    level: LEVEL,
                    topic: "Negation",
                    topic_slug: Some("negation".into()),
                    task: vec!["Complete the negation.".into()],
                    head: if is_je { "Je ".into() } else { format!("{} ", cap(subj)) },
                    tail: format!(" {}.", v.complement),
                    infinitive: v.inf,
                    answer: chunk.clone(),
                    rule: vec![
                        format!("« {} » = {} — it wraps the verb.", neg, if neg == &"jamais" { "never" } else { "not" }),
                        if v.elides {
                            format!("{} {}.", neg_clause(subj, v.elides, form, neg), v.complement)
                        } else {
                            "ne before the verb, the marker after it.".into()
                        },
                    ],
                }));
            }
        }
    }
    // Elision choices: the trap is "ne aime" / "Ne oublie pas".
    for v in NEG_VERBS.iter().filter(|v| v.elides) {
        for (subj, _subj_c, fi) in SUBJ_CAP {
            let form = v.forms[*fi];
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("neg-elide:{}:{}:ch", v.inf, subj),
                level: LEVEL,
                topic: "Negation",
                topic_slug: Some("negation".into()),
                task: vec!["Choose the right form.".into()],
                sentence: format!("{} ____ {}.", cap(subj), v.complement),
                options: vec![
                    format!("n'{} pas", form),
                    format!("ne {} pas", form),
                ],
                correct: 0, // shuffled in `generate`
                rule: vec![
                    "Before a vowel, ne becomes n'.".into(),
                    format!("{} {}.", neg_clause(subj, true, form, "pas"), v.complement),
                ],
            }));
        }
    }
    out
}
