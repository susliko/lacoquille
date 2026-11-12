//! Family: A2 sentence cluster — four topics. Exclamations (quel
//! agreeing with its noun, que/comme intensifiers), cause &
//! consequence (parce que before the cause, donc before the
//! result), purpose & opposition (pour + inf, malgré, même si), and
//! sequencing connectors (d'abord → ensuite → puis → enfin).

use crate::exercises::{ChoiceItem, ClozeItem, ExerciseItem, TransformItem};

const LEVEL: &str = "A2";

/* ── exclamations ───────────────────────────────────────────────── */

/// (noun with article, quel form, wrong form)
const QUEL_NOUNS: &[(&str, &str, &str)] = &[
    ("temps", "Quel", "Quelle"),
    ("belle vue", "Quelle", "Quel"),
    ("beau jardin", "Quel", "Quelle"),
    ("bonne idée", "Quelle", "Quel"),
    ("beaux vêtements", "Quels", "Quelles"),
    ("belles fleurs", "Quelles", "Quels"),
    ("bon repas", "Quel", "Quelle"),
    ("grande nouvelle", "Quelle", "Quel"),
    ("long voyage", "Quel", "Quelle"),
    ("jolie surprise", "Quelle", "Quel"),
    ("gros problème", "Quel", "Quelle"),
    ("bonnes nouvelles", "Quelles", "Quels"),
];

fn exclamation_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(noun, right, wrong) in QUEL_NOUNS {
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("exq:{}:ch", super::shared::slugify(noun)),
            level: LEVEL,
            topic: "Exclamations",
            topic_slug: Some("exclamations".into()),
            task: vec!["Choose the right form.".into()],
            sentence: format!("____ {} !", noun),
            options: vec![right.into(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("{} {} ! — quel agrees with the noun.", right, noun),
                "quel (m), quelle (f), quels (mpl), quelles (fpl).".into(),
            ],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("exq:{}:cl", super::shared::slugify(noun)),
            level: LEVEL,
            topic: "Exclamations",
            topic_slug: Some("exclamations".into()),
            task: vec!["Complete the exclamation.".into()],
            head: String::new(),
            tail: format!(" {} !", noun),
            infinitive: "quel",
            answer: right.to_string(),
            rule: vec![format!("{} {} !", right, noun)],
        }));
    }
    // que / comme intensifiers.
    for (plain, exclaim) in [
        ("C'est très beau.", "Que c'est beau !"),
        ("Il pleut beaucoup.", "Comme il pleut !"),
        ("Elle est très gentille.", "Qu'elle est gentille !"),
        ("Tu es grand.", "Comme tu es grand !"),
        ("C'est très cher.", "Que c'est cher !"),
        ("Elle court vite.", "Qu'elle court vite !"),
        ("C'est très difficile.", "Que c'est difficile !"),
        ("Il fait chaud.", "Qu'il fait chaud !"),
    ] {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("exq:{}:tf", super::shared::slugify(plain)),
            level: LEVEL,
            topic: "Exclamations",
            topic_slug: Some("exclamations".into()),
            task: vec!["Turn it into an exclamation".into(), "with que or comme.".into()],
            source: plain.into(),
            answer: exclaim.into(),
            rule: vec![format!("{} → {}.", plain, exclaim)],
        }));
    }
    out
}

/* ── cause & consequence ────────────────────────────────────────── */

/// (cause, consequence, à-cause noun)
const CAUSES: &[(&str, &str, &str)] = &[
    ("il pleut", "je reste ici", "la pluie"),
    ("je suis fatigué", "je me couche tôt", "la fatigue"),
    ("il est malade", "il ne travaille pas", "sa maladie"),
    ("le train est en retard", "on arrive tard", "ce retard"),
    ("elle a réussi", "elle est contente", "ce succès"),
    ("le magasin est fermé", "on rentre chez nous", "sa fermeture"),
    ("j'ai perdu mon téléphone", "je ne peux pas t'appeler", "ce problème"),
    ("il y a des travaux", "on prend le détour", "ces travaux"),
    ("il fait très chaud", "on reste à la maison", "cette chaleur"),
    ("il neige", "les écoles ferment", "la neige"),
    ("le réveil n'a pas sonné", "on a raté le train", "ce réveil"),
    ("le bus est plein", "on attend le suivant", "ce bus"),
];

