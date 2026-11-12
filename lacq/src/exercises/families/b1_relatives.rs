//! Family: B1 relatives — « dont » (verb + de, and the possessive
//! pattern), compound relatives (preposition + lequel, agreeing like
//! its antecedent) and the « ce qui / ce que / ce dont » series.

use crate::exercises::{ChoiceItem, ClozeItem, ExerciseItem, TransformItem};

const LEVEL: &str = "B1";

/// (antecedent, full answer sentence, the second source clause)
const DONT_TABLE: &[(&str, &str, &str)] = &[
    ("le film", "Voici le film dont je parle.", "Je parle de ce film."),
    ("l'ami", "Voici l'ami dont j'ai besoin.", "J'ai besoin de cet ami."),
    ("la chanteuse", "Voici la chanteuse dont je me souviens.", "Je me souviens de cette chanteuse."),
    ("le roman", "Voici le roman dont l'histoire est touchante.", "Son histoire est touchante."),
    ("la fille", "Voici la fille dont je t'avais parlé.", "Je t'avais parlé de cette fille."),
    ("le projet", "Voici le projet dont on est fier.", "On est fier de ce projet."),
    ("les vacances", "Voici les vacances dont je rêve.", "Je rêve de ces vacances."),
    ("la chanson", "Voici la chanson dont je ne connais pas le titre.", "Je ne connais pas le titre de cette chanson."),
];

fn dont_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(antecedent, full, source_second) in DONT_TABLE {
        let key = super::shared::slugify(antecedent);
        // Transform: two clauses → one with dont.
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("dnt:{}:tf", key),
            level: LEVEL,
            topic: "Dont",
            topic_slug: Some("dont".into()),
            task: vec![
                "Join with a relative pronoun:".into(),
                "the verb of the second clause takes « de ».".into(),
            ],
            source: format!("{}. {}", full.split_once(" dont").map(|(p, _)| p).unwrap_or(full), source_second),
            answer: full.to_string(),
            rule: vec![
                format!("{}.", full),
                "dont replaces de + something after the verb — or links a possessed noun.".into(),
            ],
        }));
        // Choice: dont vs que — parler de forces dont.
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("dnt:{}:ch", key),
            level: LEVEL,
            topic: "Dont",
            topic_slug: Some("dont".into()),
            task: vec!["Choose the relative pronoun.".into()],
            sentence: format!("C'est {} ____ il parle.", antecedent),
            options: vec!["dont".into(), "que".into()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("C'est {} dont il parle.", antecedent),
                "parler de → dont; a plain direct object → que.".into(),
            ],
        }));
        // Cloze: dont is the gap; tail comes from the answer sentence.
        let prefix = format!("Voici {} dont", antecedent);
        let tail = full.strip_prefix(&prefix).unwrap_or(" .").to_string();
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("dnt:{}:cl", key),
            level: LEVEL,
            topic: "Dont",
            topic_slug: Some("dont".into()),
            task: vec!["Complete with dont.".into()],
            head: format!("Voici {} ", antecedent),
            tail,
            infinitive: "dont",
            answer: "dont".into(),
            rule: vec![format!("{}.", full), "de + something after the verb → dont.".into()],
        }));
    }
    out
}

/* ── compound relatives (preposition + lequel) ───────────────────── */

/// (antecedent NP, feminine, plural, preposition, relative clause,
/// deictic phrase for the source sentence)
const CREL_ROWS: &[(&str, bool, bool, &str, &str, &str)] = &[
    ("la ville", true, false, "dans", "j'habite", "dans cette ville"),
    ("le pays", false, false, "vers", "il voyage", "vers ce pays"),
    ("les amis", false, true, "avec", "je sors", "avec ces amis"),
    ("la rue", true, false, "au bout de", "il y a un café", "au bout de cette rue"),
    ("le train", false, false, "dans", "ils sont montés", "dans ce train"),
    ("les chansons", true, true, "parmi", "elle a choisi", "parmi ces chansons"),
    ("la maison", true, false, "devant", "on s'est arrêtés", "devant cette maison"),
    ("le pont", false, false, "sous", "la rivière passe", "sous ce pont"),
];

fn lequel_form(fem: bool, plur: bool) -> &'static str {
    match (fem, plur) {
        (true, false) => "laquelle",
        (false, true) => "lesquels",
        (true, true) => "lesquelles",
        _ => "lequel",
    }
}

