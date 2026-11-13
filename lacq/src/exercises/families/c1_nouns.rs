//! Family: C1 noun group — the quelque / quel que / quoi que family
//! (spelling decides the meaning), compound relatives with à & de
//! (auquel, duquel, auxquels…) and avoiding repetition (celui/celle +
//! de, le/la/les as pro-forms).

use crate::exercises::{ChoiceItem, ClozeItem, ExerciseItem};

const LEVEL: &str = "C1";

/* ── quelque, quel que ───────────────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const QUQ_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("____ fatigué qu'il soit, il continue.", "Quelque", "Quel", "however tired he may be", "quelque + adjective + que + subjonctif = however…; « quel » ne s'emploie pas ici."),
    ("____ soit le prix, je l'achète.", "Quel que", "Quelque", "whatever the price", "quel que + être + subjonctif: two words, agreeing (quels que soient…)."),
    ("____ soient les risques, on avance.", "Quels que", "Quelques", "whatever the risks", "Plural → quels que soient."),
    ("____ soient ses excuses, je refuse.", "Quelles que", "Quelques", "her excuses (feminine plural)", "Agreement: quelles que soient."),
    ("____ jours plus tard, il avait tout oublié.", "Quelques", "Quel que", "a few days later", "quelques + noun = a few."),
    ("____ qu'il dise, je ne change pas d'avis.", "Quoi", "Quoiqu'", "whatever he says", "quoi que = whatever; quoique = although — the space decides."),
    ("____ soit sa décision, je la respecte.", "Quelle que", "Quel que", "her decision (feminine)", "Agreement: quelle que soit."),
    ("____ soit l'heure, on peut le joindre.", "Quelle que", "Quelque", "whatever the hour", "l'heure is feminine → quelle que."),
    ("____ paresseux qu'il paraisse, il finit par agir.", "Quelque", "Quel", "however lazy he may seem", "quelque + adjective + que + subjonctif."),
];

fn quq_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in QUQ_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("quq:{}:ch", key),
            level: LEVEL,
            topic: "Quelque, quel que",
            topic_slug: Some("quelque-quel-que".into()),
            task: vec![format!("{} — choose the spelling.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("quq:{}:cl", key),
            level: LEVEL,
            topic: "Quelque, quel que",
            topic_slug: Some("quelque-quel-que".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "quelque / quel que / quoi que",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    out
}

/* ── auquel, duquel & co. ────────────────────────────────────────── */

/// (antecedent, preposition, feminine, plural, relative clause) —
/// lequel agrees with the antecedent; à-de compounds are drilled.
const AUQ_ROWS: &[(&str, &str, bool, bool, &str)] = &[
    ("le projet", "à", false, false, "je pense"),
    ("la réponse", "à", true, false, "je m'attendais"),
    ("les problèmes", "à", false, true, "on fait face"),
    ("les candidates", "à", true, true, "j'ai téléphoné"),
    ("le voisin", "de", false, false, "je connais la fille"),
    ("la montagne", "au sommet de", true, false, "il y a un refuge"),
    ("la rue", "en face de", true, false, "il y a une boulangerie"),
    ("le député", "à", false, false, "je m'adresse"),
    ("les réunions", "à", true, true, "j'assiste"),
    ("l'ami", "de", false, false, "je connais le frère"),
];

fn lequel_form(prep: &str, fem: bool, plur: bool) -> String {
    let form = match (fem, plur) {
        (true, false) => "laquelle",
        (false, true) => "lesquels",
        (true, true) => "lesquelles",
        (false, false) => "lequel",
    };
    // à + lequel → auquel; à + lesquels → auxquels; de + lequel → duquel…
    let merged: String = if prep == "à" {
        match form {
            "lequel" => "auquel",
            "laquelle" => "à laquelle",
            "lesquels" => "auxquels",
            _ => "auxquelles",
        }
        .to_string()
    } else if prep == "de" {
        match form {
            "lequel" => "duquel",
            "laquelle" => "de laquelle",
            "lesquels" => "desquels",
            _ => "desquelles",
        }
        .to_string()
    } else {
        format!("{} {}", prep, form)
    };
    merged
}

