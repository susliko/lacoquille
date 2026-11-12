//! Family: the present tense — one engine, four topics. A verb table
//! (infinitive, six present forms, complement) drives cloze items per
//! subject, and error-spots catch the classic person-confusion traps.
//! Clusters: être/avoir, regular -er verbs, common irregulars, and
//! pronominal verbs (reflexive pronoun computed, elision included).

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem};

use super::shared::{SUBJ_CAP, cap, punctuate, slugify, starts_vowel};

const LEVEL: &str = "A1";

struct PVerb {
    inf: &'static str,
    /// present forms for je, tu, il/elle, nous, vous, ils/elles
    forms: [&'static str; 6],
    /// complement — must be invariable (no agreement with the subject)
    complement: &'static str,
}

/* ── être & avoir (complements kept invariable) ─────────────────── */

const EA_VERBS: &[PVerb] = &[
    PVerb { inf: "être", forms: ["suis", "es", "est", "sommes", "êtes", "sont"], complement: "ici" },
    PVerb { inf: "être", forms: ["suis", "es", "est", "sommes", "êtes", "sont"], complement: "en retard" },
    PVerb { inf: "être", forms: ["suis", "es", "est", "sommes", "êtes", "sont"], complement: "en avance" },
    PVerb { inf: "être", forms: ["suis", "es", "est", "sommes", "êtes", "sont"], complement: "dehors" },
    PVerb { inf: "avoir", forms: ["ai", "as", "a", "avons", "avez", "ont"], complement: "un frère" },
    PVerb { inf: "avoir", forms: ["ai", "as", "a", "avons", "avez", "ont"], complement: "une voiture" },
    PVerb { inf: "avoir", forms: ["ai", "as", "a", "avons", "avez", "ont"], complement: "le temps" },
    PVerb { inf: "avoir", forms: ["ai", "as", "a", "avons", "avez", "ont"], complement: "des amis ici" },
];

/* ── regular -er verbs (explicit forms — manger keeps its e) ────── */

const ER_VERBS: &[PVerb] = &[
    PVerb { inf: "parler", forms: ["parle", "parles", "parle", "parlons", "parlez", "parlent"], complement: "anglais" },
    PVerb { inf: "manger", forms: ["mange", "manges", "mange", "mangeons", "mangez", "mangent"], complement: "une pomme" },
    PVerb { inf: "habiter", forms: ["habite", "habites", "habite", "habitons", "habitez", "habitent"], complement: "à Paris" },
    PVerb { inf: "travailler", forms: ["travaille", "travailles", "travaille", "travaillons", "travaillez", "travaillent"], complement: "le samedi" },
    PVerb { inf: "jouer", forms: ["joue", "joues", "joue", "jouons", "jouez", "jouent"], complement: "au tennis" },
    PVerb { inf: "regarder", forms: ["regarde", "regardes", "regarde", "regardons", "regardez", "regardent"], complement: "la télévision" },
    PVerb { inf: "écouter", forms: ["écoute", "écoutes", "écoute", "écoutons", "écoutez", "écoutent"], complement: "de la musique" },
    PVerb { inf: "aimer", forms: ["aime", "aimes", "aime", "aimons", "aimez", "aiment"], complement: "le chocolat" },
    PVerb { inf: "penser", forms: ["pense", "penses", "pense", "pensons", "pensez", "pensent"], complement: "aux vacances" },
    PVerb { inf: "donner", forms: ["donne", "donnes", "donne", "donnons", "donnez", "donnent"], complement: "un conseil" },
];

/* ── common irregular verbs ─────────────────────────────────────── */

const IRR_VERBS: &[PVerb] = &[
    PVerb { inf: "aller", forms: ["vais", "vas", "va", "allons", "allez", "vont"], complement: "au cinéma" },
    PVerb { inf: "faire", forms: ["fais", "fais", "fait", "faisons", "faites", "font"], complement: "du sport" },
    PVerb { inf: "venir", forms: ["viens", "viens", "vient", "venons", "venez", "viennent"], complement: "à midi" },
    PVerb { inf: "prendre", forms: ["prends", "prends", "prend", "prenons", "prenez", "prennent"], complement: "le bus" },
    PVerb { inf: "dire", forms: ["dis", "dis", "dit", "disons", "dites", "disent"], complement: "bonjour" },
    PVerb { inf: "savoir", forms: ["sais", "sais", "sait", "savons", "savez", "savent"], complement: "la réponse" },
    PVerb { inf: "voir", forms: ["vois", "vois", "voit", "voyons", "voyez", "voient"], complement: "la mer" },
    PVerb { inf: "sortir", forms: ["sors", "sors", "sort", "sortons", "sortez", "sortent"], complement: "ce soir" },
    PVerb { inf: "comprendre", forms: ["comprends", "comprends", "comprend", "comprenons", "comprenez", "comprennent"], complement: "la question" },
    PVerb { inf: "lire", forms: ["lis", "lis", "lit", "lisons", "lisez", "lisent"], complement: "le journal" },
];

/* ── pronominal verbs (forms are bare stems, no reflexive pronoun) ─ */

const PRON_VERBS: &[PVerb] = &[
    PVerb { inf: "se lever", forms: ["lève", "lèves", "lève", "levons", "levez", "lèvent"], complement: "à sept heures" },
    PVerb { inf: "se réveiller", forms: ["réveille", "réveilles", "réveille", "réveillons", "réveillez", "réveillent"], complement: "tôt" },
    PVerb { inf: "s'appeler", forms: ["appelle", "appelles", "appelle", "appelons", "appelez", "appellent"], complement: "Marie" },
    PVerb { inf: "se coucher", forms: ["couche", "couches", "couche", "couchons", "couchez", "couchent"], complement: "avant minuit" },
    PVerb { inf: "se laver", forms: ["lave", "laves", "lave", "lavons", "lavez", "lavent"], complement: "les mains" },
    PVerb { inf: "se promener", forms: ["promène", "promènes", "promène", "promenons", "promenez", "promènent"], complement: "au parc" },
];

/// Reflexive pronoun chunk for a subject, elided before a vowel
/// (m'appelle, t'arrêtes…).
fn reflexive(subj: &str, form: &str) -> &'static str {
    let elide = starts_vowel(form) || form.starts_with('h');
    match subj {
        "je" => if elide { "m'" } else { "me" },
        "tu" => if elide { "t'" } else { "te" },
        "il" | "elle" => if elide { "s'" } else { "se" },
        "nous" => "nous",
        "vous" => "vous",
        _ => if elide { "s'" } else { "se" },
    }
}

