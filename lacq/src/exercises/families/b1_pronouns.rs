//! Family: B1 pronouns — double object pronouns (the fixed order
//! me/te/nous/vous → le/la/les → lui/leur before the verb) and
//! indefinite pronouns (quelqu'un, quelque chose, chacun… including
//! the « de + adjective » agreement trap).

use crate::exercises::{ChoiceItem, ClozeItem, ExerciseItem, TransformItem};

const LEVEL: &str = "B1";

/* ── double object pronouns ──────────────────────────────────────── */

/// (subject, verb, COD noun, COD pron, COI noun (with its preposition),
/// COI pron, trailing complement)
const DP_TABLE: &[(&str, &str, &str, &str, &str, &str, &str)] = &[
    ("Marie", "montre", "la photo", "le", "à ses amis", "leur", ""),
    ("Le prof", "rend", "les copies", "les", "aux étudiants", "leur", "aujourd'hui"),
    ("Il", "prête", "son vélo", "le", "à Léa", "lui", ""),
    ("Elle", "envoie", "des cartes", "les", "à ses clients", "leur", ""),
    ("Nous", "offrons", "le livre", "le", "à notre mère", "lui", "pour Noël"),
    ("Tu", "conseilles", "ce film", "le", "à ta sœur", "lui", ""),
    ("Je", "donne", "la clé", "la", "au voisin", "lui", ""),
    ("Vous", "lisez", "le poème", "le", "aux enfants", "leur", ""),
    ("Le président", "présente", "le projet", "le", "au conseil", "lui", ""),
];

fn dp_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(subj, verb, cod_np, cod, coi_np, coi, extra) in DP_TABLE {
        let pron_pair = format!("{} {}", cod, coi);
        // Transform: full sentence → pronoun version.
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("dp:{}:tf", super::shared::slugify(cod_np)),
            level: LEVEL,
            topic: "Double object pronouns",
            topic_slug: Some("double-pronouns".into()),
            task: vec![
                "Replace both complements with pronouns".into(),
                "(le/la/les + lui/leur).".into(),
            ],
            source: format!("{} {} {} {}{}.", subj, verb, cod_np, coi_np, extra_note(extra)),
            answer: format!("{} {} {}{}.", subj, pron_pair.clone(), verb, extra_note(extra)),
            rule: vec![
                format!("{} {} {}{}.", subj, pron_pair.clone(), verb, extra_note(extra)),
                "Order: le/la/les first, lui/leur second — both before the verb.".into(),
            ],
        }));
        // Choice: order trap.
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("dp:{}:ch", super::shared::slugify(cod_np)),
            level: LEVEL,
            topic: "Double object pronouns",
            topic_slug: Some("double-pronouns".into()),
            task: vec!["Choose the right order.".into()],
            sentence: format!("{} ____ {}.", subj, verb),
            options: vec![pron_pair.clone(), format!("{} {}", coi, cod)],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("{} {} {}.", subj, pron_pair, verb),
                "le/la/les always come before lui/leur.".into(),
            ],
        }));
        // Cloze: the pronoun pair as the chunk.
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("dp:{}:cl", super::shared::slugify(cod_np)),
            level: LEVEL,
            topic: "Double object pronouns",
            topic_slug: Some("double-pronouns".into()),
            task: vec!["Complete with the two pronouns.".into()],
            head: format!("{} ", subj),
            tail: if extra.is_empty() { format!(" {}.", verb) } else { format!(" {} {}.", verb, extra) },
            infinitive: "pronouns",
            answer: pron_pair.clone(),
            rule: vec![
                format!("{} {} {}{}.", subj, pron_pair, verb, extra),
                "COD pronoun first (le/la/les), COI second (lui/leur).".into(),
            ],
        }));
    }
    out
}

fn extra_note(extra: &str) -> String {
    if extra.is_empty() {
        String::new()
    } else {
        format!(" {}", extra)
    }
}

/* ── indefinite pronouns ─────────────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const IND_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("____ a sonné, mais je ne vois personne.", "Quelqu'un", "Quelqu'une", "someone", "quelqu'un — never feminine."),
    ("____ l'a répondu tout de suite.", "Quelqu'un", "Personne", "someone answered", "quelqu'un = someone; personne = nobody."),
    ("Il y a ____ à manger dans le frigo.", "quelque chose", "quelqu'un", "something", "quelque chose = something (things), quelqu'un = someone."),
    ("Est-ce que ____ veut du thé ?", "quelqu'un", "quelque chose", "anyone", "People → quelqu'un."),
    ("Chacun ____ son avis.", "donne", "donnent", "each one", "chacun is singular → donne."),
    ("____ les invités sont arrivés.", "Tous", "Tout", "all of them (masculine plural)", "tous + les = all of them."),
    ("J'ai essayé ____ les exercices.", "tous", "tout", "all (plural)", "tous before a plural noun."),
    ("____ a bien marché, merci !", "Tout", "Tous", "everything", "tout = everything (no noun), tous = everyone/all + noun."),
    ("____ des étudiants ont raté l'examen.", "Quelques-uns", "Quelqu'un", "a few of them", "quelques-uns = a few of them."),
    ("Aucun ____ répondu.", "n'a", "a", "none answered", "aucun carries its own ne: aucun n'a répondu."),
    ("Elle connaît ____ de gentil ici.", "quelqu'un", "quelqu'une", "someone kind", "quelqu'un de + adjective — the adjective stays masculine."),
    ("On m'a dit ____ de nouveau.", "quelque chose", "quelque chose de neuve", "something new", "quelque chose de + adjective stays masculine: de nouveau."),
];

fn ind_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in IND_ROWS {
        let key = super::shared::slugify(&fr.chars().take(24).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("ip:{}:ch", key),
            level: LEVEL,
            topic: "Indefinite pronouns",
            topic_slug: Some("indefinite-pronouns".into()),
            task: vec![format!("{} — choose.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("ip:{}:cl", key),
            level: LEVEL,
            topic: "Indefinite pronouns",
            topic_slug: Some("indefinite-pronouns".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "indefinite",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "double-pronouns" => dp_items(),
        "indefinite-pronouns" => ind_items(),
        _ => Vec::new(),
    }
}
