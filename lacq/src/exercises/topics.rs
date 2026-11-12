//! Topic catalog — the pedagogy knowledge the coach and the generators
//! share. Server-owned by design: the client only ever sends learner
//! history keyed by `slug`; everything a decision needs (what a topic
//! covers, which levels it spans, which generator family drills it) is
//! attached here.

/// Which parameterized generator family can produce endless items for a
/// topic. `Curated` topics are served from the hand-authored corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    /// Être vs avoir auxiliary choice (error-spot / choice).
    PcAux,
    /// Passé composé with être + participle agreement (cloze / error-spot).
    PcEtre,
    /// COD/COI pronoun replacement (transform / tiles).
    Pronouns,
    /// Subjonctif after triggers (cloze / error-spot).
    Subjonctif,
    /// Imparfait vs passé composé, marker-driven (choice).
    ImpVsPc,
    /// ne … pas / ne … jamais around the verb (transform / cloze / choice).
    Negation,
    /// c'est + noun vs il/elle est + adjective (choice / error-spot / transform).
    CestVsIlEst,
    /// à + place → y (transform / choice).
    PronounY,
    /// Preceding-object participle agreement (cloze / choice).
    AccordAntepose,
    /// Imparfait endings against habit cues (cloze / error-spot).
    Imparfait,
    /// Passé composé with avoir, no participle agreement (cloze / error-spot / choice).
    PcAvoir,
    /// The present tense — four topics from one conjugation engine
    /// (être/avoir, -er verbs, common irregulars, pronominals).
    Present,
    /// Present periphrases — être en train de / futur proche / passé récent.
    Periphrases,
    /// Modals + infinitive — pouvoir/vouloir/devoir, il faut, pour.
    Modals,
    /// Articles — gender, plural, definite/indefinite, partitive.
    Articles,
    /// Adjectives — agreement, position, possessives, demonstratives, quantity.
    Adjectives,
    /// Pronoun basics — subject pronouns & on, stressed pronouns.
    PronounBasics,
    /// Questions — three question forms, question words.
    Questions,
    /// A1 sentence cluster — il y a, dans/depuis, place/time preps, connectors.
    SentencesA1,
    /// A1 mood cluster — imperative, polite conditionnel.
    MoodsA1,
    /// No family yet — curated corpus only.
    Curated,
}

/// Where a topic sits on the progress grid — the learner-facing map
/// on the main page. Four columns, settled after the UX rounds (see
/// handoff docs/02-topics.md "Why four families"): verbs split into
/// Tense and Mood, the noun-related threads merged into Noun group,
/// sentence-level threads into Sentence. Display taxonomy only —
/// orthogonal to `Family`, which picks the generator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Section {
    Tense,
    Mood,
    Nouns,
    Sentence,
}

#[derive(Debug)]
pub struct Topic {
    pub slug: &'static str,
    /// Display title; matches curated corpus `topic` strings where possible
    /// so curated items roll up under the same slug.
    pub title: &'static str,
    /// One line describing what drilling this topic involves. This is the
    /// description the coach's decision model reads.
    pub blurb: &'static str,
    /// Valid CEFR levels, low → high.
    pub levels: &'static [&'static str],
    pub family: Family,
    /// Grid section this topic fills.
    pub section: Section,
}

pub const A1A2: &[&str] = &["A1", "A2"];
pub const A2B1: &[&str] = &["A2", "B1"];

