//! Family: si-clauses — the two B1 si-patterns: si + présent → futur
//! (real possibility) and si + imparfait → conditionnel (unreal).
//! The B2 si + plus-que-parfait pattern will join this file. The
//! result clause is a fixed generic frame so every per-verb
//! complement stays grammatical; the « si + futur » trap is drilled
//! as choice and error-spot.

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem, TransformItem};

use super::shared::{SUBJ_CAP, punctuate, starts_vowel};

const LEVEL: &str = "B1";

struct SiVerb {
    inf: &'static str,
    /// présent, imparfait, futur, conditionnel — 6 persons each
    pres: [&'static str; 6],
    imp: [&'static str; 6],
    fut: [&'static str; 6],
    cond: [&'static str; 6],
    /// complement that fits THIS verb in the si-clause
    compl: &'static str,
}

const SI_VERBS: &[SiVerb] = &[
    SiVerb {
        inf: "venir", pres: ["viens", "viens", "vient", "venons", "venez", "viennent"],
        imp: ["venais", "venais", "venait", "venions", "veniez", "venaient"],
        fut: ["viendrai", "viendras", "viendra", "viendrons", "viendrez", "viendront"],
        cond: ["viendrais", "viendrais", "viendrait", "viendrions", "viendriez", "viendraient"],
        compl: "à la fête",
    },
    SiVerb {
        inf: "finir", pres: ["finis", "finis", "finit", "finissons", "finissez", "finissent"],
        imp: ["finissais", "finissais", "finissait", "finissions", "finissiez", "finissaient"],
        fut: ["finirai", "finiras", "finira", "finirons", "finirez", "finiront"],
        cond: ["finirais", "finirais", "finirait", "finirions", "finiriez", "finiraient"],
        compl: "le travail tôt",
    },
    SiVerb {
        inf: "être", pres: ["suis", "es", "est", "sommes", "êtes", "sont"],
        imp: ["étais", "étais", "était", "étions", "étiez", "étaient"],
        fut: ["serai", "seras", "sera", "serons", "serez", "seront"],
        cond: ["serais", "serais", "serait", "serions", "seriez", "seraient"],
        compl: "en forme",
    },
    SiVerb {
        inf: "avoir", pres: ["ai", "as", "a", "avons", "avez", "ont"],
        imp: ["avais", "avais", "avait", "avions", "aviez", "avaient"],
        fut: ["aurai", "auras", "aura", "aurons", "aurez", "auront"],
        cond: ["aurais", "aurais", "aurait", "aurions", "auriez", "auraient"],
        compl: "le temps",
    },
    SiVerb {
        inf: "pouvoir", pres: ["peux", "peux", "peut", "pouvons", "pouvez", "peuvent"],
        imp: ["pouvais", "pouvais", "pouvait", "pouvions", "pouviez", "pouvaient"],
        fut: ["pourrai", "pourras", "pourra", "pourrons", "pourrez", "pourront"],
        cond: ["pourrais", "pourrais", "pourrait", "pourrions", "pourriez", "pourraient"],
        compl: "vous aider",
    },
    SiVerb {
        inf: "faire", pres: ["fais", "fais", "fait", "faisons", "faites", "font"],
        imp: ["faisais", "faisais", "faisait", "faisions", "faisiez", "faisaient"],
        fut: ["ferai", "feras", "fera", "ferons", "ferez", "feront"],
        cond: ["ferais", "ferais", "ferait", "ferions", "feriez", "feraient"],
        compl: "les courses",
    },
];

/// si + subject with the required elision, capitalized (every use is
/// sentence-initial): S'il / S'ils; « Si je », « Si tu », « Si elle »,
/// « Si nous », « Si vous », « Si elles » stay.
fn si_subj(subj: &str) -> String {
    match subj {
        "il" => "S'il".into(),
        "ils" => "S'ils".into(),
        other => format!("Si {}", other),
    }
}

/// the form with je-elision folded in ("j'aurai"), as a word list for
/// error-spot rows (["j'avais"] vs ["je", "finissais"]).
fn subj_words(subj: &str, form: &str) -> Vec<String> {
    if subj == "je" && starts_vowel(form) {
        vec![format!("j'{}", form)]
    } else {
        vec![subj.to_string(), form.to_string()]
    }
}

fn si_present_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in SI_VERBS {
        for &(subj, _c, fi) in SUBJ_CAP {
            let pres = if subj == "je" && starts_vowel(v.pres[fi]) {
                format!("j'{}", v.pres[fi])
            } else {
                v.pres[fi].to_string()
            };
            let head = if pres.starts_with("j'") { "Si ".into() } else { format!("{} ", si_subj(subj)) };
            // Cloze: si-clause présent, generic futur result.
            out.push(ExerciseItem::Cloze(ClozeItem {
                id: format!("siP:{}:{}:cl", v.inf, subj),
                level: LEVEL,
                topic: "Si + présent",
                topic_slug: Some("si-present".into()),
                task: vec!["Complete: si + présent → futur.".into()],
                head,
                tail: format!(" {}, nous serons contents.", v.compl),
                infinitive: v.inf,
                answer: pres.clone(),
                rule: vec![
                    format!("{} {}, nous serons contents.", si_subj(subj), pres),
                    "Si + présent, futur — never si + futur.".into(),
                ],
            }));
            // Cloze from the other side: si-clause fixed (tu), result futur.
            let fut = if subj == "je" && starts_vowel(v.fut[fi]) {
                format!("j'{}", v.fut[fi])
            } else {
                v.fut[fi].to_string()
            };
            out.push(ExerciseItem::Cloze(ClozeItem {
                id: format!("siP:{}:{}:cl2", v.inf, subj),
                level: LEVEL,
                topic: "Si + présent",
                topic_slug: Some("si-present".into()),
                task: vec!["Complete the result clause.".into()],
                head: format!("Si tu {} {}, ", v.pres[1], v.compl),
                tail: " aussi.".into(),
                infinitive: v.inf,
                answer: fut.clone(),
                rule: vec![
                    format!("Si tu {} {}, {} {} aussi.", v.pres[1], v.compl, subj, fut),
                    "The result clause takes the futur; si keeps the présent.".into(),
                ],
            }));
            // Choice: si + présent vs the si + futur trap. When je elides
            // (j'ai / j'aurai) the subject folds into the options.
            let pres_plain = v.pres[fi].to_string();
            let fut_trap = v.fut[fi].to_string();
            let sentence = if subj == "je" && starts_vowel(v.pres[fi]) {
                format!("Si ____ {}, nous serons contents.", v.compl)
            } else {
                format!("{} ____ {}, nous serons contents.", si_subj(subj), v.compl)
            };
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("siP:{}:{}:ch", v.inf, subj),
                level: LEVEL,
                topic: "Si + présent",
                topic_slug: Some("si-present".into()),
                task: vec!["Choose the right form.".into()],
                sentence,
                options: vec![pres_plain, fut_trap],
                correct: 0, // shuffled in `generate`
                rule: vec![
                    format!("{} {}, nous serons contents.", si_subj(subj), v.pres[fi]),
                    "After si, the futur never appears — si + présent only.".into(),
                ],
            }));
        }
        // Error-spot: si + futur.
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("siP:{}:es", v.inf),
            level: LEVEL,
            topic: "Si + présent",
            topic_slug: Some("si-present".into()),
            task: vec!["One word is wrong. Tap it.".into()],
            words: punctuate(
                [
                    vec!["Si".to_string()],
                    subj_words("tu", v.fut[1]),
                    vec![format!("{},", v.compl), "nous".into(), "serons".into(), "contents".into()],
                ]
                .concat(),
            ),
            wrong: 2,
            fix: v.pres[1].to_string(),
            fix_task: "Now write the correct form.",
            rule: vec![
                format!("Si tu {} {}, nous serons contents.", v.pres[1], v.compl),
                "si + présent → futur; « si tu viendras » is the classic trap.".into(),
            ],
        }));
    }
    out
}

