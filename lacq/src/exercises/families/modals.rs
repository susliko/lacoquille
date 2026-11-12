//! Family: modals + infinitive — one mood cluster, three topics.
//! pouvoir/vouloir/devoir + infinitive (cloze + person-confusion
//! error-spots), il faut + infinitive (devoir→il faut transform), and
//! pour + infinitive (purpose, pour vs par trap).

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem, TransformItem};

use super::shared::{SUBJ_CAP, cap, slugify, starts_vowel};

const LEVEL: &str = "A1";

struct Modal {
    inf: &'static str,
    forms: [&'static str; 6],
}

const MODALS: &[Modal] = &[
    Modal { inf: "pouvoir", forms: ["peux", "peux", "peut", "pouvons", "pouvez", "peuvent"] },
    Modal { inf: "vouloir", forms: ["veux", "veux", "veut", "voulons", "voulez", "veulent"] },
    Modal { inf: "devoir", forms: ["dois", "dois", "doit", "devons", "devez", "doivent"] },
];

/// Main verbs the modals govern, each with its complement.
const MAINS: &[(&str, &str)] = &[
    ("manger", "une pomme"),
    ("partir", "demain"),
    ("finir", "le travail"),
    ("dormir", "tôt"),
    ("venir", "avec nous"),
    ("jouer", "au tennis"),
    ("lire", "ce livre"),
    ("apprendre", "le français"),
];

/// Affirmative subject display (je + vowel elides: j'apprends).
fn subj_aff<'a>(subj: &'a str, subj_c: &'a str, next: &str) -> &'a str {
    if subj == "je" && starts_vowel(next) {
        "J'"
    } else {
        subj_c
    }
}

/* ── pouvoir / vouloir / devoir + infinitive ────────────────────── */

fn modal_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for m in MODALS {
        for (inf, comp) in MAINS {
            for (subj, subj_c, fi) in SUBJ_CAP {
                let form = m.forms[*fi];
                let head = subj_aff(subj, subj_c, form);
                out.push(ExerciseItem::Cloze(ClozeItem {
                    id: format!("md:{}:{}:{}:cl", m.inf, inf, subj),
                    level: LEVEL,
                    topic: "Pouvoir, vouloir, devoir + infinitive",
                    topic_slug: Some("modal-verbs-infinitive".into()),
                    task: vec!["Put the verb in the present.".into()],
                    head: format!("{} ", head),
                    tail: format!(" {} {}.", inf, comp),
                    infinitive: m.inf,
                    answer: format!("{} {}", form, inf),
                    rule: vec![
                        format!("{} {} {} {}.", head, form, inf, comp),
                        format!("{} + infinitive — the second verb never changes.", m.inf),
                    ],
                }));

                // Error-spot: the modal agreeing with the wrong person.
                let audible = matches!(*subj, "tu" | "nous" | "vous" | "ils" | "elles");
                let wrong = m.forms[2];
                if audible && wrong != form {
                    let mut words: Vec<String> = vec![cap(subj).to_string()];
                    words.push(wrong.to_string());
                    words.push(inf.to_string());
                    words.push(comp.to_string());
                    out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
                        id: format!("md:{}:{}:{}:es", m.inf, inf, subj),
                        level: LEVEL,
                        topic: "Pouvoir, vouloir, devoir + infinitive",
                        topic_slug: Some("modal-verbs-infinitive".into()),
                        task: vec!["One word is wrong. Tap it.".into()],
                        words,
                        wrong: 1,
                        fix: form.to_string(),
                        fix_task: "Now write the correct form.",
                        rule: vec![format!("{} with « {} » → {} {}.", m.inf, subj, form, inf)],
                    }));
                }
            }
        }
    }
    out
}

/* ── il faut + infinitive ───────────────────────────────────────── */

