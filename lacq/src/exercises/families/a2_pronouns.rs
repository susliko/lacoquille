//! Family: A2 pronouns & relatives — four topics. Relative pronouns
//! (qui subject / que object / où place), possessive pronouns (le
//! mien / la mienne — gender with the thing), tout forms (tout /
//! toute / tous / toutes), and the demonstrative trio (ça / cela /
//! ceci).

use crate::exercises::{ChoiceItem, ClozeItem, ExerciseItem, TransformItem};

const LEVEL: &str = "A2";

/* ── qui, que, où (relative pronouns) ───────────────────────────── */

/// (sentence with gap, right word, wrong word, note)
const RELATIVES: &[(&str, &str, &str, &str)] = &[
    ("La femme ____ parle est ma mère.", "qui", "que", "subject of the verb → qui"),
    ("Le livre ____ je lis est vieux.", "que", "qui", "object of the verb → que"),
    ("La ville ____ j'habite est belle.", "où", "que", "place → où"),
    ("Le chien ____ dort est vieux.", "qui", "que", "subject → qui"),
    ("La chanson ____ tu adores passe à la radio.", "que", "où", "object → que"),
    ("Le café ____ on se rencontre est calme.", "où", "qui", "place → où"),
    ("Les clés ____ ouvrent la porte sont ici.", "qui", "que", "subject → qui"),
    ("Le film ____ nous avons vu est long.", "que", "qui", "object → que"),
    ("Le jour ____ il est né était lundi.", "où", "que", "a moment in time → où"),
    ("La fille ____ chante est ma sœur.", "qui", "que", "subject → qui"),
    ("Le repas ____ elle prépare sent bon.", "que", "où", "object → que"),
    ("Le pays ____ je vais est le Japon.", "où", "qui", "place → où"),
    ("L'homme ____ travaille ici est français.", "qui", "que", "subject → qui"),
    ("Les fleurs ____ j'ai achetées sont rouges.", "que", "qui", "object → que"),
    ("Le restaurant ____ nous dînons est cher.", "où", "que", "place → où"),
    ("Le train ____ part à midi est plein.", "qui", "que", "subject → qui"),
];

fn relatives_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(sentence, right, wrong, note) in RELATIVES {
        let (head, tail) = sentence.split_once("____").unwrap();
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("rel:{}:cl", super::shared::slugify(sentence)),
            level: LEVEL,
            topic: "Qui, que, où",
            topic_slug: Some("qui-que-ou".into()),
            task: vec!["Complete with qui, que or où.".into()],
            head: head.to_string(),
            tail: tail.to_string(),
            infinitive: "qui / que / où",
            answer: right.to_string(),
            rule: vec![format!("{} — {}.", right, note)],
        }));
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("rel:{}:ch", super::shared::slugify(sentence)),
            level: LEVEL,
            topic: "Qui, que, où",
            topic_slug: Some("qui-que-ou".into()),
            task: vec!["Choose the right relative pronoun.".into()],
            sentence: sentence.replace("____", "…"),
            options: vec![right.into(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![format!("{} — {}.", right, note)],
        }));
    }
    out
}

/* ── possessive pronouns ────────────────────────────────────────── */

/// (possessive sentence, pronoun, wrong pronoun)
const POSS_PRON: &[(&str, &str, &str)] = &[
    ("C'est mon livre.", "le mien", "la mienne"),
    ("C'est ma voiture.", "la mienne", "le mien"),
    ("Ce sont mes clés.", "les miennes", "les miens"),
    ("Ce sont mes stylos.", "les miens", "les miennes"),
    ("C'est ton idée.", "la tienne", "le tien"),
    ("C'est ton sac.", "le tien", "la tienne"),
    ("C'est son frère.", "le sien", "la sienne"),
    ("C'est sa maison.", "la sienne", "le sien"),
    ("Ce sont ses gants.", "les siens", "les siennes"),
    ("C'est notre jardin.", "le nôtre", "la nôtre"),
    ("C'est notre cuisine.", "la nôtre", "le nôtre"),
    ("C'est votre avis.", "le vôtre", "la vôtre"),
    ("C'est votre porte.", "la vôtre", "le vôtre"),
    ("Ce sont leurs enfants.", "les leurs", "les nôtres"),
    ("C'est leur chien.", "le leur", "la leur"),
    ("C'est leur table.", "la leur", "le leur"),
];

fn poss_pron_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(source, pron, wrong) in POSS_PRON {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("ppr2:{}:tf", super::shared::slugify(source)),
            level: LEVEL,
            topic: "Possessive pronouns",
            topic_slug: Some("possessive-pronouns".into()),
            task: vec!["Replace the possessive + noun".into(), "with a pronoun.".into()],
            source: source.to_string(),
            answer: format!("C'est {}.", pron),
            rule: vec![
                format!("{} → C'est {}.", source, pron),
                "The pronoun keeps the gender and number of the thing.".into(),
            ],
        }));
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("ppr2:{}:ch", super::shared::slugify(source)),
            level: LEVEL,
            topic: "Possessive pronouns",
            topic_slug: Some("possessive-pronouns".into()),
            task: vec!["Choose the right pronoun.".into()],
            sentence: format!("{} C'est ____.", source),
            options: vec![pron.into(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("{} → c'est {}.", source, pron),
                "Gender comes from the thing, not from the owner.".into(),
            ],
        }));
    }
    out
}

