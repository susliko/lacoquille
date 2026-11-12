//! Family: A1 mood cluster — two topics. The imperative (tu/nous/vous
//! forms, -er tu drops the -s, negation with ne…pas) and the polite
//! conditionnel (je voudrais — the polite way to want).

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem, TransformItem};

use super::shared::{punctuate, starts_vowel};

const LEVEL: &str = "A1";

/* ── imperative ─────────────────────────────────────────────────── */

/// (infinitive, tu-form present, nous-form, vous-form, complement)
/// tu-imperative = tu-present minus the final -s for -er verbs.
const IMP_VERBS: &[(&str, &str, &str, &str, &str)] = &[
    ("parler", "parles", "parlons", "parlez", "anglais"),
    ("manger", "manges", "mangeons", "mangez", "une pomme"),
    ("regarder", "regardes", "regardons", "regardez", "la télévision"),
    ("écouter", "écoutes", "écoutons", "écoutez", "le professeur"),
    ("travailler", "travailles", "travaillons", "travaillez", "le matin"),
    ("finir", "finis", "finissons", "finissez", "tes devoirs"),
    ("sortir", "sors", "sortons", "sortez", "à midi"),
    ("faire", "fais", "faisons", "faites", "du sport"),
    ("aller", "vas", "allons", "allez", "à la piscine"),
    ("lire", "lis", "lisons", "lisez", "ce livre"),
];

fn imperative_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for (inf, tu, nous, vous, comp) in IMP_VERBS {
        // -er verbs drop the final -s in the tu imperative.
        let tu_imp: String = if inf.ends_with("er") {
            tu.trim_end_matches('s').to_string()
        } else {
            tu.to_string()
        };

        // Transform: statement → command (tu).
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("imp:{}:tu:tf", inf),
            level: LEVEL,
            topic: "Imperative",
            topic_slug: Some("imperative".into()),
            task: vec![
                "Turn it into an order —".into(),
                "one word for « tu », then the rest.".into(),
            ],
            source: format!("Tu {} {}.", tu, comp),
            answer: format!("{} {} !", tu_imp, comp),
            rule: vec![
                if inf.ends_with("er") {
                    "-er verbs drop the final -s: parle, mange, regarde.".into()
                } else {
                    "tu/nous/vous forms, no subject — finis, sortons, sortez.".into()
                },
                format!("Tu {} {} → {} {} !", tu, comp, tu_imp, comp),
            ],
        }));
        // Transform: nous form.
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("imp:{}:nous:tf", inf),
            level: LEVEL,
            topic: "Imperative",
            topic_slug: Some("imperative".into()),
            task: vec!["Turn it into a « let's » command.".into()],
            source: format!("Nous {} {}.", nous, comp),
            answer: format!("{} {} !", nous, comp),
            rule: vec![
                "The nous imperative = the nous form, no subject.".into(),
                format!("{} {} !", nous, comp),
            ],
        }));
        // Error-spot: the -s the -er tu-form must lose.
        if inf.ends_with("er") {
            let mut words: Vec<String> = vec![tu.to_string()];
            words.push(comp.to_string());
            let words = punctuate(words);
            out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
                id: format!("imp:{}:es", inf),
                level: LEVEL,
                topic: "Imperative",
                topic_slug: Some("imperative".into()),
                task: vec!["One word is wrong. Tap it.".into()],
                words,
                wrong: 0,
                fix: tu_imp.clone(),
                fix_task: "Now write the correct form.",
                rule: vec![
                    "Commands with tu drop the -s on -er verbs.".into(),
                    format!("{} {} !", tu_imp, comp),
                ],
            }));
        }
        // Choice: negative imperative — ne (n' before a vowel) … pas.
        let (neg_right, neg_wrong) = if starts_vowel(&tu_imp) {
            (format!("N'{} pas", tu_imp), format!("Ne {} pas", tu_imp))
        } else {
            (format!("Ne {} pas", tu_imp), format!("N'{} pas", tu_imp))
        };
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("imp:{}:neg:ch", inf),
            level: LEVEL,
            topic: "Imperative",
            topic_slug: Some("imperative".into()),
            task: vec!["Choose the right form.".into()],
            sentence: format!("____ {} {} !", tu_imp, comp),
            options: vec![neg_right.clone(), neg_wrong],
            correct: 0, // shuffled in `generate`
            rule: vec![
                "ne … pas wraps the command verb too.".into(),
                format!("{} {} !", neg_right, comp),
            ],
        }));
        // Cloze: the vous command.
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("imp:{}:vous:cl", inf),
            level: LEVEL,
            topic: "Imperative",
            topic_slug: Some("imperative".into()),
            task: vec!["Put the verb in the imperative (vous).".into()],
            head: "____ ".into(),
            tail: format!(" {} !", comp),
            infinitive: inf,
            answer: vous.to_string(),
            rule: vec![
                format!("{} {} !", vous, comp),
                "The vous imperative = the vous form, no subject.".into(),
            ],
        }));
    }
    out
}

/* ── polite conditionnel ────────────────────────────────────────── */

const WANTS: &[&str] = &[
    "un café",
    "un croissant",
    "un verre d'eau",
    "l'addition",
    "une table pour deux",
    "un taxi",
    "des renseignements",
    "un billet pour Paris",
];

/// (subject, voudrais form)
const VOULOIR_SUBJ: &[(&str, &str)] = &[
    ("Je", "voudrais"),
    ("Tu", "voudrais"),
    ("Elle", "voudrait"),
    ("Nous", "voudrions"),
    ("Vous", "voudriez"),
    ("Ils", "voudraient"),
];

fn polite_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for (subj, form) in VOULOIR_SUBJ {
        for want in WANTS {
            let direct = match *subj {
                "Je" => "veux",
                "Tu" => "veux",
                "Elle" => "veut",
                "Nous" => "voulons",
                "Vous" => "voulez",
                _ => "veulent",
            };
            // Transform: blunt want → polite wish.
            out.push(ExerciseItem::Transform(TransformItem {
                id: format!("pol:{}:{}:tf", subj.to_lowercase(), slugify(want)),
                level: LEVEL,
                topic: "Polite conditionnel",
                topic_slug: Some("polite-conditionnel".into()),
                task: vec![
                    "Make it polite —".into(),
                    "swap vouloir for its conditionnel.".into(),
                ],
                source: format!("{} {} {}.", subj, direct, want),
                answer: format!("{} {} {}.", subj, form, want),
                rule: vec![
                    format!("vouloir {} → {} (conditionnel).", direct, form),
                    format!("{} {} {}.", subj, form, want),
                ],
            }));
        }
        // Cloze: the polite form in place.
        for want in WANTS {
            out.push(ExerciseItem::Cloze(ClozeItem {
                id: format!("pol:{}:{}:cl", subj.to_lowercase(), slugify(want)),
                level: LEVEL,
                topic: "Polite conditionnel",
                topic_slug: Some("polite-conditionnel".into()),
                task: vec!["Complete politely.".into()],
                head: format!("{} ", subj),
                tail: format!(" {}, s'il vous plaît.", want),
                infinitive: "vouloir",
                answer: form.to_string(),
                rule: vec![
                    format!("{} {} {}, s'il vous plaît.", subj, form, want),
                    "je voudrais = I would like — softer than je veux.".into(),
                ],
            }));
        }
    }
    out
}

fn slugify(s: &str) -> String {
    super::shared::slugify(s)
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "imperative" => imperative_items(),
        "polite-conditionnel" => polite_items(),
        _ => Vec::new(),
    }
}
