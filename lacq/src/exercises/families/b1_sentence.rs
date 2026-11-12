//! Family: B1 sentence patterns — reported speech (present reporting,
//! que vs si), restrictive negation (ne… que, aucun, ni… ni), the
//! cleft « ce qui …, c'est … » and the presentative « il est / c'est
//! + adjective + de ».

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem, TransformItem};

use super::shared::punctuate;

const LEVEL: &str = "B1";

/* ── reported speech (present) ───────────────────────────────────── */

/// (direct speech with speaker frame, reported answer, cue)
const RPP_ROWS: &[(&str, &str, &str)] = &[
    ("« Je suis fatigué. »", "Il dit qu'il est fatigué.", "statement → que"),
    ("« Je t'appelle ce soir. »", "Elle dit qu'elle m'appelle ce soir.", "statement → que"),
    ("« Tu viens avec nous ? »", "Il me demande si je viens avec eux.", "yes/no question → si"),
    ("« Est-ce que le bus est passé ? »", "Elle me demande si le bus est passé.", "yes/no question → si"),
    ("« Nous partons demain. »", "Ils disent qu'ils partent demain.", "statement → que"),
    ("« Tu connais Marie ? »", "Il me demande si je connais Marie.", "yes/no question → si"),
    ("« Il fait beau aujourd'hui. »", "Elle dit qu'il fait beau aujourd'hui.", "statement → que"),
    ("« Vous aimez le café ? »", "Il nous demande si nous aimons le café.", "yes/no question → si"),
    ("« Je cherche mes clés. »", "Elle dit qu'elle cherche ses clés.", "statement → que"),
    ("« On peut garer ici ? »", "Il demande si on peut garer ici.", "yes/no question → si"),
];

fn rpp_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(direct, reported, cue) in RPP_ROWS {
        let key = super::shared::slugify(&direct.chars().take(22).collect::<String>());
        // Choice: que vs si — decided by statement vs question.
        let is_question = cue.contains("question");
        let frame = if is_question { "Il demande" } else { "Il dit" };
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("rpp:{}:ch", key),
            level: LEVEL,
            topic: "Reported speech (present)",
            topic_slug: Some("reported-speech".into()),
            task: vec![
                "Report it in the present:".into(),
                format!("{}.", cue),
            ],
            sentence: format!("{} ____ {}", frame, direct),
            options: vec![
                if is_question { "si".into() } else { "que".into() },
                if is_question { "que".into() } else { "si".into() },
            ],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("{} {} {}", frame, if is_question { "si" } else { "que" }, direct),
                "A yes/no question reports with « si », a statement with « que ».".into(),
            ],
        }));
        // Transform: the full reported sentence, pronouns shifted.
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("rpp:{}:tf", key),
            level: LEVEL,
            topic: "Reported speech (present)",
            topic_slug: Some("reported-speech".into()),
            task: vec![
                "Report it with « il/elle dit que » or".into(),
                "« il/elle demande si » — present reporting.".into(),
            ],
            source: format!("{} : {}", frame, direct),
            answer: reported.to_string(),
            rule: vec![
                reported.to_string(),
                "je → il/elle, tu → je, mes → ses — shift the pronouns with the speaker.".into(),
            ],
        }));
    }
    out
}

/* ── ne… que, aucun, ni… ni ──────────────────────────────────────── */

/// (plain sentence with ____ , correct, wrong, cue, rule)
const RQ_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Je ne mange ____ des légumes.", "que", "pas", "only vegetables", "ne… que = only — the verb stays negated but « que » limits the object."),
    ("Il n'a ____ doute sur le sujet.", "aucun", "pas de", "no doubt at all", "aucun = no… whatsoever: il n'a aucun doute."),
    ("Elle ne boit ____ thé ni café.", "ni", "pas", "neither tea nor coffee", "ni… ni joins two refused things."),
    ("Nous n'avons ____ problème.", "aucun", "aucune", "masculine noun", "aucun agrees: un problème → aucun."),
    ("On ne parle ____ français ici.", "que", "jamais", "only French", "ne… que = only."),
    ("Il n'y a ____ ticket disponible.", "aucun", "aucune", "masculine noun", "un ticket → aucun ticket."),
    ("Elle ne prend ____ sucre ____ lait.", "ni", "pas", "neither sugar nor milk", "ni… ni repeats before each refused noun."),
];

fn rq_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in RQ_ROWS {
        let key = super::shared::slugify(&fr.chars().take(24).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("rq:{}:ch", key),
            level: LEVEL,
            topic: "Ne… que, aucun, ni… ni",
            topic_slug: Some("restriction-ne-que".into()),
            task: vec![format!("{} — choose the restriction.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("rq:{}:cl", key),
            level: LEVEL,
            topic: "Ne… que, aucun, ni… ni",
            topic_slug: Some("restriction-ne-que".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "ne… que / aucun / ni… ni",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Transform: seulement → ne… que.
    for (plain, restricted) in [
        ("Je mange seulement des légumes.", "Je ne mange que des légumes."),
        ("Elle boit seulement de l'eau.", "Elle ne boit que de l'eau."),
        ("Il reste seulement cinq minutes.", "Il ne reste que cinq minutes."),
    ] {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("rq:{}:tf", super::shared::slugify(plain)),
            level: LEVEL,
            topic: "Ne… que, aucun, ni… ni",
            topic_slug: Some("restriction-ne-que".into()),
            task: vec!["Say it with « ne… que ».".into()],
            source: plain.into(),
            answer: restricted.into(),
            rule: vec![
                "ne… que = only — « que » sits before what is left.".into(),
                format!("{} → {}", plain, restricted),
            ],
        }));
    }
    // Error-spot: agreement flipped (idée is feminine).
    out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
        id: "rq:aucun:es".into(),
        level: LEVEL,
        topic: "Ne… que, aucun, ni… ni",
        topic_slug: Some("restriction-ne-que".into()),
        task: vec!["One word is wrong. Tap it.".into()],
        words: punctuate(vec![
            "Il".into(),
            "n'a".into(),
            "aucun".into(),
            "idée".into(),
        ]),
        wrong: 2,
        fix: "aucune".into(),
        fix_task: "Now write the correct form.",
        rule: vec![
            "Il n'a aucune idée — une idée → aucune.".into(),
            "aucun(e) agrees with the noun that follows.".into(),
        ],
    }));
    out
}

