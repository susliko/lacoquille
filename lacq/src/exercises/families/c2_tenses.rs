//! Family: C2 tenses — the full passé simple conjugation (all six
//! persons of the literary narrative) and fine participle agreement
//! (preceding direct object vs indirect se, rendre compte, cela as
//! antecedent).

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem};

use super::shared::{punctuate, starts_vowel};

const LEVEL: &str = "C2";

/* ── passé simple, all persons ───────────────────────────────────── */

struct PsVerb {
    inf: &'static str,
    /// je, tu, il/elle, nous, vous, ils/elles
    forms: [&'static str; 6],
    /// complement that fits every person
    compl: &'static str,
}

const PS2_VERBS: &[PsVerb] = &[
    PsVerb { inf: "parler", forms: ["parlai", "parlas", "parla", "parlâmes", "parlâtes", "parlèrent"], compl: "à voix basse" },
    PsVerb { inf: "finir", forms: ["finis", "finis", "finit", "finîmes", "finîtes", "finirent"], compl: "le travail" },
    PsVerb { inf: "partir", forms: ["partis", "partis", "partit", "partîmes", "partîtes", "partirent"], compl: "avant l'aube" },
    PsVerb { inf: "faire", forms: ["fis", "fis", "fit", "fîmes", "fîtes", "firent"], compl: "un vœu" },
    PsVerb { inf: "être", forms: ["fus", "fus", "fut", "fûmes", "fûtes", "furent"], compl: "en retard" },
    PsVerb { inf: "avoir", forms: ["eus", "eus", "eut", "eûmes", "eûtes", "eurent"], compl: "le temps" },
    PsVerb { inf: "vouloir", forms: ["voulus", "voulus", "voulut", "voulûmes", "voulûtes", "voulurent"], compl: "en savoir plus" },
    PsVerb { inf: "venir", forms: ["vins", "vins", "vint", "vînmes", "vîntes", "vinrent"], compl: "à la fête" },
    PsVerb { inf: "voir", forms: ["vis", "vis", "vit", "vîmes", "vîtes", "virent"], compl: "l'orage" },
    PsVerb { inf: "prendre", forms: ["pris", "pris", "prit", "prîmes", "prîtes", "prirent"], compl: "la fuite" },
    PsVerb { inf: "aller", forms: ["allai", "allas", "alla", "allâmes", "allâtes", "allèrent"], compl: "au manoir" },
];

const PS2_SUBJ: [&str; 6] = ["je", "tu", "il", "nous", "vous", "ils"];

fn ps2_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in PS2_VERBS {
        for (fi, subj) in PS2_SUBJ.iter().enumerate() {
            let form = v.forms[fi];
            // je + vowel-initial form: the elision folds into the answer.
            let (head, answer) = if *subj == "je" && starts_vowel(form) {
                ("Cette nuit-là, ".to_string(), format!("j'{}", form))
            } else {
                (format!("Cette nuit-là, {} ", subj), form.to_string())
            };
            out.push(ExerciseItem::Cloze(ClozeItem {
                id: format!("ps2:{}:{}:cl", v.inf, subj),
                level: LEVEL,
                topic: "Passé simple, all persons",
                topic_slug: Some("passe-simple-all-persons".into()),
                task: vec!["Literary tale — put the verb in the passé simple.".into()],
                head,
                tail: format!(" {}.", v.compl),
                infinitive: v.inf,
                answer: answer.clone(),
                rule: vec![
                    format!("Cette nuit-là, {} {} {}.", subj, v.forms[fi], v.compl),
                    "passé simple endings: -ai/-as/-a/-âmes/-âtes/-èrent, -is/-is/-it/-îmes/-îtes/-irent, -us/-us/-ut/-ûmes/-ûtes/-urent.".into(),
                ],
            }));
        }
    }
    // Choice: person & number of the literary forms.
    for (fr, good, wrong, cue, rule) in [
        ("Les convives ____ à minuit.", "partirent", "partit", "several guests", "3rd plural: -irent."),
        ("Elle ____ la fuite au petit jour.", "prit", "prirent", "one person", "3rd singular: prit."),
        ("Ils ____ l'orage sous un hangar.", "virent", "vit", "they saw", "voir, 3rd plural: virent."),
        ("Tu ____ sans un bruit, poursuit le conte.", "sortis", "sortit", "the narrator says « tu »", "2nd singular: tu sortis."),
        ("Nous ____ heureux, cette année-là.", "fûmes", "fut", "we were", "être, 1st plural: fûmes."),
        ("Vous ____ du courage en ce jour-là, raconte le journal.", "eûtes", "eut", "vous (plural)", "avoir, 2nd plural: eûtes."),
        ("Je ____ en savoir plus dès ce soir-là.", "voulus", "voulut", "the narrator « je »", "vouloir, 1st singular: voulus."),
        ("Elles ____ au manoir sans attendre l'aube.", "allèrent", "alla", "they went", "aller, 3rd plural: allèrent."),
    ] {
        let key = super::shared::slugify(&fr.chars().take(24).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("ps2:{}:ch", key),
            level: LEVEL,
            topic: "Passé simple, all persons",
            topic_slug: Some("passe-simple-all-persons".into()),
            task: vec![format!("{} — choose the passé simple form.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Error-spot: the imparfait creeps into the tale.
    for (words, wrong_i, fix, rule) in [
        (
            vec!["Les", "convives", "partaient", "à", "minuit"],
            2usize,
            "partirent",
            "A tale's completed event → partirent.",
        ),
        (
            vec!["Elle", "prenait", "la", "fuite", "au", "petit", "jour"],
            1usize,
            "prit",
            "One done deal → prit.",
        ),
    ] {
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("ps2:{}:es", super::shared::slugify(fix)),
            level: LEVEL,
            topic: "Passé simple, all persons",
            topic_slug: Some("passe-simple-all-persons".into()),
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

/* ── fine participle agreement ───────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const FINE_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Les fleurs que je lui ai ____ fleuriront demain.", "données", "donné", "« lui » is indirect, the flowers are direct", "The preceding direct object (les fleurs, féminin pluriel) drives the agreement; « lui » ne compte pas."),
    ("Elle s'est ____ compte du problème trop tard.", "rendu", "rendue", "she realised", "rendre compte → « compte » est le COD, le « se » est indirect → invariable."),
    ("La lettre qu'elle s'est ____ hier est partie.", "écrite", "écrit", "the letter she wrote (to herself)", "« que » (la lettre) précède avoir → accord: écrite."),
    ("Toutes les précautions que cela a ____ ont payé.", "demandées", "demandé", "cela demanded them", "« que » (les précautions) est COD antéposé → -es."),
    ("Les mesures qui ont été ____ sont efficaces.", "prises", "pris", "with être, always agrees", "With être the participle always agrees with the subject."),
    ("La décision qu'ils ont ____ hier soir surprend.", "prise", "prisent", "the decision they took", "COD antéposé féminin singulier → prise."),
    ("Les progrès que tu as ____ depuis septembre sont réels.", "faits", "fait", "the progress you made", "les progrès, masculin pluriel antéposé → faits."),
    ("Les efforts que cela a ____ méritent un mot de remerciement.", "demandés", "demandé", "the efforts that required", "« cela » is the subject; « que » (les efforts) is the preceding COD → -s."),
    ("Les idées qu'ils ont ____ étaient neuves.", "eues", "eu", "the ideas they had", "avoir + COD antéposé féminin pluriel → eues."),
];

fn fine_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in FINE_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("fine:{}:ch", key),
            level: LEVEL,
            topic: "Fine participle agreement",
            topic_slug: Some("fine-participle-agreement".into()),
            task: vec![format!("{} — agree carefully.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("fine:{}:cl", key),
            level: LEVEL,
            topic: "Fine participle agreement",
            topic_slug: Some("fine-participle-agreement".into()),
            task: vec![format!("{} — complete the participle.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "participle",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Error-spots.
    for (words, wrong_i, fix, rule) in [
        (
            vec!["Les", "fleurs", "que", "je", "lui", "ai", "donné"],
            6usize,
            "données",
            "« lui » is indirect; « les fleurs » drives the agreement → données.",
        ),
        (
            vec!["Elle", "s'est", "rendue", "compte", "du", "problème"],
            2usize,
            "rendu",
            "se rendre compte → « compte » is the COD → invariable rendu.",
        ),
    ] {
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("fine:{}:es", super::shared::slugify(fix)),
            level: LEVEL,
            topic: "Fine participle agreement",
            topic_slug: Some("fine-participle-agreement".into()),
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
        "passe-simple-all-persons" => ps2_items(),
        "fine-participle-agreement" => fine_items(),
        _ => Vec::new(),
    }
}
