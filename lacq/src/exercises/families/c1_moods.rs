//! Family: C1 moods — subtle subjonctif triggers (avant que vs après
//! que, de peur que, peu importe que), the mood that changes meaning
//! (un assistant qui ait / l'assistant qui a), the attenuating
//! conditionnel of hedged speech, controlled infinitive clauses
//! (faire/écouter + infinitive) and literary hypotheticals (qui l'eût
//! cru).

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem, TransformItem};

use super::shared::punctuate;

const LEVEL: &str = "C1";

/* ── subtle subjonctif cases ─────────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const SSUB_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Nous partirons ____ tu sois prêt.", "avant que", "après que", "before you're ready", "avant que + subjonctif (sois); après que takes the indicatif."),
    ("Nous sommes partis ____ tu étais prêt.", "après que", "avant que", "once you were ready", "après que + indicatif (étais) — the classic error is the subjonctif here."),
    ("Il parlait bas ____ personne ne sache ses projets.", "de peur que", "pour que", "for fear anyone would learn", "de peur que + subjonctif — afraid that it happens."),
    ("Peu importe qu'il ____ en retard.", "vienne", "vient", "it doesn't matter", "peu importe que + subjonctif."),
    ("Je le noterai ____ tu l'oublies.", "avant que", "après que", "so it won't be forgotten — before", "avant que + subjonctif (oublies is subjunctive of oublier)."),
    ("Il est rentré ____ le film a commencé.", "après que", "avant que", "once the film had started", "après que + indicatif (a commencé)."),
    ("Il a agi vite ____ personne ne le sache.", "sans que", "pour que", "without anyone knowing", "sans que + subjonctif."),
    ("____ ses excuses soient sincères, le mal est fait.", "Quoique", "Malgré que", "even though her apology is sincere", "quoique / bien que + subjonctif; « malgré que » n'est pas standard."),
];

fn ssub_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in SSUB_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("ssub:{}:ch", key),
            level: LEVEL,
            topic: "Subtle subjonctif cases",
            topic_slug: Some("subjonctif-subtle".into()),
            task: vec![format!("{} — choose the trigger.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("ssub:{}:cl", key),
            level: LEVEL,
            topic: "Subtle subjonctif cases",
            topic_slug: Some("subjonctif-subtle".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "connector",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Verb-form rows inside the subtle frames.
    for (fr, good, wrong, cue, rule) in [
        ("Nous partirons avant que tu ____ prêt.", "sois", "es", "before you're ready", "avant que + subjonctif: que tu sois."),
        ("Parlez bas, de peur que le secret ne ____ .", "sache", "sait", "for fear the secret gets out", "de peur que + subjonctif: ne sache."),
        ("Loin qu'il ____ naïf, il calcule tout.", "soit", "est", "far from being naive", "loin que + subjonctif."),
        ("Autant que je ____ , tout est prévu.", "sache", "sais", "as far as I know", "autant que je sache — a fixed subjonctif idiom."),
    ] {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("ssub:{}:ch", key),
            level: LEVEL,
            topic: "Subtle subjonctif cases",
            topic_slug: Some("subjonctif-subtle".into()),
            task: vec![format!("{} — choose the mood.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    out
}

/* ── subjonctif vs indicatif meaning ─────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule) — the mood decides
/// the meaning of the relative clause.
const SMEAN_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Je cherche un assistant qui ____ des notions de droit.", "ait", "a", "any assistant with some law notions", "An unspecified person → subjonctif (ait)."),
    ("Je cherche l'assistant qui ____ la clé du bureau.", "a", "ait", "the one who (I know he) has the key", "A specific, identified person → indicatif (a)."),
    ("C'est le meilleur film qui ____ sorti cette année.", "soit", "est", "a superlative judgement", "After a superlative judgement, the relative takes the subjonctif (soit)."),
    ("Il n'y a rien qui ____ l'arrêter.", "puisse", "peut", "nothing at all could", "Negative antecedent (rien) → subjonctif (puisse)."),
    ("Il y a quelque chose qui ____ l'arrêter.", "peut", "puisse", "there IS something", "Affirmative antecedent → indicatif (peut)."),
    ("Y a-t-il quelqu'un qui ____ où il vit ?", "sache", "sait", "does anyone at all know", "Interrogative antecedent → subjonctif (sache)."),
    ("Voilà l'homme qui ____ la réponse.", "sait", "sache", "the man who really knows", "Identified antecedent → indicatif (sait)."),
    ("C'est le seul recours qui ____ encore possible.", "soit", "est", "the only one left", "Restrictive superlative (le seul) → subjonctif (soit)."),
    ("Il n'y a personne qui ____ m'aider ce soir.", "puisse", "peut", "nobody at all", "Negative antecedent (personne) → subjonctif (puisse)."),
];

fn smean_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in SMEAN_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("smean:{}:ch", key),
            level: LEVEL,
            topic: "Subjonctif vs indicatif meaning",
            topic_slug: Some("subjonctif-meaning".into()),
            task: vec![format!("{} — the mood changes the meaning.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("smean:{}:cl", key),
            level: LEVEL,
            topic: "Subjonctif vs indicatif meaning",
            topic_slug: Some("subjonctif-meaning".into()),
            task: vec![format!("{} — complete.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "verb",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    out
}

/* ── attenuating conditionnel ────────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule)
const ATTN_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Selon nos informations, l'usine ____ fermée l'an prochain.", "serait", "est", "unverified news", "The conditionnel marks information the speaker does not confirm (le conditionnel journalistique)."),
    ("Il ____ que le projet change de cap.", "semblerait", "semble", "a hedged claim", "Il semblerait que — the speaker distances themselves from the claim."),
    ("____-vous m'indiquer la sortie ?", "Pourriez", "Pouvez", "an extra polite request", "Pourriez-vous — the conditionnel as the height of politeness."),
    ("On ____ croire qu'ils se sont mis d'accord.", "pourrait", "peut", "one might think", "On pourrait croire — hedged by the conditionnel."),
    ("Une telle décision ____ des conséquences graves.", "aurait", "a", "speculation", "The conditionnel speculates: aurait."),
    ("Vous ____ vérifier l'adresse avant d'envoyer.", "devriez", "devez", "soft advice", "Vous devriez — advice, not obligation."),
    ("J'____ une réponse avant vendredi.", "aimerais", "veux", "a polite wish", "J'aimerais — softer than je veux."),
    ("Ce projet ____ être viable, à en juger par les premiers essais.", "semblerait", "semble", "a cautious judgement", "semblerait — the judgement stays cautious."),
    ("Le ministre ____ visiter les lieux jeudi.", "devrait", "doit", "an announcement not yet confirmed", "devrait + infinitive — reported, not confirmed."),
];

fn attn_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in ATTN_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("attn:{}:ch", key),
            level: LEVEL,
            topic: "Attenuating conditionnel",
            topic_slug: Some("conditionnel-attenuation".into()),
            task: vec![format!("{} — soften it.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("attn:{}:cl", key),
            level: LEVEL,
            topic: "Attenuating conditionnel",
            topic_slug: Some("conditionnel-attenuation".into()),
            task: vec![format!("{} — complete with the conditionnel.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "verb",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Transform: blunt → hedged.
    for (plain, hedged) in [
        ("Il faut relire le contrat.", "Il faudrait relire le contrat."),
        ("Il faut réserver tôt.", "Il faudrait réserver tôt."),
        ("Vous devez vérifier l'adresse.", "Vous devriez vérifier l'adresse."),
        ("Je veux un devis détaillé.", "Je voudrais un devis détaillé."),
    ] {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("attn:{}:tf", super::shared::slugify(plain)),
            level: LEVEL,
            topic: "Attenuating conditionnel",
            topic_slug: Some("conditionnel-attenuation".into()),
            task: vec!["Soften it with the conditionnel.".into()],
            source: plain.into(),
            answer: hedged.into(),
            rule: vec![
                hedged.to_string(),
                "il faut → il faudrait, vous devez → vous devriez, je veux → je voudrais.".into(),
            ],
        }));
    }
    out
}

/* ── infinitive clauses ──────────────────────────────────────────── */