fn cap_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

fn crel_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(np, fem, plur, prep, clause, deictic) in CREL_ROWS {
        let rel = lequel_form(fem, plur);
        let chunk = format!("{} {}", prep, rel);
        let full = format!("Voici {} {} {}.", np, chunk, clause);
        // Choice: agreement of lequel (gender flipped as the trap).
        let wrong_rel = lequel_form(!fem, plur);
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("crel:{}:ch", super::shared::slugify(np)),
            level: LEVEL,
            topic: "Compound relatives",
            topic_slug: Some("compound-relatives".into()),
            task: vec!["Agree the relative pronoun.".into()],
            sentence: format!("Voici {} {} ____ {}.", np, prep, clause),
            options: vec![rel.to_string(), wrong_rel.to_string()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                full.clone(),
                "lequel agrees with the antecedent — like its article.".into(),
            ],
        }));
        // Cloze: prep + lequel as one chunk.
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("crel:{}:cl", super::shared::slugify(np)),
            level: LEVEL,
            topic: "Compound relatives",
            topic_slug: Some("compound-relatives".into()),
            task: vec!["Complete with the preposition + lequel form.".into()],
            head: format!("Voici {} ", np),
            tail: format!(" {}.", clause),
            infinitive: "lequel",
            answer: chunk.clone(),
            rule: vec![
                full.clone(),
                "preposition + lequel (laquelle / lesquels / lesquelles).".into(),
            ],
        }));
        // Transform: two sentences → one compound relative.
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("crel:{}:tf", super::shared::slugify(np)),
            level: LEVEL,
            topic: "Compound relatives",
            topic_slug: Some("compound-relatives".into()),
            task: vec![
                "Join with preposition +".into(),
                "lequel (agree it).".into(),
            ],
            source: format!("Voici {}. {} {}.", np, cap_first(clause), deictic),
            answer: full.clone(),
            rule: vec![full],
        }));
    }
    out
}

/* ── ce qui / ce que / ce dont ───────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const CQQ_TABLE: &[(&str, &str, &str, &str, &str)] = &[
    ("____ m'étonne, c'est son courage.", "Ce qui", "Ce que", "the courage is the subject of the inner clause", "ce qui = what (subject)."),
    ("Je ne comprends pas ____ tu dis.", "ce que", "ce qui", "tu dis quelque chose", "ce que = what (direct object)."),
    ("Voilà ____ j'ai besoin.", "ce dont", "ce que", "avoir besoin de", "verb + de → ce dont."),
    ("____ m'intéresse, c'est l'histoire.", "Ce qui", "Ce dont", "l'histoire m'intéresse — no gap", "ce qui — the inner clause is complete."),
    ("Dis-moi ____ tu veux.", "ce que", "ce dont", "tu veux quelque chose", "ce que — direct object."),
    ("Raconte-moi ____ tu rêves.", "ce dont", "ce qui", "rêver de", "rêver de → ce dont."),
    ("C'est exactement ____ je pensais.", "à quoi", "ce dont", "penser à", "penser à → à quoi; ce dont goes with de-verbs."),
    ("____ arrive est inévitable.", "Ce qui", "Ce que", "something happens", "ce qui — « arrive » has no object."),
    ("Écoute ____ le prof dit.", "ce que", "ce dont", "le prof dit quelque chose", "ce que — direct object."),
    ("Je me demande ____ il s'agit.", "de quoi", "ce que", "s'agir de", "il s'agit de quoi → de quoi il s'agit."),
];

fn cqq_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in CQQ_TABLE {
        let key = super::shared::slugify(&fr.chars().take(24).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("cqq:{}:ch", key),
            level: LEVEL,
            topic: "Ce qui, ce que, ce dont",
            topic_slug: Some("ce-qui-ce-que-ce-dont".into()),
            task: vec![format!("{} — choose the pronoun.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("cqq:{}:cl", key),
            level: LEVEL,
            topic: "Ce qui, ce que, ce dont",
            topic_slug: Some("ce-qui-ce-que-ce-dont".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "ce qui / ce que / ce dont",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "dont" => dont_items(),
        "compound-relatives" => crel_items(),
        "ce-qui-ce-que-ce-dont" => cqq_items(),
        _ => Vec::new(),
    }
}
