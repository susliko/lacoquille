//! Family: C1 tenses — passé simple & passé antérieur as *recognition*
//! (spot the literary tense, feel the completed narrative event) and
//! the agreement of pronominal verbs (se COD → agrees, se COI or
//! followed object → invariable).

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem};

use super::shared::punctuate;

const LEVEL: &str = "C1";

/* ── passé simple (recognize) ────────────────────────────────────── */

/// (infinitive, 3sg, 3pl, singular frame with ____, plural frame with
/// ____) — the recognition core.
const PS_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("ouvrir", "ouvrit", "ouvrirent", "Soudain, il ____ la porte.", "Soudain, ils ____ la porte."),
    ("partir", "partit", "partirent", "Le vieux chevalier ____ sans un mot.", "Les chevaliers ____ sans un mot."),
    ("lire", "lut", "lurent", "Elle ____ la lettre en silence.", "Elles ____ la lettre en silence."),
    ("prendre", "prit", "prirent", "Il ____ la ville à l'aube.", "Les soldats ____ la ville à l'aube."),
    ("comprendre", "comprit", "comprurent", "Il ____ enfin la vérité.", "Ils ____ enfin la vérité."),
    ("disparaître", "disparut", "disparurent", "Le cerf ____ dans la forêt.", "Les cerfs ____ dans la forêt."),
    ("éclater", "éclata", "éclatèrent", "Un bruit ____ dans le couloir.", "Des bruits ____ dans le couloir."),
    ("lancer", "lança", "lancèrent", "Le magicien ____ un sortilège.", "Les sorciers ____ un sortilège."),
];

fn ps_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(inf, sg, pl, sg_frame, pl_frame) in PS_ROWS {
        // Cloze: produce the 3sg / 3pl literary form.
        for (person, form, frame) in [("il", sg, sg_frame), ("ils", pl, pl_frame)] {
            out.push(ExerciseItem::Cloze(ClozeItem {
                id: format!("ps:{}:{}:cl", inf, person),
                level: LEVEL,
                topic: "Passé simple (recognize)",
                topic_slug: Some("passe-simple-recognize".into()),
                task: vec!["Written tale — put the verb in the passé simple.".into()],
                head: frame.split("____").next().unwrap().to_string(),
                tail: frame.split("____").nth(1).unwrap().to_string(),
                infinitive: inf,
                answer: form.to_string(),
                rule: vec![
                    format!("{} — « {} ».", frame.replace("____", form), form),
                    "passé simple = the completed event of written narrative; the imparfait paints background.".into(),
                ],
            }));
        }
        // Choice: the event form vs the background imparfait.
        let imp_sg = imparfait_of(inf);
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("ps:{}:ch", inf),
            level: LEVEL,
            topic: "Passé simple (recognize)",
            topic_slug: Some("passe-simple-recognize".into()),
            task: vec!["A sudden event in a tale — choose the tense.".into()],
            sentence: sg_frame.to_string(),
            options: vec![sg.to_string(), imp_sg.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("{} — « {} ».", sg_frame.replace("____", sg), sg),
                "A completed narrative event takes the passé simple in writing.".into(),
            ],
        }));
    }
    // Recognition: what does the literary form express?
    for (sentence, form) in [
        ("« Il alla à la fenêtre. »", "alla"),
        ("« Ils virent la mer pour la première fois. »", "virent"),
        ("« Ce fut un désastre. »", "fut"),
        ("« Ils firent feu. »", "firent"),
    ] {
        let key = super::shared::slugify(&form.chars().take(12).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("ps:{}:rec", key),
            level: LEVEL,
            topic: "Passé simple (recognize)",
            topic_slug: Some("passe-simple-recognize".into()),
            task: vec![format!("What does « {} » express?", form)],
            sentence: sentence.to_string(),
            options: vec![
                "A completed event — the passé simple".into(),
                "An ongoing background — the imparfait".into(),
            ],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("{} — {} is a passé simple form: one done deal.", sentence, form),
                "Recognising the passé simple unlocks whole novels (Camus, Maupassant…).".into(),
            ],
        }));
    }
    // Error-spots: the background form creeps into the event slot.
    for (words, wrong_i, fix, rule) in [
        (
            vec!["Soudain", "il", "ouvrait", "la", "porte"],
            2usize,
            "ouvrit",
            "« Soudain » announces an event → passé simple.",
        ),
        (
            vec!["Le", "chevalier", "partait", "sans", "un", "mot"],
            2usize,
            "partit",
            "One over-and-done departure → partit.",
        ),
        (
            vec!["Les", "soldats", "prenaient", "la", "ville", "à", "l'aube"],
            2usize,
            "prirent",
            "3rd plural event → prirent.",
        ),
    ] {
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("ps:{}:es", super::shared::slugify(fix)),
            level: LEVEL,
            topic: "Passé simple (recognize)",
            topic_slug: Some("passe-simple-recognize".into()),
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

/// the imparfait 3sg decoy for the same verb.
fn imparfait_of(inf: &str) -> &'static str {
    match inf {
        "ouvrir" => "ouvrait",
        "partir" => "partait",
        "lire" => "lisait",
        "prendre" => "prenait",
        "comprendre" => "comprenait",
        "disparaître" => "disparaissait",
        "éclater" => "éclatait",
        _ => "lançait",
    }
}

