//! Family: C2 style — adjective position & meaning (grand homme /
//! homme grand, ancien, propre, cher), literary negations (point,
//! nul, guère, nullement), stylistic word order (fronted attributes
//! and participles, voici que), archaic fixed phrases (point n'est
//! besoin, belle lurette, de par) and register shifts (spoken →
//! written).

use crate::exercises::{ChoiceItem, ClozeItem, ExerciseItem, TransformItem};

const LEVEL: &str = "C2";

/* ── adjective position & meaning ────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const APOS2_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("J'ai rencontré un ____.", "grand homme", "homme grand", "a great man (fame, character)", "Pre-position = figurative: un grand homme = a great man; post-position = literal size."),
    ("J'ai embauché un ____.", "homme grand", "grand homme", "a tall man (physically)", "After the noun, grand is literal: un homme grand = a tall man."),
    ("C'est mon ____ professeur.", "ancien", "nouveau", "my former teacher", "ancien before the noun = former; after = old / antique."),
    ("Elle collectionne des meubles ____.", "anciens", "anciens de famille", "antique furniture", "After the noun, ancien = antique (des meubles anciens); ses anciens meubles = his former pieces."),
    ("C'est ma ____ voiture.", "propre", "sale", "my own car", "propre before the noun = own; after = clean."),
    ("Il a laissé une ____ voiture dans le garage.", "propre", "à lui", "a clean car", "Post-position: une voiture propre = a clean car."),
    ("Mon ____ ami m'a écrit.", "cher", "coûteux", "dear friend (letter opening)", "cher before = dear (affection); after = expensive (une robe chère)."),
    ("C'est une ____ robe.", "chère", "aimée", "an expensive dress", "After the noun: chère = costly; mon cher ami = dear friend."),
    ("La ____ semaine de l'année est souvent calme.", "dernière", "passée", "the final week of the year", "la dernière semaine de l'année = the final one; la semaine dernière = last week (time)."),
    ("____, nous étions encore à Rome.", "La semaine dernière", "La dernière semaine", "during the most recent week", "Time expressions put dernier after: la semaine dernière."),
    ("Un ____ charme se dégageait du lieu.", "certain", "sûr", "a certain charm (vague)", "un certain charme = a certain (indefinite); une preuve certaine = sure."),
];

fn apos2_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in APOS2_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("apos2:{}:ch", key),
            level: LEVEL,
            topic: "Adjective position & meaning",
            topic_slug: Some("adjective-position-meaning".into()),
            task: vec![format!("{} — position decides the meaning.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("apos2:{}:cl", key),
            level: LEVEL,
            topic: "Adjective position & meaning",
            topic_slug: Some("adjective-position-meaning".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "adjective",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    out
}

/* ── literary negations ──────────────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const LNEG_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Il n'____ point de doute à mes yeux.", "y a", "avait", "literary « there is no doubt »", "ne… point = the literary ne… pas."),
    ("____ ne l'ignorait.", "Nul", "On", "literary register: nobody", "Nul (ne) = personne — the literary negative subject."),
    ("Elle ne parlait ____ de ses années de guerre.", "guère", "jamais", "hardly (rarely)", "ne… guère = hardly, scarcely — a literary flavour."),
    ("Ce retard n'est ____ grave.", "nullement", "rien", "not at all (formal)", "ne… nullement = not in the least — formal/written."),
    ("En ce pays, on ne va ____ sans escorte.", "nulle part", "jamais", "nowhere (written)", "ne… nulle part = nowhere."),
    ("Nul ne ____ ce que l'avenir réservait.", "savait", "sait", "literary past narration", "nul + imparfait — the literary past context."),
    ("Point n'est ____ d'insister.", "besoin", "utile", "the fixed formula", "point n'est besoin de = inutile de — archaic/formal."),
    ("Rien ne ____ plus à ajouter.", "restait", "reste", "literary past", "rien ne restait — the literary past context."),
];

fn lneg_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in LNEG_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("lneg:{}:ch", key),
            level: LEVEL,
            topic: "Literary negations",
            topic_slug: Some("literary-negations".into()),
            task: vec![format!("{} — choose the literary negation.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("lneg:{}:cl", key),
            level: LEVEL,
            topic: "Literary negations",
            topic_slug: Some("literary-negations".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "negation",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Recognition: decode the literary negation.
    out.push(ExerciseItem::Choice(ChoiceItem {
        id: "lneg:recognition:ch".into(),
        level: LEVEL,
        topic: "Literary negations",
        topic_slug: Some("literary-negations".into()),
        task: vec!["« Nul ne sait » équivaut, en langue courante, à :".into()],
        sentence: "« Nul ne sait où il est passé. »".into(),
        options: vec![
            "personne ne sait où il est passé".into(),
            "tout le monde sait où il est passé".into(),
        ],
        correct: 0, // shuffled in `generate`
        rule: vec![
            "Nul ne sait = personne ne sait — nul is the literary negative subject.".into(),
            "Point, nul, guère, nullement: the written negation family.".into(),
        ],
    }));
    // Transform: modern → literary.
    for (plain, literary) in [
        ("Personne ne savait la vérité.", "Nul ne savait la vérité."),
        ("Elle n'a pas hésité une seconde.", "Elle n'a point hésité une seconde."),
        ("Il n'a presque pas dormi.", "Il n'a guère dormi."),
    ] {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("lneg:{}:tf", super::shared::slugify(&plain.chars().take(20).collect::<String>())),
            level: LEVEL,
            topic: "Literary negations",
            topic_slug: Some("literary-negations".into()),
            task: vec!["Rewrite it in literary negation.".into()],
            source: plain.into(),
            answer: literary.into(),
            rule: vec![
                literary.to_string(),
                "personne → nul, pas → point, presque pas → guère.".into(),
            ],
        }));
    }
    out
}

/* ── stylistic word order ────────────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const STWO_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("____ fut sa surprise en le reconnaissant.", "Grande", "Une grande", "literary fronted attribute", "Fronted attribute + inversion (grande fut sa surprise) — written emphasis."),
    ("____ fut leur déception à l'annonce du report.", "Immense", "Une immense", "fronted attribute", "Same pattern: immense fut leur déception."),
    ("____ le seuil, ils saluèrent le chien.", "Franchi", "Ils ont franchi", "literary fronted participle", "Fronted past participle (franchi le seuil, ils…) = after crossing the threshold."),
    ("____ les derniers visiteurs, le gardien ferma le portail.", "Partis", "Après les", "literary absolute participle", "Partis les derniers visiteurs, … = once the last visitors had left."),
    ("Longue ____ la liste des candidats cette année.", "est", "a", "written emphasis", "Fronted attribute + inversion: longue est la liste."),
    ("Et ____ que la pluie cessa, on rentra.", "voici", "regarde", "the literary pivot", "Et voici que… — a literary narrative hinge (voici que + clause)."),
    ("De longues années ____ sans qu'il revienne.", "s'écoulèrent", "ont écoulé", "literary passé simple", "Passé simple in literary narration: s'écoulèrent (se is part of the verb)."),
    ("____, ce phénomène de marées doubles.", "Curieux", "C'est curieux que", "spoken-style segmentation in writing", "Segmentation (Curieux, ce phénomène !) imports spoken emphasis into prose."),
];

fn stwo_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in STWO_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("stwo:{}:ch", key),
            level: LEVEL,
            topic: "Stylistic word order",
            topic_slug: Some("stylistic-word-order".into()),
            task: vec![format!("{} — mind the word order.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("stwo:{}:cl", key),
            level: LEVEL,
            topic: "Stylistic word order",
            topic_slug: Some("stylistic-word-order".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "stylistic order",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    out
}

/* ── archaic fixed phrases ───────────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const ARCH_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("____ n'est besoin d'insister.", "Point", "Rien", "archaic « no need to insist »", "point n'est besoin de = inutile de — archaic/formal formula."),
    ("Il y a ____ qu'on ne s'était vus.", "belle lurette", "belle journée", "a very long time ago", "il y a belle lurette (que) = depuis très longtemps."),
    ("Sitôt ____, sitôt approuvé.", "dit", "dire", "no sooner said than approved", "sitôt… sitôt… = dès que — a fixed formula."),
    ("Chose promise, chose ____.", "due", "dite", "a promise must be kept", "chose promise, chose due — the proverb."),
    ("____ la loi, la vente est annulée.", "De par", "Par", "archaic « by decree of »", "de par + authority = by virtue of (de par la loi)."),
    ("____ grâce, épargnez-le !", "De", "Par", "an archaic plea", "de grâce ! = archaic plea (modern: par pitié / s'il vous plaît)."),
    ("Il n'a ____ de vos conseils.", "cure", "soin", "archaic: he cares nothing for", "n'avoir cure de = ne pas se soucier de — literary/archaic."),
    ("____ son âge, il monta l'escalier d'un trait.", "N'obstant", "Sans obéir à", "archaic « despite »", "n'obstant = malgré — legal/archaic register."),
    ("Le cas ____ , on reportera le vote.", "échéant", "venant", "formal: if need be", "le cas échéant = if the need arises — formal legalese."),
];

fn arch_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in ARCH_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("arch:{}:ch", key),
            level: LEVEL,
            topic: "Archaic fixed phrases",
            topic_slug: Some("archaic-phrases".into()),
            task: vec![format!("{} — choose the fixed phrase.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("arch:{}:cl", key),
            level: LEVEL,
            topic: "Archaic fixed phrases",
            topic_slug: Some("archaic-phrases".into()),
            task: vec![format!("{} — complete the formula.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "fixed phrase",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    out
}

/* ── register shifts ─────────────────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const REG_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("____ un problème technique avec le serveur.", "Nous avons rencontré", "On a eu", "a written incident report", "Written register: nous + a precise verb; on + avoir is spoken."),
    ("____ marche pas, le traducteur.", "Ça", "Cela", "casual speech", "ça + dropped ne = spoken; cela + ne = written."),
    ("____ une réunion à dix heures.", "Y a", "Il existe", "a spoken announcement", "Y a (dropped il) = spoken; il y a / il existe in writing."),
    ("Je vous ____ informé dans les meilleurs délais.", "tiendrai", "tiens", "a formal promise in a letter", "tiendrai + informé — the written promise formula."),
    ("____ me transmettre le dossier avant lundi ?", "Pourriez-vous", "Tu peux", "a polite email", "Pourriez-vous — the written polite request."),
    ("Une anomalie s'est ____ sur le serveur cette nuit.", "produite", "produit", "a written incident report", "se produire — the written alternative to « il s'est passé un truc »."),
];

fn reg_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in REG_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("reg:{}:ch", key),
            level: LEVEL,
            topic: "Register shifts",
            topic_slug: Some("register-shifts".into()),
            task: vec![format!("{} — pick the right register.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("reg:{}:cl", key),
            level: LEVEL,
            topic: "Register shifts",
            topic_slug: Some("register-shifts".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "register",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Transform: spoken → written.
    for (spoken, written) in [
        ("On a eu des soucis avec le délai.", "Nous avons rencontré des difficultés avec le délai."),
        ("Ça marche pas, le traducteur.", "Cela ne fonctionne pas."),
        ("Y a un souci dans le contrat.", "Il y a un problème dans le contrat."),
        ("Je te tiens au courant vite.", "Je vous tiendrai informé dans les meilleurs délais."),
        ("Tu peux envoyer le doc ?", "Pourriez-vous nous transmettre le document ?"),
    ] {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("reg:{}:tf", super::shared::slugify(&spoken.chars().take(20).collect::<String>())),
            level: LEVEL,
            topic: "Register shifts",
            topic_slug: Some("register-shifts".into()),
            task: vec!["Rewrite it in written register.".into()],
            source: spoken.into(),
            answer: written.into(),
            rule: vec![
                written.to_string(),
                "on → nous, dropped ne restored, vague nouns sharpened, vous for tu.".into(),
            ],
        }));
    }
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "adjective-position-meaning" => apos2_items(),
        "literary-negations" => lneg_items(),
        "stylistic-word-order" => stwo_items(),
        "archaic-phrases" => arch_items(),
        "register-shifts" => reg_items(),
        _ => Vec::new(),
    }
}
