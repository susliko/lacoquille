//! Family: object pronouns (transform). COD in the passé composé with
//! computed participle agreement; COI in the present.

use crate::exercises::{ExerciseItem, TransformItem};

use super::shared::{SUBJ_CAP, avoir_pc, slugify};

pub(crate) struct CodNoun {
    pub(crate) np: &'static str,
    pub(crate) pron: &'static str,
    pub(crate) feminine: bool,
    pub(crate) plural: bool,
}

pub(crate) const COD_NOUNS: &[CodNoun] = &[
    CodNoun { np: "le livre", pron: "le", feminine: false, plural: false },
    CodNoun { np: "le journal", pron: "le", feminine: false, plural: false },
    CodNoun { np: "le pain", pron: "le", feminine: false, plural: false },
    CodNoun { np: "la lettre", pron: "la", feminine: true, plural: false },
    CodNoun { np: "la voiture", pron: "la", feminine: true, plural: false },
    CodNoun { np: "les fleurs", pron: "les", feminine: true, plural: true },
    CodNoun { np: "les clés", pron: "les", feminine: true, plural: true },
];

pub(crate) const COD_VERBS: &[&str] = &["voir", "lire", "acheter", "connaître", "prendre", "finir"];

pub(crate) fn cod_participle(inf: &str) -> &'static str {
    match inf {
        "voir" => "vu",
        "lire" => "lu",
        "acheter" => "acheté",
        "connaître" => "connu",
        "prendre" => "pris",
        _ => "fini",
    }
}

pub(crate) fn gender_number(n: &CodNoun) -> &'static str {
    match (n.feminine, n.plural) {
        (false, false) => "masculine singular",
        (true, false) => "feminine singular",
        (false, true) => "masculine plural",
        (true, true) => "feminine plural",
    }
}

struct CoiNoun {
    np: &'static str,
    pron: &'static str,
}

const COI_NOUNS: &[CoiNoun] = &[
    CoiNoun { np: "à Marie", pron: "lui" },
    CoiNoun { np: "à sa sœur", pron: "lui" },
    CoiNoun { np: "à son voisin", pron: "lui" },
    CoiNoun { np: "à ses parents", pron: "leur" },
    CoiNoun { np: "à ses collègues", pron: "leur" },
];

struct CoiVerb {
    inf: &'static str,
    /// present forms for je, tu, il/elle, nous, vous, ils/elles
    forms: [&'static str; 6],
}

const COI_VERBS: &[CoiVerb] = &[
    CoiVerb { inf: "téléphoner", forms: ["téléphone", "téléphones", "téléphone", "téléphonons", "téléphonez", "téléphonent"] },
    CoiVerb { inf: "écrire", forms: ["écris", "écris", "écrit", "écrivons", "écrivez", "écrivent"] },
    CoiVerb { inf: "parler", forms: ["parle", "parles", "parle", "parlons", "parlez", "parlent"] },
    CoiVerb { inf: "demander", forms: ["demande", "demandes", "demande", "demandons", "demandez", "demandent"] },
    CoiVerb { inf: "répondre", forms: ["réponds", "réponds", "répond", "répondons", "répondez", "répondent"] },
];

pub(crate) fn items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    // COD transforms (passé composé).
    for n in COD_NOUNS {
        for inf in COD_VERBS {
            for (subj, subj_c, _) in SUBJ_CAP {
                let base = cod_participle(inf);
                let mut agreed = base.to_string();
                if n.feminine {
                    agreed.push('e');
                }
                if n.plural {
                    agreed.push('s');
                }
                let level = if agreed == base { "A2" } else { "B1" };
                let av = avoir_pc(subj);
                let source = if *subj == "je" {
                    format!("J'{} {} {}.", av, base, n.np)
                } else {
                    format!("{} {} {} {}.", subj_c, av, base, n.np)
                };
                let pron_chunk = if n.pron == "les" {
                    format!("les {}", av)
                } else {
                    format!("l'{}", av)
                };
                let answer = if *subj == "je" {
                    format!("Je {} {}", pron_chunk, agreed)
                } else {
                    format!("{} {} {}", subj_c, pron_chunk, agreed)
                };
                out.push(ExerciseItem::Transform(TransformItem {
                    id: format!("pron-cod:{}:{}:{}", inf, slugify(n.np), subj),
                    level,
                    topic: "Object pronouns",
                    topic_slug: Some("pronouns-cod-coi".into()),
                    task: vec![
                        "Rewrite it, replacing the object".into(),
                        "with a pronoun.".into(),
                    ],
                    source,
                    answer,
                    rule: vec![
                        format!("{} is {} → {}.", super::shared::cap(n.np), gender_number(n), n.pron),
                        "The pronoun goes before avoir.".into(),
                    ],
                }));
            }
        }
    }
    // COI transforms (present).
    for n in COI_NOUNS {
        for v in COI_VERBS {
            for (subj, subj_c, fi) in SUBJ_CAP {
                let form = v.forms[*fi];
                let source = format!("{} {} {}.", subj_c, form, n.np);
                let answer = format!("{} {} {}", subj_c, n.pron, form);
                out.push(ExerciseItem::Transform(TransformItem {
                    id: format!("pron-coi:{}:{}:{}", v.inf, slugify(n.np), subj),
                    level: "A2",
                    topic: "Object pronouns",
                    topic_slug: Some("pronouns-cod-coi".into()),
                    task: vec![
                        "Rewrite it, replacing the object".into(),
                        "with a pronoun.".into(),
                    ],
                    source,
                    answer,
                    rule: vec![
                        "À + a person becomes lui (singular) or leur".into(),
                        "(plural), placed before the conjugated verb.".into(),
                    ],
                }));
            }
        }
    }
    out
}
