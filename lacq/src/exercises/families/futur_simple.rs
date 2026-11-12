//! Family: futur simple — verb tables with the future forms, habit
//! cues from tomorrow onward. Same engine shape as the imparfait
//! family: cloze per subject + person-confusion error-spots.

use crate::exercises::{ClozeItem, ErrorSpotItem, ExerciseItem};

use super::shared::{SUBJ_CAP, cap, punctuate, starts_vowel};

const LEVEL: &str = "A2";

struct FVerb {
    inf: &'static str,
    /// futur simple for je, tu, il/elle, nous, vous, ils/elles
    forms: [&'static str; 6],
    complement: &'static str,
}

const F_VERBS: &[FVerb] = &[
    FVerb { inf: "parler", forms: ["parlerai", "parleras", "parlera", "parlerons", "parlerez", "parleront"], complement: "au téléphone" },
    FVerb { inf: "finir", forms: ["finirai", "finiras", "finira", "finirons", "finirez", "finiront"], complement: "le travail" },
    FVerb { inf: "vendre", forms: ["vendrai", "vendras", "vendra", "vendrons", "vendrez", "vendront"], complement: "la voiture" },
    FVerb { inf: "prendre", forms: ["prendrai", "prendras", "prendra", "prendrons", "prendrez", "prendront"], complement: "le train" },
    FVerb { inf: "aller", forms: ["irai", "iras", "ira", "irons", "irez", "iront"], complement: "au cinéma" },
    FVerb { inf: "être", forms: ["serai", "seras", "sera", "serons", "serez", "seront"], complement: "en vacances" },
    FVerb { inf: "avoir", forms: ["aurai", "auras", "aura", "aurons", "aurez", "auront"], complement: "le temps" },
    FVerb { inf: "faire", forms: ["ferai", "feras", "fera", "ferons", "ferez", "feront"], complement: "du ski" },
    FVerb { inf: "venir", forms: ["viendrai", "viendras", "viendra", "viendrons", "viendrez", "viendront"], complement: "nous voir" },
    FVerb { inf: "voir", forms: ["verrai", "verras", "verra", "verrons", "verrez", "verront"], complement: "la mer" },
];

const CUES: &[&str] = &["Demain,", "La semaine prochaine,", "L'année prochaine,"];

fn subj_head(cue: &str, subj: &str, form: &str) -> (String, String) {
    if subj == "je" && starts_vowel(form) {
        // the elision folds into the answer chunk: « j'aurai »
        (format!("{} ", cue), format!("j'{}", form))
    } else {
        (format!("{} {} ", cue, subj), form.to_string())
    }
}

pub(crate) fn items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in F_VERBS {
        for &(subj, _subj_c, fi) in SUBJ_CAP {
            let form = v.forms[fi];
            for (ci, cue) in CUES.iter().enumerate() {
                let (head, answer) = subj_head(cue, subj, form);
                out.push(ExerciseItem::Cloze(ClozeItem {
                    id: format!("fut:{}:{}:{}:cl", v.inf, subj, ci),
                    level: LEVEL,
                    topic: "Futur simple",
                    topic_slug: Some("futur-simple".into()),
                    task: vec!["Put the verb in the futur simple.".into()],
                    head,
                    tail: format!(" {}.", v.complement),
                    infinitive: v.inf,
                    answer,
                    rule: vec![
                        format!("{} {} {}.", cue, subj, form),
                        "Endings: -ai, -as, -a, -ons, -ez, -ont (on the stem, irregulars too).".into(),
                    ],
                }));
                let audible = matches!(subj, "tu" | "nous" | "vous" | "ils" | "elles");
                let wrong = v.forms[2];
                if audible && wrong != form {
                    let mut words: Vec<String> = vec![cap(subj).to_string()];
                    words.push(wrong.to_string());
                    words.push(v.complement.to_string());
                    let words = punctuate(words);
                    out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
                        id: format!("fut:{}:{}:{}:es", v.inf, subj, ci),
                        level: LEVEL,
                        topic: "Futur simple",
                        topic_slug: Some("futur-simple".into()),
                        task: vec!["One word is wrong. Tap it.".into()],
                        words,
                        wrong: 1,
                        fix: form.to_string(),
                        fix_task: "Now write the correct form.",
                        rule: vec![format!("{} with « {} » → {}.", v.inf, subj, form)],
                    }));
                }
            }
        }
    }
    out
}
