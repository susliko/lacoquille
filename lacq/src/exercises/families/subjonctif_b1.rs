//! Family: B1 subjonctif — présent forms of the irregular stems
//! (que je sois, que j'aie, que j'aille…) and the choix between
//! subjonctif and indicatif: wish/possibility triggers take the
//! subjonctif, belief/certainty triggers keep the indicative.

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem};

use super::shared::{punctuate, starts_vowel};

const LEVEL: &str = "B1";

/* ── subjonctif présent: forms ───────────────────────────────────── */

/// irregular subjonctif présent stems, for je, tu, il/elle, nous, vous, ils/elles
struct SVerb {
    inf: &'static str,
    forms: [&'static str; 6],
    /// indicative présent 3rd singular — the wrong-form decoy after « qu'il »
    ind_il: &'static str,
    complement: &'static str,
}

const S_VERBS: &[SVerb] = &[
    SVerb { inf: "être", forms: ["sois", "sois", "soit", "soyons", "soyez", "soient"], ind_il: "est", complement: "à l'heure" },
    SVerb { inf: "avoir", forms: ["aie", "aies", "ait", "ayons", "ayez", "aient"], ind_il: "a", complement: "ses clés" },
    SVerb { inf: "faire", forms: ["fasse", "fasses", "fasse", "fassions", "fassiez", "fassent"], ind_il: "fait", complement: "ses devoirs" },
    SVerb { inf: "aller", forms: ["aille", "ailles", "aille", "allions", "alliez", "aillent"], ind_il: "va", complement: "chez le médecin" },
    SVerb { inf: "pouvoir", forms: ["puisse", "puisses", "puisse", "puissions", "puissiez", "puissent"], ind_il: "peut", complement: "venir" },
    SVerb { inf: "savoir", forms: ["sache", "saches", "sache", "sachions", "sachiez", "sachent"], ind_il: "sait", complement: "la vérité" },
    SVerb { inf: "vouloir", forms: ["veuille", "veuilles", "veuille", "voulions", "vouliez", "veuillent"], ind_il: "veut", complement: "rester" },
    SVerb { inf: "prendre", forms: ["prenne", "prennes", "prenne", "prenions", "preniez", "prennent"], ind_il: "prend", complement: "son médicament" },
    SVerb { inf: "finir", forms: ["finisse", "finisses", "finisse", "finissions", "finissiez", "finissent"], ind_il: "finit", complement: "le chantier" },
    SVerb { inf: "pousser", forms: ["pousse", "pousses", "pousse", "poussions", "poussiez", "poussent"], ind_il: "pousse", complement: "sur les murs" },
];

/// (subject, display form index)
const S_SUBJ: &[(&str, usize)] = &[
    ("je", 0),
    ("tu", 1),
    ("il", 2),
    ("elle", 2),
    ("nous", 3),
    ("vous", 4),
    ("ils", 5),
    ("elles", 5),
];

/// que + subject, elided before il/elle/ils/elles.
fn que_subj(subj: &str) -> String {
    if matches!(subj, "il" | "elle" | "ils" | "elles") {
        format!("qu'{}", subj)
    } else {
        format!("que {}", subj)
    }
}

/// il faut que / je veux que / pour que, per subject of the SUB clause.
fn trigger_for(subj: &str) -> &'static str {
    match subj {
        "je" => "Il faut",
        "tu" => "Je veux",
        "nous" => "Il vaut mieux",
        "vous" => "Je préfère",
        _ => "Il faut",
    }
}

fn sform_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in S_VERBS {
        for &(subj, fi) in S_SUBJ {
            let form = v.forms[fi];
            let trigger = trigger_for(subj);
            // je + vowel: « qu'il faut que j'aie » — the elision lands in
            // the answer chunk.
            let (answer, hint_form) = if subj == "je" && starts_vowel(form) {
                (format!("j'{}", form), format!("j'{}", form))
            } else {
                (form.to_string(), form.to_string())
            };
            let tail = if v.complement.is_empty() { ".".into() } else { format!(" {}.", v.complement) };
            out.push(ExerciseItem::Cloze(ClozeItem {
                id: format!("sform:{}:{}:cl", v.inf, subj),
                level: LEVEL,
                topic: "Subjonctif présent: forms",
                topic_slug: Some("subjonctif-forms".into()),
                task: vec!["Put the verb in the subjonctif présent.".into()],
                head: format!("{} {} ", trigger, que_subj(subj)),
                tail: tail.clone(),
                infinitive: v.inf,
                answer: answer.clone(),
                rule: vec![
                    format!("{} {} {}.", trigger, que_subj(subj), hint_form),
                    "il faut que / je veux que + subjonctif.".into(),
                ],
            }));
            // Choice: subjonctif vs the indicative form after qu'il.
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("sform:{}:{}:ch", v.inf, subj),
                level: LEVEL,
                topic: "Subjonctif présent: forms",
                topic_slug: Some("subjonctif-forms".into()),
                task: vec!["Choose the subjonctif form.".into()],
                sentence: format!("Il faut {} ____ {}.", que_subj(subj), v.complement),
                options: vec![form.to_string(), v.ind_il.to_string()],
                correct: 0, // shuffled in `generate`
                rule: vec![
                    format!("Il faut {} {} {}.", que_subj(subj), form, v.complement),
                    format!("« {} » is the subjonctif; « {} » is the indicatif.", form, v.ind_il),
                ],
            }));
        }
        // Error-spot: the indicative sneaks in after il faut que.
        if v.forms[2] != v.ind_il {
            out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
                id: format!("sform:{}:il:es", v.inf),
                level: LEVEL,
                topic: "Subjonctif présent: forms",
                topic_slug: Some("subjonctif-forms".into()),
                task: vec!["One word is wrong. Tap it.".into()],
                words: punctuate(vec![
                    "Il".into(),
                    "faut".into(),
                    "qu'il".into(),
                    v.ind_il.into(),
                    v.complement.into(),
                ]),
                wrong: 3,
                fix: v.forms[2].to_string(),
                fix_task: "Now write the correct form.",
                rule: vec![
                    format!("Il faut qu'il {} {}.", v.forms[2], v.complement),
                    "il faut que is a subjonctif trigger.".into(),
                ],
            }));
        }
    }
    out
}

