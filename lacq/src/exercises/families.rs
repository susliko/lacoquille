//! Parameterized exercise families — the endless tier. Each family owns
//! tagged word banks and composes items from (bank × bank × bank) slot
//! combinations, so a topic yields far more distinct items than any
//! fixed corpus. Items keep the exact shapes the client's views render.
//!
//! Ids are derived from the slot values (stable across requests), which
//! is what lets the feed avoid immediate repeats via `exclude`.

use std::collections::HashSet;

use super::gen::Rng;
use super::topics::{Family, Topic};
use super::{
    ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem, TransformItem,
};

/* ── Shared banks: être/avoir verbs + subject pronouns ──────────── */

struct EtreVerb {
    inf: &'static str,
    /// Base (masculine singular) participle; agreement is computed.
    p: &'static str,
    complement: &'static str,
}

const ETRE_VERBS: &[EtreVerb] = &[
    EtreVerb { inf: "aller", p: "allé", complement: "au cinéma" },
    EtreVerb { inf: "arriver", p: "arrivé", complement: "en retard" },
    EtreVerb { inf: "partir", p: "parti", complement: "à midi" },
    EtreVerb { inf: "rester", p: "resté", complement: "à la maison" },
    EtreVerb { inf: "sortir", p: "sorti", complement: "avec des amis" },
    EtreVerb { inf: "tomber", p: "tombé", complement: "dans la rue" },
    EtreVerb { inf: "retourner", p: "retourné", complement: "à Paris" },
    EtreVerb { inf: "monter", p: "monté", complement: "au troisième étage" },
];

struct AvoirVerb {
    inf: &'static str,
    p: &'static str,
    complement: &'static str,
}

const AVOIR_VERBS: &[AvoirVerb] = &[
    AvoirVerb { inf: "manger", p: "mangé", complement: "au restaurant" },
    AvoirVerb { inf: "parler", p: "parlé", complement: "au téléphone" },
    AvoirVerb { inf: "regarder", p: "regardé", complement: "la télévision" },
    AvoirVerb { inf: "finir", p: "fini", complement: "le travail" },
    AvoirVerb { inf: "vendre", p: "vendu", complement: "sa voiture" },
    AvoirVerb { inf: "prendre", p: "pris", complement: "le train" },
];

struct Pron {
    p: &'static str,
    feminine: bool,
    plural: bool,
    /// être form for this pronoun.
    etre: &'static str,
    /// avoir form for this pronoun.
    avoir: &'static str,
}

const PRONS: &[Pron] = &[
    Pron { p: "il", feminine: false, plural: false, etre: "est", avoir: "a" },
    Pron { p: "elle", feminine: true, plural: false, etre: "est", avoir: "a" },
    Pron { p: "nous", feminine: false, plural: true, etre: "sommes", avoir: "avons" },
    Pron { p: "vous", feminine: false, plural: true, etre: "êtes", avoir: "avez" },
    Pron { p: "ils", feminine: false, plural: true, etre: "sont", avoir: "ont" },
    Pron { p: "elles", feminine: true, plural: true, etre: "sont", avoir: "ont" },
];

impl Pron {
    fn participle(&self, base: &str) -> String {
        let mut p = base.to_string();
        if self.feminine {
            p.push('e');
        }
        if self.plural {
            p.push('s');
        }
        p
    }
    fn agree_note(&self) -> &'static str {
        match (self.feminine, self.plural) {
            (false, false) => "no agreement mark",
            (true, false) => "add -e",
            (false, true) => "add -s",
            (true, true) => "add -es",
        }
    }
}