fn auq_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(np, prep, fem, plur, clause) in AUQ_ROWS {
        let chunk = lequel_form(prep, fem, plur);
        let full = format!("Voici {} {} {}.", np, chunk, clause);
        // Choice: agreement + contraction (wrong = flipped gender).
        let wrong = lequel_form(prep, !fem, plur);
        if wrong != chunk {
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("auq:{}:ch", super::shared::slugify(np)),
                level: LEVEL,
                topic: "Auquel, duquel & co.",
                topic_slug: Some("auquel-duquel".into()),
                task: vec!["Agree the compound relative.".into()],
                sentence: format!("Voici {} ____ {}.", np, clause),
                options: vec![chunk.clone(), wrong],
                correct: 0, // shuffled in `generate`
                rule: vec![
                    full.clone(),
                    "à + lequel → auquel (auxquels); de + lequel → duquel (desquels); else prep + lequel.".into(),
                ],
            }));
        }
        // Cloze: the whole compound as the chunk.
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("auq:{}:cl", super::shared::slugify(np)),
            level: LEVEL,
            topic: "Auquel, duquel & co.",
            topic_slug: Some("auquel-duquel".into()),
            task: vec!["Complete with the preposition + lequel form.".into()],
            head: format!("Voici {} ", np),
            tail: format!(" {}.", clause),
            infinitive: "lequel",
            answer: chunk.clone(),
            rule: vec![
                full,
                "Verb + à (penser à, assister à) or possessive de → the compound relative.".into(),
            ],
        }));
    }
    out
}

/* ── ellipsis & avoiding repetition ──────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const ELL_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Tu ranges tes affaires ? — Je ____ range demain.", "les", "le", "tes affaires (several things)", "A plural direct object → les."),
    ("Sa maison est plus grande que ____ de son frère.", "celle", "celui", "the house of his brother", "celle + de + person replaces a feminine noun."),
    ("Ces chaises coûtent moins cher que ____ de la boutique d'à côté.", "celles", "ceux", "the chairs next door", "Feminine plural → celles."),
    ("Tu prends ton parapluie ? — Je ____ ai déjà pris.", "l'ai", "le", "ton parapluie (singular, masculine)", "Before a vowel the pronoun elides — fold it into the answer chunk: je l'ai."),
    ("Vos idées et ____ de vos collègues se complètent.", "celles", "celui", "the ideas of your colleagues", "celles de = those of."),
    ("Deux chemins s'offrent à nous : ____ de gauche monte au fort.", "celui", "celle", "the left-hand path", "le chemin → celui."),
    ("Cette valise et ____ de ma sœur pèsent le même poids.", "celle", "celui", "my sister's suitcase", "la valise → celle."),
    ("Tu connais leurs enfants ? — Je ____ vois chaque semaine.", "les", "la", "their children (plural)", "Plural → les."),
    ("Son dossier aboutira plus vite que ____ des autres équipes.", "celui", "celle", "the dossier of the other teams", "le dossier → celui."),
    ("Ces clés-ci et ____ de ta mère traînent toujours.", "celles", "celui", "your mother's keys", "les clés → celles."),
];

fn ell_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in ELL_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("ell:{}:ch", key),
            level: LEVEL,
            topic: "Ellipsis & avoiding repetition",
            topic_slug: Some("ellipsis-repetition".into()),
            task: vec![format!("{} — avoid the repetition.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("ell:{}:cl", key),
            level: LEVEL,
            topic: "Ellipsis & avoiding repetition",
            topic_slug: Some("ellipsis-repetition".into()),
            task: vec![format!("{} — replace the noun.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "pronoun",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "quelque-quel-que" => quq_items(),
        "auquel-duquel" => auq_items(),
        "ellipsis-repetition" => ell_items(),
        _ => Vec::new(),
    }
}