/* ── subjonctif vs indicatif ─────────────────────────────────────── */

/// (main-clause frame with ____, correct, wrong, English cue, rule).
/// Every row is a real minimal pair or a locked-mood frame.
const SVI_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Je veux qu'il ____ à l'heure.", "soit", "est", "I want → wish", "vouloir que → subjonctif (soit)."),
    ("Je pense qu'il ____ à l'heure.", "est", "soit", "I think → belief", "penser que → indicatif (est)."),
    ("Il faut que tu ____ tes devoirs.", "fasses", "fais", "necessity", "il faut que → subjonctif (fasses)."),
    ("Je sais qu'il ____ ses devoirs.", "fait", "fasse", "I know → fact", "savoir que → indicatif (fait)."),
    ("Il est possible qu'elle ____ demain.", "vienne", "vient", "possibility", "il est possible que → subjonctif (vienne)."),
    ("Il est certain qu'elle ____ demain.", "vient", "vienne", "certainty", "il est certain que → indicatif (vient)."),
    ("J'espère qu'il ____ beau ce week-end.", "fera", "fasse", "hope = belief", "espérer que → indicatif (fera), unlike souhaiter que."),
    ("Je souhaite qu'il ____ beau ce week-end.", "fasse", "fera", "a wish", "souhaiter que → subjonctif (fasse)."),
    ("Il est important que nous ____ à l'heure.", "soyons", "sommes", "importance", "il est important que → subjonctif (soyons)."),
    ("Je crois qu'il ____ raison.", "a", "ait", "belief", "croire que → indicatif (a)."),
    ("Je ne trouve pas qu'il ____ raison.", "ait", "a", "doubt", "negated finding → subjonctif (ait)."),
    ("Elle voudrait que vous ____ avec nous.", "veniez", "venez", "a wish", "vouloir que → subjonctif (veniez)."),
    ("Il est peu probable qu'ils ____ le délai.", "respectent", "respecteront", "low probability", "il est peu probable que → subjonctif (here it looks like the présent, but the mood is subjonctif)."),
    ("Nous espérons que vous ____ bien ce travail.", "aimerez", "aimiez", "hope = belief", "espérer que → indicatif (aimerez)."),
];

fn svi_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in SVI_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("svi:{}:ch", key),
            level: LEVEL,
            topic: "Subjonctif vs indicatif",
            topic_slug: Some("subjonctif-vs-indicatif".into()),
            task: vec![format!("{} — choose the mood.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("svi:{}:cl", key),
            level: LEVEL,
            topic: "Subjonctif vs indicatif",
            topic_slug: Some("subjonctif-vs-indicatif".into()),
            task: vec![format!("{} — put the verb in the right mood.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: inf_of_svi(good),
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Error-spot: wish frames with the indicative in them.
    for (words, wrong_i, fix, rule) in [
        (
            vec!["Il", "faut", "qu'il", "est", "à", "l'heure"],
            3usize,
            "soit",
            "il faut que → subjonctif: qu'il soit.",
        ),
        (
            vec!["Je", "veux", "que", "tu", "fais", "attention"],
            4usize,
            "fasses",
            "vouloir que → subjonctif: que tu fasses.",
        ),
    ] {
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("svi:{}:es", super::shared::slugify(&fix)),
            level: LEVEL,
            topic: "Subjonctif vs indicatif",
            topic_slug: Some("subjonctif-vs-indicatif".into()),
            task: vec!["One word is wrong. Tap it.".into()],
            words: punctuate(words.iter().map(|w| w.to_string()).collect()),
            wrong: wrong_i,
            fix: fix.into(),
            fix_task: "Now write the correct form.",
            rule: vec![rule.to_string()],
        }));
    }
    out
}

fn inf_of_svi(good: &str) -> &'static str {
    match good {
        "soit" | "soyons" => "être",
        "est" => "être",
        "fasses" | "fera" | "fasse" => "faire",
        "fait" => "faire",
        "vienne" | "vient" | "veniez" => "venir",
        "a" | "ait" => "avoir",
        "respectent" => "respecter",
        "aimerez" | "aimiez" => "aimer",
        _ => "faire",
    }
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "subjonctif-forms" => sform_items(),
        "subjonctif-vs-indicatif" => svi_items(),
        _ => Vec::new(),
    }
}
