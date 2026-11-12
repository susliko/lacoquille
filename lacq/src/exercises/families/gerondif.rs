//! Family: gérondif — « en + participe présent » for simultaneity and
//! means ("il maigrit en mangeant moins"). Formation traps get their
//! own error-spots: mangeant (keep the e), commençant (cedilla),
//! appelant / jetant (never double the consonant), achetant (è → e).

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem, TransformItem};

use super::shared::punctuate;

const LEVEL: &str = "B1";

/// (infinitive, gérondif, complement) — all take the nous-stem + -ant.
const GER_VERBS: &[(&str, &str, &str)] = &[
    ("parler", "en parlant", "au téléphone"),
    ("conduire", "en conduisant", "la radio"),
    ("faire", "en faisant", "du sport"),
    ("cuisiner", "en cuisinant", "un bon dîner"),
    ("courir", "en courant", "le bus"),
    ("lire", "en lisant", "le journal"),
    ("travailler", "en travaillant", "vite"),
    ("dormir", "en dormant", "au bureau"),
];

/// (main clause, simultaneous 3sg clause, gérondif, noun after it) →
/// transform target.
const GER_PAIRS: &[(&str, &str, &str, &str)] = &[
    ("Il reste en forme", "il fait du sport", "en faisant", "du sport"),
    ("Elle apprend le français", "elle regarde des films", "en regardant", "des films"),
    ("Il a gagné du temps", "il a pris un raccourci", "en prenant", "un raccourci"),
    ("Elle gagne sa vie", "elle écrit des articles", "en écrivant", "des articles"),
    ("On protège la planète", "on recycle ses déchets", "en recyclant", "ses déchets"),
    ("Tu progresses vite", "tu répètes chaque jour", "en répétant", "chaque jour"),
    ("Ils se détendent", "ils écoutent de la musique", "en écoutant", "de la musique"),
    ("Elle se blesse", "elle court sur le verglas", "en courant", "sur le verglas"),
];

/// (wrong form, correct form, sentence using the wrong one — no final
/// period, `punctuate` adds it)
const GER_TRAPS: &[(&str, &str, &str)] = &[
    ("mangant", "mangeant", "En mangant trop, il grossit"),
    ("commencant", "commençant", "En commencant tôt, on finit tôt"),
    ("appellant", "appelant", "En appellant un taxi, elle a gagné du temps"),
    ("achétant", "achetant", "En achétant en ligne, on économise"),
    ("jettant", "jetant", "En jettant moins, on protège la planète"),
    ("réppétant", "répétant", "En réppétant chaque jour, on apprend vite"),
];

fn gerondif_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(inf, gerond, complement) in GER_VERBS {
        // Cloze: the en + gérant chunk.
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("ger:{}:cl", inf),
            level: LEVEL,
            topic: "Gérondif",
            topic_slug: Some("gerondif".into()),
            task: vec!["Complete with « en » + the participe présent.".into()],
            head: "Il reste en forme ".into(),
            tail: format!(" {}.", complement),
            infinitive: inf,
            answer: gerond.to_string(),
            rule: vec![
                format!("Il reste en forme {} {}.", gerond, complement),
                "en + the nous-form of the present, minus -ons, plus -ant.".into(),
            ],
        }));
        // Choice: gérant vs bare infinitive after en.
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("ger:{}:ch", inf),
            level: LEVEL,
            topic: "Gérondif",
            topic_slug: Some("gerondif".into()),
            task: vec!["Choose the right form.".into()],
            sentence: format!("Il reste en forme en ____ {}.", complement),
            options: vec![gerond.strip_prefix("en ").unwrap().to_string(), inf.to_string()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("Il reste en forme {} {}.", gerond, complement),
                "After « en », the verb takes the participe présent, never the infinitive.".into(),
            ],
        }));
    }
    // Transform: two clauses → one sentence with the gérondif.
    for &(main, clause, gerond, tail_np) in GER_PAIRS {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("ger:{}:tf", super::shared::slugify(&tail_np.chars().take(12).collect::<String>())),
            level: LEVEL,
            topic: "Gérondif",
            topic_slug: Some("gerondif".into()),
            task: vec![
                "Join the two ideas with « en » +".into(),
                "the participe présent.".into(),
            ],
            source: format!("{}. En même temps, {}.", main, clause),
            answer: format!("{} {} {}.", main, gerond, tail_np),
            rule: vec![
                format!("{} {} {}.", main, gerond, tail_np),
                "en + participe présent = while doing / by doing.".into(),
            ],
        }));
    }
    // Error-spots: the formation traps.
    for &(wrong, good, sentence) in GER_TRAPS {
        let mut words: Vec<String> = sentence.split(' ').map(|s| s.to_string()).collect();
        // the wrong form appears mid-sentence; find its index
        let idx = words
            .iter()
            .position(|w| w.trim_end_matches(|c: char| !c.is_alphabetic()) == wrong)
            .unwrap_or(0);
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("ger:{}:es", super::shared::slugify(good)),
            level: LEVEL,
            topic: "Gérondif",
            topic_slug: Some("gerondif".into()),
            task: vec!["One word is wrong. Tap it.".into()],
            words: punctuate(words),
            wrong: idx,
            fix: good.to_string(),
            fix_task: "Now write the correct form.",
            rule: vec![
                format!("{} — never « {} ».", good, wrong),
                formation_note(good).to_string(),
            ],
        }));
    }
    out
}

fn formation_note(good: &str) -> &'static str {
    match good {
        "mangeant" => "-ger verbs keep the e: en mangeant.",
        "commençant" => "-cer verbs take the cedilla before a: en commençant.",
        "appelant" => "appeler loses one l: en appelant (like appelle → appelant).",
        "achetant" => "acheter keeps the e plain: en achetant (the è only appears before -es/-e).",
        "jetant" => "jeter loses one t: en jetant.",
        "répétant" => "répéter keeps one t and no double accent: en répétant.",
        _ => "nous-form minus -ons, plus -ant.",
    }
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "gerondif" => gerondif_items(),
        _ => Vec::new(),
    }
}