pub const TOPICS: &[Topic] = &[
    Topic {
        slug: "pc-etre",
        title: "Passé composé with être",
        blurb: "Forming the passé composé with être and making the participle agree with the subject (elle est allée, ils sont partis).",
        levels: A2B1,
        family: Family::PcEtre,
        section: Section::Tense,
    },
    Topic {
        slug: "pc-aux",
        title: "Auxiliary choice",
        blurb: "Choosing between être and avoir in the passé composé (elle est tombée, not elle a tombée).",
        levels: &["A2"],
        family: Family::PcAux,
        section: Section::Tense,
    },
    Topic {
        slug: "imparfait-vs-pc",
        title: "Imparfait vs passé composé",
        blurb: "Choosing the imparfait for background and habits versus the passé composé for events (pendant que je dormais… le téléphone a sonné).",
        levels: A2B1,
        family: Family::ImpVsPc,
        section: Section::Tense,
    },
    Topic {
        slug: "pronouns-cod-coi",
        title: "Object pronouns",
        blurb: "Replacing direct and indirect objects with le, la, les, lui, leur and placing them before the verb (Je la connais, Tu lui téléphones).",
        levels: A2B1,
        family: Family::Pronouns,
        section: Section::Nouns,
    },
    Topic {
        slug: "subjonctif",
        title: "Subjonctif after feelings and doubt",
        blurb: "Using the subjonctif after expressions of feeling, doubt and necessity (je doute qu'il ait raison, je suis content que tu viennes).",
        levels: &["B1"],
        family: Family::Subjonctif,
        section: Section::Mood,
    },
    Topic {
        slug: "pc-avoir",
        title: "Passé composé with avoir",
        blurb: "Forming the passé composé with avoir and the past participle (nous avons mangé).",
        levels: A1A2,
        family: Family::PcAvoir,
        section: Section::Tense,
    },
    Topic {
        slug: "imparfait",
        title: "Imparfait forms",
        blurb: "Forming the imparfait of common verbs (je parlais, nous étions, elles venaient).",
        levels: A1A2,
        family: Family::Imparfait,
        section: Section::Tense,
    },
    Topic {
        slug: "pronoun-y",
        title: "The pronoun y",
        blurb: "Replacing à + place with y and placing it before the verb (elle va à Paris → elle y va).",
        levels: &["B1"],
        family: Family::PronounY,
        section: Section::Nouns,
    },
    Topic {
        slug: "cest-vs-il-est",
        title: "C'est vs il/elle est",
        blurb: "Choosing between c'est + noun and il/elle est + adjective (c'est une bonne idée, c'est évident).",
        levels: A1A2,
        family: Family::CestVsIlEst,
        section: Section::Sentence,
    },
    Topic {
        slug: "negation",
        title: "Negation",
        blurb: "Wrapping the conjugated verb in ne … pas (elle ne parle pas anglais).",
        levels: &["A1"],
        family: Family::Negation,
        section: Section::Sentence,
    },
    Topic {
        slug: "accord-antepose",
        title: "Agreement with preceding object",
        blurb: "Making the participle agree when the direct object precedes the verb (les fleurs que j'ai achetées).",
        levels: &["B1"],
        family: Family::AccordAntepose,
        section: Section::Tense,
    },
    // ── A1 tense ────────────────────────────────────────────────
    Topic {
        slug: "present-etre-avoir",
        title: "Present: être & avoir",
        blurb: "The two essential present-tense verbs: je suis, tu es… and j'ai, tu as… with simple complements.",
        levels: &["A1"],
        family: Family::Present,
        section: Section::Tense,
    },
    Topic {
        slug: "present-er-verbs",
        title: "Present: -er verbs",
        blurb: "The regular -er pattern that covers most French verbs: je parle, tu parles, il parle, nous parlons, vous parlez, ils parlent.",
        levels: &["A1"],
        family: Family::Present,
        section: Section::Tense,
    },
    Topic {
        slug: "present-irregulars",
        title: "Present: common irregular verbs",
        blurb: "The everyday irregulars a learner meets first: aller, faire, venir, prendre, dire, savoir, voir, sortir, comprendre, lire.",
        levels: &["A1"],
        family: Family::Present,
        section: Section::Tense,
    },
    Topic {
        slug: "present-pronominal",
        title: "Present: pronominal verbs",
        blurb: "Reflexive verbs with the agreeing pronoun: je me lève, tu te lèves, il s'appelle — including the m'/t'/s' elision.",
        levels: &["A1"],
        family: Family::Present,
        section: Section::Tense,
    },
    Topic {
        slug: "etre-en-train-de",
        title: "Être en train de",
        blurb: "Saying what is happening right now: elle est en train de manger — être en train de + infinitive.",
        levels: &["A1"],
        family: Family::Periphrases,
        section: Section::Tense,
    },
    Topic {
        slug: "futur-proche",
        title: "Futur proche",
        blurb: "The near future with aller + infinitive: je vais partir demain, nous allons manger.",
        levels: &["A1"],
        family: Family::Periphrases,
        section: Section::Tense,
    },
    Topic {
        slug: "passe-recent",
        title: "Passé récent",
        blurb: "The recent past with venir de + infinitive: je viens de finir, elle vient de partir.",
        levels: &["A1"],
        family: Family::Periphrases,
        section: Section::Tense,
    },
    // ── A1 mood ─────────────────────────────────────────────────
    Topic {
        slug: "imperative",
        title: "Imperative",
        blurb: "Giving commands: parle !, mangez !, allons-y — tu/nous/vous forms without the subject, and ne … pas around them.",
        levels: &["A1"],
        family: Family::MoodsA1,
        section: Section::Mood,
    },
    Topic {
        slug: "polite-conditionnel",
        title: "Polite conditionnel",
        blurb: "Making requests soft with je voudrais — the polite way to want (je voudrais un café, s'il vous plaît).",
        levels: &["A1"],
        family: Family::MoodsA1,
        section: Section::Mood,
    },
    Topic {
        slug: "modal-verbs-infinitive",
        title: "Pouvoir, vouloir, devoir + infinitive",
        blurb: "The three essential modals: je peux, je veux, je dois + a second verb that stays in the infinitive.",
        levels: &["A1"],
        family: Family::Modals,
        section: Section::Mood,
    },
    Topic {
        slug: "il-faut-infinitive",
        title: "Il faut + infinitive",
        blurb: "Expressing necessity impersonally: il faut partir, il faut finir — one form, no person.",
        levels: &["A1"],
        family: Family::Modals,
        section: Section::Mood,
    },
    Topic {
        slug: "pour-infinitive",
        title: "Pour + infinitive",
        blurb: "Saying what something is for: je lis pour apprendre — pour + infinitive (never par).",
        levels: &["A1"],
        family: Family::Modals,
        section: Section::Mood,
    },
    // ── A1 noun group ───────────────────────────────────────────
    Topic {
        slug: "gender-of-nouns",
        title: "Gender of nouns",
        blurb: "Every French noun is masculine or feminine, and the article shows it: un problème, une idée.",
        levels: &["A1"],
        family: Family::Articles,
        section: Section::Nouns,
    },
    Topic {
        slug: "plural-of-nouns",
        title: "Plural of nouns",
        blurb: "Making nouns plural: most take -s, but -eau takes -x, -al takes -aux, and -s/-x/-z don't change.",
        levels: &["A1"],
        family: Family::Articles,
        section: Section::Nouns,
    },
    Topic {
        slug: "definite-indefinite-articles",
        title: "Definite & indefinite articles",
        blurb: "un/une/des versus le/la/les (l' before a vowel) — choosing the article that fits the noun and the meaning.",
        levels: &["A1"],
        family: Family::Articles,
        section: Section::Nouns,
    },
    Topic {
        slug: "partitive-articles",
        title: "Partitive articles",
        blurb: "Saying some with food and quantities: du pain, de la confiture, de l'eau — and the negative that turns them into de.",
        levels: &["A1"],
        family: Family::Articles,
        section: Section::Nouns,
    },
    Topic {
        slug: "adjective-agreement",
        title: "Adjective agreement",
        blurb: "Adjectives match their noun's gender and number: un garçon grand, une fille grande, des filles grandes.",
        levels: &["A1"],
        family: Family::Adjectives,
        section: Section::Nouns,
    },
    Topic {
        slug: "adjective-position",
        title: "Adjective position",
        blurb: "Most adjectives follow the noun (une voiture rouge), but the BAGS crowd — beauty, age, goodness, size — goes before (une grande maison).",
        levels: &["A1"],
        family: Family::Adjectives,
        section: Section::Nouns,
    },
    Topic {
        slug: "possessive-adjectives",
        title: "Possessive adjectives",
        blurb: "mon/ma/mes, ton/ta/tes, son/sa/ses… — agreeing with the thing owned, not the owner (son frère, even for Marie).",
        levels: &["A1"],
        family: Family::Adjectives,
        section: Section::Nouns,
    },
    Topic {
        slug: "demonstrative-adjectives",
        title: "Demonstrative adjectives",
        blurb: "Pointing at things: ce chien, cet arbre, cette table, ces livres — with the cet-before-a-vowel rule.",
        levels: &["A1"],
        family: Family::Adjectives,
        section: Section::Nouns,
    },
    Topic {
        slug: "quantity-adverbs",
        title: "Quantity & intensity adverbs",
        blurb: "très + adjective (très content) versus beaucoup de + noun (beaucoup d'amis) — the pairing learners mix up.",
        levels: &["A1"],
        family: Family::Adjectives,
        section: Section::Nouns,
    },
    Topic {
        slug: "subject-pronouns",
        title: "Subject pronouns & on",
        blurb: "Replacing subjects with je, tu, il, elle, nous, vous, ils, elles — and the informal on that takes a 3rd-person form.",
        levels: &["A1"],
        family: Family::PronounBasics,
        section: Section::Nouns,
    },
    Topic {
        slug: "stressed-pronouns",
        title: "Stressed pronouns",
        blurb: "The strong forms after prepositions and after c'est: avec moi, pour toi, c'est lui, c'est eux.",
        levels: &["A1"],
        family: Family::PronounBasics,
        section: Section::Nouns,
    },
    // ── A1 sentence ─────────────────────────────────────────────
    Topic {
        slug: "three-question-forms",
        title: "Three question forms",
        blurb: "Asking three ways: intonation (tu vas bien ?), est-ce que (est-ce que tu vas bien ?), and inversion (vas-tu bien ?).",
        levels: &["A1"],
        family: Family::Questions,
        section: Section::Sentence,
    },
    Topic {
        slug: "question-words",
        title: "Question words",
        blurb: "qui, que, où, quand, comment, pourquoi, combien — the words that open questions, in inversion and est-ce que forms.",
        levels: &["A1"],
        family: Family::Questions,
        section: Section::Sentence,
    },
    Topic {
        slug: "il-ya-cest-voila",
        title: "Il y a, c'est, voilà",
        blurb: "Presenting and pointing: il y a deux chiens (there are), c'est mon ami, voilà le train — and the il y a / il a trap.",
        levels: &["A1"],
        family: Family::SentencesA1,
        section: Section::Sentence,
    },
    Topic {
        slug: "dans-depuis-il-ya",
        title: "Dans, depuis, il y a",
        blurb: "Three time expressions learners swap wrongly: dans (in, future), depuis (since/for, ongoing), il y a (ago, past).",
        levels: &["A1"],
        family: Family::SentencesA1,
        section: Section::Sentence,
    },
    Topic {
        slug: "prepositions-place",
        title: "Prepositions of place",
        blurb: "Where things and people are: à Paris, en France, au Japon, chez Marie, dans le sac, sur la table.",
        levels: &["A1"],
        family: Family::SentencesA1,
        section: Section::Sentence,
    },
    Topic {
        slug: "prepositions-time",
        title: "Prepositions of time",
        blurb: "When things happen: avant le dîner, après le cours, pendant les vacances, sous la pluie.",
        levels: &["A1"],
        family: Family::SentencesA1,
        section: Section::Sentence,
    },
    Topic {
        slug: "connectors-basic",
        title: "Et, ou, mais, alors",
        blurb: "The four connectors that shape a sentence: and, or, but, so — chosen by the relation between the clauses.",
        levels: &["A1"],
        family: Family::SentencesA1,
        section: Section::Sentence,
    },
];