fn cap(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// Sentence words with the final word carrying the period.
fn punctuate(words: Vec<String>) -> Vec<String> {
    let mut w = words;
    if let Some(last) = w.last_mut() {
        last.push('.');
    }
    w
}

/* ── Family: passé composé with être (cloze + error-spot) ───────── */

const PC_ETRE_LEVEL: &str = "A2";

fn pc_etre_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in ETRE_VERBS {
        for pr in PRONS {
            let agreed = pr.participle(v.p);
            out.push(ExerciseItem::Cloze(ClozeItem {
                id: format!("pc-etre:{}:{}:cloze", v.inf, pr.p),
                level: PC_ETRE_LEVEL,
                topic: "Passé composé with être",
                topic_slug: Some("pc-etre".into()),
                task: vec!["Put the verb in the passé composé.".into()],
                head: format!("Hier soir, {} ", pr.p),
                tail: format!(" {}.", v.complement),
                infinitive: v.inf,
                answer: format!("{} {}", pr.etre, agreed),
                rule: vec![
                    "With être, the participle agrees with the".into(),
                    format!("subject: {} → {} ({}).", pr.p, agreed, pr.agree_note()),
                ],
            }));

            // Agreement error is only visible when the participle changes.
            if agreed != v.p {
                let mut words = vec![
                    cap(pr.p),
                    pr.etre.to_string(),
                    v.p.to_string(),
                ];
                words.extend(v.complement.split(' ').map(|w| w.to_string()));
                let words = punctuate(words);
                out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
                    id: format!("pc-etre:{}:{}:es", v.inf, pr.p),
                    level: PC_ETRE_LEVEL,
                    topic: "Passé composé with être",
                    topic_slug: Some("pc-etre".into()),
                    task: vec!["One word is wrong. Tap it.".into()],
                    words,
                    wrong: 2,
                    fix: agreed.clone(),
                    fix_task: "Now write the correct form.",
                    rule: vec![
                        "With être, the participle agrees with the".into(),
                        format!("subject: {} → {}.", pr.p, agreed),
                    ],
                }));
            }
        }
    }
    out
}

/* ── Family: auxiliary choice (choice + error-spot) ─────────────── */

const PC_AUX_LEVEL: &str = "A2";

fn pc_aux_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    // Être verbs: the trap is avoir.
    for v in ETRE_VERBS {
        for pr in PRONS {
            let agreed = pr.participle(v.p);
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("pc-aux:{}:{}:choice", v.inf, pr.p),
                level: PC_AUX_LEVEL,
                topic: "Auxiliary choice",
                topic_slug: Some("pc-aux".into()),
                task: vec!["Choose the right form.".into()],
                sentence: format!("Hier, {} ____ {}.", pr.p, v.complement),
                options: vec![
                    format!("{} {}", pr.etre, agreed),
                    format!("{} {}", pr.avoir, agreed),
                ],
                correct: 0, // shuffled in `generate`
                rule: vec![
                    format!("{} takes être in the passé composé.", cap(v.inf)),
                    "Arrival and movement verbs conjugate with être, not avoir.".into(),
                ],
            }));

            let mut words = vec![cap(pr.p), pr.avoir.to_string(), agreed.clone()];
            words.extend(v.complement.split(' ').map(|w| w.to_string()));
            let words = punctuate(words);
            out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
                id: format!("pc-aux:{}:{}:es", v.inf, pr.p),
                level: PC_AUX_LEVEL,
                topic: "Auxiliary choice",
                topic_slug: Some("pc-aux".into()),
                task: vec!["One word is wrong. Tap it.".into()],
                words,
                wrong: 1,
                fix: pr.etre.to_string(),
                fix_task: "Now write the correct form.",
                rule: vec![
                    format!("{} takes être: {} {} {}.", v.inf, pr.p, pr.etre, agreed),
                    "Its passé composé never uses avoir.".into(),
                ],
            }));
        }
    }
    // Avoir verbs: the trap is être.
    for v in AVOIR_VERBS {
        for pr in PRONS {
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("pc-aux:{}:{}:choice", v.inf, pr.p),
                level: PC_AUX_LEVEL,
                topic: "Auxiliary choice",
                topic_slug: Some("pc-aux".into()),
                task: vec!["Choose the right form.".into()],
                sentence: format!("Hier, {} ____ {}.", pr.p, v.complement),
                options: vec![
                    format!("{} {}", pr.avoir, v.p),
                    format!("{} {}", pr.etre, v.p),
                ],
                correct: 0,
                rule: vec![
                    format!("{} takes avoir in the passé composé.", cap(v.inf)),
                    "Most verbs form the passé composé with avoir.".into(),
                ],
            }));
        }
    }
    out
}