/* ── tout, toute, tous, toutes ──────────────────────────────────── */

/// (sentence with gap, right form, wrong form)
const TOUTS: &[(&str, &str, &str)] = &[
    ("____ le monde sait ça.", "Tout", "Toute"),
    ("____ la ville parle de lui.", "Toute", "Tout"),
    ("____ les enfants adorent ça.", "Tous", "Toutes"),
    ("____ les maisons sont neuves.", "Toutes", "Tous"),
    ("____ le train est plein.", "Tout", "Toute"),
    ("____ la journée, il pleut.", "Toute", "Tout"),
    ("____ les étudiants ont réussi.", "Tous", "Toutes"),
    ("____ les fenêtres sont ouvertes.", "Toutes", "Tous"),
    ("Il travaille ____ le temps.", "tout", "toute"),
    ("Elle a passé ____ la nuit dehors.", "toute", "tout"),
    ("Ils sont arrivés ____ les trois.", "tous", "toutes"),
    ("Elles sont contentes ____ les deux.", "toutes", "tous"),
    ("____ le quartier en parle.", "Tout", "Toute"),
    ("____ la famille est réunie.", "Toute", "Tout"),
    ("____ mes amis sont là.", "Tous", "Toutes"),
    ("____ mes copines sont venues.", "Toutes", "Tous"),
];

fn tout_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(sentence, right, wrong) in TOUTS {
        let (head, tail) = sentence.split_once("____").unwrap();
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("tout:{}:cl", super::shared::slugify(sentence)),
            level: LEVEL,
            topic: "Tout, toute, tous, toutes",
            topic_slug: Some("tout-toute-tous".into()),
            task: vec!["Complete with tout, toute, tous or toutes.".into()],
            head: head.to_string(),
            tail: tail.to_string(),
            infinitive: "tout",
            answer: right.to_string(),
            rule: vec![
                format!("{} {}.", right, tail.trim()),
                "tout agrees with what follows: le→tout, la→toute, les m→tous, les f→toutes.".into(),
            ],
        }));
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("tout:{}:ch", super::shared::slugify(sentence)),
            level: LEVEL,
            topic: "Tout, toute, tous, toutes",
            topic_slug: Some("tout-toute-tous".into()),
            task: vec!["Choose the right form.".into()],
            sentence: sentence.replace("____", "…"),
            options: vec![right.into(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![format!("{} {}.", right, tail.trim())],
        }));
    }
    out
}

/* ── ça, cela, ceci ─────────────────────────────────────────────── */

/// (sentence with gap, right word, cue note)
const DEMOS: &[(&str, &str, &str)] = &[
    ("Regarde ici : ____ est mon dessin.", "ceci", "near the speaker → ceci"),
    ("Regarde ici : ____ est ma collection.", "ceci", "near → ceci"),
    ("Tiens, ____ est pour toi.", "ceci", "handing something over → ceci"),
    ("____ est mon passeport, regardez.", "ceci", "near → ceci"),
    ("Tu vois là-bas ? ____ est ta maison.", "cela", "far from both → cela"),
    ("____ m'étonne beaucoup.", "Cela", "general → cela"),
    ("____ dépend de vous.", "Cela", "general → cela"),
    ("Après ____, on est partis.", "cela", "after that → cela"),
    ("____ coûte combien ?", "Ça", "everyday speech → ça"),
    ("____ va, et toi ?", "Ça", "everyday speech → ça"),
    ("Qu'est-ce que ____ veut dire ?", "ça", "everyday speech → ça"),
    ("____ alors, quelle surprise !", "Ça", "everyday speech → ça"),
    ("____ est une excellente idée.", "Cela", "formal → cela"),
    ("N'oublie pas ____.", "cela", "formal → cela"),
    ("____ me plaît beaucoup, merci.", "Ça", "everyday speech → ça"),
    ("D'où vient ____ ?", "cela", "general → cela"),
];

fn ca_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(sentence, right, note) in DEMOS {
        let (head, tail) = sentence.split_once("____").unwrap();
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("ca:{}:cl", super::shared::slugify(sentence)),
            level: LEVEL,
            topic: "Ça, cela, ceci",
            topic_slug: Some("ca-cela-ceci".into()),
            task: vec!["Complete with ça, cela or ceci.".into()],
            head: head.to_string(),
            tail: tail.to_string(),
            infinitive: "ça / cela / ceci",
            answer: right.to_string(),
            rule: vec![
                format!("{} — {}.", right, note),
                "ceci = this (near), cela/ça = that (far or general).".into(),
            ],
        }));
        let wrong = match right {
            "ceci" => "cela",
            "Ça" => "Ceci",
            "Cela" => "Ceci",
            "ça" => "ceci",
            _ => "ceci",
        };
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("ca:{}:ch", super::shared::slugify(sentence)),
            level: LEVEL,
            topic: "Ça, cela, ceci",
            topic_slug: Some("ca-cela-ceci".into()),
            task: vec!["Choose the right word.".into()],
            sentence: sentence.replace("____", "…"),
            options: vec![right.into(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![format!("{} — {}.", right, note)],
        }));
    }
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "qui-que-ou" => relatives_items(),
        "possessive-pronouns" => poss_pron_items(),
        "tout-toute-tous" => tout_items(),
        "ca-cela-ceci" => ca_items(),
        _ => Vec::new(),
    }
}