pub fn by_slug(slug: &str) -> Option<&'static Topic> {
    TOPICS.iter().find(|t| t.slug == slug)
}

/// Every curated corpus topic title → its catalog slug. The progress
/// endpoint serves this map so the client can fold its fixed-session
/// history (keyed by curated title) into catalog topics. `slug_for_title`
/// reads it; keep new corpus titles here.
pub const CURATED_TITLES: &[(&str, &str)] = &[
    ("Passé composé with être", "pc-etre"),
    ("Auxiliary choice", "pc-aux"),
    ("Imparfait vs passé composé", "imparfait-vs-pc"),
    ("Subjonctif after feelings", "subjonctif"),
    ("Subjonctif after doubt", "subjonctif"),
    ("Passé composé with avoir", "pc-avoir"),
    ("Imparfait of -er verbs", "imparfait"),
    ("Imparfait of être", "imparfait"),
    ("Imparfait of venir", "imparfait"),
    ("Direct object pronouns", "pronouns-cod-coi"),
    ("Indirect object pronouns", "pronouns-cod-coi"),
    ("Object pronouns", "pronouns-cod-coi"),
    ("Double object pronouns", "pronouns-cod-coi"),
    ("The pronoun y", "pronoun-y"),
    ("C'est vs il/elle est", "cest-vs-il-est"),
    ("Negation", "negation"),
    ("Agreement with preceding object", "accord-antepose"),
];

