//! Family: B2 subjonctif — deeper triggers after verbs & impersonals
//! (craindre que, il se peut que), the conjunction series (jusqu'à ce
//! que, avant que, sans que…) and the subjonctif passé (que j'aie
//! fini, qu'elle soit partie).

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem};

use super::shared::{punctuate, starts_vowel};

const LEVEL: &str = "B2";

/* ── subjonctif after verbs & impersonals ────────────────────────── */

const SB2_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Je crains qu'il ne ____ trop tard.", "vienne", "vient", "fear", "craindre que → subjonctif (the ne is explétive)."),
    ("Elle est sûre qu'il ____ raison.", "a", "ait", "certainty", "être sûr que → indicatif."),
    ("Il se peut que le train ____ retardé.", "soit", "est", "possibility", "il se peut que → subjonctif."),
    ("Je suis certain qu'elle ____ demain.", "viendra", "vienne", "certainty", "être certain que → indicatif."),
    ("Nous redoutons qu'il y ____ des conséquences.", "ait", "a", "dread", "redouter que → subjonctif."),
    ("Il est rare qu'il ____ avant midi.", "se lève", "se lèvera", "rarity", "il est rare que → subjonctif."),
    ("Tu admets que c'____ possible.", "est", "soit", "admission = belief", "admettre que → indicatif."),
    ("J'attends qu'il ____ prêt.", "soit", "est", "waiting", "attendre que → subjonctif."),
    ("Il vaut mieux que tu ____ tout.", "saches", "sais", "advice", "il vaut mieux que → subjonctif."),
    ("Elle est ravie que vous ____ venus.", "soyez", "êtes", "emotion", "être ravi que → subjonctif."),
];