/* ── ce qui…, c'est ──────────────────────────────────────────────── */

/// (plain sentence, cleft answer, rule)
const CLEFT_ROWS: &[(&str, &str, &str)] = &[
    ("Son courage m'étonne.", "Ce qui m'étonne, c'est son courage.", "ce qui for a subject"),
    ("Ton aide me touche.", "Ce qui me touche, c'est ton aide.", "ce qui for a subject"),
    ("Le silence m'ennuie.", "Ce qui m'ennuie, c'est le silence.", "ce qui for a subject"),
    ("Je veux du calme.", "Ce que je veux, c'est du calme.", "ce que for an object"),
    ("Il me faut des vacances.", "Ce qu'il me faut, c'est des vacances.", "ce que with il me faut"),
    ("J'apprécie ton honnêteté.", "Ce que j'apprécie, c'est ton honnêteté.", "ce que for an object"),
    ("J'ai besoin de repos.", "Ce dont j'ai besoin, c'est de repos.", "ce dont with avoir besoin de"),
    ("Je rêve d'une maison.", "Ce dont je rêve, c'est d'une maison.", "ce dont with rêver de"),
];

fn cleft_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(plain, cleft, note) in CLEFT_ROWS {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("cleft:{}:tf", super::shared::slugify(plain)),
            level: LEVEL,
            topic: "Ce qui…, c'est",
            topic_slug: Some("ce-qui-cest".into()),
            task: vec![
                "Emphasise it with a cleft:".into(),
                "« ce … , c'est … ».".into(),
            ],
            source: plain.into(),
            answer: cleft.into(),
            rule: vec![
                cleft.to_string(),
                format!("{} — the comma is written, and « c'est » carries the emphasis.", note),
            ],
        }));
        // Cloze: the c'est half — head keeps the written comma.
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("cleft:{}:cl", super::shared::slugify(plain)),
            level: LEVEL,
            topic: "Ce qui…, c'est",
            topic_slug: Some("ce-qui-cest".into()),
            task: vec!["Complete the cleft.".into()],
            head: format!("{}, ", cleft.split(", c'est").next().unwrap()),
            tail: format!(" {}.", tail_of(cleft)),
            infinitive: "ce qui / ce que / ce dont",
            answer: "c'est".into(),
            rule: vec![cleft.to_string(), "The cleft always closes with « c'est ».".into()],
        }));
    }
    out
}

fn tail_of(cleft: &str) -> String {
    cleft
        .split(", c'est ")
        .nth(1)
        .unwrap_or("")
        .trim_end_matches('.')
        .to_string()
}

/* ── il est / c'est + adjective + de ─────────────────────────────── */

/// (adjective, verb phrase, c'est-compatible?)
const IEC_ROWS: &[(&str, &str)] = &[
    ("facile", "apprendre le français"),
    ("important", "vérifier l'adresse"),
    ("difficile", "se lever tôt"),
    ("inutile", "crier"),
    ("utile", "poser des questions"),
    ("agréable", "marcher au bord de la mer"),
    ("possible", "réserver en ligne"),
    ("rare", "trouver une place"),
];

fn iec_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(adj, phrase) in IEC_ROWS {
        // Transform: « Apprendre le français est facile. » →
        // « Il est facile d'apprendre le français. »
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("iec:{}:tf", adj),
            level: LEVEL,
            topic: "Il est / c'est + adjective + de",
            topic_slug: Some("il-est-cest-adjectif-de".into()),
            task: vec![
                "Rewrite with « il est + adjective »:".into(),
                "mind the elision of « de ».".into(),
            ],
            source: format!("{} est {}.", cap_first2(phrase), adj),
            answer: format!("Il est {} {}.", adj, de_join(phrase)),
            rule: vec![
                format!("Il est {} {}.", adj, de_join(phrase)),
                "il est + adjective + de + infinitive — de elides before a vowel: d'apprendre.".into(),
            ],
        }));
        // Cloze: adjective + de-phrase as the answer (elision stays inside).
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("iec:{}:cl", adj),
            level: LEVEL,
            topic: "Il est / c'est + adjective + de",
            topic_slug: Some("il-est-cest-adjectif-de".into()),
            task: vec!["Complete with the adjective + de / d'.".into()],
            head: "Il est ".into(),
            tail: ".".into(),
            infinitive: phrase,
            answer: format!("{} {}", adj, de_join(phrase)),
            rule: vec![
                format!("Il est {} {}.", adj, de_join(phrase)),
                "de elides before a vowel: d'apprendre, d'attendre.".into(),
            ],
        }));
    }
    out
}

fn de_join(phrase: &str) -> String {
    let first = phrase.chars().next().unwrap_or('x');
    if "aàâeéèêiîoôuùûh".contains(first.to_ascii_lowercase()) {
        format!("d'{}", phrase)
    } else {
        format!("de {}", phrase)
    }
}

fn cap_first2(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "reported-speech" => rpp_items(),
        "restriction-ne-que" => rq_items(),
        "ce-qui-cest" => cleft_items(),
        "il-est-cest-adjectif-de" => iec_items(),
        _ => Vec::new(),
    }
}
