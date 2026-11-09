//! Family: imparfait forms. Cues signal habit/background; the verb
//! table carries both the imparfait and present forms, so cloze items
//! drill the endings and error-spot items catch a present form sitting
//! in a past habit ("Avant, tu manges…" → mangeais).

use crate::exercises::{ClozeItem, ErrorSpotItem, ExerciseItem};

use super::shared::{SUBJ_CAP, cap, punctuate, starts_vowel};

struct ImpVerb {
    inf: &'static str,
    /// imparfait for je, tu, il/elle, nous, vous, ils/elles
    imp: [&'static str; 6],
    /// present for je, tu, il/elle, nous, vous, ils/elles
    pres: [&'static str; 6],
    complement: &'static str,
    /// Regular -er/-ir/-re endings land at A1; irregular stems at A2.
    regular: bool,
}

const IMP_VERBS: &[ImpVerb] = &[
    ImpVerb {
        inf: "parler",
        imp: ["parlais", "parlais", "parlait", "parlions", "parliez", "parlaient"],
        pres: ["parle", "parles", "parle", "parlons", "parlez", "parlent"],
        complement: "au téléphone",
        regular: true,
    },
    ImpVerb {
        inf: "manger",
        imp: ["mangeais", "mangeais", "mangeait", "mangions", "mangiez", "mangeaient"],
        pres: ["mange", "manges", "mange", "mangeons", "mangez", "mangent"],
        complement: "à la cantine",
        regular: true,
    },
    ImpVerb {
        inf: "regarder",
        imp: ["regardais", "regardais", "regardait", "regardions", "regardiez", "regardaient"],
        pres: ["regarde", "regardes", "regarde", "regardons", "regardez", "regardent"],
        complement: "la télévision",
        regular: true,
    },
    ImpVerb {
        inf: "finir",
        imp: ["finissais", "finissais", "finissait", "finissions", "finissiez", "finissaient"],
        pres: ["finis", "finis", "finit", "finissons", "finissez", "finissent"],
        complement: "ses devoirs",
        regular: true,
    },
    ImpVerb {
        inf: "vendre",
        imp: ["vendais", "vendais", "vendait", "vendions", "vendiez", "vendaient"],
        pres: ["vends", "vends", "vend", "vendons", "vendez", "vendent"],
        complement: "des fleurs au marché",
        regular: true,
    },
    ImpVerb {
        inf: "prendre",
        imp: ["prenais", "prenais", "prenait", "prenions", "preniez", "prenaient"],
        pres: ["prends", "prends", "prend", "prenons", "prenez", "prennent"],
        complement: "le bus",
        regular: false,
    },
    ImpVerb {
        inf: "faire",
        imp: ["faisais", "faisais", "faisait", "faisions", "faisiez", "faisaient"],
        pres: ["fais", "fais", "fait", "faisons", "faites", "font"],
        complement: "du ski",
        regular: false,
    },
    ImpVerb {
        inf: "aller",
        imp: ["allais", "allais", "allait", "allions", "alliez", "allaient"],
        pres: ["vais", "vas", "va", "allons", "allez", "vont"],
        complement: "au parc",
        regular: false,
    },
    ImpVerb {
        inf: "être",
        imp: ["étais", "étais", "était", "étions", "étiez", "étaient"],
        pres: ["suis", "es", "est", "sommes", "êtes", "sont"],
        complement: "à la campagne",
        regular: false,
    },
    ImpVerb {
        inf: "avoir",
        imp: ["avais", "avais", "avait", "avions", "aviez", "avaient"],
        pres: ["ai", "as", "a", "avons", "avez", "ont"],
        complement: "beaucoup de temps",
        regular: false,
    },
    ImpVerb {
        inf: "venir",
        imp: ["venais", "venais", "venait", "venions", "veniez", "venaient"],
        pres: ["viens", "viens", "vient", "venons", "venez", "viennent"],
        complement: "nous voir",
        regular: false,
    },
    ImpVerb {
        inf: "pouvoir",
        imp: ["pouvais", "pouvais", "pouvait", "pouvions", "pouviez", "pouvaient"],
        pres: ["peux", "peux", "peut", "pouvons", "pouvez", "peuvent"],
        complement: "rester dehors",
        regular: false,
    },
];

/// Habit/background cues — every sentence carries one.
const CUES: &[&str] = &["Avant,", "Chaque été,", "Le dimanche,"];

/// Subject display with elision: j'étais, not Je étais. Mid-sentence
/// subjects stay lowercase ("Avant, tu parlais").
fn subj_head(cue: &str, subj: &str, form: &str) -> String {
    if subj == "je" && starts_vowel(form) {
        format!("{} j'", cue)
    } else {
        format!("{} {} ", cue, subj)
    }
}

pub(crate) fn items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for v in IMP_VERBS {
        let level = if v.regular { "A1" } else { "A2" };
        for (subj, _subj_c, fi) in SUBJ_CAP {
            let form = v.imp[*fi];
            for (ci, cue) in CUES.iter().enumerate() {
                // Cloze: complete the imparfait in the habit frame.
                let head = subj_head(cue, subj, form);
                out.push(ExerciseItem::Cloze(ClozeItem {
                    id: format!("imp:{}:{}:{}:cl", v.inf, subj, ci),
                    level,
                    topic: "Imparfait forms",
                    topic_slug: Some("imparfait".into()),
                    task: vec!["Put the verb in the imparfait.".into()],
                    head,
                    tail: format!(" {}.", v.complement),
                    infinitive: v.inf,
                    answer: form.to_string(),
                    rule: vec![
                        format!("{} signals a habit or background → imparfait.", cue),
                        format!("{} → {}.", v.inf, form),
                    ],
                }));

                // Error-spot: present form sitting in a past habit,
                // drilled for il / nous / vous.
                if *fi >= 2 && *fi <= 4 {
                    let wrong_form = v.pres[*fi];
                    let mut words: Vec<String> =
                        cue.split(' ').map(|w| w.to_string()).collect();
                    words.push(subj.to_string());
                    words.push(wrong_form.to_string());
                    let wrong = words.len() - 1;
                    let comp: Vec<String> =
                        v.complement.split(' ').map(|w| w.to_string()).collect();
                    words.extend(comp);
                    let words = punctuate(words);
                    out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
                        id: format!("imp:{}:{}:{}:es", v.inf, subj, ci),
                        level: "A2",
                        topic: "Imparfait forms",
                        topic_slug: Some("imparfait".into()),
                        task: vec!["One word is wrong. Tap it.".into()],
                        words,
                        wrong,
                        fix: form.to_string(),
                        fix_task: "Now write the correct form.",
                        rule: vec![
                            format!("{} signals a habit → imparfait.", cue),
                            format!("{} {} {}.", cap(subj), form, v.complement),
                        ],
                    }));
                }
            }
        }
    }
    out
}
