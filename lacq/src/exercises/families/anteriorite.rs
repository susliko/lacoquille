//! Family: anteriority — futur antérieur (auxiliary futur + past
//! participle: quand tu seras arrivé, nous serons prêts) and the past
//! infinitive (après avoir fini / après être arrivé).

use crate::exercises::{ChoiceItem, ClozeItem, ExerciseItem, TransformItem};

use super::shared::starts_vowel;

const LEVEL: &str = "B2";

/* ── futur antérieur ─────────────────────────────────────────────── */

const FA_AV: [&str; 6] = ["aurai", "auras", "aura", "aurons", "aurez", "auront"];
const FA_ETRE: [&str; 6] = ["serai", "seras", "sera", "serons", "serez", "seront"];

/// (subject, form index, feminine?)
const FA_SUBJ: &[(&str, usize, bool)] = &[
    ("je", 0, false),
    ("tu", 1, false),
    ("il", 2, false),
    ("elle", 2, true),
    ("nous", 3, false),
    ("vous", 4, false),
    ("ils", 5, false),
    ("elles", 5, true),
];

struct FaVerb {
    inf: &'static str,
    etre_aux: bool,
    p: &'static str,
    compl: &'static str,
}

const FA_VERBS: &[FaVerb] = &[
    FaVerb { inf: "finir", etre_aux: false, p: "fini", compl: "le travail" },
    FaVerb { inf: "manger", etre_aux: false, p: "mangé", compl: "" },
    FaVerb { inf: "lire", etre_aux: false, p: "lu", compl: "le journal" },
    FaVerb { inf: "faire", etre_aux: false, p: "fait", compl: "les courses" },
    FaVerb { inf: "prendre", etre_aux: false, p: "pris", compl: "le train" },
    FaVerb { inf: "arriver", etre_aux: true, p: "arrivé", compl: "" },
    FaVerb { inf: "partir", etre_aux: true, p: "parti", compl: "" },
    FaVerb { inf: "rentrer", etre_aux: true, p: "rentré", compl: "à la maison" },
];

fn fa_clause(subj: &str, si: usize, feminine: bool, v: &FaVerb) -> String {
    let aux = if v.etre_aux { FA_ETRE[si] } else { FA_AV[si] };
    let subj_aux = if subj == "je" && starts_vowel(aux) {
        format!("j'{}", aux)
    } else {
        format!("{} {}", subj, aux)
    };
    let mut participle = v.p.to_string();
    if v.etre_aux {
        if feminine {
            participle.push('e');
        }
        if matches!(subj, "nous" | "vous" | "ils" | "elles") {
            participle.push('s');
        }
    }
    if v.compl.is_empty() {
        format!("{} {}", subj_aux, participle)
    } else {
        format!("{} {} {}", subj_aux, participle, v.compl)
    }
}

fn fa_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in FA_VERBS {
        for &(subj, si, feminine) in FA_SUBJ {
            let clause = fa_clause(subj, si, feminine, v);
            // Cloze: quand-clause with the futur antérieur.
            out.push(ExerciseItem::Cloze(ClozeItem {
                id: format!("fa:{}:{}:cl", v.inf, subj),
                level: LEVEL,
                topic: "Futur antérieur",
                topic_slug: Some("futur-anterieur".into()),
                task: vec!["Put the verb in the futur antérieur.".into()],
                head: "Quand ".into(),
                tail: ", nous serons prêts.".into(),
                infinitive: v.inf,
                answer: clause.clone(),
                rule: vec![
                    format!("Quand {}, nous serons prêts.", clause),
                    "futur antérieur = futur of the auxiliary + past participle — the earlier future event.".into(),
                ],
            }));
        }
        // Choice: futur simple vs futur antérieur after quand (il-form).
        let aux = if v.etre_aux { FA_ETRE[2] } else { FA_AV[2] };
        let fut3 = match v.inf {
            "finir" => "finira",
            "manger" => "mangera",
            "lire" => "lira",
            "faire" => "fera",
            "prendre" => "prendra",
            "arriver" => "arrivera",
            "partir" => "partira",
            _ => "rentrera",
        };
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("fa:{}:ch", v.inf),
            level: LEVEL,
            topic: "Futur antérieur",
            topic_slug: Some("futur-anterieur".into()),
            task: vec!["The finishing comes first — choose the tense.".into()],
            sentence: "Quand il ____ , on fêtera.".into(),
            options: vec![format!("{} {}", aux, v.p), fut3.to_string()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("Quand il {} {}, on fêtera.", aux, v.p),
                "quand + futur antérieur puts the earlier event first; « quand il finira » is the trap.".into(),
            ],
        }));
    }
    out
}