fn ilfaut_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    // Transform: devoir → il faut (person disappears).
    for (inf, comp) in MAINS {
        for (subj, subj_c, fi) in [("je", "Je", 0), ("tu", "Tu", 1), ("ils", "Ils", 5)] {
            let form = MODALS[2].forms[fi];
            let head = subj_aff(subj, subj_c, form);
            let source = format!("{} {} {} {}.", head, form, inf, comp);
            out.push(ExerciseItem::Transform(TransformItem {
                id: format!("fault:{}:{}:tf", inf, subj),
                level: LEVEL,
                topic: "Il faut + infinitive",
                topic_slug: Some("il-faut-infinitive".into()),
                task: vec![
                    "Rewrite it with « il faut » —".into(),
                    "the obligation without the person.".into(),
                ],
                source,
                answer: format!("Il faut {} {}.", inf, comp),
                rule: vec![
                    "il faut + infinitive = it is necessary to…".into(),
                    format!("Il faut {} {}.", inf, comp),
                ],
            }));
        }
        // Cloze: the faut in place.
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("fault:{}:cl", inf),
            level: LEVEL,
            topic: "Il faut + infinitive",
            topic_slug: Some("il-faut-infinitive".into()),
            task: vec!["Complete the sentence.".into()],
            head: "Il ".into(),
            tail: format!(" {} {}.", inf, comp),
            infinitive: "falloir",
            answer: "faut".into(),
            rule: vec![
                "falloir has one form: il faut.".into(),
                format!("Il faut {} {}.", inf, comp),
            ],
        }));
        // Error-spot: the famous "Il faute" slip.
        let mut words: Vec<String> = vec!["Il".into(), "faute".into()];
        words.push(inf.to_string());
        words.push(comp.to_string());
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("fault:{}:es", inf),
            level: LEVEL,
            topic: "Il faut + infinitive",
            topic_slug: Some("il-faut-infinitive".into()),
            task: vec!["One word is wrong. Tap it.".into()],
            words,
            wrong: 1,
            fix: "faut".into(),
            fix_task: "Now write the correct form.",
            rule: vec![
                "It's faut, never faute (la faute is a mistake!).".into(),
                format!("Il faut {} {}.", inf, comp),
            ],
        }));
    }
    out
}

/* ── pour + infinitive ──────────────────────────────────────────── */

const PURPOSES: &[&str] = &[
    "apprendre le français",
    "rester en forme",
    "gagner du temps",
    "finir à l'heure",
    "économiser de l'argent",
    "se détendre",
];

/// (verb, forms for je, nous, ils) — the purpose clause stays bare.
const PO_VERBS: &[(&str, [&str; 3])] = &[
    ("lire", ["lis", "lisons", "lisent"]),
    ("travailler", ["travaille", "travaillons", "travaillent"]),
    ("courir", ["cours", "courons", "courent"]),
];

fn pour_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for (inf, forms) in PO_VERBS {
        for (subj, subj_c, form) in
            [("je", "Je", forms[0]), ("nous", "Nous", forms[1]), ("ils", "Ils", forms[2])]
        {
            for purpose in PURPOSES {
                let head = subj_aff(subj, subj_c, form);
                out.push(ExerciseItem::Cloze(ClozeItem {
                    id: format!("po:{}:{}:{}:cl", inf, subj, slugify(purpose)),
                    level: LEVEL,
                    topic: "Pour + infinitive",
                    topic_slug: Some("pour-infinitive".into()),
                    task: vec!["Complete with « pour » or « par ».".into()],
                    head: format!("{} {} ", head, form),
                    tail: format!(" {}.", purpose),
                    infinitive: "pour",
                    answer: "pour".into(),
                    rule: vec![
                        format!("{} {} pour {}.", head, form, purpose),
                        "pour + infinitive = in order to… — par never means that.".into(),
                    ],
                }));
                out.push(ExerciseItem::Choice(ChoiceItem {
                    id: format!("po:{}:{}:{}:ch", inf, subj, slugify(purpose)),
                    level: LEVEL,
                    topic: "Pour + infinitive",
                    topic_slug: Some("pour-infinitive".into()),
                    task: vec!["Choose the right word.".into()],
                    sentence: format!("{} {} ____ {}.", head, form, purpose),
                    options: vec!["pour".into(), "par".into()],
                    correct: 0, // shuffled in `generate`
                    rule: vec![
                        "pour = in order to; par = by (through, via).".into(),
                        format!("{} {} pour {}.", head, form, purpose),
                    ],
                }));
            }
        }
    }
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "modal-verbs-infinitive" => modal_items(),
        "il-faut-infinitive" => ilfaut_items(),
        "pour-infinitive" => pour_items(),
        _ => Vec::new(),
    }
}
