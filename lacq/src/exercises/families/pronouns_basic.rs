//! Family: pronoun basics — two topics. Subject pronouns & on (replace
//! a noun with the right pronoun; on = nous with a 3rd-person form)
//! and stressed pronouns (moi/toi/lui… after prepositions and after
//! c'est). Choice traps only exist where the stressed form differs
//! from the weak form — moi/je, toi/tu, lui/il, eux/ils.

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem, TransformItem};

use super::shared::{punctuate, slugify};

const LEVEL: &str = "A1";

fn cap_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/* ── subject pronouns & on ──────────────────────────────────────── */

/// (noun phrase, pronoun it stands for, capitalized pronoun)
const SUBJECT_NPS: &[(&str, &str, &str)] = &[
    ("Marie", "elle", "Elle"),
    ("Pierre", "il", "Il"),
    ("ma sœur", "elle", "Elle"),
    ("mon frère", "il", "Il"),
    ("la voiture", "elle", "Elle"),
    ("le livre", "il", "Il"),
    ("mes parents", "ils", "Ils"),
    ("les enfants", "ils", "Ils"),
    ("les fleurs", "elles", "Elles"),
    ("les livres", "ils", "Ils"),
    ("la table et le lit", "ils", "Ils"),
    ("Anne et Sophie", "elles", "Elles"),
];

/// (infinitive tag, elle-form, ils-form, nous-form)
const SUB_VERBS: &[(&str, &str, &str, &str)] = &[
    ("parler-francais", "parle français", "parlent français", "parlons français"),
    ("habiter-ici", "habite ici", "habitent ici", "habitons ici"),
    ("travailler-beaucoup", "travaille beaucoup", "travaillent beaucoup", "travaillons beaucoup"),
    ("aimer-le-cafe", "aime le café", "aiment le café", "aimons le café"),
    ("regarder-la-tele", "regarde la télé", "regardent la télé", "regardons la télé"),
    ("finir-a-cinq-heures", "finit à cinq heures", "finissent à cinq heures", "finissons à cinq heures"),
    ("ecouter-la-radio", "écoute la radio", "écoutent la radio", "écoutons la radio"),
    ("choisir-un-livre", "choisit un livre", "choisissent un livre", "choisissons un livre"),
];

fn subject_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(np, pron, pron_cap) in SUBJECT_NPS {
        let plural = pron == "ils" || pron == "elles";
        for &(tag, sg, pl, _nous) in SUB_VERBS {
            let form = if plural { pl } else { sg };
            let wrong_cap = match pron {
                "elle" => "Il",
                "il" => "Elle",
                "ils" => "Elles",
                _ => "Ils",
            };
            out.push(ExerciseItem::Transform(TransformItem {
                id: format!("subp:{}:{}:tf", slugify(np), tag),
                level: LEVEL,
                topic: "Subject pronouns & on",
                topic_slug: Some("subject-pronouns".into()),
                task: vec!["Replace the subject with a pronoun.".into()],
                source: format!("{} {}.", np, form),
                answer: format!("{} {}.", pron_cap, form),
                rule: vec![
                    format!("{} → {}.", np, pron_cap),
                    format!("{} {}.", pron_cap, form),
                ],
            }));
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("subp:{}:{}:ch", slugify(np), tag),
                level: LEVEL,
                topic: "Subject pronouns & on",
                topic_slug: Some("subject-pronouns".into()),
                task: vec!["Choose the right pronoun.".into()],
                sentence: format!("____ {}.", form),
                options: vec![pron_cap.to_string(), wrong_cap.into()],
                correct: 0, // shuffled in `generate`
                rule: vec![
                    format!("{} = {}.", np, pron_cap),
                    format!("{} {}.", pron_cap, form),
                ],
            }));
        }
    }
    // On = nous: same meaning, third-person singular verb form.
    for &(tag, sg, _pl, nous) in SUB_VERBS {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("subp-on:{}:tf", tag),
            level: LEVEL,
            topic: "Subject pronouns & on",
            topic_slug: Some("subject-pronouns".into()),
            task: vec![
                "Rewrite it with « on » —".into(),
                "same meaning, third-person form.".into(),
            ],
            source: format!("Nous {}.", nous),
            answer: format!("On {}.", sg),
            rule: vec![
                "on = nous (informal) — but the verb is 3rd person".into(),
                format!("singular: on {} (not {}).", sg, nous),
            ],
        }));
    }
    out
}