/// Affirmative subject display: J'habite / Je pense / Elle.
fn subj_aff<'a>(subj: &'a str, subj_c: &'a str, form: &'a str) -> &'a str {
    if subj == "je" && (starts_vowel(form) || form.starts_with('h')) {
        "J'"
    } else {
        subj_c
    }
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "present-etre-avoir" => ea_items(),
        "present-er-verbs" => er_items(),
        "present-irregulars" => irr_items(),
        "present-pronominal" => pron_items(),
        _ => Vec::new(),
    }
}

fn ea_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in EA_VERBS {
        for &(subj, subj_c, fi) in SUBJ_CAP {
            let form = v.forms[fi];
            let head = subj_aff(subj, subj_c, form);
            out.push(ExerciseItem::Cloze(ClozeItem {
                id: format!("pea:{}:{}:{}:cl", v.inf, subj, slugify(v.complement)),
                level: LEVEL,
                topic: "Present: être & avoir",
                topic_slug: Some("present-etre-avoir".into()),
                task: vec!["Put the verb in the present.".into()],
                head: format!("{} ", head),
                tail: format!(" {}.", v.complement),
                infinitive: v.inf,
                answer: form.to_string(),
                rule: vec![
                    format!("{} {} {}.", head, form, v.complement),
                    if v.inf == "être" {
                        "être: je suis, tu es, il est, nous sommes, vous êtes, ils sont.".into()
                    } else {
                        "avoir: j'ai, tu as, il a, nous avons, vous avez, ils ont.".into()
                    },
                ],
            }));

            // Error-spot: a form from the wrong person.
            let wrong = if fi == 0 { v.forms[2] } else { v.forms[0] };
            if wrong != form {
                let mut words: Vec<String> = vec![head.to_string()];
                words.push(wrong.to_string());
                words.push(v.complement.to_string());
                let words = punctuate(words);
                out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
                    id: format!("pea:{}:{}:{}:es", v.inf, subj, slugify(v.complement)),
                    level: LEVEL,
                    topic: "Present: être & avoir",
                    topic_slug: Some("present-etre-avoir".into()),
                    task: vec!["One word is wrong. Tap it.".into()],
                    words,
                    wrong: 1,
                    fix: form.to_string(),
                    fix_task: "Now write the correct form.",
                    rule: vec![
                        format!("With « {} », {} takes {}.", subj_aff(subj, subj_c, form), v.inf, form),
                        format!("{} {} {}.", head, form, v.complement),
                    ],
                }));
            }
        }
    }
    out
}