/* ── passé antérieur ─────────────────────────────────────────────── */

/// (clause start, auxiliary chunk, participle + complement, inf,
/// être-aux?) — literary anteriority: quand il eut fini, on partit.
const PA_ROWS: &[(&str, &str, &str, &str, bool)] = &[
    ("Quand il ", "eut", "fini le travail", "finir", false),
    ("Quand la pluie ", "eut", "cessé", "cesser", false),
    ("Quand il ", "eut", "lu le journal", "lire", false),
    ("Quand le courrier ", "eut", "été distribué", "être distribué", true),
    ("À peine elle ", "eut", "appris la nouvelle", "apprendre", false),
    ("Quand il ", "fut", "arrivé", "arriver", true),
    ("Quand les invités ", "furent", "partis", "partir", true),
    ("Quand il ", "eut", "bu son café", "boire", false),
    ("À peine il ", "eut", "fermé la porte", "fermer", false),
    ("Quand le soleil ", "fut", "couché", "se coucher", true),
];

fn pa_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for (i, &(head, aux, rest, inf, _etre)) in PA_ROWS.iter().enumerate() {
        let tail = match i {
            0 => ", tout le monde partit.",
            1 => ", on sortit.",
            2 => ", on servit le dîner.",
            3 => ", chacun lut son courrier.",
            4 => " qu'elle s'évanouit.",
            5 => ", on servit l'apéritif.",
            6 => ", la maison s'endormit.",
            7 => ", il paya et sortit.",
            8 => ", le chat rentra.",
            _ => ", les étoiles parurent.",
        };
        let answer = format!("{} {}", aux, rest);
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("pa:{}:cl", inf),
            level: LEVEL,
            topic: "Passé antérieur (recognize)",
            topic_slug: Some("passe-anterieur".into()),
            task: vec!["Literary narration — put the verb in the passé antérieur.".into()],
            head: head.to_string(),
            tail: tail.to_string(),
            infinitive: inf,
            answer: answer.clone(),
            rule: vec![
                format!("{}{}{}", head, answer, tail),
                "passé antérieur = passé simple of the auxiliary + participle — the earlier of two literary past events.".into(),
            ],
        }));
    }
    // Choice: literary passé antérieur vs spoken plus-que-parfait.
    for (fr, good, wrong, cue, rule) in [
        ("Quand il ____ le travail, tout le monde partit.", "eut fini", "avait fini", "literary anteriority", "Written narration: passé antérieur; « avait fini » belongs to spoken French."),
        ("Quand il ____ , on servit le dîner.", "fut arrivé", "était arrivé", "être auxiliary, literary", "Motion verbs take the passé antérieur with être: quand il fut arrivé."),
        ("À peine ____ la nouvelle qu'elle s'évanouit.", "eut-elle appris", "elle avait appris", "à peine … que", "À peine + inversion + passé antérieur: à peine eut-elle appris…"),
    ] {
        let key = super::shared::slugify(&fr.chars().take(22).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("pa:{}:ch", key),
            level: LEVEL,
            topic: "Passé antérieur (recognize)",
            topic_slug: Some("passe-anterieur".into()),
            task: vec![format!("{} — keep the literary register.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Recognition: what « eut fini » corresponds to.
    out.push(ExerciseItem::Choice(ChoiceItem {
        id: "pa:recognition:ch".into(),
        level: LEVEL,
        topic: "Passé antérieur (recognize)",
        topic_slug: Some("passe-anterieur".into()),
        task: vec!["« Quand il eut fini » corresponds, in everyday French, to:".into()],
        sentence: "« Quand il eut fini le travail, on partit. »".into(),
        options: vec![
            "quand il avait fini le travail".into(),
            "quand il a fini le travail".into(),
        ],
        correct: 0, // shuffled in `generate`
        rule: vec![
            "Quand il eut fini = quand il avait fini — anteriority, told in the literary tense.".into(),
            "You will meet it in Maupassant and Flaubert; you will rarely produce it.".into(),
        ],
    }));
    // Error-spot: the spoken aux slips into the literary frame.
    out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
        id: "pa:eut:es".into(),
        level: LEVEL,
        topic: "Passé antérieur (recognize)",
        topic_slug: Some("passe-anterieur".into()),
        task: vec!["One word is wrong. Tap it.".into()],
        words: punctuate(vec![
            "Quand".into(),
            "il".into(),
            "avait".into(),
            "fini".into(),
            "tout".into(),
            "le".into(),
            "monde".into(),
            "partit".into(),
        ]),
        wrong: 2,
        fix: "eut".into(),
        fix_task: "Now write the correct form.",
        rule: vec![
            "Quand il eut fini, tout le monde partit.".into(),
            "In the literary tense pair, the earlier event takes the passé antérieur.".into(),
        ],
    }));
    out
}

