//! Family: B1 tenses — plus-que-parfait (auxiliary imparfait + past
//! participle, "the earlier of two past events") and the narrative
//! mix (imparfait background vs passé composé events vs PQP
//! anteriority). English cues keep every story row unambiguous.

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem};

use super::shared::{punctuate, starts_vowel};

const LEVEL: &str = "B1";

/* ── plus-que-parfait ────────────────────────────────────────────── */

/// avoir / être in the imparfait, for je, tu, il/elle, nous, vous, ils/elles.
const AV_IMPARFAIT: [&str; 6] = ["avais", "avais", "avait", "avions", "aviez", "avaient"];
const ETRE_IMPARFAIT: [&str; 6] = ["étais", "étais", "était", "étions", "étiez", "étaient"];

/// (subject, imparfait-aux form index, feminine?)
const PQP_SUBJ: &[(&str, usize, bool)] = &[
    ("je", 0, false),
    ("tu", 1, false),
    ("il", 2, false),
    ("elle", 2, true),
    ("nous", 3, false),
    ("vous", 4, false),
    ("ils", 5, false),
    ("elles", 5, true),
];

struct PqpVerb {
    inf: &'static str,
    /// être auxiliary → the participle agrees with the subject
    etre_aux: bool,
    /// base (masculine singular) participle
    p: &'static str,
    /// complement after the participle ("" for motion verbs)
    complement: &'static str,
}

const PQP_VERBS: &[PqpVerb] = &[
    PqpVerb { inf: "finir", etre_aux: false, p: "fini", complement: "le rapport" },
    PqpVerb { inf: "manger", etre_aux: false, p: "mangé", complement: "" },
    PqpVerb { inf: "lire", etre_aux: false, p: "lu", complement: "le journal" },
    PqpVerb { inf: "faire", etre_aux: false, p: "fait", complement: "les courses" },
    PqpVerb { inf: "recevoir", etre_aux: false, p: "reçu", complement: "la réponse" },
    PqpVerb { inf: "partir", etre_aux: true, p: "parti", complement: "" },
    PqpVerb { inf: "arriver", etre_aux: true, p: "arrivé", complement: "" },
    PqpVerb { inf: "rester", etre_aux: true, p: "resté", complement: "à la maison" },
];

/// (context clause — the SECOND event) — the PQP clause is the event
/// BEFORE this one.
const WHEN_CLAUSES: &[&str] = &[
    "Quand je suis arrivé,",
    "Quand la police est arrivée,",
    "Quand le film a commencé,",
    "Quand nous sommes rentrés,",
    "À mon réveil,",
    "Quand le train est parti,",
];

/// "j'avais déjà fini le rapport" / "elle était déjà partie" /
/// "ils étaient déjà restés"
fn pqp_clause(subj: &str, si: usize, feminine: bool, v: &PqpVerb) -> String {
    let aux = if v.etre_aux { ETRE_IMPARFAIT[si] } else { AV_IMPARFAIT[si] };
    let subj_aux = if subj == "je" && starts_vowel(aux) {
        format!("j'{}", aux)
    } else {
        format!("{} {}", subj, aux)
    };
    let mut participle = v.p.to_string();
    if v.etre_aux {
        let plural = matches!(subj, "nous" | "vous" | "ils" | "elles");
        if feminine {
            participle.push('e');
        }
        if plural {
            participle.push('s');
        }
    }
    if v.complement.is_empty() {
        format!("{} déjà {}", subj_aux, participle)
    } else {
        format!("{} déjà {} {}", subj_aux, participle, v.complement)
    }
}

