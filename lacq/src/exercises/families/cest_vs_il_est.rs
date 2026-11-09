//! Family: c'est vs il/elle est. Choice items cover the three classic
//! contexts (c'est + noun, il est + adjective + que-clause, c'est +
//! adjective after the noun is named), error-spot puts the wrong one in
//! a sentence, and the transform drills noun → stressed pronoun.

use crate::exercises::{ChoiceItem, ErrorSpotItem, ExerciseItem, TransformItem};

use super::shared::{punctuate, slugify};

struct NounPhrase {
    /// Indefinite article + noun (+ adjective): the c'est zone.
    np: &'static str,
}

const NOUN_PHRASES: &[NounPhrase] = &[
    NounPhrase { np: "un bon restaurant" },
    NounPhrase { np: "une vieille église" },
    NounPhrase { np: "un film intéressant" },
    NounPhrase { np: "des chaussures chères" },
    NounPhrase { np: "un célèbre écrivain" },
    NounPhrase { np: "une belle plage" },
    NounPhrase { np: "un problème difficile" },
    NounPhrase { np: "des amis sympathiques" },
    NounPhrase { np: "une bonne idée" },
    NounPhrase { np: "un grand jardin" },
];

/// Adjectives that take il est + que + clause (never c'est + que).
struct QueClause {
    adj: &'static str,
    clause: &'static str,
}

const QUE_CLAUSES: &[QueClause] = &[
    QueClause { adj: "évident", clause: "qu'elle a raison" },
    QueClause { adj: "clair", clause: "qu'il est tard" },
    QueClause { adj: "certain", clause: "qu'il pleut" },
    QueClause { adj: "probable", clause: "qu'elle viendra demain" },
    QueClause { adj: "normal", clause: "qu'il dort encore" },
    QueClause { adj: "surprenant", clause: "qu'ils disent ça" },
    QueClause { adj: "vrai", clause: "qu'elle parle bien français" },
    QueClause { adj: "faux", clause: "qu'il déteste le café" },
];

/// (named thing, adjective phrase) — after naming the noun, the
/// standing comment is C'est + adjective.
const BARE_ADJ: &[(&str, &str)] = &[
    ("Cette voiture ?", "vraiment chère."),
    ("Ce manteau ?", "vraiment cher."),
    ("Cette maison ?", "très vieille."),
    ("Ce café ?", "très bon."),
    ("Cette idée ?", "vraiment bonne."),
];

/// (determiner + noun, agreeing pronoun, agreeing adjective)
const GENDERED: &[(&str, &str, &str)] = &[
    ("Ce livre", "Il", "cher"),
    ("Cette voiture", "Elle", "chère"),
    ("Ce film", "Il", "long"),
    ("Cette ville", "Elle", "grande"),
    ("Ce problème", "Il", "difficile"),
    ("Cette idée", "Elle", "bonne"),
    ("Ce manteau", "Il", "chaud"),
    ("Cette chanson", "Elle", "nouvelle"),
    ("Ce café", "Il", "bon"),
    ("Cette maison", "Elle", "vieille"),
];

pub(crate) fn items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();

    // Choice: c'est + noun (article present → c'est).
    for n in NOUN_PHRASES {
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("cest-n:{}:ch", slugify(n.np)),
            level: "A1",
            topic: "C'est vs il/elle est",
            topic_slug: Some("cest-vs-il-est".into()),
            task: vec!["Choose the right form.".into()],
            sentence: format!("____ {}.", n.np),
            options: vec!["C'est".into(), "Il est".into()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                "With an article (un, une, des), use c'est:".into(),
                format!("C'est {}.", n.np),
            ],
        }));
    }
    // Choice: adjective + que-clause → il est.
    for q in QUE_CLAUSES {
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("cest-q:{}:ch", q.adj),
            level: "A1",
            topic: "C'est vs il/elle est",
            topic_slug: Some("cest-vs-il-est".into()),
            task: vec!["Choose the right form.".into()],
            sentence: format!("____ {} {}.", q.adj, q.clause),
            options: vec!["Il est".into(), "C'est".into()],
            correct: 0,
            rule: vec![
                "Before que + a clause, the adjective is il est:".into(),
                format!("Il est {} {}.", q.adj, q.clause),
            ],
        }));
    }
    // Choice: the noun is already named → c'est + adjective.
    for (lead, comment) in BARE_ADJ {
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("cest-b:{}:ch", slugify(lead)),
            level: "A1",
            topic: "C'est vs il/elle est",
            topic_slug: Some("cest-vs-il-est".into()),
            task: vec!["Choose the right form.".into()],
            sentence: format!("{} ____ {}", lead, comment),
            options: vec!["C'est".into(), "Il est".into()],
            correct: 0,
            rule: vec![
                "Once the thing is named, the comment is".into(),
                "c'est + adjective — even when it is feminine.".into(),
            ],
        }));
    }
    // Error-spot: il est before a noun phrase.
    for n in NOUN_PHRASES {
        let mut words: Vec<String> = vec!["Il".into(), "est".into()];
        words.extend(n.np.split(' ').map(|w| w.to_string()));
        let words = punctuate(words);
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("cest-n:{}:es", slugify(n.np)),
            level: "A2",
            topic: "C'est vs il/elle est",
            topic_slug: Some("cest-vs-il-est".into()),
            task: vec!["One word is wrong. Tap it.".into()],
            words,
            wrong: 0,
            fix: "C'est".into(),
            fix_task: "Now write the correct form.",
            rule: vec![
                "An article means c'est:".into(),
                format!("C'est {}.", n.np),
            ],
        }));
    }
    // Error-spot: c'est before que.
    for q in QUE_CLAUSES {
        let mut words: Vec<String> =
            vec!["C'est".into(), q.adj.to_string()];
        words.extend(q.clause.split(' ').map(|w| w.to_string()));
        let words = punctuate(words);
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("cest-q:{}:es", q.adj),
            level: "A2",
            topic: "C'est vs il/elle est",
            topic_slug: Some("cest-vs-il-est".into()),
            task: vec!["One word is wrong. Tap it.".into()],
            words,
            wrong: 0,
            fix: "Il est".into(),
            fix_task: "Now write the correct form.",
            rule: vec![
                "que + a clause means il est:".into(),
                format!("Il est {} {}.", q.adj, q.clause),
            ],
        }));
    }
    // Transform: named noun → stressed pronoun + adjective.
    for (i, (det_noun, pron, adj)) in GENDERED.iter().enumerate() {
        out.push(ExerciseItem::Transform(TransformItem {
            id: format!("cest-t:{}:tf", i),
            level: "A2",
            topic: "C'est vs il/elle est",
            topic_slug: Some("cest-vs-il-est".into()),
            task: vec![
                "Rewrite it, replacing the noun".into(),
                "with a pronoun.".into(),
            ],
            source: format!("{} est {}.", det_noun, adj),
            answer: format!("{} est {}.", pron, adj),
            rule: vec![
                format!("{} → {} keeps the gender,", det_noun, pron),
                format!("and the adjective agrees: {} {}.", pron.to_lowercase(), adj),
            ],
        }));
    }
    out
}