/// Cloze + error-spot for a plain (non-pronominal) verb table.
fn conjugation_items(
    verbs: &[PVerb],
    id_prefix: &str,
    title: &'static str,
    slug: &'static str,
    rule_note: &'static str,
) -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in verbs {
        for &(subj, subj_c, fi) in SUBJ_CAP {
            let form = v.forms[fi];
            let head = subj_aff(subj, subj_c, form);
            out.push(ExerciseItem::Cloze(ClozeItem {
                id: format!("{}:{}:{}:cl", id_prefix, v.inf, subj),
                level: LEVEL,
                topic: title,
                topic_slug: Some(slug.into()),
                task: vec!["Put the verb in the present.".into()],
                head: format!("{} ", head),
                tail: format!(" {}.", v.complement),
                infinitive: v.inf,
                answer: form.to_string(),
                rule: vec![format!("{} {} {}.", head, form, v.complement), rule_note.into()],
            }));

            // Error-spot only where the wrong form is audible/visible:
            // skip je (its form differs everywhere but reads naturally
            // only for tu/nous/vous/ils/elles mismatches).
            let audible = matches!(subj, "tu" | "nous" | "vous" | "ils" | "elles");
            let wrong = v.forms[2]; // the il form as the trap
            if audible && wrong != form {
                let mut words: Vec<String> = vec![cap(subj).to_string()];
                words.push(wrong.to_string());
                words.push(v.complement.to_string());
                let words = punctuate(words);
                out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
                    id: format!("{}:{}:{}:es", id_prefix, v.inf, subj),
                    level: LEVEL,
                    topic: title,
                    topic_slug: Some(slug.into()),
                    task: vec!["One word is wrong. Tap it.".into()],
                    words,
                    wrong: 1,
                    fix: form.to_string(),
                    fix_task: "Now write the correct form.",
                    rule: vec![format!("{} with « {} » → {}.", v.inf, subj, form)],
                }));
            }
        }
    }
    out
}

fn er_items() -> Vec<ExerciseItem> {
    conjugation_items(
        ER_VERBS,
        "per",
        "Present: -er verbs",
        "present-er-verbs",
        "Drop -er, add -e, -es, -e, -ons, -ez, -ent.",
    )
}

fn irr_items() -> Vec<ExerciseItem> {
    conjugation_items(
        IRR_VERBS,
        "pirr",
        "Present: common irregular verbs",
        "present-irregulars",
        "Irregular verbs: learn the forms by heart.",
    )
}

fn pron_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in PRON_VERBS {
        for &(subj, _subj_c, fi) in SUBJ_CAP {
            let form = v.forms[fi];
            let refl = reflexive(subj, form);
            let chunk = if refl.ends_with('\'') {
                format!("{}{}", refl, form)
            } else {
                format!("{} {}", refl, form)
            };
            let head_word = if subj == "je" { "Je".to_string() } else { cap(subj) };

            out.push(ExerciseItem::Cloze(ClozeItem {
                id: format!("ppr:{}:{}:cl", v.inf, subj),
                level: LEVEL,
                topic: "Present: pronominal verbs",
                topic_slug: Some("present-pronominal".into()),
                task: vec!["Complete the pronominal verb.".into()],
                head: format!("{} ", head_word),
                tail: format!(" {}.", v.complement),
                infinitive: v.inf,
                answer: chunk.clone(),
                rule: vec![
                    format!("{} — the reflexive pronoun matches the subject.", v.inf),
                    format!("{} {} {}.", head_word, chunk, v.complement),
                ],
            }));

            // Choice: the wrong reflexive pronoun.
            if matches!(subj, "je" | "tu" | "il" | "vous") {
                let wrong_refl = match subj {
                    "il" => "le",
                    _ => "se",
                };
                out.push(ExerciseItem::Choice(ChoiceItem {
                    id: format!("ppr:{}:{}:ch", v.inf, subj),
                    level: LEVEL,
                    topic: "Present: pronominal verbs",
                    topic_slug: Some("present-pronominal".into()),
                    task: vec!["Choose the right form.".into()],
                    sentence: format!("{} ____ {}.", head_word, v.complement),
                    options: vec![chunk.clone(), format!("{} {}", wrong_refl, form)],
                    correct: 0, // shuffled in `generate`
                    rule: vec![
                        format!("{} — {} takes {} {}.", v.inf, subj, refl, form),
                        format!("{} {} {}.", head_word, chunk, v.complement),
                    ],
                }));
            }
        }
    }
    out
}
