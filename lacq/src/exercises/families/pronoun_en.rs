//! Family: the pronoun en — de + thing and partitive quantities
//! replaced by en, before the verb (Tu en manges, J'en parle).
//! Mirror of the pronoun-y engine: transform + j'en elision trap.

use crate::exercises::{ChoiceItem, ExerciseItem, TransformItem};

use super::shared::{SUBJ_CAP, slugify, starts_vowel};

const LEVEL: &str = "A2";

struct EnCase {
    inf: &'static str,
    /// present forms je..ils
    forms: [&'static str; 6],
    /// the de-phrase / partitive that en replaces
    phrase: &'static str,
}

const EN_CASES: &[EnCase] = &[
    EnCase { inf: "manger", forms: ["mange", "manges", "mange", "mangeons", "mangez", "mangent"], phrase: "du pain" },
    EnCase { inf: "manger", forms: ["mange", "manges", "mange", "mangeons", "mangez", "mangent"], phrase: "de la viande" },
    EnCase { inf: "boire", forms: ["bois", "bois", "boit", "buvons", "buvez", "boivent"], phrase: "du café" },
    EnCase { inf: "boire", forms: ["bois", "bois", "boit", "buvons", "buvez", "boivent"], phrase: "de l'eau" },
    EnCase { inf: "acheter", forms: ["achète", "achètes", "achète", "achetons", "achetez", "achètent"], phrase: "des fleurs" },
    EnCase { inf: "parler", forms: ["parle", "parles", "parle", "parlons", "parlez", "parlent"], phrase: "de son voyage" },
    EnCase { inf: "parler", forms: ["parle", "parles", "parle", "parlons", "parlez", "parlent"], phrase: "de ce film" },
    EnCase { inf: "parler", forms: ["parle", "parles", "parle", "parlons", "parlez", "parlent"], phrase: "de mes amis" },
    EnCase { inf: "avoir besoin", forms: ["ai besoin", "as besoin", "a besoin", "avons besoin", "avez besoin", "ont besoin"], phrase: "de repos" },
    EnCase { inf: "discuter", forms: ["discute", "discutes", "discute", "discutons", "discutez", "discutent"], phrase: "de politique" },
];

fn subj_aff<'a>(subj: &'a str, subj_c: &'a str, form: &str) -> &'a str {
    if subj == "je" && starts_vowel(form) {
        "J'"
    } else {
        subj_c
    }
}

pub(crate) fn items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for c in EN_CASES {
        for &(subj, subj_c, fi) in SUBJ_CAP {
            let form = c.forms[fi];
            // Some forms already carry the reflexive pronoun — the
            // pronoun en goes AFTER it (Je m'en souviens).
            let source = format!("{} {} {}.", subj_aff(subj, subj_c, form), form, c.phrase);
            let answer = if subj == "je" {
                format!("J'en {}.", form)
            } else {
                format!("{} en {}.", subj_c, form)
            };
            out.push(ExerciseItem::Transform(TransformItem {
                id: format!("en:{}:{}:{}:tf", slugify(c.inf), slugify(c.phrase), subj),
                level: LEVEL,
                topic: "The pronoun en",
                topic_slug: Some("pronoun-en".into()),
                task: vec![
                    "Rewrite it, replacing the phrase".into(),
                    "with en.".into(),
                ],
                source,
                answer,
                rule: vec![
                    "de + thing → en, placed before the verb.".into(),
                    if subj == "je" {
                        "je + en elides: j'en.".into()
                    } else {
                        format!("{} en {}.", subj_c, form)
                    },
                ],
            }));
        }
        // Choice: the elision trap — je en is always wrong.
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("en-elide:{}:{}:ch", slugify(c.inf), slugify(c.phrase)),
            level: LEVEL,
            topic: "The pronoun en",
            topic_slug: Some("pronoun-en".into()),
            task: vec!["Choose the right answer.".into()],
            sentence: format!("Tu {} {} ? — ____", c.forms[1], c.phrase),
            options: vec![
                format!("J'en {}", c.forms[0]),
                format!("Je en {}", c.forms[0]),
            ],
            correct: 0, // shuffled in `generate`
            rule: vec![
                "en never keeps its own vowel: je + en → j'en.".into(),
                format!("j'en {}.", c.forms[0]),
            ],
        }));
    }
    out
}