/* ── Family: object pronouns (transform + tiles) ────────────────── */

struct CodNoun {
    np: &'static str,
    pron: &'static str,
    feminine: bool,
    plural: bool,
}

const COD_NOUNS: &[CodNoun] = &[
    CodNoun { np: "le livre", pron: "le", feminine: false, plural: false },
    CodNoun { np: "le journal", pron: "le", feminine: false, plural: false },
    CodNoun { np: "le pain", pron: "le", feminine: false, plural: false },
    CodNoun { np: "la lettre", pron: "la", feminine: true, plural: false },
    CodNoun { np: "la voiture", pron: "la", feminine: true, plural: false },
    CodNoun { np: "les fleurs", pron: "les", feminine: true, plural: true },
    CodNoun { np: "les clés", pron: "les", feminine: true, plural: true },
];

const COD_VERBS: &[&str] = &["voir", "lire", "acheter", "connaître", "prendre", "finir"];

fn cod_participle(inf: &str) -> &'static str {
    match inf {
        "voir" => "vu",
        "lire" => "lu",
        "acheter" => "acheté",
        "connaître" => "connu",
        "prendre" => "pris",
        _ => "fini",
    }
}

struct CoiNoun {
    np: &'static str,
    pron: &'static str,
}

const COI_NOUNS: &[CoiNoun] = &[
    CoiNoun { np: "à Marie", pron: "lui" },
    CoiNoun { np: "à sa sœur", pron: "lui" },
    CoiNoun { np: "à son voisin", pron: "lui" },
    CoiNoun { np: "à ses parents", pron: "leur" },
    CoiNoun { np: "à ses collègues", pron: "leur" },
];

struct CoiVerb {
    inf: &'static str,
    /// present forms for je, tu, il/elle, nous, vous, ils/elles
    forms: [&'static str; 6],
}

const COI_VERBS: &[CoiVerb] = &[
    CoiVerb { inf: "téléphoner", forms: ["téléphone", "téléphones", "téléphone", "téléphonons", "téléphonez", "téléphonent"] },
    CoiVerb { inf: "écrire", forms: ["écris", "écris", "écrit", "écrivons", "écrivez", "écrivent"] },
    CoiVerb { inf: "parler", forms: ["parle", "parles", "parle", "parlons", "parlez", "parlent"] },
    CoiVerb { inf: "demander", forms: ["demande", "demandes", "demande", "demandons", "demandez", "demandent"] },
    CoiVerb { inf: "répondre", forms: ["réponds", "réponds", "répond", "répondons", "répondez", "répondent"] },
];

/// (subject, capitalized, form index for present-tense banks)
const SUBJ_CAP: &[(&str, &str, usize)] = &[
    ("je", "Je", 0),
    ("tu", "Tu", 1),
    ("il", "Il", 2),
    ("elle", "Elle", 2),
    ("nous", "Nous", 3),
    ("vous", "Vous", 4),
    ("ils", "Ils", 5),
    ("elles", "Elles", 5),
];

fn avoir_pc(subj: &str) -> &'static str {
    match subj {
        "je" => "ai",
        "tu" => "as",
        "il" | "elle" => "a",
        "nous" => "avons",
        "vous" => "avez",
        _ => "ont",
    }
}

fn gender_number(n: &CodNoun) -> &'static str {
    match (n.feminine, n.plural) {
        (false, false) => "masculine singular",
        (true, false) => "feminine singular",
        (false, true) => "masculine plural",
        (true, true) => "feminine plural",
    }
}

fn slugify(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .replace("--", "-")
        .trim_matches('-')
        .to_string()
}