/// parce que + cause, with the elision join done properly.
fn avec_parce_que(head: &str, cause: &str) -> String {
    if starts_vowel(cause) {
        format!("{}, parce qu'{}.", head, cause)
    } else {
        format!("{}, parce que {}.", head, cause)
    }
}

fn cause_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(cause, consequence, cause_np) in CAUSES {
        let cons_cap = cap(consequence);
        let cause_cap = cap(cause);
        let pq = if starts_vowel(cause) { "parce qu'" } else { "parce que" };
        // Choice: the cause slot — parce que (elided) vs donc.
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("cause:{}:ch", super::shared::slugify(cause)),
            level: LEVEL,
            topic: "Cause & consequence",
            topic_slug: Some("cause-consequence".into()),
            task: vec!["Choose the right connector.".into()],
            sentence: format!("{} ____ {}.", cons_cap, cause),
            options: vec![pq.into(), "donc".into()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                avec_parce_que(&cons_cap, cause),
                "The cause comes after parce que; the result after donc.".into(),
            ],
        }));
        // Cloze: donc before the result — no elision after donc.
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("cause:{}:cl", super::shared::slugify(cause)),
            level: LEVEL,
            topic: "Cause & consequence",
            topic_slug: Some("cause-consequence".into()),
            task: vec!["Complete the result.".into()],
            head: format!("{}, ", cause_cap),
            tail: format!(" {}.", consequence),
            infinitive: "donc",
            answer: "donc".into(),
            rule: vec![
                format!("{}, donc {}.", cause_cap, consequence),
                "The result comes after donc; the cause after parce que.".into(),
            ],
        }));
        // Transform: the same idea with à cause de + noun.
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("cause:{}:tf", super::shared::slugify(cause)),
            level: LEVEL,
            topic: "Cause & consequence",
            topic_slug: Some("cause-consequence".into()),
            task: vec!["Say it with « à cause de » + a noun.".into()],
            source: avec_parce_que(&cons_cap, cause),
            answer: format!("À cause de {}, {}.", cause_np, consequence),
            rule: vec![
                "à cause de + noun = because of…".into(),
                format!("À cause de {}, {}.", cause_np, consequence),
            ],
        }));
    }
    out
}

fn cap(s: &str) -> String {
    super::shared::cap(s)
}

fn starts_vowel(s: &str) -> bool {
    super::shared::starts_vowel(s)
}

/* ── purpose & opposition ───────────────────────────────────────── */

/// (with-gap sentence, right, wrong, note)
const PURPOSE_OPP: &[(&str, &str, &str, &str)] = &[
    ("Je sors ____ la pluie.", "malgré", "même si", "malgré + noun"),
    ("Je sors, ____ il pleut.", "même si", "malgré", "même si + clause"),
    ("____ le froid, on nage.", "Malgré", "même si", "malgré + noun"),
    ("On nage, ____ l'eau est glacée.", "même si", "malgré", "même si + clause"),
    ("____ sa fatigue, elle continue.", "Malgré", "même si", "malgré + noun"),
    ("Elle continue, ____ elle est fatiguée.", "même si", "malgré", "même si + clause"),
    ("____ les embouteillages, on est à l'heure.", "Malgré", "même si", "malgré + noun"),
    ("On est à l'heure, ____ il y avait du trafic.", "même si", "malgré", "même si + clause"),
    ("Il court ____ être en forme.", "pour", "malgré", "pour + infinitive = in order to"),
    ("J'étudie ____ réussir.", "pour", "malgré", "pour + infinitive = in order to"),
    ("Elle économise ____ partir en voyage.", "pour", "malgré", "purpose"),
    ("Il travaille tard ____ finir le projet.", "pour", "même si", "purpose"),
    ("____ tout, ce n'était pas grave.", "Malgré", "même si", "malgré tout = anyway"),
    ("Je viens ____ tu m'appelles pas.", "même si", "malgré", "même si + clause"),
    ("____ son âge, il court vite.", "Malgré", "même si", "malgré + noun"),
    ("On sort, ____ c'est tard.", "même si", "malgré", "même si + clause"),
];

