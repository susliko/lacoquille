//! Family: questions — two topics. Three question forms (intonation →
//! est-ce que → inversion, transforms from one statement) and question
//! words (qui/que/où/quand/comment/pourquoi/combien — cloze + choice,
//! plus the est-ce que variant).

use crate::exercises::{ChoiceItem, ClozeItem, ExerciseItem, TransformItem};

use super::shared::slugify;

const LEVEL: &str = "A1";

/* ── three question forms ───────────────────────────────────────── */

/// (statement, est-ce que form, inversion form)
const STATEMENTS: &[(&str, &str, &str)] = &[
    ("Tu vas bien.", "Est-ce que tu vas bien ?", "Vas-tu bien ?"),
    ("Tu parles français.", "Est-ce que tu parles français ?", "Parles-tu français ?"),
    ("Il travaille le samedi.", "Est-ce qu'il travaille le samedi ?", "Travaille-t-il le samedi ?"),
    ("Nous mangeons ici.", "Est-ce que nous mangeons ici ?", "Mangeons-nous ici ?"),
    ("Vous partez demain.", "Est-ce que vous partez demain ?", "Partez-vous demain ?"),
    ("Elle aime le café.", "Est-ce qu'elle aime le café ?", "Aime-t-elle le café ?"),
    ("Ils jouent au tennis.", "Est-ce qu'ils jouent au tennis ?", "Jouent-ils au tennis ?"),
    ("Tu finis à cinq heures.", "Est-ce que tu finis à cinq heures ?", "Finis-tu à cinq heures ?"),
    ("Elle vient avec nous.", "Est-ce qu'elle vient avec nous ?", "Vient-elle avec nous ?"),
    ("Vous voulez un café.", "Est-ce que vous voulez un café ?", "Voulez-vous un café ?"),
];

fn three_forms_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for (stmt, esq, inv) in STATEMENTS {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("q3:{}:tf-esq", slugify_stmt(stmt)),
            level: LEVEL,
            topic: "Three question forms",
            topic_slug: Some("three-question-forms".into()),
            task: vec![
                "Turn it into a question with « est-ce que ».".into(),
                "Keep every word in place.".into(),
            ],
            source: format!("{}", stmt),
            answer: format!("{}", esq),
            rule: vec![
                "est-ce que + the statement, word order unchanged.".into(),
                format!("{} → {}.", stmt, esq),
            ],
        }));
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("q3:{}:tf-inv", slugify_stmt(stmt)),
            level: LEVEL,
            topic: "Three question forms",
            topic_slug: Some("three-question-forms".into()),
            task: vec![
                "Turn it into a question with inversion —".into(),
                "pronoun attached to the verb with a hyphen.".into(),
            ],
            source: format!("{}", stmt),
            answer: format!("{}", inv),
            rule: vec![
                "Swap the subject pronoun after the verb, joined by a hyphen.".into(),
                format!("{} → {}.", stmt, inv),
            ],
        }));
        // Choice: the est-ce que spelling trap — "Est" without the
        // hyphen. The que/qu' tail stays in the sentence.
        let stem = esq
            .trim_end_matches('?')
            .trim()
            .strip_prefix("Est-ce ")
            .unwrap_or(esq.trim_end_matches('?').trim())
            .trim_start();
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("q3:{}:ch", slugify_stmt(stmt)),
            level: LEVEL,
            topic: "Three question forms",
            topic_slug: Some("three-question-forms".into()),
            task: vec!["Choose the right form.".into()],
            sentence: format!("____ {} ?", stem),
            options: vec!["Est-ce".into(), "Est".into()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                "Est-ce que — with a hyphen — is one frozen expression.".into(),
                format!("{}.", esq),
            ],
        }));
    }
    out
}

fn slugify_stmt(s: &str) -> String {
    super::shared::slugify(&s.to_lowercase())
}

/* ── question words ─────────────────────────────────────────────── */

/// (word, question with inversion, est-ce que variant, distractor word)
const QWORDS: &[(&str, &str, &str, &str)] = &[
    ("Où", "Où habites-tu ?", "Où est-ce que tu habites ?", "Quand"),
    ("Quand", "Quand arrives-tu ?", "Quand est-ce que tu arrives ?", "Où"),
    ("Comment", "Comment vas-tu ?", "Comment est-ce que tu vas ?", "Pourquoi"),
    ("Pourquoi", "Pourquoi pleures-tu ?", "Pourquoi est-ce que tu pleures ?", "Comment"),
    ("Combien", "Combien coûte ce livre ?", "Combien est-ce que ça coûte ?", "Qui"),
    ("Qui", "Qui vient ce soir ?", "Qui est-ce qui vient ce soir ?", "Que"),
    ("Que", "Que fais-tu ?", "Qu'est-ce que tu fais ?", "Qui"),
    ("Où", "Où travailles-tu ?", "Où est-ce que tu travailles ?", "Comment"),
    ("Quand", "Quand part le train ?", "Quand est-ce que le train part ?", "Pourquoi"),
    ("Comment", "Comment dit-on « chat » ?", "Comment est-ce qu'on dit « chat » ?", "Que"),
    ("Pourquoi", "Pourquoi ris-tu ?", "Pourquoi est-ce que tu ris ?", "Où"),
    ("Combien", "Combien as-tu de frères ?", "Combien de frères as-tu ?", "Quand"),
];

fn qword_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for (word, question, esq, distractor) in QWORDS {
        // Cloze: the question word is the gap.
        let rest = question.split_once(' ').unwrap().1;
        let plain = drop_trailing_q(rest);
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("qw:{}:{}:cl", word.to_lowercase(), slugify(rest)),
            level: LEVEL,
            topic: "Question words",
            topic_slug: Some("question-words".into()),
            task: vec!["Complete the question with the right word.".into()],
            head: "____ ".into(),
            tail: format!(" {} ?", plain),
            infinitive: "question word",
            answer: word.to_string(),
            rule: vec![
                format!("{} {} — {} asks for that missing info.", word, plain, word),
            ],
        }));
        // Choice: word vs plausible distractor.
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("qw:{}:{}:ch", word.to_lowercase(), slugify(rest)),
            level: LEVEL,
            topic: "Question words",
            topic_slug: Some("question-words".into()),
            task: vec!["Choose the right question word.".into()],
            sentence: format!("____ {} ?", plain),
            options: vec![word.to_string(), distractor.to_string()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                qword_note(word).into(),
                format!("{} {}.", word, plain),
            ],
        }));
        // Transform: est-ce que variant keeps the word, flattens inversion.
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("qw:{}:{}:tf", word.to_lowercase(), slugify(rest)),
            level: LEVEL,
            topic: "Question words",
            topic_slug: Some("question-words".into()),
            task: vec!["Ask it with « est-ce que » instead.".into()],
            source: format!("{} ?", question),
            answer: esq.to_string(),
            rule: vec![format!("{} → {}.", question, esq)],
        }));
    }
    out
}

fn drop_trailing_q(s: &str) -> String {
    s.trim_end_matches('?').trim().to_string()
}

fn qword_note(word: &str) -> &'static str {
    match word {
        "Où" => "où = where (place)",
        "Quand" => "quand = when (time)",
        "Comment" => "comment = how (manner)",
        "Pourquoi" => "pourquoi = why (reason)",
        "Combien" => "combien = how much/many (quantity)",
        "Qui" => "qui = who (person)",
        _ => "que = what (thing)",
    }
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "three-question-forms" => three_forms_items(),
        "question-words" => qword_items(),
        _ => Vec::new(),
    }
}