/* ── agreement of pronominal verbs ───────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const PV_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Elle ____ à six heures.", "s'est levée", "s'est levé", "got up (no other object)", "No other object → se is direct → the participle agrees: levée."),
    ("Elles ____ à six heures.", "se sont levées", "se sont levés", "they got up (feminine)", "Feminine plural subject → -es."),
    ("Ils ____ ce matin.", "se sont rencontrés", "se sont rencontré", "met each other", "rencontrer quelqu'un → se is direct → agreement."),
    ("Ils ____ hier soir.", "se sont téléphoné", "se sont téléphonés", "phoned each other", "téléphoner à → se is indirect → invariable."),
    ("Elle ____ les mains avant le dîner.", "s'est lavé", "s'est lavée", "washed her hands", "A following direct object (les mains) → s' is indirect → invariable."),
    ("Elle ____ avant le dîner.", "s'est lavée", "s'est lavé", "had a wash", "No other object → agreement: lavée."),
    ("Marie et Julie ____ pendant des heures.", "se sont parlé", "se sont parlées", "talked to each other", "parler à → indirect → invariable."),
    ("Elles ____ la veille du départ.", "se sont revues", "se sont revu", "saw each other again", "voir quelqu'un → direct → agrees: revues."),
    ("Elles ____ dans le salon.", "se sont vues", "se sont vu", "met in the living room", "Feminine plural + direct se → vues."),
    ("Elle ____ si tout allait bien.", "s'est demandé", "s'est demandée", "wondered", "The clause is the direct object → s' is indirect → invariable."),
    ("Ils ____ au théâtre.", "se sont donné rendez-vous", "se sont donnés rendez-vous", "arranged to meet", "donner rendez-vous à → se indirect → invariable."),
];

fn pv_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in PV_ROWS {
        let key = super::shared::slugify(&fr.chars().take(24).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("pv:{}:ch", key),
            level: LEVEL,
            topic: "Agreement of pronominal verbs",
            topic_slug: Some("pronominal-agreement".into()),
            task: vec![format!("{} — agree (or don't).", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("pv:{}:cl", key),
            level: LEVEL,
            topic: "Agreement of pronominal verbs",
            topic_slug: Some("pronominal-agreement".into()),
            task: vec![format!("{} — complete the participle form.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "pronominal verb",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Error-spots: the wrong agreement slips in.
    for (words, wrong_i, fix, rule) in [
        (
            vec!["Elle", "s'est", "lavé", "avant", "le", "dîner"],
            2usize,
            "lavée",
            "No other object → s' is direct → lavée.",
        ),
        (
            vec!["Ils", "se", "sont", "téléphonés", "hier", "soir"],
            3usize,
            "téléphoné",
            "téléphoner à → se is indirect → invariable.",
        ),
    ] {
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("pv:{}:es", super::shared::slugify(fix)),
            level: LEVEL,
            topic: "Agreement of pronominal verbs",
            topic_slug: Some("pronominal-agreement".into()),
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
        "passe-simple-recognize" => ps_items(),
        "passe-anterieur" => pa_items(),
        "pronominal-agreement" => pv_items(),
        _ => Vec::new(),
    }
}
