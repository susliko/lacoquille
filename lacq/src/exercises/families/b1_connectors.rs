//! Family: B1 connectors — cause & consequence extended (grâce à,
//! c'est pourquoi, si bien que), purpose (pour + inf vs pour que +
//! subj), concession & opposition (bien que + subj, malgré + noun,
//! même si + ind) and narrative time markers.

use crate::exercises::{ChoiceItem, ClozeItem, ExerciseItem, TransformItem};

const LEVEL: &str = "B1";

/* ── cause & consequence (extended) ──────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const CEX_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("____ ton aide, j'ai fini à temps.", "Grâce à", "À cause de", "a positive cause", "grâce à for good outcomes, à cause de for bad ones."),
    ("____ la pluie, le vol a été annulé.", "À cause de", "Grâce à", "a bad cause", "à cause de introduces a harmful cause."),
    ("Elle s'entraîne tous les jours, ____ elle a gagné.", "si bien que", "parce que", "so much so that", "si bien que links a cause to its full result."),
    ("Il ne répond plus, ____ je m'inquiète.", "c'est pourquoi", "mais", "that's why", "c'est pourquoi = that's why — consequence connector."),
    ("____ réviser chaque soir, il connaît tout par cœur.", "À force de", "Grâce à", "by dint of, again and again", "à force de + infinitive = by doing it again and again."),
    ("Le magasin est fermé, ____ on rentre chez nous.", "donc", "pourquoi", "so", "donc draws the consequence."),
    ("____ ses efforts, il a réussi.", "Grâce à", "À cause de", "a positive cause", "ses efforts are good → grâce à."),
    ("Elle a dit oui, ____ tout le monde a applaudi.", "si bien que", "quand", "so much so that", "si bien que closes the story with the result."),
];

fn cex_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in CEX_ROWS {
        let key = super::shared::slugify(&fr.chars().take(24).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("cex:{}:ch", key),
            level: LEVEL,
            topic: "Cause & consequence (extended)",
            topic_slug: Some("cause-consequence-extended".into()),
            task: vec![format!("{} — choose the connector.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("cex:{}:cl", key),
            level: LEVEL,
            topic: "Cause & consequence (extended)",
            topic_slug: Some("cause-consequence-extended".into()),
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

/* ── pour que, afin de ───────────────────────────────────────────── */

/// (same-subject sentence, other-subject sentence, the conjugated
/// chunk that follows « pour que ») — same subject → pour + inf,
/// different subject → pour que + subjonctif.
const PQ_ROWS: &[(&str, &str, &str, &str)] = &[
    ("Je répète pour être clair.", "Je répète pour que tu comprennes.", "comprendre", "comprennes"),
    ("Elle parle lentement pour se faire entendre.", "Elle parle lentement pour que nous l'entendions.", "entendre", "l'entendions"),
    ("Il note l'adresse pour ne pas l'oublier.", "Il note l'adresse pour que je ne l'oublie pas.", "oublier", "l'oublie"),
    ("Nous partons tôt pour éviter le trafic.", "Nous partons tôt pour que vous évitiez le trafic.", "éviter", "évitiez"),
    ("Tu lis le mode d'emploi pour bien l'installer.", "Tu lis le mode d'emploi pour qu'il s'installe bien.", "s'installer", "s'installe"),
    ("Je garde la place pour revenir vite.", "Je garde la place pour qu'on revienne vite.", "revenir", "revienne"),
    ("Elle chuchote pour ne pas réveiller l'enfant.", "Elle chuchote pour que l'enfant ne se réveille pas.", "se réveiller", "se réveille"),
    ("On ouvre les fenêtres pour aérer.", "On ouvre les fenêtres pour que l'air circule.", "circuler", "circule"),
];

fn pq_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(same, other, inf2, chunk) in PQ_ROWS {
        let key = super::shared::slugify(&same.chars().take(22).collect::<String>());
        // Choice: pour que vs pour — different subject decides.
        let blanked = if other.contains("pour que") {
            other.replacen("pour que", "____", 1)
        } else {
            other.replacen("pour qu'", "____ ", 1)
        };
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("pq:{}:ch", key),
            level: LEVEL,
            topic: "Pour que, afin de",
            topic_slug: Some("pour-que-afin-de".into()),
            task: vec!["Different subject — choose the right frame.".into()],
            sentence: blanked,
            options: vec!["pour que".into(), "pour".into()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                other.to_string(),
                "Two different subjects → pour que + subjonctif.".into(),
            ],
        }));
        // Cloze: the subjunctive chunk after pour que.
        let (head, tail) = other.split_once(chunk).unwrap_or((other, ""));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("pq:{}:cl", key),
            level: LEVEL,
            topic: "Pour que, afin de",
            topic_slug: Some("pour-que-afin-de".into()),
            task: vec!["Complete: pour que + subjonctif.".into()],
            head: head.to_string(),
            tail: format!("{}.", tail.trim_end_matches('.')),
            infinitive: inf2,
            answer: chunk.to_string(),
            rule: vec![
                other.to_string(),
                "pour que is always followed by the subjonctif.".into(),
            ],
        }));
        // Transform: same subject → pour + infinitive.
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("pq:{}:tf", key),
            level: LEVEL,
            topic: "Pour que, afin de",
            topic_slug: Some("pour-que-afin-de".into()),
            task: vec![
                "Same subject — say it with".into(),
                "« pour » + infinitive.".into(),
            ],
            source: other.to_string(),
            answer: same.to_string(),
            rule: vec![
                same.to_string(),
                "One subject only → pour + infinitive, no subjonctif.".into(),
            ],
        }));
    }
    out
}