fn pronouns_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    // COD transforms (passé composé).
    for n in COD_NOUNS {
        for inf in COD_VERBS {
            for (subj, subj_c, _) in SUBJ_CAP {
                let base = cod_participle(inf);
                let mut agreed = base.to_string();
                if n.feminine {
                    agreed.push('e');
                }
                if n.plural {
                    agreed.push('s');
                }
                let level = if agreed == base { "A2" } else { "B1" };
                let av = avoir_pc(subj);
                let source = if *subj == "je" {
                    format!("J'{} {} {}.", av, base, n.np)
                } else {
                    format!("{} {} {} {}.", subj_c, av, base, n.np)
                };
                let pron_chunk = if n.pron == "les" {
                    format!("les {}", av)
                } else {
                    format!("l'{}", av)
                };
                let answer = if *subj == "je" {
                    format!("Je {} {}", pron_chunk, agreed)
                } else {
                    format!("{} {} {}", subj_c, pron_chunk, agreed)
                };
                out.push(ExerciseItem::Transform(TransformItem {
                    id: format!("pron-cod:{}:{}:{}", inf, slugify(n.np), subj),
                    level,
                    topic: "Object pronouns",
                    topic_slug: Some("pronouns-cod-coi".into()),
                    task: vec![
                        "Rewrite it, replacing the object".into(),
                        "with a pronoun.".into(),
                    ],
                    source,
                    answer,
                    rule: vec![
                        format!("{} is {} → {}.", cap(n.np), gender_number(n), n.pron),
                        "The pronoun goes before avoir.".into(),
                    ],
                }));
            }
        }
    }
    // COI transforms (present).
    for n in COI_NOUNS {
        for v in COI_VERBS {
            for (subj, subj_c, fi) in SUBJ_CAP {
                let form = v.forms[*fi];
                let source = format!("{} {} {}.", subj_c, form, n.np);
                let answer = format!("{} {} {}", subj_c, n.pron, form);
                out.push(ExerciseItem::Transform(TransformItem {
                    id: format!("pron-coi:{}:{}:{}", v.inf, slugify(n.np), subj),
                    level: "A2",
                    topic: "Object pronouns",
                    topic_slug: Some("pronouns-cod-coi".into()),
                    task: vec![
                        "Rewrite it, replacing the object".into(),
                        "with a pronoun.".into(),
                    ],
                    source,
                    answer,
                    rule: vec![
                        "À + a person becomes lui (singular) or leur".into(),
                        "(plural), placed before the conjugated verb.".into(),
                    ],
                }));
            }
        }
    }
    out
}

/* ── Family: subjonctif (cloze + error-spot) ────────────────────── */

struct Trigger {
    expr: &'static str,
    kind: &'static str,
}

const TRIGGERS: &[Trigger] = &[
    Trigger { expr: "Je suis content que", kind: "a feeling" },
    Trigger { expr: "Nous sommes désolés que", kind: "a feeling" },
    Trigger { expr: "Elle a peur que", kind: "a feeling" },
    Trigger { expr: "C'est dommage que", kind: "a feeling" },
    Trigger { expr: "Je doute que", kind: "doubt" },
    Trigger { expr: "Je ne pense pas que", kind: "doubt" },
    Trigger { expr: "Il est possible que", kind: "doubt" },
    Trigger { expr: "Il faut que", kind: "necessity" },
];

struct SubjVerb {
    inf: &'static str,
    /// (subjunctive, indicative) for tu, il/elle, nous, vous, ils/elles
    forms: [(&'static str, &'static str); 5],
    /// Complement used in error-spot sentences (with final period).
    es_complement: &'static str,
}

const SUBJ_VERBS: &[SubjVerb] = &[
    SubjVerb {
        inf: "avoir",
        forms: [("aies", "as"), ("ait", "a"), ("ayons", "avons"), ("ayez", "avez"), ("aient", "ont")],
        es_complement: "raison.",
    },
    SubjVerb {
        inf: "être",
        forms: [("sois", "es"), ("soit", "est"), ("soyons", "sommes"), ("soyez", "êtes"), ("soient", "sont")],
        es_complement: "malade.",
    },
    SubjVerb {
        inf: "venir",
        forms: [("viennes", "viens"), ("vienne", "vient"), ("venions", "venons"), ("veniez", "venez"), ("viennent", "viennent")],
        es_complement: "demain.",
    },
    SubjVerb {
        inf: "faire",
        forms: [("fasses", "fais"), ("fasse", "fait"), ("fassions", "faisons"), ("fassiez", "faites"), ("fassent", "font")],
        es_complement: "ses devoirs.",
    },
    SubjVerb {
        inf: "aller",
        forms: [("ailles", "vas"), ("aille", "va"), ("allions", "allons"), ("alliez", "allez"), ("aillent", "vont")],
        es_complement: "au cinéma.",
    },
];