fn si_imparfait_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in SI_VERBS {
        for &(subj, _c, fi) in SUBJ_CAP {
            let imp = if subj == "je" && starts_vowel(v.imp[fi]) {
                format!("j'{}", v.imp[fi])
            } else {
                v.imp[fi].to_string()
            };
            let head = if imp.starts_with("j'") { "Si ".into() } else { format!("{} ", si_subj(subj)) };
            // Cloze: the si-clause imparfait, conditionnel result.
            out.push(ExerciseItem::Cloze(ClozeItem {
                id: format!("siI:{}:{}:cl", v.inf, subj),
                level: LEVEL,
                topic: "Si + imparfait",
                topic_slug: Some("si-imparfait".into()),
                task: vec!["Complete: si + imparfait → conditionnel.".into()],
                head,
                tail: format!(" {}, nous serions contents.", v.compl),
                infinitive: v.inf,
                answer: imp.clone(),
                rule: vec![
                    format!("{} {}, nous serions contents.", si_subj(subj), imp),
                    "Si + imparfait, conditionnel — an unreal present.".into(),
                ],
            }));
            // Choice: conditionnel vs futur in the result clause.
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("siI:{}:{}:ch", v.inf, subj),
                level: LEVEL,
                topic: "Si + imparfait",
                topic_slug: Some("si-imparfait".into()),
                task: vec!["Choose the right form.".into()],
                sentence: {
                    let si_imp = if subj == "je" && starts_vowel(v.imp[fi]) {
                        format!("j'{}", v.imp[fi])
                    } else {
                        format!("{} {}", si_subj(subj), v.imp[fi])
                    };
                    format!("{} {}, nous ____ contents.", si_imp, v.compl)
                },
                options: vec![v.cond[3].to_string(), v.fut[3].to_string()],
                correct: 0, // shuffled in `generate`
                rule: vec![
                    format!("Si {} {} {}, nous {} contents.", subj, v.imp[fi], v.compl, v.cond[3]),
                    "si + imparfait → conditionnel, never the futur.".into(),
                ],
            }));
            // Choice: imparfait vs présent in the si-clause — the subject
            // folds into the options when je elides.
            let pres = if subj == "je" && starts_vowel(v.pres[fi]) {
                format!("j'{}", v.pres[fi])
            } else {
                v.pres[fi].to_string()
            };
            let (sentence, rule_imp) = if subj == "je" && starts_vowel(v.imp[fi]) {
                (
                    format!("Si ____ {}, nous serions contents.", v.compl),
                    format!("Si {}, nous serions contents.", imp),
                )
            } else {
                (
                    format!("{} ____ {}, nous serions contents.", si_subj(subj), v.compl),
                    format!("Si {} {}, nous serions contents.", subj, imp),
                )
            };
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("siI:{}:{}:ch2", v.inf, subj),
                level: LEVEL,
                topic: "Si + imparfait",
                topic_slug: Some("si-imparfait".into()),
                task: vec!["Unreal — choose the tense.".into()],
                sentence,
                options: vec![imp.clone(), pres],
                correct: 0, // shuffled in `generate`
                rule: vec![
                    rule_imp,
                    "The imparfait marks the hypothesis as unreal.".into(),
                ],
            }));
        }
        // Error-spot: futur in the result clause.
        let si_words = subj_words("je", v.imp[0]);
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("siI:{}:es", v.inf),
            level: LEVEL,
            topic: "Si + imparfait",
            topic_slug: Some("si-imparfait".into()),
            task: vec!["One word is wrong. Tap it.".into()],
            words: punctuate(
                [
                    vec!["Si".to_string()],
                    si_words.clone(),
                    vec![format!("{},", v.compl), "nous".into()],
                    vec![v.fut[3].to_string()],
                    vec![v.compl.to_string()],
                ]
                .concat(),
            ),
            wrong: 1 + si_words.len() + 1,
            fix: v.cond[3].to_string(),
            fix_task: "Now write the correct form.",
            rule: vec![
                format!("Si {} {}, nous {} {}.", imp0_je(v), v.compl, v.cond[3], v.compl),
                "si + imparfait → conditionnel.".into(),
            ],
        }));
    }
    // Transform: the classic « si j'étais riche » pattern.
    out.push(ExerciseItem::Transform(TransformItem {
        id: "siI:riche:tf".into(),
        level: LEVEL,
        topic: "Si + imparfait",
        topic_slug: Some("si-imparfait".into()),
        task: vec!["Say the wish with si + imparfait.".into()],
        source: "Je ne suis pas riche. Je ne voyage pas.".into(),
        answer: "Si j'étais riche, je voyagerais.".into(),
        rule: vec![
            "Si j'étais riche, je voyagerais.".into(),
            "Unreal present: si + imparfait, conditionnel in the result.".into(),
        ],
    }));
    out
}

/// « j'avais » / « je finissais » — je + imparfait with elision folded.
fn imp0_je(v: &SiVerb) -> String {
    if starts_vowel(v.imp[0]) {
        format!("j'{}", v.imp[0])
    } else {
        format!("je {}", v.imp[0])
    }
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "si-present" => si_present_items(),
        "si-imparfait" => si_imparfait_items(),
        _ => Vec::new(),
    }
}
