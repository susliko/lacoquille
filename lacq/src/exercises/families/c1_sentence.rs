//! Family: C1 sentence patterns — avoir beau & quand bien même,
//! formal connectors (en effet, par conséquent, or…), stylistic
//! inversion (peut-être viendra-t-il), the expletive ne (avant qu'il
//! ne…, plus… qu'on ne le croit), formal impersonals (il convient de,
//! il s'agit de) and spoken-vs-written register.

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem, TransformItem};

use super::shared::punctuate;

const LEVEL: &str = "C1";

/* ── avoir beau, quand bien même ─────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const ABEAU_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Il ____ pleuvoir, le pique-nique est maintenu.", "a beau", "peut bien", "it may rain as much as it likes", "avoir beau + infinitive = no matter how / in vain — one verb, no que."),
    ("J'____ comprendre la théorie, je rate les exercices.", "ai beau", "peux bien", "however hard I try", "j'ai beau comprendre — concession without a conjunction."),
    ("Elle ____ insister, le médecin refuse.", "a beau", "insiste bien", "however much she insists", "elle a beau insister."),
    ("Nous ____ courir, le bus est parti.", "avons beau", "courons vite", "however fast we run", "nous avons beau courir."),
    ("Ils ____ le savoir, ils le nient.", "ont beau", "le savent bien", "they know it full well and still deny it", "avoir beau + infinitive, then the resigned clause."),
    ("____ même tu le dirais, il ne te croira pas.", "Quand bien", "Quand bon", "even if you told him (unreal)", "quand bien même + conditionnel — the extreme concession."),
    ("____ il pleuvrait, le match est maintenu.", "Quand bien même", "Bien que", "even if it rained (unreal)", "quand bien même + conditionnel (pleuvrait); bien que + subjonctif would need a different form."),
];

fn abeau_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in ABEAU_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("abeau:{}:ch", key),
            level: LEVEL,
            topic: "Avoir beau, quand bien même",
            topic_slug: Some("avoir-beau".into()),
            task: vec![format!("{} — choose the concession.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("abeau:{}:cl", key),
            level: LEVEL,
            topic: "Avoir beau, quand bien même",
            topic_slug: Some("avoir-beau".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "avoir beau / quand bien même",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    out
}

/* ── formal connectors ───────────────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const FCONN_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Il n'a pas étudié ; ____ il a raté l'examen.", "par conséquent", "en effet", "a consequence", "par conséquent draws the result; en effet justifies a claim."),
    ("Dépêche-toi, ____ tu vas rater le train.", "sinon", "par conséquent", "otherwise", "sinon = otherwise — short and universal."),
    ("Il connaît le sujet ; ____ il a écrit trois livres dessus.", "en effet", "mais", "a justification", "en effet backs up what was just said."),
    ("Le document est signé ; ____ une copie manque.", "en outre", "en effet", "an additional point", "en outre = besides — adds a fact."),
    ("Il a tout perdu ; ____ il garde le sourire.", "pourtant", "parce que", "a contradiction", "pourtant = yet."),
    ("Le temps presse ; ____, restons calmes.", "or", "hors", "a literary « now »", "or = now, yet (literary connector); hors = outside."),
    ("____ les finances, on tranchera demain.", "Quant à", "Envers", "as for the finances", "quant à = as for; envers = towards (a person)."),
    ("La réunion durera une heure ; ____ le vote suivra.", "ensuite", "après", "the vote comes after", "ensuite — the written sequencing adverb."),
];

fn fconn_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in FCONN_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("fconn:{}:ch", key),
            level: LEVEL,
            topic: "Formal connectors",
            topic_slug: Some("formal-connectors".into()),
            task: vec![format!("{} — choose the connector.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("fconn:{}:cl", key),
            level: LEVEL,
            topic: "Formal connectors",
            topic_slug: Some("formal-connectors".into()),
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

/* ── formal inversion ────────────────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const FINV_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Peut-être ____ demain.", "viendra-t-il", "il viendra", "maybe he'll come tomorrow", "Sentence-initial peut-être forces inversion: peut-être viendra-t-il."),
    ("Sans doute ____ raison.", "a-t-elle", "elle a", "she is (no doubt) right", "sans doute + inversion: a-t-elle — mind the hyphenated -t-."),
    ("Aussi ____-il essentiel de se renseigner.", "est", "sont", "therefore it is essential", "Fronted aussi (= therefore) + inversion: aussi est-il."),
    ("____ les occasions de le revoir.", "Rares sont", "Rare est", "occasions are rare (literary)", "Fronted attribute + inversion, agreeing: rares sont…"),
    ("Telle ____ la réalité de ce métier.", "est", "sont", "such is the reality", "telle est… — a fixed literary inversion."),
    ("À peine ____ fini que le téléphone sonna.", "eut-il", "il eut", "he had barely finished", "À peine + inversion + passé antérieur: à peine eut-il fini que…"),
];

fn finv_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in FINV_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("finv:{}:ch", key),
            level: LEVEL,
            topic: "Formal inversion",
            topic_slug: Some("formal-inversion".into()),
            task: vec![format!("{} — mind the inversion.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("finv:{}:cl", key),
            level: LEVEL,
            topic: "Formal inversion",
            topic_slug: Some("formal-inversion".into()),
            task: vec![format!("{} — complete with the inverted form.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "verb",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Error-spots: the uninverted subject slips in.
    for (words, wrong_i, fix, rule) in [
        (
            vec!["Peut-être", "il", "viendra", "demain"],
            1usize,
            "viendra-t-il",
            "peut-être at the front → subject-verb inversion with the hyphen.",
        ),
        (
            vec!["Sans", "doute", "elle", "a", "raison"],
            2usize,
            "a-t-elle",
            "sans doute at the front → sans doute a-t-elle raison.",
        ),
    ] {
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("finv:{}:es", super::shared::slugify(fix)),
            level: LEVEL,
            topic: "Formal inversion",
            topic_slug: Some("formal-inversion".into()),
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

/* ── expletive ne ────────────────────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule) — the verb stays
/// subjonctif/indicatif; the ne is pure style.
const EXNE_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Je crains qu'il ne ____ en retard.", "vienne", "vient", "fear, with the expletive ne", "craindre que + (ne) + subjonctif: ne vienne — the ne is not a negation."),
    ("Dépêche-toi, de peur que le magasin ne ____ fermé.", "soit", "est", "for fear it is closed", "de peur que + (ne) + subjonctif."),
    ("Avant qu'il ne ____ trop tard, appelons-la.", "soit", "est", "before it is too late", "avant que + (ne) + subjonctif."),
    ("Il travaille plus qu'on ne le ____.", "croit", "croie", "more than one believes", "After a comparative (plus… que), the ne rides with an indicatif verb: ne le croit."),
    ("Elle est moins sévère qu'on ne le ____.", "dit", "dise", "less strict than people say", "moins… que → ne explétif + indicatif (dit)."),
    ("Loin qu'il ____ naïf, il calcule tout.", "soit", "est", "far from being naive", "loin que + subjonctif, often with ne: loin qu'il ne soit."),
    ("Peu s'en ____ que le lac ne déborde.", "faut", "fait", "it almost happened (literary)", "peu s'en faut que + ne + subjonctif — a literary near-miss."),
];

fn exne_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in EXNE_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("exne:{}:ch", key),
            level: LEVEL,
            topic: "Expletive ne",
            topic_slug: Some("expletive-ne".into()),
            task: vec![format!("{} — the ne changes nothing about the mood.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Cloze: type the verb WITH the expletive ne.
    for (fr, good, cue, rule) in [
        ("Je crains qu'il ____ en retard.", "ne vienne", "fear", "craindre que → ne vienne (subjonctif with the expletive ne)."),
        ("Dépêche-toi, de peur que le magasin ____ fermé.", "ne soit", "for fear", "de peur que → ne soit."),
        ("Il travaille plus qu'on le ____.", "ne croit", "than one believes", "plus… que → on ne le croit — the expletive ne rides with the indicatif."),
    ] {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("exne:{}:cl", key),
            level: LEVEL,
            topic: "Expletive ne",
            topic_slug: Some("expletive-ne".into()),
            task: vec![format!("{} — keep the expletive ne.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "verb",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    out
}

/* ── impersonal formal style ─────────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const IMPF_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("____ de vérifier chaque détail avant l'envoi.", "Il convient", "Ça convient", "it is advisable", "il convient de + infinitive — the formal recommendation; « ça » is out of place in writing."),
    ("____ que chacun signe le registre.", "Il importe", "Ça importe", "it matters that everyone signs", "il importe que + subjonctif — formal necessity."),
    ("____ d'un malentendu, rien de grave.", "Il s'agit", "Il est", "it is a matter of", "il s'agit de = it is about / a matter of."),
    ("____ de trancher en dernière instance.", "Il appartient au juge", "C'est le juge qui appartient", "it falls to the judge", "il appartient à X de + infinitive — formal duty."),
    ("____ que les délais se raccourcissent.", "Il est à noter", "Il est noté", "worth noting", "il est à noter que — the « à + infinitive » nuance: yet to be noted."),
    ("____ vous de commencer quand vous voulez.", "Il ne tient qu'à", "Il tient à", "it's entirely up to you", "il ne tient qu'à vous de… — the negative is part of the formula."),
    ("____ que tout soit prêt avant midi.", "Il va de soi", "Ça va de soi que", "it goes without saying", "il va de soi que — the written impersonal; « ça va de soi » is spoken."),
    ("____ à chacun de respecter le silence des lieux.", "Il incombe", "Ça incombe", "it is everyone's formal duty", "il incombe à X de + infinitive — formal obligation."),
];

fn impf_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in IMPF_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("impf:{}:ch", key),
            level: LEVEL,
            topic: "Impersonal formal style",
            topic_slug: Some("impersonal-formal".into()),
            task: vec![format!("{} — keep it formal.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("impf:{}:cl", key),
            level: LEVEL,
            topic: "Impersonal formal style",
            topic_slug: Some("impersonal-formal".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "impersonal",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    out
}

/* ── spoken vs written French ────────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const SPW_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("____ est difficile de trancher.", "Cela", "Ça", "a written essay", "In writing, prefer cela to the spoken ça."),
    ("____ sommes arrivés avant la tempête.", "Nous", "On", "a written report", "Written French keeps nous; on is the spoken default."),
    ("____ existe plusieurs solutions à ce problème.", "Il", "On", "a formal analysis", "il existe = there exist (formal); il y a is everyday."),
    ("C'est cher, ____.", "ça", "cela", "a casual chat", "In speech ça is natural; cela sounds stiff."),
    ("____ un problème technique avec le serveur.", "Il y a", "Il existe", "everyday speech", "Il y a — the neutral everyday form."),
    ("____ dit, nous reportons la réunion.", "Cela", "Ça", "a formal email", "cela dit = that said — the written transition."),
    ("____ faut pas exagérer.", "Il ne", "Ça", "a written rule", "il ne faut pas — the written impersonal."),
    ("Nous ____ bien reçu votre dossier.", "avons", "a", "a formal letter", "nous + avons — written register keeps the full forms."),
];

fn spw_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in SPW_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("spw:{}:ch", key),
            level: LEVEL,
            topic: "Spoken vs written French",
            topic_slug: Some("spoken-vs-written".into()),
            task: vec![format!("{} — pick the right register.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("spw:{}:cl", key),
            level: LEVEL,
            topic: "Spoken vs written French",
            topic_slug: Some("spoken-vs-written".into()),
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
        ("Y a une réunion à dix heures.", "Il y a une réunion à dix heures."),
        ("Je te tiens au courant vite.", "Je vous tiendrai informé dans les meilleurs délais."),
    ] {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("spw:{}:tf", super::shared::slugify(&spoken.chars().take(20).collect::<String>())),
            level: LEVEL,
            topic: "Spoken vs written French",
            topic_slug: Some("spoken-vs-written".into()),
            task: vec!["Rewrite it in written register.".into()],
            source: spoken.into(),
            answer: written.into(),
            rule: vec![
                written.to_string(),
                "on → nous, dropped ne restored, ça → cela, vague nouns sharpened.".into(),
            ],
        }));
    }
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "avoir-beau" => abeau_items(),
        "formal-connectors" => fconn_items(),
        "formal-inversion" => finv_items(),
        "expletive-ne" => exne_items(),
        "impersonal-formal" => impf_items(),
        "spoken-vs-written" => spw_items(),
        _ => Vec::new(),
    }
}