/// (sentence with ____, correct infinitive, wrong, cue, rule) —
/// perception and causative verbs keep the second verb infinitive.
const IINF_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Le patron fait ____ les dossiers ce soir.", "signer", "signent", "he has them signed (by someone)", "faire + infinitive: the second verb stays infinitive."),
    ("Elle regarde les enfants ____ dans le jardin.", "jouer", "jouent", "watches them play", "regarder + infinitive: elle regarde les enfants jouer."),
    ("Nous l'entendons ____ sous la douche.", "chanter", "chante", "we hear him sing", "entendre + infinitive."),
    ("Je les ai laissés ____ la photo au mur.", "accrocher", "accroché", "let them hang it", "laisser + infinitive; the photo is the object of accrocher."),
    ("Laisse-la ____ !", "finir", "finira", "let her finish", "laisser + infinitive — no tense change."),
    ("Elle fait ____ sa voiture par un professionnel.", "réparer", "réparé", "she has it repaired", "faire réparer — the causee rides after par."),
    ("On m'a fait ____ l'incident au commissariat.", "raconter", "raconté", "they made me tell it", "faire + infinitive, even in the passé composé."),
    ("Elle se fait ____ par le chef pour sa ponctualité.", "remarquer", "remarquée", "she keeps getting noticed", "se faire + infinitive — a passive-flavoured causative; the verb stays infinitive."),
];

