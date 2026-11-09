//! Family: agreement with a preceding direct object (B1). The object
//! — a relative-clause head or a preposed pronoun — forces the
//! participle to agree: les fleurs que j'ai achetées, je l'ai vue.

use crate::exercises::{ChoiceItem, ClozeItem, ExerciseItem};

use super::pronouns::{COD_NOUNS, COD_VERBS, cod_participle, gender_number};
use super::shared::slugify;

const LEVEL: &str = "B1";

/// (subject, relative-clause chunk after "que")
const REL_SUBJ: &[(&str, &str)] = &[
    ("je", "que j'ai"),
    ("tu", "que tu as"),
    ("il", "qu'il a"),
    ("nous", "que nous avons"),
    ("vous", "que vous avez"),
    ("ils", "qu'ils ont"),
];

/// Participle with the COD noun's agreement marks applied.
fn agreed(base: &str, n: &super::pronouns::CodNoun) -> String {
    let mut p = base.to_string();
    if n.feminine {
        p.push('e');
    }
    if n.plural {
        p.push('s');
    }
    p
}

fn agreement_rule(np: &str, n: &super::pronouns::CodNoun, participle: &str) -> Vec<String> {
    let marks = match (n.feminine, n.plural) {
        (false, false) => "no change",
        (true, false) => "add -e",
        (false, true) => "add -s",
        (true, true) => "add -es",
    };
    vec![
        format!("The object {} ({}) moved before the verb,", np, gender_number(n)),
        format!("so the participle agrees: {} — {}.", participle, marks),
    ]
}

pub(crate) fn items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for n in COD_NOUNS {
        for inf in COD_VERBS {
            let base = cod_participle(inf);
            let agreed_form = agreed(base, n);

            // Cloze: relative clause — "Les fleurs que j'ai ____."
            for (subj, rel) in REL_SUBJ {
                out.push(ExerciseItem::Cloze(ClozeItem {
                    id: format!("accord:{}:{}:{}:cl", inf, slugify(n.np), subj),
                    level: LEVEL,
                    topic: "Agreement with preceding object",
                    topic_slug: Some("accord-antepose".into()),
                    task: vec!["Put the verb in the passé composé.".into()],
                    head: format!("{} {} ", super::shared::cap(n.np), rel),
                    tail: ".".into(),
                    infinitive: inf,
                    answer: agreed_form.clone(),
                    rule: agreement_rule(n.np, n, &agreed_form),
                }));
            }

            // Choice: question + answer with the pronoun in front.
            let (pron_chunk, options) = if n.pron == "les" {
                (
                    "les ai",
                    vec![agreed_form.clone(), format!("{}s", base), base.to_string()],
                )
            } else if n.feminine {
                (
                    "l'ai",
                    vec![agreed_form.clone(), base.to_string(), format!("{}s", base)],
                )
            } else {
                (
                    "l'ai",
                    vec![base.to_string(), format!("{}e", base), format!("{}es", base)],
                )
            };
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("accord:{}:{}:ch", inf, slugify(n.np)),
                level: LEVEL,
                topic: "Agreement with preceding object",
                topic_slug: Some("accord-antepose".into()),
                task: vec!["Choose the right form.".into()],
                sentence: format!(
                    "Tu as {} {} ? — Oui, je {} ____.",
                    base, n.np, pron_chunk
                ),
                options,
                correct: 0, // shuffled in `generate`
                rule: vec![
                    format!("{} = {} → the pronoun {} replaces it,", n.np, gender_number(n), n.pron),
                    format!("and the participle agrees: je {} {}.", pron_chunk, agreed_form),
                ],
            }));
        }
    }
    out
}