const SUBJ_PRONS: &[&str] = &["tu", "il", "nous", "vous", "ils"];

fn subjonctif_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for t in TRIGGERS {
        for v in SUBJ_VERBS {
            for (pi, pr) in SUBJ_PRONS.iter().enumerate() {
                let (subj_form, indic_form) = v.forms[pi];
                let trig_slug = slugify(&t.expr.to_lowercase());
                out.push(ExerciseItem::Cloze(ClozeItem {
                    id: format!("subj:{}:{}:{}:cloze", trig_slug, v.inf, pr),
                    level: "B1",
                    topic: "Subjonctif after feelings and doubt",
                    topic_slug: Some("subjonctif".into()),
                    task: vec!["Put the verb in the right form.".into()],
                    head: format!("{} {} ", t.expr, pr),
                    tail: ".".into(),
                    infinitive: v.inf,
                    answer: subj_form.to_string(),
                    rule: vec![
                        format!("{} expresses {} — the clause that follows takes the subjonctif.", t.expr, t.kind),
                    ],
                }));
                // Error-spot: indicative after the trigger.
                let mut words: Vec<String> =
                    t.expr.split(' ').map(|w| w.to_string()).collect();
                words.push(pr.to_string());
                words.push(indic_form.to_string());
                let comp: Vec<String> =
                    v.es_complement.split(' ').map(|w| w.to_string()).collect();
                let wrong = words.len() - 1;
                words.extend(comp);
                out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
                    id: format!("subj:{}:{}:{}:es", trig_slug, v.inf, pr),
                    level: "B1",
                    topic: "Subjonctif after feelings and doubt",
                    topic_slug: Some("subjonctif".into()),
                    task: vec!["One word is wrong. Tap it.".into()],
                    words,
                    wrong,
                    fix: subj_form.to_string(),
                    fix_task: "Now write the correct form.",
                    rule: vec![
                        format!("{} expresses {} — the clause takes", t.expr, t.kind),
                        format!("the subjonctif: qu'{} {}.", pr, subj_form),
                    ],
                }));
            }
        }
    }
    out
}

/* ── Family: imparfait vs passé composé (choice) ────────────────── */

struct ImpPcVerb {
    inf: &'static str,
    /// (imparfait, passé composé) for il, nous, elles
    il: (&'static str, &'static str),
    nous: (&'static str, &'static str),
    elles: (&'static str, &'static str),
    complement: &'static str,
    il_only: bool,
}

const IMPPC_VERBS: &[ImpPcVerb] = &[
    ImpPcVerb {
        inf: "aller",
        il: ("allait", "est allé"),
        nous: ("allions", "sommes allés"),
        elles: ("allaient", "sont allées"),
        complement: "à la plage",
        il_only: false,
    },
    ImpPcVerb {
        inf: "dormir",
        il: ("dormait", "a dormi"),
        nous: ("dormions", "avons dormi"),
        elles: ("dormaient", "ont dormi"),
        complement: "toute la journée",
        il_only: false,
    },
    ImpPcVerb {
        inf: "parler",
        il: ("parlait", "a parlé"),
        nous: ("parlions", "avons parlé"),
        elles: ("parlaient", "ont parlé"),
        complement: "au téléphone",
        il_only: false,
    },
    ImpPcVerb {
        inf: "regarder",
        il: ("regardait", "a regardé"),
        nous: ("regardions", "avons regardé"),
        elles: ("regardaient", "ont regardé"),
        complement: "la télévision",
        il_only: false,
    },
    ImpPcVerb {
        inf: "faire",
        il: ("faisait", "a fait"),
        nous: ("faisions", "avons fait"),
        elles: ("faisaient", "ont fait"),
        complement: "ses devoirs",
        il_only: false,
    },
    ImpPcVerb {
        inf: "marcher",
        il: ("marchait", "a marché"),
        nous: ("marchions", "avons marché"),
        elles: ("marchaient", "ont marché"),
        complement: "au parc",
        il_only: false,
    },
    ImpPcVerb {
        inf: "pleuvoir",
        il: ("pleuvait", "a plu"),
        nous: ("", ""),
        elles: ("", ""),
        complement: "",
        il_only: true,
    },
];

