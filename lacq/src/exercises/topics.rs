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
        family: Family::Curated,
        section: Section::Tense,
    },
    Topic {
        slug: "imparfait",
        title: "Imparfait forms",
        blurb: "Forming the imparfait of common verbs (je parlais, nous étions, elles venaient).",
        levels: A1A2,
        family: Family::Curated,
        section: Section::Tense,
    },
    Topic {
        slug: "pronoun-y",
        title: "The pronoun y",
        blurb: "Replacing à + place with y and placing it before the verb (elle va à Paris → elle y va).",
        levels: &["B1"],
        family: Family::Curated,
        section: Section::Nouns,
    },
    Topic {
        slug: "cest-vs-il-est",
        title: "C'est vs il/elle est",
        blurb: "Choosing between c'est + noun and il/elle est + adjective (c'est une bonne idée, c'est évident).",
        levels: A1A2,
        family: Family::Curated,
        section: Section::Sentence,
    },
    Topic {
        slug: "negation",
        title: "Negation",
        blurb: "Wrapping the conjugated verb in ne … pas (elle ne parle pas anglais).",
        levels: &["A1"],
        family: Family::Curated,
        section: Section::Sentence,
    },
    Topic {
        slug: "accord-antepose",
        title: "Agreement with preceding object",
        blurb: "Making the participle agree when the direct object precedes the verb (les fleurs que j'ai achetées).",
        levels: &["B1"],
        family: Family::Curated,
        section: Section::Tense,
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
