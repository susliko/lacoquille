//! Family: conditionnel — présent forms (future-in-the-past frames:
//! « il a promis qu'il viendrait ») and présent uses (politeness,
//! wishes, advice). The B2 conditionnel passé will join this file.

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem, TransformItem};

use super::shared::{SUBJ_CAP, punctuate, starts_vowel};

const LEVEL: &str = "B1";

/* ── conditionnel présent: forms ─────────────────────────────────── */

struct CVerb {
    inf: &'static str,
    /// conditionnel présent for je, tu, il/elle, nous, vous, ils/elles
    forms: [&'static str; 6],
    complement: &'static str,
}

const C_VERBS: &[CVerb] = &[
    CVerb { inf: "parler", forms: ["parlerais", "parlerais", "parlerait", "parlerions", "parleriez", "parleraient"], complement: "au directeur" },
    CVerb { inf: "finir", forms: ["finirais", "finirais", "finirait", "finirions", "finiriez", "finiraient"], complement: "le dossier" },
    CVerb { inf: "vendre", forms: ["vendrais", "vendrais", "vendrait", "vendrions", "vendriez", "vendraient"], complement: "sa voiture" },
    CVerb { inf: "prendre", forms: ["prendrais", "prendrais", "prendrait", "prendrions", "prendriez", "prendraient"], complement: "un autre chemin" },
    CVerb { inf: "aller", forms: ["irais", "irais", "irait", "irions", "iriez", "iraient"], complement: "au Maroc" },
    CVerb { inf: "être", forms: ["serais", "serais", "serait", "serions", "seriez", "seraient"], complement: "content" },
    CVerb { inf: "avoir", forms: ["aurais", "aurais", "aurait", "aurions", "auriez", "auraient"], complement: "le temps" },
    CVerb { inf: "faire", forms: ["ferais", "ferais", "ferait", "ferions", "feriez", "feraient"], complement: "une pause" },
    CVerb { inf: "venir", forms: ["viendrais", "viendrais", "viendrait", "viendrions", "viendriez", "viendraient"], complement: "nous aider" },
    CVerb { inf: "voir", forms: ["verrais", "verrais", "verrait", "verrions", "verriez", "verraient"], complement: "un médecin" },
    CVerb { inf: "pouvoir", forms: ["pourrais", "pourrais", "pourrait", "pourrions", "pourriez", "pourraient"], complement: "gagner" },
    CVerb { inf: "devoir", forms: ["devrais", "devrais", "devrait", "devrions", "devriez", "devraient"], complement: "réagir" },
];

/// imparfait-framed reporting cues — conditionnel = future in the past.
const C_CUES: &[&str] = &["Il a promis", "Il espérait", "Elle pensait"];

/// que + subject with the right elision: qu'il / qu'elle / qu'ils /
/// qu'elles elide; « que je », « que tu », « que nous », « que vous » stay.
fn que_subj(subj: &str) -> String {
    if matches!(subj, "il" | "elle" | "ils" | "elles") {
        format!("qu'{}", subj)
    } else {
        format!("que {}", subj)
    }
}

fn cond_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in C_VERBS {
        for &(subj, _subj_c, fi) in SUBJ_CAP {
            let form = v.forms[fi];
            // Cloze per cue — je elides into the answer chunk (j'irais).
            for (ci, cue) in C_CUES.iter().enumerate() {
                let (head, answer) = if subj == "je" && starts_vowel(form) {
                    (format!("{} ", que_subj(subj)), format!("j'{}", form))
                } else {
                    (format!("{} {} ", cue, que_subj(subj)), form.to_string())
                };
                out.push(ExerciseItem::Cloze(ClozeItem {
                    id: format!("cond:{}:{}:{}:cl", v.inf, subj, ci),
                    level: LEVEL,
                    topic: "Conditionnel présent: forms",
                    topic_slug: Some("conditionnel-forms".into()),
                    task: vec!["Put the verb in the conditionnel présent.".into()],
                    head,
                    tail: format!(" {}.", v.complement),
                    infinitive: v.inf,
                    answer,
                    rule: vec![
                        format!("{} {} {} {}.", cue, que_subj(subj), form, v.complement),
                        "Endings like the imparfait (-ais, -ait…) on the futur stem: il a promis qu'il viendrait.".into(),
                    ],
                }));
            }
            // Error-spot: futur sneaks in where the conditionnel belongs.
            let audible = matches!(subj, "tu" | "nous" | "vous" | "ils" | "elles");
            let futur_form = futur_of(v, fi);
            if audible && futur_form != form {
                out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
                    id: format!("cond:{}:{}:es", v.inf, subj),
                    level: LEVEL,
                    topic: "Conditionnel présent: forms",
                    topic_slug: Some("conditionnel-forms".into()),
                    task: vec!["One word is wrong. Tap it.".into()],
                    words: punctuate(vec![
                        "Il".into(),
                        "a".into(),
                        "promis".into(),
                        que_subj("il"),
                        futur_form.into(),
                        v.complement.into(),
                    ]),
                    wrong: 4,
                    fix: v.forms[2].to_string(),
                    fix_task: "Now write the correct form.",
                    rule: vec![
                        format!("Il a promis qu'il {} {}.", v.forms[2], v.complement),
                        "A promise reported in the past takes the conditionnel, not the futur.".into(),
                    ],
                }));
            }
        }
        // Choice: conditionnel vs futur for the same reported promise.
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("cond:{}:ch", v.inf),
            level: LEVEL,
            topic: "Conditionnel présent: forms",
            topic_slug: Some("conditionnel-forms".into()),
            task: vec!["Choose the right form.".into()],
            sentence: format!("Il a promis qu'il ____ {}.", v.complement),
            options: vec![v.forms[2].to_string(), futur_of(v, 2).to_string()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                format!("Il a promis qu'il {} {}.", v.forms[2], v.complement),
                "The reporting verb is in the past → the promised action takes the conditionnel.".into(),
            ],
        }));
    }
    out
}