const IMP_MARKERS: &[&str] = &[
    "Chaque été,",
    "Tous les jours,",
    "Le dimanche,",
    "Pendant des années,",
    "Tous les matins,",
];

const PC_MARKERS: &[&str] = &[
    "Soudain,",
    "Tout à coup,",
    "Un matin,",
    "Hier soir,",
    "À ce moment-là,",
];

fn imppc_rule(is_habit: bool) -> &'static str {
    if is_habit {
        "A repeated habit or ongoing background takes the imparfait."
    } else {
        "A sudden, completed event takes the passé composé."
    }
}

fn imppc_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in IMPPC_VERBS {
        let subjects: &[(&str, &(&'static str, &'static str))] = if v.il_only {
            &[("il", &v.il)]
        } else {
            &[("il", &v.il), ("nous", &v.nous), ("elles", &v.elles)]
        };
        for (subj, forms) in subjects {
            for (mi, marker) in IMP_MARKERS.iter().chain(PC_MARKERS.iter()).enumerate() {
                let is_habit = mi < IMP_MARKERS.len();
                let (right, wrong) = if is_habit {
                    (forms.0, forms.1)
                } else {
                    (forms.1, forms.0)
                };
                let sentence = if v.complement.is_empty() {
                    format!("{} {} ____.", marker, subj)
                } else {
                    format!("{} {} ____ {}.", marker, subj, v.complement)
                };
                out.push(ExerciseItem::Choice(ChoiceItem {
                    id: format!("impvspc:{}:{}:{}", mi, v.inf, subj),
                    level: "A2",
                    topic: "Imparfait vs passé composé",
                    topic_slug: Some("imparfait-vs-pc".into()),
                    task: vec!["Choose the right form.".into()],
                    sentence,
                    options: vec![right.to_string(), wrong.to_string()],
                    correct: 0, // shuffled in `generate`
                    rule: vec![imppc_rule(is_habit).into()],
                }));
            }
        }
    }
    out
}

/* ── Sampling driver ────────────────────────────────────────────── */