/* ── stressed pronouns ──────────────────────────────────────────── */

const AVEC_VERBS: &[&str] = &[
    "vient avec",
    "part avec",
    "travaille avec",
    "dîne avec",
    "reste avec",
    "joue avec",
    "sort avec",
];

/// Only the subjects whose stressed form DIFFERS from the weak form —
/// the rest (elle, nous, vous, elles) have no contrast to drill.
const STRESSED_TRAPS: &[(&str, &str, &str)] = &[
    ("je", "Je", "moi"),
    ("tu", "Tu", "toi"),
    ("il", "Il", "lui"),
    ("ils", "Ils", "eux"),
];

/// (noun phrase, stressed pronoun after c'est)
const CEST_NPS: &[(&str, &str)] = &[
    ("Marie", "elle"),
    ("Pierre", "lui"),
    ("mon frère", "lui"),
    ("ma sœur", "elle"),
    ("mes parents", "eux"),
    ("Anne et Sophie", "elles"),
    ("le docteur", "lui"),
    ("mes amis", "eux"),
];

fn stressed_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(subj, cap_form, stressed) in STRESSED_TRAPS {
        for verb in AVEC_VERBS {
            let form = avec_form(subj, verb);
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("stp:{}:{}:ch", slugify(subj), slugify(verb)),
                level: LEVEL,
                topic: "Stressed pronouns",
                topic_slug: Some("stressed-pronouns".into()),
                task: vec!["Choose the right pronoun.".into()],
                sentence: format!("{} {} ____.", cap_form, form),
                options: vec![stressed.to_string(), subj.to_string()],
                correct: 0, // shuffled in `generate`
                rule: vec![
                    "After a preposition (avec, pour, chez…), French uses".into(),
                    format!("the stressed pronoun: {} {} {}.", cap_form, form, stressed),
                ],
            }));
        }
    }
    for &(np, stressed) in CEST_NPS {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("stp-cest:{}:tf", slugify(np)),
            level: LEVEL,
            topic: "Stressed pronouns",
            topic_slug: Some("stressed-pronouns".into()),
            task: vec!["Replace the noun with a stressed pronoun.".into()],
            source: format!("C'est {}.", np),
            answer: format!("C'est {}.", stressed),
            rule: vec![
                "After c'est, the stressed form stands alone:".into(),
                format!("C'est {}.", stressed),
            ],
        }));
    }
    out
}

/// Verb forms per subject for the « avec » verbs (elle-form in, per-subject out).
fn avec_form(subj: &str, verb_elle: &str) -> &'static str {
    match (subj, verb_elle) {
        ("je", "vient avec") => "viens avec",
        ("je", "part avec") => "pars avec",
        ("je", "travaille avec") => "travaille avec",
        ("je", "dîne avec") => "dîne avec",
        ("je", "reste avec") => "reste avec",
        ("je", "joue avec") => "joue avec",
        ("je", "sort avec") => "sors avec",
        ("tu", "vient avec") => "viens avec",
        ("tu", "part avec") => "pars avec",
        ("tu", "travaille avec") => "travailles avec",
        ("tu", "dîne avec") => "dînes avec",
        ("tu", "reste avec") => "restes avec",
        ("tu", "joue avec") => "joues avec",
        ("tu", "sort avec") => "sors avec",
        ("il", "vient avec") => "vient avec",
        ("il", "part avec") => "part avec",
        ("il", "travaille avec") => "travaille avec",
        ("il", "dîne avec") => "dîne avec",
        ("il", "reste avec") => "reste avec",
        ("il", "joue avec") => "joue avec",
        ("il", "sort avec") => "sort avec",
        ("ils", "vient avec") => "viennent avec",
        ("ils", "part avec") => "partent avec",
        ("ils", "travaille avec") => "travaillent avec",
        ("ils", "dîne avec") => "dînent avec",
        ("ils", "reste avec") => "restent avec",
        ("ils", "joue avec") => "jouent avec",
        ("ils", "sort avec") => "sortent avec",
        _ => "est avec",
    }
}