/// Map a curated corpus `topic` title to its catalog slug, so curated
/// items roll up under the same topic the coach picked.
pub fn slug_for_title(title: &str) -> Option<&'static str> {
    CURATED_TITLES
        .iter()
        .find(|(t, _)| *t == title)
        .map(|(_, slug)| *slug)
}

/// Clamp a requested level into the topic's valid range (nearest valid).
pub fn clamp_level(topic: &Topic, level: &str) -> &'static str {
    if topic.levels.contains(&level) {
        // Safety: levels come from static literals.
        topic.levels.iter().find(|l| **l == level).copied().unwrap()
    } else {
        topic.levels[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_is_wellformed() {
        assert!(TOPICS.len() >= 8);
        for t in TOPICS {
            assert!(!t.slug.is_empty(), "{}", t.slug);
            assert!(t.levels.len() >= 1);
            assert!(t.levels.iter().all(|l| ["A1", "A2", "B1", "B2"].contains(l)));
        }
    }

    #[test]
    fn slug_lookup_roundtrip() {
        for t in TOPICS {
            assert_eq!(by_slug(t.slug).unwrap().title, t.title);
        }
    }

    #[test]
    fn curated_titles_map_to_catalog() {
        // Every topic title in the curated corpus maps somewhere.
        for title in [
            "Passé composé with être",
            "Auxiliary choice",
            "Imparfait vs passé composé",
            "Subjonctif after feelings",
            "Subjonctif after doubt",
            "Passé composé with avoir",
            "Imparfait of -er verbs",
            "Imparfait of être",
            "Imparfait of venir",
            "Direct object pronouns",
            "Indirect object pronouns",
            "Object pronouns",
            "Double object pronouns",
            "The pronoun y",
            "C'est vs il/elle est",
            "Negation",
            "Agreement with preceding object",
        ] {
            assert!(slug_for_title(title).is_some(), "unmapped: {}", title);
        }
    }

    #[test]
    fn clamps_out_of_range_level() {
        let t = by_slug("pc-etre").unwrap();
        assert_eq!(clamp_level(t, "B2"), "A2");
        assert_eq!(clamp_level(t, "B1"), "B1");
    }

    #[test]
    fn grid_sections_covered() {
        // The progress grid has a column per section; each must carry
        // at least one topic so no column renders permanently hollow.
        for section in [Section::Tense, Section::Mood, Section::Nouns, Section::Sentence] {
            assert!(
                TOPICS.iter().any(|t| t.section == section),
                "no topic in section {:?}",
                section
            );
        }
        // Every curated title maps to a topic that exists.
        for (_, slug) in CURATED_TITLES {
            assert!(by_slug(slug).is_some(), "curated title maps to unknown slug {}", slug);
        }
    }
}