/// Generate `count` items for a topic from its family. `exclude` holds
/// recently served item ids; excluded combos are skipped unless the
/// family can't otherwise fill the count. Choice options come back
/// shuffled with `correct` re-indexed. Returns empty for topics with
/// no family yet (callers fall back to the curated corpus).
pub fn generate(
    topic: &Topic,
    level: Option<&str>,
    count: usize,
    exclude: &HashSet<String>,
    seed: u64,
) -> Vec<ExerciseItem> {
    let mut pool = match topic.family {
        Family::PcAux => pc_aux_items(),
        Family::PcEtre => pc_etre_items(),
        Family::Pronouns => pronouns_items(),
        Family::Subjonctif => subjonctif_items(),
        Family::ImpVsPc => imppc_items(),
        Family::Curated => return Vec::new(),
    };
    if let Some(l) = level {
        pool.retain(|i| match i {
            ExerciseItem::Cloze(x) => x.level == l,
            ExerciseItem::Choice(x) => x.level == l,
            ExerciseItem::Tiles(x) => x.level == l,
            ExerciseItem::Transform(x) => x.level == l,
            ExerciseItem::ErrorSpot(x) => x.level == l,
            ExerciseItem::Dictation(x) => x.level == l,
        });
    }
    let mut rng = Rng::new(seed);
    rng.shuffle(&mut pool);
    // Shuffle choice options and re-index `correct`.
    for item in &mut pool {
        if let ExerciseItem::Choice(c) = item {
            let mut opts: Vec<(usize, String)> =
                c.options.iter().enumerate().map(|(i, o)| (i, o.clone())).collect();
            rng.shuffle(&mut opts);
            c.correct = opts.iter().position(|(i, _)| *i == c.correct).unwrap_or(0);
            c.options = opts.into_iter().map(|(_, o)| o).collect();
        }
    }
    let mut picked: Vec<ExerciseItem> = Vec::with_capacity(count);
    let mut fallback: Vec<ExerciseItem> = Vec::new();
    for item in pool {
        if picked.len() == count {
            break;
        }
        if exclude.contains(item.id()) {
            fallback.push(item);
        } else {
            picked.push(item);
        }
    }
    for item in fallback {
        if picked.len() == count {
            break;
        }
        picked.push(item);
    }
    picked
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exercises::topics;

    fn gen(slug: &str, level: Option<&str>, count: usize, seed: u64) -> Vec<ExerciseItem> {
        topics::by_slug(slug)
            .map(|t| generate(t, level, count, &HashSet::new(), seed))
            .unwrap_or_default()
    }

    #[test]
    fn every_family_topic_produces_items() {
        for t in topics::TOPICS {
            if t.family != Family::Curated {
                let items = gen(t.slug, None, 5, 1);
                assert!(items.len() == 5, "{} produced {}", t.slug, items.len());
                assert!(items.iter().all(|i| i.id().starts_with(|p: char| p.is_ascii_alphabetic())));
            }
        }
    }

    #[test]
    fn pools_are_large() {
        // The whole point: families vastly exceed any fixed batch.
        assert!(pc_etre_items().len() >= 40);
        assert!(pc_aux_items().len() >= 60);
        assert!(pronouns_items().len() >= 200);
        assert!(subjonctif_items().len() >= 300);
        assert!(imppc_items().len() >= 80);
    }

    #[test]
    fn choice_answers_stay_valid_after_shuffle() {
        for seed in 0..25 {
            for item in gen("pc-aux", None, 6, seed) {
                if let ExerciseItem::Choice(c) = item {
                    assert!(c.correct < c.options.len());
                    // The être form must be flagged correct for être verbs.
                    if c.id.contains("aller") || c.id.contains("partir") {
                        assert!(c.options[c.correct].starts_with("est")
                            || c.options[c.correct].starts_with("sont")
                            || c.options[c.correct].starts_with("sommes")
                            || c.options[c.correct].starts_with("êtes"));
                    }
                }
            }
        }
    }

    #[test]
    fn level_filter_respected() {
        let items = gen("pronouns-cod-coi", Some("A2"), 10, 3);
        assert!(!items.is_empty());
        assert!(items.iter().all(|i| match i {
            ExerciseItem::Transform(t) => t.level == "A2",
            _ => true,
        }));
        // B1 pairs (visible agreement) are excluded at A2.
        assert!(!items.iter().any(|i| i.id().contains("fleurs") || i.id().contains("clés")));
    }

    #[test]
    fn ids_are_distinct_within_pool() {
        let items = gen("subjonctif", None, 400, 7);
        let mut ids: Vec<&str> = items.iter().map(|i| i.id()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total);
    }

    #[test]
    fn exclusion_avoids_recent_items() {
        let t = topics::by_slug("pc-etre").unwrap();
        let first = generate(t, None, 5, &HashSet::new(), 11);
        let exclude: HashSet<String> = first.iter().map(|i| i.id().to_string()).collect();
        let second = generate(t, None, 5, &exclude, 12);
        assert_eq!(second.len(), 5);
        let overlap = second.iter().filter(|i| exclude.contains(i.id())).count();
        // Pool is large enough that a fresh 5 should avoid all excluded ids.
        assert_eq!(overlap, 0, "overlapping ids: {:?}", second.iter().map(|i| i.id()).collect::<Vec<_>>());
    }

    #[test]
    fn curated_topics_have_no_family_output() {
        assert!(gen("negation", None, 5, 1).is_empty());
    }

    #[test]
    fn error_spot_indices_valid() {
        for item in gen("subjonctif", None, 30, 5).into_iter().chain(gen("pc-aux", None, 30, 5)) {
            if let ExerciseItem::ErrorSpot(e) = item {
                assert!(e.wrong < e.words.len(), "{:?}", e.id);
                assert!(!e.fix.is_empty());
            }
        }
    }
}