fn pqp_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in PQP_VERBS {
        for &(subj, si, feminine) in PQP_SUBJ {
            let clause = pqp_clause(subj, si, feminine, v);
            let aux = if v.etre_aux { ETRE_IMPARFAIT[si] } else { AV_IMPARFAIT[si] };
            // Cloze: "Quand je suis arrivé, [ elle était déjà partie ]."
            for (wi, when) in WHEN_CLAUSES.iter().enumerate() {
                out.push(ExerciseItem::Cloze(ClozeItem {
                    id: format!("pqp:{}:{}:{}:cl", v.inf, subj, wi),
                    level: LEVEL,
                    topic: "Plus-que-parfait",
                    topic_slug: Some("plus-que-parfait".into()),
                    task: vec!["Put the verb in the plus-que-parfait.".into()],
                    head: format!("{} ", when),
                    tail: ".".into(),
                    infinitive: v.inf,
                    answer: clause.clone(),
                    rule: vec![
                        format!("{} {}.", when, clause),
                        "plus-que-parfait = imparfait of the auxiliary + past participle — the event that came first.".into(),
                    ],
                }));
            }
            // Choice: PC vs PQP — the quand-clause settles which event came first.
            let participle = {
                let mut p = v.p.to_string();
                if v.etre_aux {
                    if feminine {
                        p.push('e');
                    }
                    if matches!(subj, "nous" | "vous" | "ils" | "elles") {
                        p.push('s');
                    }
                }
                p
            };
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("pqp:{}:{}:ch", v.inf, subj),
                level: LEVEL,
                topic: "Plus-que-parfait",
                topic_slug: Some("plus-que-parfait".into()),
                task: vec!["Choose the right tense.".into()],
                sentence: format!("Quand je suis arrivé, {} ____ déjà {}.", subj, participle),
                options: vec![aux.to_string(), if v.etre_aux { "est".into() } else { "a".into() }],
                correct: 0, // shuffled in `generate`
                rule: vec![
                    format!("Quand je suis arrivé, {}.", clause),
                    "Arriving happens after — the earlier event takes the plus-que-parfait.".into(),
                ],
            }));
        }
        // Error-spot: PC sneaks in where the PQP belongs.
        if !v.etre_aux {
            let words = punctuate(vec![
                "Il".into(),
                "a".into(),
                "déjà".into(),
                v.p.into(),
                "quand".into(),
                "je".into(),
                "suis".into(),
                "arrivé".into(),
            ]);
            out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
                id: format!("pqp:{}:il:es", v.inf),
                level: LEVEL,
                topic: "Plus-que-parfait",
                topic_slug: Some("plus-que-parfait".into()),
                task: vec!["One word is wrong. Tap it.".into()],
                words,
                wrong: 1,
                fix: "avait".into(),
                fix_task: "Now write the correct form.",
                rule: vec![
                    "Il avait déjà mangé quand je suis arrivé.".into(),
                    "Two past events: the earlier one takes the plus-que-parfait.".into(),
                ],
            }));
        }
    }
    // être-auxiliary agreement trap: "Elle était parti" → "partie".
    out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
        id: "pqp:elle-agreement:es".into(),
        level: LEVEL,
        topic: "Plus-que-parfait",
        topic_slug: Some("plus-que-parfait".into()),
        task: vec!["One word is wrong. Tap it.".into()],
        words: punctuate(vec![
            "Elle".into(),
            "était".into(),
            "parti".into(),
            "quand".into(),
            "je".into(),
            "suis".into(),
            "arrivé".into(),
        ]),
        wrong: 2,
        fix: "partie".into(),
        fix_task: "Now write the correct form.",
        rule: vec![
            "Elle était déjà partie.".into(),
            "With être in the plus-que-parfait, the participle agrees with the subject.".into(),
        ],
    }));
    out
}

/* ── past tenses in a story ──────────────────────────────────────── */