fn iinf_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in IINF_ROWS {
        let key = super::shared::slugify(&fr.chars().take(26).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("iinf:{}:ch", key),
            level: LEVEL,
            topic: "Infinitive clauses",
            topic_slug: Some("infinitive-clauses".into()),
            task: vec![format!("{} — choose the form.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("iinf:{}:cl", key),
            level: LEVEL,
            topic: "Infinitive clauses",
            topic_slug: Some("infinitive-clauses".into()),
            task: vec![format!("{} — complete with the infinitive.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "infinitive",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Transform: two clauses → one controlled-infinitive sentence.
    for (source, answer, rule) in [
        (
            "Elle écoute les enfants. Les enfants chantent.",
            "Elle écoute les enfants chanter.",
            "perception verb + direct object + infinitive",
        ),
        (
            "Le garagiste répare sa voiture.",
            "Elle fait réparer sa voiture par le garagiste.",
            "faire + infinitive + par + agent",
        ),
        (
            "Le professeur signe les dossiers, grâce à son assistant.",
            "Le professeur fait signer les dossiers par son assistant.",
            "faire + infinitive hands the action to someone else",
        ),
    ] {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("iinf:{}:tf", super::shared::slugify(&source.chars().take(20).collect::<String>())),
            level: LEVEL,
            topic: "Infinitive clauses",
            topic_slug: Some("infinitive-clauses".into()),
            task: vec![
                "Join into one sentence with a".into(),
                "controlled infinitive.".into(),
            ],
            source: source.into(),
            answer: answer.into(),
            rule: vec![answer.to_string(), rule.to_string()],
        }));
    }
    out
}

/* ── literary hypotheticals ──────────────────────────────────────── */

/// (sentence with ____, correct, wrong, cue, rule) — conditionnel
/// passé II: il eût fait = il aurait fait, in literary dress.
const LITH_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("Qui l'____ cru ?", "eût", "eut", "the set phrase", "Qui l'eût cru ? = who would have believed it — eût is the literary conditionnel; eut is the passé simple."),
    ("S'il avait pu, il ____ venu nous voir.", "fût", "serait", "literary unreal past (être)", "il fût venu = il serait venu in literary style."),
    ("À sa place, j'____ fait la même chose.", "eusse", "aurais", "literary unreal remark", "j'eusse fait = j'aurais fait."),
    ("Elle ____ tout donné pour lui.", "eût", "aurait", "literary speculation", "elle eût tout donné — the conditionnel passé II."),
    ("Tu l'____ aimé, ce petit village.", "eusse", "aurais", "tu form, literary", "tu eusses aimé — the literary tu of the unreal past."),
    ("Ils ____ été fiers de toi.", "eussent", "auraient", "literary unreal past", "ils eussent été fiers — 3rd plural of the conditionnel passé II."),
];

fn lith_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in LITH_ROWS {
        let key = super::shared::slugify(&fr.chars().take(24).collect::<String>());
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("lith:{}:ch", key),
            level: LEVEL,
            topic: "Literary hypotheticals",
            topic_slug: Some("literary-hypotheticals".into()),
            task: vec![format!("{} — choose the literary form.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("lith:{}:cl", key),
            level: LEVEL,
            topic: "Literary hypotheticals",
            topic_slug: Some("literary-hypotheticals".into()),
            task: vec![format!("{} — type the literary auxiliary.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: "auxiliary",
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Recognition: decode the literary form.
    out.push(ExerciseItem::Choice(ChoiceItem {
        id: "lith:recognition:ch".into(),
        level: LEVEL,
        topic: "Literary hypotheticals",
        topic_slug: Some("literary-hypotheticals".into()),
        task: vec!["« Il eût accepté » équivaut à :".into()],
        sentence: "« Il eût accepté sans hésiter. »".into(),
        options: vec![
            "il aurait accepté sans hésiter".into(),
            "il a accepté sans hésiter".into(),
        ],
        correct: 0, // shuffled in `generate`
        rule: vec![
            "Il eût accepté = il aurait accepté — the conditionnel passé, literary 2nd form.".into(),
            "Read it in novels; write it only when you want the full literary register.".into(),
        ],
    }));
    // Error-spot: the passé simple slips where the conditionnel belongs.
    out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
        id: "lith:eut:es".into(),
        level: LEVEL,
        topic: "Literary hypotheticals",
        topic_slug: Some("literary-hypotheticals".into()),
        task: vec!["One word is wrong. Tap it.".into()],
        words: punctuate(vec!["Qui".into(), "l'eut".into(), "cru".into()]),
        wrong: 1,
        fix: "l'eût".into(),
        fix_task: "Now write the correct form.",
        rule: vec![
            "Qui l'eût cru ? — the accent changes eut (passé simple) into eût (conditionnel).".into(),
            "Without the circumflex the phrase means « who had it believed », nonsense here.".into(),
        ],
    }));
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "subjonctif-subtle" => ssub_items(),
        "subjonctif-meaning" => smean_items(),
        "conditionnel-attenuation" => attn_items(),
        "infinitive-clauses" => iinf_items(),
        "literary-hypotheticals" => lith_items(),
        _ => Vec::new(),
    }
}
