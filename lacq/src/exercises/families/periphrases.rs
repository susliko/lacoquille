//! Family: present periphrases — one action bank, three framings.
//! être en train de + infinitive (right now), futur proche (aller +
//! infinitive), passé récent (venir de + infinitive). Transforms move
//! the plain present into the frame; choices catch the frame traps
//! (est mange, vais à partir, vient à…).

use crate::exercises::{ChoiceItem, ExerciseItem, TransformItem};

use super::shared::{SUBJ_CAP, starts_vowel};

const LEVEL: &str = "A1";

struct Act {
    inf: &'static str,
    /// present form for je..ils (the plain action)
    forms: [&'static str; 6],
    complement: &'static str,
}

const ACTIONS: &[Act] = &[
    Act { inf: "manger", forms: ["mange", "manges", "mange", "mangeons", "mangez", "mangent"], complement: "une pomme" },
    Act { inf: "partir", forms: ["pars", "pars", "part", "partons", "partez", "partent"], complement: "ce soir" },
    Act { inf: "travailler", forms: ["travaille", "travailles", "travaille", "travaillons", "travaillez", "travaillent"], complement: "jusqu'à six heures" },
    Act { inf: "dormir", forms: ["dors", "dors", "dort", "dormons", "dormez", "dorment"], complement: "tôt" },
    Act { inf: "lire", forms: ["lis", "lis", "lit", "lisons", "lisez", "lisent"], complement: "un roman" },
    Act { inf: "regarder", forms: ["regarde", "regardes", "regarde", "regardons", "regardez", "regardent"], complement: "un film" },
    Act { inf: "finir", forms: ["finis", "finis", "finit", "finissons", "finissez", "finissent"], complement: "le travail" },
    Act { inf: "sortir", forms: ["sors", "sors", "sort", "sortons", "sortez", "sortent"], complement: "avec des amis" },
    Act { inf: "écrire", forms: ["écris", "écris", "écrit", "écrivons", "écrivez", "écrivent"], complement: "une lettre" },
    Act { inf: "jouer", forms: ["joue", "joues", "joue", "jouons", "jouez", "jouent"], complement: "au tennis" },
];

/// Être forms per subject index (être en train de).
const ETRE: [&str; 6] = ["suis", "es", "est", "sommes", "êtes", "sont"];
/// Aller forms per subject index (futur proche).
const ALLER: [&str; 6] = ["vais", "vas", "va", "allons", "allez", "vont"];
/// Venir forms per subject index (passé récent).
const VENIR: [&str; 6] = ["viens", "viens", "vient", "venons", "venez", "viennent"];

/// Affirmative subject display: je elides before a vowel (j'écris).
fn subj_aff<'a>(subj: &'a str, subj_c: &'a str, form: &'a str) -> &'a str {
    if subj == "je" && starts_vowel(form) {
        "J'"
    } else {
        subj_c
    }
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "etre-en-train-de" => items_for(slug, "En train", &ETRE, "en train de", "est mange", "être en train de"),
        "futur-proche" => items_for(slug, "Proche", &ALLER, "", "vais à partir", "aller + infinitive"),
        "passe-recent" => items_for(slug, "Récent", &VENIR, "de", "vient à manger", "venir de + infinitive"),
        _ => Vec::new(),
    }
}

/// The frame is `{helper} {glue}{infinitive}`: glue is "en train de",
/// "" (aller takes the bare infinitive) or "de".
fn items_for(
    slug: &str,
    id_tag: &str,
    helper: &[&str; 6],
    glue: &'static str,
    wrong_frame: &'static str,
    frame_name: &'static str,
) -> Vec<ExerciseItem> {
    let title: &'static str = match slug {
        "etre-en-train-de" => "Être en train de",
        "futur-proche" => "Futur proche",
        _ => "Passé récent",
    };
    let mut out = Vec::new();
    for a in ACTIONS {
        for (subj, subj_c, fi) in SUBJ_CAP {
            let form = a.forms[*fi];
            let aux = helper[*fi];
            let source = format!("{} {} {}.", subj_aff(subj, subj_c, form), form, a.complement);
            let answer = format!("{} {} {} {}.", subj_aff(subj, subj_c, form), aux, glue, a.inf)
                .replace("  ", " ");
            out.push(ExerciseItem::Transform(TransformItem {
                id: format!("{id_tag}:{}:{}:tf", a.inf, subj),
                level: LEVEL,
                topic: title,
                topic_slug: Some(slug.into()),
                task: vec![
                    "Rewrite it in the other frame —".into(),
                    format!("keep the meaning, use « {} ».", frame_name),
                ],
                source,
                answer,
                rule: vec![
                    format!("The frame is {}.", frame_name),
                    format!("{} {} {}.", subj_aff(subj, subj_c, form), aux, {
                        if glue.is_empty() {
                            format!("{}", a.inf)
                        } else {
                            format!("{} {}", glue, a.inf)
                        }
                    }),
                ],
            }));
        }
        // Choice: the broken frame as the trap, on a fixed subject (il).
        let aux = helper[2];
        let right = if glue.is_empty() {
            format!("{} {}", aux, a.inf)
        } else {
            format!("{} {} {}", aux, glue, a.inf)
        };
        let wrong = wrong_frame.replace("est", aux).replace("vais", aux).replace("vient", aux);
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("{id_tag}:{}:ch", a.inf),
            level: LEVEL,
            topic: title,
            topic_slug: Some(slug.into()),
            task: vec!["Choose the right form.".into()],
            sentence: format!("Il ____ {}.", a.complement),
            options: vec![right.clone(), wrong],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("{} — the helper, then the bare infinitive.", frame_name),
                format!("Il {} {}.", right, a.complement),
            ],
        }));
    }
    out
}