/// (sentence with ____, English cue fixing the meaning, correct form,
/// two decoys, rule). The English cue makes every row unambiguous.
const STORY_ROWS: &[(&str, &str, &str, [&str; 2], &str)] = &[
    ("Quand je suis entré, il ____ la télévision.", "he was in the middle of watching", "regardait", ["a regardé", "regarde"], "Background in progress → imparfait."),
    ("Il ____ son assiette, puis il a rangé la table.", "finished (one event, then the next)", "a fini", ["finissait", "finira"], "Successive events → passé composé."),
    ("La réunion ____ quand je suis arrivé au bureau.", "had already started before I got there", "avait commencé", ["commençait", "a commencé"], "Before another past event → plus-que-parfait."),
    ("À l'école, nous ____ souvent ensemble.", "used to, a habit", "déjeunions", ["avons déjeuné", "déjeunerons"], "Habit in the past → imparfait."),
    ("Elle ____ son portefeuille, elle a appelé la police.", "lost it (the event that triggered the next)", "a perdu", ["perdait", "avait perdu"], "One-time event → passé composé."),
    ("Le train ____ déjà quand nous sommes arrivés sur le quai.", "had already left", "était parti", ["partait", "est parti"], "être + imparfait + participle = plus-que-parfait."),
    ("Il pleuvait et le vent ____ fort.", "was blowing (weather background)", "soufflait", ["a soufflé", "souffle"], "Description / background → imparfait."),
    ("D'abord elle ____ le mot dans le dictionnaire, ensuite elle a rédigé.", "looked it up (step one)", "a consulté", ["consultait", "avait consulté"], "Narrative step → passé composé."),
    ("Je ne ____ pas qu'il avait déménagé.", "didn't know (state of mind)", "savais", ["ai su", "savait"], "States and feelings → imparfait."),
    ("Tout à coup, il ____ un bruit étrange.", "heard (one sudden event)", "a entendu", ["entendait", "avait entendu"], "Sudden event → passé composé."),
    ("Quand le taxi est arrivé, nous ____ nos valises.", "had already finished packing", "avions fini", ["finissions", "avons fini"], "Earlier of two past events → plus-que-parfait."),
    ("Pendant que le chef ____, les serveurs dressaient les tables.", "was cooking (in progress)", "cuisinait", ["a cuisiné", "cuisinera"], "Two parallel backgrounds → imparfait."),
];

fn slug_story(s: &str) -> String {
    super::shared::slugify(&s.chars().take(24).collect::<String>())
}

fn story_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, cue, good, decoys, rule) in STORY_ROWS {
        let key = slug_story(fr);
        // Choice: three past forms, the English cue picks one.
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("story:{}:ch", key),
            level: LEVEL,
            topic: "Past tenses in a story",
            topic_slug: Some("past-tenses-story".into()),
            task: vec![format!("{} — choose the form.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), decoys[0].into(), decoys[1].into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        // Cloze: same slot, the English cue rides in the task.
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("story:{}:cl", key),
            level: LEVEL,
            topic: "Past tenses in a story",
            topic_slug: Some("past-tenses-story".into()),
            task: vec![format!("Put the verb in the right past tense — {}.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: inf_of(good),
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Error-spots: rows where one word flips the tense story.
    for (fr_words, wrong_i, fix, key, rule) in [
        (
            vec!["Le", "train", "est", "déjà", "parti", "quand", "nous", "sommes", "arrivés"],
            2usize,
            "était",
            "train-parti",
            "The leaving happened before the arriving → plus-que-parfait.",
        ),
        (
            vec!["Quand", "je", "suis", "entré,", "il", "a", "regardé", "la", "télévision"],
            6usize,
            "regardait",
            "entre-regarde",
            "A background in progress takes the imparfait.",
        ),
    ] {
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("story:{}:es", key),
            level: LEVEL,
            topic: "Past tenses in a story",
            topic_slug: Some("past-tenses-story".into()),
            task: vec!["One word is wrong. Tap it.".into()],
            words: punctuate(fr_words.iter().map(|w| w.to_string()).collect()),
            wrong: wrong_i,
            fix: fix.into(),
            fix_task: "Now write the correct form.",
            rule: vec![format!("… {} …", fix), rule.to_string()],
        }));
    }
    out
}

fn inf_of(good: &str) -> &'static str {
    match good {
        "regardait" | "regarde" => "regarder",
        "a fini" | "finissait" | "avions fini" | "finissions" => "finir",
        "avait commencé" | "commençait" => "commencer",
        "déjeunions" => "déjeuner",
        "a perdu" | "perdait" => "perdre",
        "était parti" | "partait" => "partir",
        "soufflait" => "souffler",
        "a consulté" | "consultait" => "consulter",
        "savais" | "ai su" => "savoir",
        "a entendu" | "entendait" => "entendre",
        _ => "cuisiner",
    }
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "plus-que-parfait" => pqp_items(),
        "past-tenses-story" => story_items(),
        _ => Vec::new(),
    }
}