/// (first sentence, auxiliary, past-infinitive chunk, after-clause) —
/// all sentences use « on » so no participle agreement is needed.
const PINF_ROWS: &[(&str, &str, &str, &str)] = &[
    ("On a fini le travail.", "avoir", "avoir fini", "on a éteint l'ordinateur"),
    ("On a bien mangé.", "avoir", "avoir mangé", "on a rangé la table"),
    ("On a dîné ensemble.", "avoir", "avoir dîné", "on est sortis"),
    ("On a révisé ses cours.", "avoir", "avoir révisé", "on a passé l'examen"),
    ("On a lu le journal.", "avoir", "avoir lu", "on a rendu le livre"),
    ("On est arrivé à midi.", "être", "être arrivé", "on a fait du café"),
    ("On est parti tôt.", "être", "être parti", "on a fermé la maison"),
    ("On s'est levé à six heures.", "être", "s'être levé", "on a préparé le petit-déjeuner"),
];

fn pinf_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(first, aux, chunk, after) in PINF_ROWS {
        let key = super::shared::slugify(&chunk.chars().take(12).collect::<String>());
        // Transform: two clauses joined with après + past infinitive.
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("pinf:{}:tf", key),
            level: LEVEL,
            topic: "Past infinitive",
            topic_slug: Some("past-infinitive".into()),
            task: vec![
                "Join with « après » + the past infinitive".into(),
                "(avoir/être + participle).".into(),
            ],
            source: format!("{}. Ensuite, {}.", first.trim_end_matches('.'), after),
            answer: format!("Après {}, {}.", chunk, after),
            rule: vec![
                format!("Après {}, {}.", chunk, after),
                "après + past infinitive = the action completed before the next one.".into(),
            ],
        }));
        // Cloze: the past-infinitive chunk.
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("pinf:{}:cl", key),
            level: LEVEL,
            topic: "Past infinitive",
            topic_slug: Some("past-infinitive".into()),
            task: vec!["Complete with après + past infinitive.".into()],
            head: "Après ".into(),
            tail: format!(", {}.", after),
            infinitive: "past infinitive",
            answer: chunk.to_string(),
            rule: vec![
                format!("Après {}, {}.", chunk, after),
                "Motion and pronominal verbs take être: après être arrivé, après s'être levé.".into(),
            ],
        }));
        // Choice: avoir vs être — decided by the verb.
        let wrong_aux = if aux == "avoir" { "être" } else { "avoir" };
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("pinf:{}:ch", key),
            level: LEVEL,
            topic: "Past infinitive",
            topic_slug: Some("past-infinitive".into()),
            task: vec!["Choose the auxiliary.".into()],
            sentence: format!("Après ____ {}, {}.", chunk.split(' ').last().unwrap_or(""), after),
            options: vec![aux.to_string(), wrong_aux.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("Après {}, {}.", chunk, after),
                "The same auxiliary as in the passé composé.".into(),
            ],
        }));
    }
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "futur-anterieur" => fa_items(),
        "past-infinitive" => pinf_items(),
        _ => Vec::new(),
    }
}
