//! Family: passive voice — être + past participle (agrees with the
//! subject), the agent rider « par … », and the « on » → agentless
//! passive shortcut. Transform, agreement choices, cloze, error-spots.

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem, TransformItem};

use super::shared::punctuate;

const LEVEL: &str = "B1";

struct PassRow {
    /// passive subject
    subj: &'static str,
    /// the active sentence the learner rewrites
    active: &'static str,
    /// être chunk: est / sont / a été / ont été / sera / seront
    aux: &'static str,
    /// masculine singular participle
    p: &'static str,
    fem: bool,
    plur: bool,
    /// rider after the participle (agent or adverb), "" if none
    tail: &'static str,
    verb: &'static str,
}

const PASS_ROWS: &[PassRow] = &[
    PassRow { subj: "La souris", active: "Le chat mange la souris.", aux: "est", p: "mangé", fem: true, plur: false, tail: "par le chat", verb: "manger" },
    PassRow { subj: "Les Misérables", active: "Victor Hugo a écrit Les Misérables.", aux: "ont été", p: "écrit", fem: false, plur: true, tail: "par Victor Hugo", verb: "écrire" },
    PassRow { subj: "Le voleur", active: "La police a arrêté le voleur.", aux: "a été", p: "arrêté", fem: false, plur: false, tail: "par la police", verb: "arrêter" },
    PassRow { subj: "La lettre", active: "Le secrétaire a envoyé la lettre.", aux: "a été", p: "envoyé", fem: true, plur: false, tail: "par le secrétaire", verb: "envoyer" },
    PassRow { subj: "Le tabac", active: "On interdit le tabac ici.", aux: "est", p: "interdit", fem: false, plur: false, tail: "ici", verb: "interdire" },
    PassRow { subj: "Les frites", active: "On sert les frites avec le repas.", aux: "sont", p: "servi", fem: true, plur: true, tail: "avec le repas", verb: "servir" },
    PassRow { subj: "La tour Eiffel", active: "On a construit la tour Eiffel en 1889.", aux: "a été", p: "construit", fem: true, plur: false, tail: "en 1889", verb: "construire" },
    PassRow { subj: "Les baguettes", active: "On achète les baguettes chaque matin.", aux: "sont", p: "acheté", fem: true, plur: true, tail: "chaque matin", verb: "acheter" },
    PassRow { subj: "Le pont", active: "On réparera le pont l'été prochain.", aux: "sera", p: "réparé", fem: false, plur: false, tail: "l'été prochain", verb: "réparer" },
    PassRow { subj: "Les classes", active: "On annulera les classes demain.", aux: "seront", p: "annulé", fem: true, plur: true, tail: "demain", verb: "annuler" },
];

/// masculine singular → agreed form (feminine adds -e, plural -s).
fn participle(fem: bool, plur: bool, base: &str) -> String {
    let mut p = base.to_string();
    if fem {
        p.push('e');
    }
    if plur {
        p.push('s');
    }
    p
}

fn passive_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for r in PASS_ROWS {
        let key = super::shared::slugify(r.subj);
        let pp = participle(r.fem, r.plur, r.p);
        let passive = format!("{} {} {} {}.", r.subj, r.aux, pp, r.tail);
        // Transform: active → passive (être + agreed participle).
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("pas:{}:tf", key),
            level: LEVEL,
            topic: "Passive voice",
            topic_slug: Some("passive-voice".into()),
            task: vec![
                "Say it in the passive voice".into(),
                "(être + participle, agreed).".into(),
            ],
            source: r.active.to_string(),
            answer: passive.clone(),
            rule: vec![
                passive.clone(),
                "être + past participle — the participle agrees with the subject.".into(),
            ],
        }));
        // Choice: the participle agreement.
        let wrong_pp = participle(!r.fem, !r.plur, r.p);
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("pas:{}:ch", key),
            level: LEVEL,
            topic: "Passive voice",
            topic_slug: Some("passive-voice".into()),
            task: vec!["Agree the participle.".into()],
            sentence: format!("{} {} ____ {}.", r.subj, r.aux, r.tail),
            options: vec![pp.clone(), wrong_pp],
            correct: 0, // shuffled in `generate`
            rule: vec![
                passive.clone(),
                "The past participle agrees with the passive subject.".into(),
            ],
        }));
        // Cloze: the être + participle chunk.
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("pas:{}:cl", key),
            level: LEVEL,
            topic: "Passive voice",
            topic_slug: Some("passive-voice".into()),
            task: vec!["Complete the passive with être + participle.".into()],
            head: format!("{} ", r.subj),
            tail: format!(" {}.", r.tail),
            infinitive: r.verb,
            answer: format!("{} {}", r.aux, pp),
            rule: vec![passive, "être in the right tense + agreed participle.".into()],
        }));
    }
    // Error-spots: agreement flipped.
    for (words, wrong_i, fix, rule) in [
        (
            vec!["La", "lettre", "a", "été", "envoyé", "par", "le", "secrétaire"],
            4usize,
            "envoyée",
            "« La lettre » is feminine singular → the participle takes -e.",
        ),
        (
            vec!["Les", "frites", "sont", "servi", "avec", "le", "repas"],
            3usize,
            "servies",
            "« Les frites » is feminine plural → -es.",
        ),
    ] {
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("pas:{}:es", super::shared::slugify(fix)),
            level: LEVEL,
            topic: "Passive voice",
            topic_slug: Some("passive-voice".into()),
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

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "passive-voice" => passive_items(),
        _ => Vec::new(),
    }
}