fn purpose_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(sentence, right, wrong, note) in PURPOSE_OPP {
        let (head, tail) = sentence.split_once("____").unwrap();
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("pop:{}:cl", super::shared::slugify(sentence)),
            level: LEVEL,
            topic: "Purpose & opposition",
            topic_slug: Some("purpose-opposition".into()),
            task: vec!["Complete with malgré, même si or pour.".into()],
            head: head.to_string(),
            tail: tail.to_string(),
            infinitive: "malgré / même si / pour",
            answer: right.to_string(),
            rule: vec![format!("{} — {}.", right, note)],
        }));
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("pop:{}:ch", super::shared::slugify(sentence)),
            level: LEVEL,
            topic: "Purpose & opposition",
            topic_slug: Some("purpose-opposition".into()),
            task: vec!["Choose the right connector.".into()],
            sentence: sentence.replace("____", "…"),
            options: vec![right.into(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![format!("{} — {}.", right, note)],
        }));
    }
    out
}

/* ── sequencing connectors ──────────────────────────────────────── */

/// (story with one gap, right connector)
const SEQUENCES: &[(&str, &str)] = &[
    ("____, je me lève. Ensuite, je prends ma douche. Enfin, je m'habille.", "D'abord"),
    ("Je me lève. ____, je prends ma douche. Enfin, je m'habille.", "Ensuite"),
    ("Je me lève, je prends ma douche. ____, je m'habille.", "Enfin"),
    ("____, on lit le menu. Puis on commande. Enfin on mange.", "D'abord"),
    ("On lit le menu, on commande. ____, on mange.", "Puis"),
    ("J'ouvre le livre. ____, je lis un chapitre. Enfin, je ferme.", "Ensuite"),
    ("Il rentre. ____, il dîne. Enfin, il se couche.", "Ensuite"),
    ("Il rentre, il dîne. ____, il se couche.", "Enfin"),
    ("____, je cherche une recette. Ensuite, j'achète. Enfin, je cuisine.", "D'abord"),
    ("Je cherche une recette, j'achète. ____, je cuisine.", "Enfin"),
    ("____, elle s'étire. Puis elle boit un café. Enfin elle sort.", "D'abord"),
    ("Elle s'étire, boit un café. ____, elle sort.", "Enfin"),
    ("Nous montons. ____, nous visitons. Enfin, nous redescendons.", "Ensuite"),
    ("____, on choisit. Ensuite, on paie. Enfin, on repart.", "D'abord"),
    ("On arrive, on s'installe. ____, le concert commence.", "Puis"),
    ("Je tape le code. ____, la porte s'ouvre. Enfin j'entre.", "Ensuite"),
];

fn sequencing_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(story, right) in SEQUENCES {
        let (head, tail) = story.split_once("____").unwrap();
        let wrong = match right {
            "D'abord" => "Enfin",
            "Ensuite" => "D'abord",
            "Puis" => "Enfin",
            _ => "D'abord",
        };
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("seq:{}:cl", super::shared::slugify(story)),
            level: LEVEL,
            topic: "Sequencing connectors",
            topic_slug: Some("sequencing-connectors".into()),
            task: vec!["Complete with d'abord, ensuite, puis or enfin.".into()],
            head: head.to_string(),
            tail: tail.to_string(),
            infinitive: "connector",
            answer: right.to_string(),
            rule: vec![
                format!("{} {}.", right, tail.trim()),
                "d'abord (first) → ensuite/puis (then) → enfin (finally).".into(),
            ],
        }));
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("seq:{}:ch", super::shared::slugify(story)),
            level: LEVEL,
            topic: "Sequencing connectors",
            topic_slug: Some("sequencing-connectors".into()),
            task: vec!["Choose the right connector.".into()],
            sentence: story.replace("____", "…"),
            options: vec![right.into(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![format!("{} {}.", right, tail.trim())],
        }));
    }
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "exclamations" => exclamation_items(),
        "cause-consequence" => cause_items(),
        "purpose-opposition" => purpose_items(),
        "sequencing-connectors" => sequencing_items(),
        _ => Vec::new(),
    }
}
