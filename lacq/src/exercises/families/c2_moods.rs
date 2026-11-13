//! Family: C2 moods — the imparfait du subjonctif (qu'il vînt, qu'il
//! fût, qu'il eût) as recognition, and the conditionnel passé 2e forme
//! (il eût refusé, ils fussent venus) as production.

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem};

use super::shared::punctuate;

const LEVEL: &str = "C2";

/* ── subjonctif imparfait ────────────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const SIMP_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("La reine souhaitait qu'il ____ à la cour.", "vînt", "vienne", "literary narration in the past", "imparfait du subjonctif of venir: qu'il vînt — the literary past-context form."),
    ("Qui qu'il ____, personne ne bougea.", "fût", "soit", "whoever he was (literary)", "être: qu'il fût — imparfait du subjonctif."),
    ("Il fallait que nous le ____ avant la nuit.", "fissions", "fassions", "literary: that we do it", "faire: que nous fissions is the imparfait; fassions is the présent du subjonctif."),
    ("Ils craignaient que ce calme ne ____ qu'un mirage.", "fût", "était", "literary fear", "After craindre in the past, the literary form is the imparfait du subjonctif."),
    ("Elle eût voulu que tu l'____ prévenue.", "eusses", "es", "a literary past wish", "avoir: que tu eusses prévenu — imparfait du subjonctif."),
    ("On craignait que la nouvelle ne ____ publiée trop tôt.", "fût", "soit", "literary worry", "être: ne fût publiée — the literary past subjonctif."),
    ("Le roi exigea que chacun ____ présent à l'aube.", "fût", "soit", "a chronicle in the passé simple", "In a past-tense literary context: que chacun fût présent."),
    ("Elle était fâchée que je ne l'____ pas attendue.", "eusse", "aie", "literary past reproach", "avoir: que je ne l'eusse pas attendue."),
];

fn simp_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in SIMP_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("simp:{}:ch", key),
            level: LEVEL,
            topic: "Subjonctif imparfait",
            topic_slug: Some("subjonctif-imparfait".into()),
            task: vec![format!("{} — choose the literary form.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("simp:{}:cl", key),
            level: LEVEL,
            topic: "Subjonctif imparfait",
            topic_slug: Some("subjonctif-imparfait".into()),
            task: vec![format!("{} — type the literary form.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "imparfait du subjonctif",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Recognition: decode into modern French.
    out.push(ExerciseItem::Choice(ChoiceItem {
        id: "simp:recognition:ch".into(),
        level: LEVEL,
        topic: "Subjonctif imparfait",
        topic_slug: Some("subjonctif-imparfait".into()),
        task: vec!["« Qu'il vînt » corresponds, in modern French, to:".into()],
        sentence: "« On souhaitait qu'il vînt. »".into(),
        options: vec![
            "qu'il vienne".into(),
            "qu'il viendra".into(),
        ],
        correct: 0, // shuffled in `generate`
        rule: vec![
            "Qu'il vînt = qu'il vienne — the imparfait du subjonctif lives only in writing today.".into(),
            "Recognise it in Proust or Camus; use the présent du subjonctif when you speak.".into(),
        ],
    }));
    out
}

/* ── conditionnel passé 2e forme ─────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const CP2_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("S'il avait su, il ____ refusé.", "eût", "aurait", "literary unreal past", "il eût refusé = il aurait refusé — the conditionnel passé, literary 2nd form."),
    ("S'ils avaient été prévenus, ils ____ venus.", "fussent", "seraient", "être-auxiliary, literary", "ils fussent venus = ils seraient venus."),
    ("À sa place, j'____ tout accepté.", "eusse", "aurais", "literary unreal remark", "j'eusse tout accepté — the literary j'-form."),
    ("Qui l'____ soupçonné ?", "eût", "aurait", "the literary set phrase", "Qui l'eût soupçonné ? = who would have suspected it."),
    ("Nous l'____ vu avec plaisir.", "eussions", "aurions", "literary we-form", "nous l'eussions vu — 1st plural."),
    ("Il l'____ aidé sans hésiter.", "eût", "aurait", "literary he-form", "il l'eût aidé."),
    ("Elles n'____ jamais signé un tel contrat.", "eussent", "auraient", "literary they-form", "elles n'eussent jamais signé — 3rd plural feminine."),
    ("Vous l'____ admiré, ce paysage.", "eussiez", "auriez", "literary vous-form", "vous l'eussiez admiré — 2nd plural."),
];

fn cp2_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in CP2_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("cp2:{}:ch", key),
            level: LEVEL,
            topic: "Conditionnel passé 2e forme",
            topic_slug: Some("conditionnel-passe-deuxieme".into()),
            task: vec![format!("{} — choose the literary form.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("cp2:{}:cl", key),
            level: LEVEL,
            topic: "Conditionnel passé 2e forme",
            topic_slug: Some("conditionnel-passe-deuxieme".into()),
            task: vec![format!("{} — type the literary auxiliary.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "conditionnel passé II",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Error-spot: the standard form slips into the literary frame.
    for (words, wrong_i, fix, rule) in [
        (
            vec!["Nul", "n'aurait", "osé", "le", "dire"],
            1usize,
            "n'eût",
            "Literary register: nul n'eût osé le dire.",
        ),
        (
            vec!["Qui", "l'aurait", "cru", "dans", "cette", "cour"],
            1usize,
            "l'eût",
            "Qui l'eût cru ? — the literary conditionnel passé.",
        ),
    ] {
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("cp2:{}:es", super::shared::slugify(fix)),
            level: LEVEL,
            topic: "Conditionnel passé 2e forme",
            topic_slug: Some("conditionnel-passe-deuxieme".into()),
            task: vec!["One word is wrong. Tap it.".into()],
            words: punctuate(words.iter().map(|w| w.to_string()).collect()),
            wrong: wrong_i,
            fix: fix.into(),
            fix_task: "Now write the correct form.",
            rule: vec![rule.to_string()],
        }));
    }
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "subjonctif-imparfait" => simp_items(),
        "conditionnel-passe-deuxieme" => cp2_items(),
        _ => Vec::new(),
    }
}