/// the futur form for the same person slot (conditional minus its -ais-).
fn futur_of(v: &CVerb, fi: usize) -> &'static str {
    let table: &[(&str, [&str; 6])] = &[
        ("parler", ["parlerai", "parleras", "parlera", "parlerons", "parlerez", "parleront"]),
        ("finir", ["finirai", "finiras", "finira", "finirons", "finirez", "finiront"]),
        ("vendre", ["vendrai", "vendras", "vendra", "vendrons", "vendrez", "vendront"]),
        ("prendre", ["prendrai", "prendras", "prendra", "prendrons", "prendrez", "prendront"]),
        ("aller", ["irai", "iras", "ira", "irons", "irez", "iront"]),
        ("être", ["serai", "seras", "sera", "serons", "serez", "seront"]),
        ("avoir", ["aurai", "auras", "aura", "aurons", "aurez", "auront"]),
        ("faire", ["ferai", "feras", "fera", "ferons", "ferez", "feront"]),
        ("venir", ["viendrai", "viendras", "viendra", "viendrons", "viendrez", "viendront"]),
        ("voir", ["verrai", "verras", "verra", "verrons", "verrez", "verront"]),
        ("pouvoir", ["pourrai", "pourras", "pourra", "pourrons", "pourrez", "pourront"]),
        ("devoir", ["devrai", "devras", "devra", "devrons", "devrez", "devront"]),
    ];
    let f = table.iter().find(|(inf, _)| *inf == v.inf).map(|(_, forms)| forms[fi]);
    match f {
        Some(x) => x,
        // fallback: never reached with the C_VERBS table above
        None => v.forms[fi],
    }
}

/* ── conditionnel présent: uses ──────────────────────────────────── */