/* ── concession & opposition ─────────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const CPO_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("____ il soit malade, il vient travailler.", "Bien que", "Malgré", "ill (clause)", "bien que + subjonctif (soit); malgré + noun only."),
    ("____ sa fatigue, elle continue.", "Malgré", "Bien qu'elle", "her tiredness (noun)", "malgré + noun; bien que + clause with subjonctif."),
    ("Il sort, ____ il pleut des cordes.", "même si", "malgré", "it's raining (clause)", "même si + indicatif — a plain fact."),
    ("____ les embouteillages, nous sommes à l'heure.", "Malgré", "Bien que", "traffic jams (noun)", "malgré + noun."),
    ("Elle a accepté, ____ elle soit très occupée.", "bien qu'elle", "malgré elle", "she's very busy (clause)", "bien que + subjonctif."),
    ("____ le prix, j'ai acheté la robe.", "Malgré", "Bien que", "the price (noun)", "malgré + noun; bien que needs a clause."),
    ("____ soit fatigué, il aide toujours.", "Bien qu'il", "Malgré qu'il", "he is tired (clause)", "bien qu'il + subjonctif; « malgré que » is non-standard."),
    ("On est à l'heure, ____ il y avait du trafic.", "même si", "malgré", "there was traffic (clause)", "même si + indicatif."),
];

fn cpo_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in CPO_ROWS {
        let key = super::shared::slugify(&fr.chars().take(24).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("cpo:{}:ch", key),
            level: LEVEL,
            topic: "Concession & opposition",
            topic_slug: Some("concession-opposition".into()),
            task: vec![format!("{} — choose.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("cpo:{}:cl", key),
            level: LEVEL,
            topic: "Concession & opposition",
            topic_slug: Some("concession-opposition".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "connector",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Transform: malgré + noun → bien que + clause.
    out.push(ExerciseItem::Transform(TransformItem {
        id: "cpo:fatigue:tf".into(),
        level: LEVEL,
        topic: "Concession & opposition",
        topic_slug: Some("concession-opposition".into()),
        task: vec!["Say it with « bien que » + subjonctif.".into()],
        source: "Malgré sa fatigue, elle continue.".into(),
        answer: "Bien qu'elle soit fatiguée, elle continue.".into(),
        rule: vec![
            "Bien qu'elle soit fatiguée, elle continue.".into(),
            "malgré + noun becomes bien que + clause with the subjonctif.".into(),
        ],
    }));
    out
}

/* ── narrative time markers ──────────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const NTM_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Tout était calme. ____, un bruit a retenti.", "Soudain", "Enfin", "suddenly", "soudain / tout à coup = a sudden break in the story."),
    ("Il pleuvait. ____, elle a choisi de rester.", "Alors", "Pendant", "so (drawing the consequence)", "alors links a situation to its consequence."),
    ("____ elle cuisinait, il a mis la table.", "Pendant que", "Depuis que", "while", "pendant que + indicatif = two parallel actions."),
    ("____ j'habite ici, je n'ai jamais vu ça.", "Depuis que", "Pendant que", "since (a starting point)", "depuis que = since the time that."),
    ("____ il est arrivé, la soirée a changé.", "Au moment où", "Pendant", "at the very moment", "au moment où pins the exact instant."),
    ("Elle a hésité. ____, elle a dit non.", "Enfin", "Soudain", "finally (after hesitation)", "enfin closes a hesitation."),
    ("Le téléphone a sonné ____ on dinait.", "pendant que", "au moment où…", "while we were eating", "pendant que + clause = during."),
    ("____ le film a fini, on est allés dîner.", "Quand", "Depuis que", "when (a single event)", "quand for one-off events; depuis que for ongoing ones."),
];

fn ntm_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in NTM_ROWS {
        let key = super::shared::slugify(&fr.chars().take(24).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("ntm:{}:ch", key),
            level: LEVEL,
            topic: "Narrative time markers",
            topic_slug: Some("narrative-time-markers".into()),
            task: vec![format!("{} — choose the marker.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("ntm:{}:cl", key),
            level: LEVEL,
            topic: "Narrative time markers",
            topic_slug: Some("narrative-time-markers".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "marker",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Transform: two plain sentences joined by a marker.
    out.push(ExerciseItem::Transform(TransformItem {
        id: "ntm:soudain:tf".into(),
        level: LEVEL,
        topic: "Narrative time markers",
        topic_slug: Some("narrative-time-markers".into()),
        task: vec!["Join with a time marker".into(), "(the sudden one).".into()],
        source: "Tout était calme. Un bruit a retenti.".into(),
        answer: "Tout était calme. Soudain, un bruit a retenti.".into(),
        rule: vec![
            "Tout était calme. Soudain, un bruit a retenti.".into(),
            "soudain / tout à coup breaks a quiet scene.".into(),
        ],
    }));
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "cause-consequence-extended" => cex_items(),
        "pour-que-afin-de" => pq_items(),
        "concession-opposition" => cpo_items(),
        "narrative-time-markers" => ntm_items(),
        _ => Vec::new(),
    }
}
