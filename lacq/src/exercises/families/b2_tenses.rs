//! Family: B2 tenses — sequence of tenses (past main clause pulls
//! the subordinate into the past) and exceptional participle
//! agreement (the preceding direct object: les fleurs que j'ai
//! cueillies).

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem};

use super::shared::punctuate;

const LEVEL: &str = "B2";

/* ── sequence of tenses ──────────────────────────────────────────── */

/// (main-clause frame with ____, correct, wrong, cue, rule)
const SEQ_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Il a dit qu'il ____ demain.", "viendrait", "viendra", "main clause in the past", "A past main clause pulls the subordinate into the conditionnel."),
    ("Il dit qu'il ____ demain.", "viendra", "viendrait", "main clause in the present", "A present main clause keeps the futur in the subordinate."),
    ("Elle pensait que nous ____ tôt.", "partirions", "partirons", "main clause in the past", "pensait (past) → partirions (conditionnel)."),
    ("Je ne savais pas que tu ____ ici.", "travaillais", "travaille", "both actions in the past", "A past main + a simultaneous past event → imparfait."),
    ("Nous espérons que vous ____ bien le travail.", "aimerez", "aimiez", "main clause in the present", "espérer que (present) → futur."),
    ("Elle espérait que vous ____ bien le travail.", "aimeriez", "aimerez", "main clause in the past", "espérait (past) → conditionnel."),
    ("Je pense qu'il ____ raison.", "a", "ait", "main clause in the present, belief", "penser que (present, affirmative) → indicatif."),
    ("Je pensais qu'il ____ raison.", "avait", "aura", "main clause in the past, belief", "pensais (past) → avait (imparfait)."),
    ("Nous saurons quand il ____.", "arrivera", "arriverait", "main clause in the future", "A future main clause keeps the futur after quand."),
    ("Ils ont cru que le magasin ____ ouvert.", "était", "sera", "main clause in the past", "croire que (past) → était (imparfait)."),
];

fn seq_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in SEQ_ROWS {
        let key = super::shared::slugify(&fr.chars().take(24).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("seq2:{}:ch", key),
            level: LEVEL,
            topic: "Sequence of tenses",
            topic_slug: Some("sequence-of-tenses".into()),
            task: vec![format!("{} — choose the tense.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("seq2:{}:cl", key),
            level: LEVEL,
            topic: "Sequence of tenses",
            topic_slug: Some("sequence-of-tenses".into()),
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

/* ── exceptional participle agreement ────────────────────────────── */

/// (antecedent with que, past participle agreeing with it, plain
/// masculine singular form for the wrong option)
const PART_ROWS: &[(&str, &str, &str)] = &[
    ("la pomme qu'il a", "mangée", "mangé"),
    ("les fleurs que j'ai", "cueillies", "cueilli"),
    ("la lettre que j'ai", "écrite", "écrit"),
    ("les films que nous avons", "vus", "vu"),
    ("la chanson qu'elle a", "chantée", "chanté"),
    ("les clés que tu as", "perdues", "perdu"),
    ("le gâteau qu'ils ont", "mangé", "mangés"),
    ("les livres qu'on a", "lus", "lu"),
    ("l'idée que j'ai", "eue", "eu"),
    ("les photos que j'ai", "prises", "pris"),
    ("les vélos que vous avez", "vendus", "vendu"),
    ("la porte qu'il a", "ouverte", "ouvert"),
];

fn part_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(np, good, wrong) in PART_ROWS {
        let key = super::shared::slugify(np);
        // Cloze: agree the participle with the preceding object.
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("part:{}:cl", key),
            level: LEVEL,
            topic: "Exceptional participle agreement",
            topic_slug: Some("exceptional-participle".into()),
            task: vec!["Agree the participle with the preceding object.".into()],
            head: format!("Voici {} ", np),
            tail: " ce matin.".into(),
            infinitive: "participle",
            answer: good.to_string(),
            rule: vec![
                format!("Voici {} {} ce matin.", np, good),
                "With avoir, the participle agrees with a direct object placed BEFORE the verb.".into(),
            ],
        }));
        // Choice: agreed vs unagreed.
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("part:{}:ch", key),
            level: LEVEL,
            topic: "Exceptional participle agreement",
            topic_slug: Some("exceptional-participle".into()),
            task: vec!["Agree the participle.".into()],
            sentence: format!("{} ____ ce matin.", np),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("{} {} ce matin.", np, good),
                "The object comes before « avoir » → agreement applies.".into(),
            ],
        }));
    }
    // Error-spot: the unagreed form slips in.
    out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
        id: "part:fleurs:es".into(),
        level: LEVEL,
        topic: "Exceptional participle agreement",
        topic_slug: Some("exceptional-participle".into()),
        task: vec!["One word is wrong. Tap it.".into()],
        words: punctuate(vec![
            "Les".into(),
            "fleurs".into(),
            "que".into(),
            "j'ai".into(),
            "cueilli".into(),
        ]),
        wrong: 4,
        fix: "cueillies".into(),
        fix_task: "Now write the correct form.",
        rule: vec![
            "Les fleurs que j'ai cueillies.".into(),
            "« les fleurs » is feminine plural and precedes avoir → -es.".into(),
        ],
    }));
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "sequence-of-tenses" => seq_items(),
        "exceptional-participle" => part_items(),
        _ => Vec::new(),
    }
}