/// (sentence with ____, correct, wrong, English cue, rule)
const USE_ROWS: &[(&str, &str, &str, &str, &str)] = &[
    ("____-vous m'aider, s'il vous plaît ?", "Pourriez", "Pouvez", "a polite request", "The conditionnel softens a request: pourriez-vous."),
    ("Je ____ bien connaître la réponse.", "voudrais", "veux", "a wish", "Je voudrais = I would like — softer than je veux."),
    ("Tu ____ moins parler au téléphone.", "devrais", "dois", "a piece of advice", "Tu devrais = you should — advice, not obligation."),
    ("____-je prendre votre sac ?", "Puis-je", "Pourrais-je", "plain permission", "Puis-je is the plain form; pourrais-je is extra polite."),
    ("On ____ dire que c'est de sa faute.", "pourrait", "peut", "a hedged claim", "On pourrait dire = one could say — distance."),
    ("À sa place, je ____ tout refaire.", "ferais", "fais", "a hypothesis about someone else", "À sa place, je ferais = if I were in his place."),
    ("Vous ____ à droite après l'église.", "tourneriez", "tournez", "gentle directions", "The conditionnel makes directions sound polite."),
    ("____ être médecin si j'étais plus patient.", "J'aimerais", "J'aime", "a dream", "J'aimerais être = I would like to be."),
    ("Il ____ mieux d'acheter le billet à l'avance.", "serait", "est", "a softened judgement", "Il serait mieux de… = it would be better to…"),
    ("Nous ____ un dessert, s'il vous plaît.", "voudrions", "voulons", "ordering politely", "Nous voudrions = we would like — the polite café phrase."),
];

fn cond_use_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(fr, good, wrong, cue, rule) in USE_ROWS {
        let key = super::shared::slugify(&fr.chars().take(22).collect::<String>());
        // Choice: conditionnel vs the plain form, cue settles it.
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("condU:{}:ch", key),
            level: LEVEL,
            topic: "Conditionnel présent: uses",
            topic_slug: Some("conditionnel-uses".into()),
            task: vec![format!("{} — choose the form.", cue)],
            sentence: fr.to_string(),
            options: vec![good.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
        // Cloze: same slot for free.
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("condU:{}:cl", key),
            level: LEVEL,
            topic: "Conditionnel présent: uses",
            topic_slug: Some("conditionnel-uses".into()),
            task: vec![format!("{} — complete politely.", cue)],
            head: fr.split("____").next().unwrap().to_string(),
            tail: fr.split("____").nth(1).unwrap().to_string(),
            infinitive: inf_of_use(good),
            answer: good.to_string(),
            rule: vec![fr.replace("____", good), rule.to_string()],
        }));
    }
    // Transform: plain → polite.
    for (plain, polite) in [
        ("Tu peux m'aider ?", "Tu pourrais m'aider ?"),
        ("Tu veux un café ?", "Tu voudrais un café ?"),
        ("Vous devez appeler un médecin.", "Vous devriez appeler un médecin."),
        ("Je veux savoir le prix.", "Je voudrais savoir le prix."),
        ("Pouvez-vous répéter ?", "Pourriez-vous répéter ?"),
    ] {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("condU:{}:tf", super::shared::slugify(plain)),
            level: LEVEL,
            topic: "Conditionnel présent: uses",
            topic_slug: Some("conditionnel-uses".into()),
            task: vec!["Make it polite with the conditionnel.".into()],
            source: plain.into(),
            answer: polite.into(),
            rule: vec![
                "The conditionnel (pouvoir → pourrais, vouloir → voudrais, devoir → devrais) turns an order into a request.".into(),
                format!("{} → {}", plain, polite),
            ],
        }));
    }
    out
}

fn inf_of_use(good: &str) -> &'static str {
    match good {
        "Pourriez" => "pouvoir",
        "voudrais" => "vouloir",
        "devrais" => "devoir",
        "Puis-je" => "pouvoir",
        "pourrait" => "pouvoir",
        "ferais" => "faire",
        "tourneriez" => "tourner",
        "aimerais" => "aimer",
        "serait" => "être",
        _ => "faire",
    }
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "conditionnel-forms" => cond_items(),
        "conditionnel-uses" => cond_use_items(),
        _ => Vec::new(),
    }
}