fn sb2_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in SB2_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("sb2:{}:ch", key),
            level: LEVEL,
            topic: "Subjonctif after verbs & impersonals",
            topic_slug: Some("subjonctif-verbs-impersonals".into()),
            task: vec![format!("{} — choose the mood.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("sb2:{}:cl", key),
            level: LEVEL,
            topic: "Subjonctif after verbs & impersonals",
            topic_slug: Some("subjonctif-verbs-impersonals".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "verb",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    out
}

/* ── subjonctif after conjunctions ───────────────────────────────── */

/// (sentence with ____, correct conjunction, wrong, cue, rule)
const SCONJ_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Attends ____ nous ayons fini.", "jusqu'à ce que", "jusqu'à", "wait until we finish", "jusqu'à ce que + subjonctif; jusqu'à + noun only."),
    ("Il est sorti ____ nous puissions parler.", "avant que", "après que", "before we could talk", "avant que + subjonctif."),
    ("Je viendrai ____ tu sois prêt.", "à condition que", "à condition de", "on the condition that you're ready", "à condition que + subjonctif (de + infinitive only when there is no new subject)."),
    ("Elle écoute ____ personne ne comprenne.", "sans que", "sans", "without anyone understanding", "sans que + subjonctif before a clause; sans + noun."),
    ("____ il fasse noir, je lis encore.", "Bien que", "Bien", "even though it's dark", "bien que + subjonctif — elided to bien qu'il."),
];

const SCONJ_TABLE: &[(&str, &str, &str, &str, &str)] = &[
    ("Le maire a parlé ____ le concert commence.", "avant que", "après que", "before the concert starts", "avant que + subjonctif; « après que » is followed by the indicatif."),
    ("Il t'aidera ____ tu aies fini tes devoirs.", "pourvu que", "pourvu de", "provided you finish", "pourvu que + subjonctif."),
    ("Je note l'adresse ____ je l'oublie.", "avant que", "après que", "before I forget it", "avant que + subjonctif (j'oublie is the subjunctive of oublier)."),
];

fn sconj_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in SCONJ_ROWS.iter().chain(SCONJ_TABLE.iter()) {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("sconj:{}:ch", key),
            level: LEVEL,
            topic: "Subjonctif after conjunctions",
            topic_slug: Some("subjonctif-conjunctions".into()),
            task: vec![format!("{} — choose the conjunction.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("sconj:{}:cl", key),
            level: LEVEL,
            topic: "Subjonctif after conjunctions",
            topic_slug: Some("subjonctif-conjunctions".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "conjunction",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    out
}

/* ── subjonctif passé ────────────────────────────────────────────── */

/// (inf, être auxiliary?, participle, complement) — no pronominal
/// verbs here so the chunk needs no reflexive pronoun.
const SPASSE_VERBS: &[(&str, bool, &str, &str)] = &[
    ("finir", false, "fini", "le dossier"),
    ("lire", false, "lu", "le rapport"),
    ("comprendre", false, "compris", "la blague"),
    ("recevoir", false, "reçu", "la réponse"),
    ("partir", true, "parti", ""),
    ("arriver", true, "arrivé", ""),
    ("sortir", true, "sorti", ""),
    ("aller", true, "allé", "au cinéma"),
];

/// (subject after que, être/avoir subjonctif form pairs, feminine?, plural?)
const SP_SUBJ: &[(&str, (&str, &str), bool, bool)] = &[
    ("je", ("aie", "sois"), false, false),
    ("tu", ("aies", "sois"), false, false),
    ("il", ("ait", "soit"), false, false),
    ("elle", ("ait", "soit"), true, false),
    ("nous", ("ayons", "soyons"), false, true),
    ("vous", ("ayez", "soyez"), false, true),
    ("ils", ("aient", "soient"), false, true),
    ("elles", ("aient", "soient"), true, true),
];

fn spasse_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(inf, etre, pp, compl) in SPASSE_VERBS {
        for &(subj, (aux_av, aux_etre), fem, plur) in SP_SUBJ {
            let aux = if etre { aux_etre } else { aux_av };
            let mut participle = pp.to_string();
            if etre {
                if fem {
                    participle.push('e');
                }
                if plur {
                    participle.push('s');
                }
            }
            // que + subject with je-elision folded into the chunk
            let (head, chunk) = if subj == "je" && starts_vowel(aux) {
                (
                    "Je suis content ".to_string(),
                    format!("j'{} {}", aux, participle),
                )
            } else {
                let que_subj = if matches!(subj, "il" | "elle" | "ils" | "elles") {
                    format!("qu'{}", subj)
                } else {
                    format!("que {}", subj)
                };
                (format!("Je suis content {} ", que_subj), format!("{} {}", aux, participle))
            };
            let tail = if compl.is_empty() { ".".to_string() } else { format!(" {}.", compl) };
            out.push(ExerciseItem::Cloze(ClozeItem {
                id: format!("spasse:{}:{}:cl", inf, subj),
                level: LEVEL,
                topic: "Subjonctif passé",
                topic_slug: Some("subjonctif-passe".into()),
                task: vec!["Put the verb in the subjonctif passé.".into()],
                head: head.clone(),
                tail: tail.clone(),
                infinitive: inf,
                answer: chunk.clone(),
                rule: vec![
                    format!("{}{}{}", head, chunk, tail),
                    "subjonctif passé = avoir/être in the subjonctif + past participle.".into(),
                ],
            }));
        }
        // Choice: subjonctif passé vs indicatif présent (qu'il).
        let (good, wrong) = if etre {
            ("soit arrivé", "est arrivé")
        } else {
            ("ait fini", "a fini")
        };
        let compl_s = if compl.is_empty() { String::new() } else { format!(" {}", compl) };
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("spasse:{}:ch", inf),
            level: LEVEL,
            topic: "Subjonctif passé",
            topic_slug: Some("subjonctif-passe".into()),
            task: vec!["Choose the subjonctif passé.".into()],
            sentence: format!("Je suis content qu'il ____{}.", compl_s),
            options: vec![good.into(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("Je suis content qu'il {}{}.", good, compl_s),
                "Emotion about a completed action → subjonctif passé.".into(),
            ],
        }));
    }
    // Error-spot: indicative slips in after the emotion trigger.
    out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
        id: "spasse:fini:es".into(),
        level: LEVEL,
        topic: "Subjonctif passé",
        topic_slug: Some("subjonctif-passe".into()),
        task: vec!["One word is wrong. Tap it.".into()],
        words: punctuate(vec![
            "Je".into(),
            "suis".into(),
            "content".into(),
            "qu'il".into(),
            "a".into(),
            "fini".into(),
        ]),
        wrong: 4,
        fix: "ait".into(),
        fix_task: "Now write the correct form.",
        rule: vec![
            "Je suis content qu'il ait fini.".into(),
            "être content que → subjonctif passé: qu'il ait fini.".into(),
        ],
    }));
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "subjonctif-verbs-impersonals" => sb2_items(),
        "subjonctif-conjunctions" => sconj_items(),
        "subjonctif-passe" => spasse_items(),
        _ => Vec::new(),
    }
}