/* ── uses of on (A2) ────────────────────────────────────────────── */

const LEVEL2: &str = "A2";

/// (inf, il-form, nous-form, ils-form, complement, cue, second cue)
const ON_VERBS: &[(&str, &str, &str, &str, &str, &str, &str)] = &[
    ("manger", "mange", "mangeons", "mangent", "au restaurant", "ce soir", "d'habitude"),
    ("partir", "part", "partons", "partent", "en Provence", "ce week-end", "en juin"),
    ("sortir", "sort", "sortons", "sortent", "danser", "le samedi soir", "souvent"),
    ("dîner", "dîne", "dînons", "dînent", "chez Léo", "ce soir", "le vendredi"),
    ("voyager", "voyage", "voyageons", "voyagent", "en train", "cet été", "en général"),
    ("rentrer", "rentre", "rentrons", "rentrent", "avant minuit", "ce soir", "en hiver"),
    ("travailler", "travaille", "travaillons", "travaillent", "beaucoup", "le lundi", "en ce moment"),
    ("prendre", "prend", "prenons", "prennent", "le café en terrasse", "le matin", "le week-end"),
    ("parler", "parle", "parlons", "parlent", "français", "en France", "ici"),
];

fn on2_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(inf, sg, nous, pl, complement, cue, cue2) in ON_VERBS {
        // Transform: nous → on, same meaning, 3rd-person singular form.
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("on2:{}:tf", inf),
            level: LEVEL2,
            topic: "Uses of on",
            topic_slug: Some("uses-of-on".into()),
            task: vec![
                "Rewrite it with « on » —".into(),
                "same meaning, third-person form.".into(),
            ],
            source: format!("Nous {} {}.", nous, complement),
            answer: format!("On {} {}.", sg, complement),
            rule: vec![
                "on = nous (informal), but the verb is 3rd person".into(),
                format!("singular: on {} {}.", sg, complement),
            ],
        }));
        // Cloze: on + 3rd-person singular, with a cue up front.
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("on2:{}:cl", inf),
            level: LEVEL2,
            topic: "Uses of on",
            topic_slug: Some("uses-of-on".into()),
            task: vec!["Complete with the right form.".into()],
            head: format!("{}, on ", cap_first(cue2)),
            tail: format!(" {}.", complement),
            infinitive: inf,
            answer: sg.to_string(),
            rule: vec![
                format!("{}, on {} {}.", cap_first(cue2), sg, complement),
                "on always takes the 3rd-person singular form.".into(),
            ],
        }));
        // Choice: singular vs plural trap.
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("on2:{}:ch", inf),
            level: LEVEL2,
            topic: "Uses of on",
            topic_slug: Some("uses-of-on".into()),
            task: vec!["Choose the right form.".into()],
            sentence: format!("{}, on ____ {}.", cap_first(cue), complement),
            options: vec![sg.into(), pl.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("{}, on {} {}.", cap_first(cue), sg, complement),
                format!("on takes the singular — not {}.", pl),
            ],
        }));
        // Error-spot: the plural form sneaks in after on.
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("on2:{}:es", inf),
            level: LEVEL2,
            topic: "Uses of on",
            topic_slug: Some("uses-of-on".into()),
            task: vec!["One word is wrong. Tap it.".into()],
            words: punctuate(vec![
                "On".into(),
                pl.into(),
                complement.to_string(),
            ]),
            wrong: 1,
            fix: sg.to_string(),
            fix_task: "Now write the correct form.",
            rule: vec![
                format!("on {} {} — never {}.", sg, complement, pl),
                "on = one/we/people in general: always singular.".into(),
            ],
        }));
    }
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "subject-pronouns" => subject_items(),
        "stressed-pronouns" => stressed_items(),
        "uses-of-on" => on2_items(),
        _ => Vec::new(),
    }
}
