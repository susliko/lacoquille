//! Family: passé composé with avoir. Cloze drills avoir + participle
//! (regular participles at A1, irregular at A2); error-spot and choice
//! catch the classic false agreement (elle a mangée → mangé) — with
//! avoir the participle never agrees with what follows.

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem};

use super::shared::{SUBJ_CAP, avoir_pc, punctuate};

struct PcVerb {
    inf: &'static str,
    /// past participle (base — avoir never agrees with the object)
    p: &'static str,
    complement: &'static str,
    regular: bool,
}

const PC_VERBS: &[PcVerb] = &[
    // Regular participles — A1.
    PcVerb { inf: "manger", p: "mangé", complement: "une pomme", regular: true },
    PcVerb { inf: "parler", p: "parlé", complement: "avec le voisin", regular: true },
    PcVerb { inf: "regarder", p: "regardé", complement: "un film", regular: true },
    PcVerb { inf: "finir", p: "fini", complement: "le travail", regular: true },
    PcVerb { inf: "vendre", p: "vendu", complement: "sa voiture", regular: true },
    // Irregular participles — A2.
    PcVerb { inf: "prendre", p: "pris", complement: "le train", regular: false },
    PcVerb { inf: "faire", p: "fait", complement: "les courses", regular: false },
    PcVerb { inf: "voir", p: "vu", complement: "un ami", regular: false },
    PcVerb { inf: "lire", p: "lu", complement: "le journal", regular: false },
    PcVerb { inf: "écrire", p: "écrit", complement: "une lettre", regular: false },
    PcVerb { inf: "boire", p: "bu", complement: "un café", regular: false },
    PcVerb { inf: "mettre", p: "mis", complement: "son manteau", regular: false },
];

/// False-agreement marks the error-spot/choice items plant.
fn wrongly_agreed(base: &str, subj: &str) -> String {
    match subj {
        "elle" => format!("{}e", base),
        "ils" => format!("{}s", base),
        _ => format!("{}es", base), // elles
    }
}

/// "Hier, j'ai " / "Hier, tu as " — mid-sentence subject lowercase,
/// je elides with its avoir form.
fn head(subj: &str) -> String {
    if subj == "je" {
        "Hier, j'ai ".into()
    } else {
        format!("Hier, {} {} ", subj, avoir_pc(subj))
    }
}

pub(crate) fn items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in PC_VERBS {
        let level = if v.regular { "A1" } else { "A2" };

        // Cloze: avoir + participle.
        for (subj, _subj_c, _fi) in SUBJ_CAP {
            out.push(ExerciseItem::Cloze(ClozeItem {
                id: format!("pca:{}:{}:cl", v.inf, subj),
                level,
                topic: "Passé composé with avoir",
                topic_slug: Some("pc-avoir".into()),
                task: vec!["Put the verb in the passé composé.".into()],
                head: head(subj),
                tail: format!(" {}.", v.complement),
                infinitive: v.inf,
                answer: v.p.to_string(),
                rule: if v.regular {
                    vec![
                        "avoir + participle — the participle keeps".into(),
                        "its -é/-i/-u ending with avoir.".into(),
                    ]
                } else {
                    vec![
                        format!("{} has an irregular participle: {}.", v.inf, v.p),
                        "With avoir, nothing after the verb changes it.".into(),
                    ]
                },
            }));
        }

        if v.regular {
            continue;
        }
        // Error-spot: false agreement, drilled at A2.
        for subj in ["elle", "ils", "elles"] {
            let wrong_form = wrongly_agreed(v.p, subj);
            let mut words: Vec<String> = vec!["Hier".into(), subj.to_string(), avoir_pc(subj).to_string()];
            words.push(wrong_form);
            let wrong = words.len() - 1;
            let comp: Vec<String> = v.complement.split(' ').map(|w| w.to_string()).collect();
            words.extend(comp);
            let words = punctuate(words);
            out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
                id: format!("pca:{}:{}:es", v.inf, subj),
                level: "A2",
                topic: "Passé composé with avoir",
                topic_slug: Some("pc-avoir".into()),
                task: vec!["One word is wrong. Tap it.".into()],
                words,
                wrong,
                fix: v.p.to_string(),
                fix_task: "Now write the correct form.",
                rule: vec![
                    "With avoir the participle never agrees with".into(),
                    format!("what follows: {} a {} {}.", subj, v.p, v.complement),
                ],
            }));
        }
        // Choice: the same trap as a two-option pick.
        for subj in ["elle", "ils"] {
            out.push(ExerciseItem::Choice(ChoiceItem {
                id: format!("pca:{}:{}:ch", v.inf, subj),
                level: "A2",
                topic: "Passé composé with avoir",
                topic_slug: Some("pc-avoir".into()),
                task: vec!["Choose the right form.".into()],
                sentence: format!(
                    "Hier, {} {} ____ {}.",
                    subj,
                    avoir_pc(subj),
                    v.complement
                ),
                options: vec![v.p.to_string(), wrongly_agreed(v.p, subj)],
                correct: 0, // shuffled in `generate`
                rule: vec![
                    format!("avoir + {} — no agreement mark.", v.p),
                    format!("{} {} {} {}.", subj, avoir_pc(subj), v.p, v.complement),
                ],
            }));
        }
    }
    out
}
