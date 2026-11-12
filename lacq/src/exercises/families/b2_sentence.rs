//! Family: B2 sentence patterns — « au cas où » + conditionnel and
//! « à condition que / pourvu que » + subjonctif; advanced opposition
//! (néanmoins, en revanche, quoique, quand bien même) and advanced
//! purpose & cause (de peur que, de sorte que, vu que, étant donné).

use crate::exercises::{ChoiceItem, ClozeItem, ExerciseItem, TransformItem};

const LEVEL: &str = "B2";

/* ── au cas où, à condition que ──────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const ACO_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Prends un parapluie ____ il pleuvrait.", "au cas où", "au cas que", "in case it rains", "au cas où + conditionnel — never « au cas que »."),
    ("Je laisse la porte ouverte ____ tu rentrerais tard.", "au cas où", "pour que", "in case you came home late", "au cas où + conditionnel (a mere contingency)."),
    ("Je viendrai ____ tu m'invites.", "à condition que", "à condition de", "the invitation is the condition (a clause)", "à condition que + subjonctif."),
    ("Il achètera la voiture ____ le prix baisse.", "à condition que", "au cas où", "price drop = his condition", "à condition que + subjonctif (baisse is subjunctive here)."),
    ("Nous gagnerons ____ nous restions concentrés.", "pourvu que", "pourvu de", "provided we stay focused", "pourvu que + subjonctif."),
    ("Note le code ____ tu ne l'oublies.", "de peur que", "au cas où", "for fear you forget it", "de peur que + subjonctif — afraid that it happens."),
    (" ____ la réunion soit reportée, la date reste la même.", "Au cas où", "À condition que", "in case it is postponed", "au cas où = a contingency, no condition attached."),
    ("Tu sortiras ____ tu aies fini tes devoirs.", "à condition que", "au cas où", "homework first — a real condition", "à condition que + subjonctif sets a requirement."),
];

fn aco_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in ACO_ROWS {
        let key = super::shared::slugify(&fr.chars().take(24).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("aco:{}:ch", key),
            level: LEVEL,
            topic: "Au cas où, à condition que",
            topic_slug: Some("au-cas-ou-condition".into()),
            task: vec![format!("{} — choose the connector.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("aco:{}:cl", key),
            level: LEVEL,
            topic: "Au cas où, à condition que",
            topic_slug: Some("au-cas-ou-condition".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "connector",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Transform: pour que → au cas où nuance is not mechanical; instead
    // drill the au cas où + conditionnel formation.
    out.push(ExerciseItem::Transform(TransformItem {
        id: "aco:parapluie:tf".into(),
        level: LEVEL,
        topic: "Au cas où, à condition que",
        topic_slug: Some("au-cas-ou-condition".into()),
        task: vec![
            "Say it with « au cas où »:".into(),
            "mind the conditionnel after it.".into(),
        ],
        source: "Prends un parapluie. Il pleut, c'est possible.".into(),
        answer: "Prends un parapluie au cas où il pleuvrait.".into(),
        rule: vec![
            "Prends un parapluie au cas où il pleuvrait.".into(),
            "au cas où + conditionnel présent (pleuvrait), never au cas que.".into(),
        ],
    }));
    out
}

/* ── advanced opposition & concession ────────────────────────────── */

const OPP_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Il est jeune. ____, il est très compétent.", "Néanmoins", "De plus", "young, yet competent", "néanmoins = nevertheless — concedes and opposes."),
    ("Les prix montent. ____, la demande baisse.", "En revanche", "Donc", "opposite directions", "en revanche opposes two parallel facts."),
    ("____ il soit tard, on continue.", "Quoique", "Quand", "even though it's late", "quoique + subjonctif (soit)."),
    ("____ il pleuvrait, on sortirait.", "Quand bien même", "Bien sûr", "even if it rained (unreal)", "quand bien même + conditionnel: an extreme concession."),
    ("Elle a accepté le poste ; ____ elle le regrette.", "toutefois", "parce que", "acceptance with regret", "toutefois = however, a formal nevertheless."),
    ("Le film est long, ____ jamais ennuyeux.", "mais", "donc", "long but never boring", "mais keeps the opposition simple."),
    ("____ ses douze ans, il parle trois langues.", "À peine", "Malgré", "he is only twelve", "à peine… que: he is only twelve and yet."),
    ("Quand bien même tu ____ raison, il faudrait des preuves.", "aurais", "auront", "even if you were right", "quand bien même + conditionnel (aurais); « il faudrait des preuves »."),
];

fn opp_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in OPP_ROWS {
        let key = super::shared::slugify(&fr.chars().take(24).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("opp2:{}:ch", key),
            level: LEVEL,
            topic: "Advanced opposition & concession",
            topic_slug: Some("opposition-avancee".into()),
            task: vec![format!("{} — choose.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("opp2:{}:cl", key),
            level: LEVEL,
            topic: "Advanced opposition & concession",
            topic_slug: Some("opposition-avancee".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "connector",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Transform: bien que → quoique (same mood, different shape).
    out.push(ExerciseItem::Transform(TransformItem {
        id: "opp2:tard:tf".into(),
        level: LEVEL,
        topic: "Advanced opposition & concession",
        topic_slug: Some("opposition-avancee".into()),
        task: vec!["Say it with « quoique ».".into()],
        source: "Bien qu'il soit tard, on continue.".into(),
        answer: "Quoiqu'il soit tard, on continue.".into(),
        rule: vec![
            "Quoiqu'il soit tard, on continue.".into(),
            "bien que and quoique both take the subjonctif.".into(),
        ],
    }));
    out
}

/* ── advanced purpose & cause ────────────────────────────────────── */

const BCA_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Il parle fort ____ tout le monde entende.", "de sorte que", "parce que", "so that everyone hears", "de sorte que + subjonctif = so that (a result aimed at)."),
    ("Ferme la fenêtre ____ le bruit ne rentre pas.", "de peur que", "parce que", "for fear the noise gets in", "de peur que + subjonctif (often with the explétive ne)."),
    ("____ la crise, les prix ont baissé.", "Vu", "Voyant", "given the crisis", "vu + noun = given… (cause without a verb)."),
    ("____ son âge, il conduit encore.", "Étant donné", "Donné", "given his age", "étant donné + noun = given…."),
    ("Elle a accepté ____ ne pas le contrarier.", "de peur", "pour peur de", "for fear of contradicting him", "de peur de + infinitive."),
];

const BCA_TABLE: &[(&str, &str, &str, &str, &str)] = &[
    ("Nous avons hâté le pas ____ arriver avant la nuit.", "afin de", "afin que", "in order to arrive", "afin de + infinitive (same subject); afin que + subjonctif."),
    ("Il a tout dit ____ on le sache.", "pour que", "pour", "so that people know", "pour que + subjonctif when the subjects differ."),
    ("Étant donné ____ il pleut, on reste.", "que", "qu'", "given that it's raining", "étant donné que + indicatif = since."),
];

fn bca_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in BCA_ROWS.iter().chain(BCA_TABLE.iter()) {
        let key = super::shared::slugify(&fr.chars().take(24).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("bca:{}:ch", key),
            level: LEVEL,
            topic: "Advanced purpose & cause",
            topic_slug: Some("but-cause-avancee".into()),
            task: vec![format!("{} — choose.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("bca:{}:cl", key),
            level: LEVEL,
            topic: "Advanced purpose & cause",
            topic_slug: Some("but-cause-avancee".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "connector",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "au-cas-ou-condition" => aco_items(),
        "opposition-avancee" => opp_items(),
        "but-cause-avancee" => bca_items(),
        _ => Vec::new(),
    }
}
