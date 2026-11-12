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

/* ── ne … pas / jamais (A1) ─────────────────────────────────────── */

fn negation_items() -> Vec<ExerciseItem> {
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

/* ── ne … plus / rien / personne (A2) ───────────────────────────── */

const LEVEL2: &str = "A2";

struct Neg2Verb {
    inf: &'static str,
    /// present forms je, tu, il/elle, nous, vous, ils/elles
    forms: [&'static str; 6],
    complement: &'static str,
    /// verb form starts with a vowel → ne → n' (all subjects)
    elides: bool,
    /// "Je ne … rien" makes sense (object can be dropped)
    rien_ok: bool,
    /// "Je ne … personne" makes sense (object can be a person)
    pers_ok: bool,
    /// bare English verb for the choice cues
    en: &'static str,
}

const NEG2_VERBS: &[Neg2Verb] = &[
    Neg2Verb { inf: "manger", forms: ["mange", "manges", "mange", "mangeons", "mangez", "mangent"], complement: "de la viande", elides: false, rien_ok: true, pers_ok: false, en: "eat" },
    Neg2Verb { inf: "regarder", forms: ["regarde", "regardes", "regarde", "regardons", "regardez", "regardent"], complement: "la télévision", elides: false, rien_ok: true, pers_ok: true, en: "watch" },
    Neg2Verb { inf: "fumer", forms: ["fume", "fumes", "fume", "fumons", "fumez", "fument"], complement: "à la terrasse", elides: false, rien_ok: false, pers_ok: false, en: "smoke" },
    Neg2Verb { inf: "travailler", forms: ["travaille", "travailles", "travaille", "travaillons", "travaillez", "travaillent"], complement: "le samedi", elides: false, rien_ok: false, pers_ok: false, en: "work" },
    Neg2Verb { inf: "comprendre", forms: ["comprends", "comprends", "comprend", "comprenons", "comprenez", "comprennent"], complement: "la question", elides: false, rien_ok: true, pers_ok: false, en: "understand" },
    Neg2Verb { inf: "voir", forms: ["vois", "vois", "voit", "voyons", "voyez", "voient"], complement: "la mer", elides: false, rien_ok: true, pers_ok: true, en: "see" },
    Neg2Verb { inf: "attendre", forms: ["attends", "attends", "attend", "attendons", "attendez", "attendent"], complement: "le bus", elides: true, rien_ok: true, pers_ok: true, en: "wait for" },
    Neg2Verb { inf: "connaître", forms: ["connais", "connais", "connaît", "connaissons", "connaissez", "connaissent"], complement: "ce quartier", elides: false, rien_ok: false, pers_ok: true, en: "know" },
    Neg2Verb { inf: "sortir", forms: ["sors", "sors", "sort", "sortons", "sortez", "sortent"], complement: "le soir", elides: false, rien_ok: false, pers_ok: false, en: "go out" },
    Neg2Verb { inf: "écouter", forms: ["écoute", "écoutes", "écoute", "écoutons", "écoutez", "écoutent"], complement: "la radio", elides: true, rien_ok: true, pers_ok: true, en: "listen to" },
];

/// negated clause with a marker after the verb (plus / rien / personne),
/// elision already joined: Je n'attends plus / Tu ne manges rien.
fn neg2_clause(subj: &str, elides: bool, form: &str, marker: &str) -> String {
    if elides {
        format!("{} n'{} {}", cap(subj), form, marker)
    } else {
        format!("{} ne {} {}", cap(subj), form, marker)
    }
}

/// clause + complement (or a bare period when there is none).
fn with_tail(clause: String, complement: &str) -> String {
    if complement.is_empty() {
        format!("{}.", clause)
    } else {
        format!("{} {}.", clause, complement)
    }
}

fn neg2_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in NEG2_VERBS {
        for (subj, subj_c, fi) in SUBJ_CAP {
            let form = v.forms[*fi];
            let tail = format!(" {}.", v.complement);
            // Transform: affirmative → ne … plus (complement kept).
            out.push(ExerciseItem::Transform(TransformItem {
                id: format!("neg2:{}:{}:tf-plus", v.inf, subj),
                level: LEVEL2,
                topic: "Ne… plus, rien, personne",
                topic_slug: Some("ne-plus-rien-personne".into()),
                task: vec!["Say it with « ne … plus » — not anymore.".into()],
                source: format!("{} {}{}", subj_aff(subj, subj_c, v.elides), form, tail),
                answer: with_tail(neg2_clause(subj, v.elides, form, "plus"), v.complement),
                rule: vec![
                    "ne … plus = not anymore — it wraps the verb like pas.".into(),
                    with_tail(neg2_clause(subj, v.elides, form, "plus"), v.complement),
                ],
            }));
            // Cloze: the ne … plus chunk around the verb.
            let chunk = if v.elides {
                format!("n'{} plus", form)
            } else {
                format!("ne {} plus", form)
            };
            out.push(ExerciseItem::Cloze(ClozeItem {
                id: format!("neg2:{}:{}:cl", v.inf, subj),
                level: LEVEL2,
                topic: "Ne… plus, rien, personne",
                topic_slug: Some("ne-plus-rien-personne".into()),
                task: vec!["Complete with ne … plus.".into()],
                head: format!("{} ", cap(subj)),
                tail: tail.clone(),
                infinitive: v.inf,
                answer: chunk.clone(),
                rule: vec![
                    format!("{} {}.", cap(subj), chunk),
                    "plus comes after the verb, ne goes before it.".into(),
                ],
            }));
            // Transform: ne … rien — the object drops out.
            if v.rien_ok {
                out.push(ExerciseItem::Transform(TransformItem {
                    id: format!("neg2:{}:{}:tf-rien", v.inf, subj),
                    level: LEVEL2,
                    topic: "Ne… plus, rien, personne",
                    topic_slug: Some("ne-plus-rien-personne".into()),
                    task: vec!["Say it with « ne … rien » — NOTHING.".into()],
                    source: format!("{} {}{}", subj_aff(subj, subj_c, v.elides), form, tail),
                    answer: format!("{}.", neg2_clause(subj, v.elides, form, "rien")),
                    rule: vec![
                        "rien replaces the thing — the noun drops out:".into(),
                        format!("{}.", neg2_clause(subj, v.elides, form, "rien")),
                    ],
                }));
            }
            // Transform: ne … personne — nobody.
            if v.pers_ok {
                out.push(ExerciseItem::Transform(TransformItem {
                    id: format!("neg2:{}:{}:tf-pers", v.inf, subj),
                    level: LEVEL2,
                    topic: "Ne… plus, rien, personne",
                    topic_slug: Some("ne-plus-rien-personne".into()),
                    task: vec!["Say it with « ne … personne » — NOBODY.".into()],
                    source: format!("{} {}{}", subj_aff(subj, subj_c, v.elides), form, tail),
                    answer: format!("{}.", neg2_clause(subj, v.elides, form, "personne")),
                    rule: vec![
                        "personne replaces the person — the noun drops out:".into(),
                        format!("{}.", neg2_clause(subj, v.elides, form, "personne")),
                    ],
                }));
            }
        }
        // Choice: plus vs pas, English cue settles the meaning.
        for (subj_en, aux, subj_fr, fi2) in
            [("He", "doesn't", "il", 2usize), ("They", "don't", "ils", 5usize)]
        {
            let form2 = v.forms[fi2];
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("neg2:{}:{}:ch", v.inf, subj_fr),
                level: LEVEL2,
                topic: "Ne… plus, rien, personne",
                topic_slug: Some("ne-plus-rien-personne".into()),
                task: vec![format!("{} {} {} anymore.", subj_en, aux, v.en)],
                sentence: format!("{} ne {} ____.", cap(subj_fr), form2),
                options: vec!["plus".into(), "pas".into()],
                correct: 0, // shuffled in `generate`
                rule: vec![
                    format!("{} ne {} plus.", cap(subj_fr), form2),
                    "anymore → plus; plain not → pas.".into(),
                ],
            }));
        }
    }
    out
}

/// Compat shim — the dispatcher flips to [`items_slug`] when the
/// family dispatch goes slug-aware.
pub(crate) fn items() -> Vec<ExerciseItem> {
    items_slug("negation")
}

pub(crate) fn items_slug(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "negation" => negation_items(),
        "ne-plus-rien-personne" => neg2_items(),
        _ => Vec::new(),
    }
}
