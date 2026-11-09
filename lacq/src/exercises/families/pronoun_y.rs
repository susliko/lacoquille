//! Family: the pronoun y — à + place → y, before the verb. Transform
//! drills the replacement; choice drills the j'y elision trap.

use crate::exercises::{ChoiceItem, ExerciseItem, TransformItem};

use super::shared::{SUBJ_CAP, slugify, starts_vowel};

const LEVEL: &str = "B1";

struct YCase {
    inf: &'static str,
    /// present forms for je, tu, il/elle, nous, vous, ils/elles
    forms: [&'static str; 6],
    /// À (+ article) + place.
    destination: &'static str,
}

const Y_CASES: &[YCase] = &[
    YCase { inf: "aller", forms: ["vais", "vas", "va", "allons", "allez", "vont"], destination: "au cinéma" },
    YCase { inf: "aller", forms: ["vais", "vas", "va", "allons", "allez", "vont"], destination: "à Paris" },
    YCase { inf: "habiter", forms: ["habite", "habites", "habite", "habitons", "habitez", "habitent"], destination: "à Lyon" },
    YCase { inf: "habiter", forms: ["habite", "habites", "habite", "habitons", "habitez", "habitent"], destination: "aux États-Unis" },
    YCase { inf: "rester", forms: ["reste", "restes", "reste", "restons", "restez", "restent"], destination: "au bureau" },
    YCase { inf: "rester", forms: ["reste", "restes", "reste", "restons", "restez", "restent"], destination: "à la montagne" },
    YCase { inf: "travailler", forms: ["travaille", "travailles", "travaille", "travaillons", "travaillez", "travaillent"], destination: "à la piscine" },
    YCase { inf: "travailler", forms: ["travaille", "travailles", "travaille", "travaillons", "travaillez", "travaillent"], destination: "au théâtre" },
    YCase { inf: "retourner", forms: ["retourne", "retournes", "retourne", "retournons", "retournez", "retournent"], destination: "au stade" },
    YCase { inf: "penser", forms: ["pense", "penses", "pense", "pensons", "pensez", "pensent"], destination: "aux vacances" },
    YCase { inf: "penser", forms: ["pense", "penses", "pense", "pensons", "pensez", "pensent"], destination: "au concert" },
    YCase { inf: "descendre", forms: ["descends", "descends", "descend", "descendons", "descendez", "descendent"], destination: "à Marseille" },
];

pub(crate) fn items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for c in Y_CASES {
        for (subj, subj_c, fi) in SUBJ_CAP {
            let form = c.forms[*fi];
            // Transform: replace the place with y.
            let source = if *subj == "je" && (starts_vowel(form) || form.starts_with('h')) {
                format!("J'{} {}.", form, c.destination)
            } else if *subj == "je" {
                format!("Je {} {}.", form, c.destination)
            } else {
                format!("{} {} {}.", subj_c, form, c.destination)
            };
            let answer = if *subj == "je" {
                format!("J'y {}.", form)
            } else {
                format!("{} y {}.", subj_c, form)
            };
            out.push(ExerciseItem::Transform(TransformItem {
                id: format!("y:{}:{}:{}:tf", c.inf, slugify(c.destination), subj),
                level: LEVEL,
                topic: "The pronoun y",
                topic_slug: Some("pronoun-y".into()),
                task: vec![
                    "Rewrite it, replacing the place".into(),
                    "with y.".into(),
                ],
                source,
                answer,
                rule: vec![
                    "À (+ article) + place becomes y, placed".into(),
                    "before the verb.".into(),
                    if *subj == "je" {
                        "Before je it elides: j'y.".into()
                    } else {
                        format!("{} y {}.", subj_c, form)
                    },
                ],
            }));
        }
        // Choice: the elision trap — je y is always wrong.
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("y-elide:{}:{}:ch", c.inf, slugify(c.destination)),
            level: LEVEL,
            topic: "The pronoun y",
            topic_slug: Some("pronoun-y".into()),
            task: vec!["Choose the right answer.".into()],
            sentence: format!("Tu {} {} ? — ____", c.forms[1], c.destination),
            options: vec![
                format!("J'y {}", c.forms[0]),
                format!("Je y {}", c.forms[0]),
            ],
            correct: 0, // shuffled in `generate`
            rule: vec![
                "y never keeps its own vowel:".into(),
                format!("je + y → j'y ({0} y {1} → j'y {1}).", "tu", c.forms[1]),
            ],
        }));
    }
    out
}
