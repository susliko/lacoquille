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
    /// Futur simple — verb tables with tomorrow-onward cues.
    FuturSimple,
    /// Verbs with prepositions — à/de + infinitive pairs.
    VerbsPreps,
    /// Comparison — comparative (plus/moins/aussi … que) and superlative.
    Compare,
    /// The pronoun en — de + thing replaced, j'en elision trap.
    PronounEn,
    /// A2 pronoun cluster — relatives, possessive pronouns, tout, ça/cela/ceci.
    A2Pronouns,
    /// A2 sentence cluster — exclamations, cause, purpose/opposition, sequencing.
    SentencesA2,
    /// B1 past-tense cluster — plus-que-parfait, narrative tenses.
    B1Tenses,
    /// Conditionnel — présent forms and uses (B2: conditionnel passé).
    Conditionnel,
    /// B1 subjonctif — présent forms of irregular stems, vs indicatif.
    SubjonctifB1,
    /// Passive voice — être + agreed participle, par-agent, on-shortcut.
    Passive,
    /// Gérondif — en + participe présent, formation traps.
    Gerondif,
    /// B1 pronoun cluster — double object pronouns, indefinite pronouns.
    B1Pronouns,
    /// B1 relative cluster — dont, compound relatives, ce qui/ce que/ce dont.
    B1Relatives,
    /// Si-clauses — présent→futur, imparfait→conditionnel (B2: +pqp).
    SiClauses,
    /// B1 sentence patterns — reported speech, restriction, cleft, presentative.
    B1Sentence,
    /// Nominalisation — verbs into gendered nouns.
    Nominalisation,
    /// B1 connector cluster — cause, purpose, concession, time markers.
    B1Connectors,
    /// Anteriority — futur antérieur, past infinitive.
    Anteriorite,
    /// B2 subjonctif — verbs & impersonals, conjunctions, subjonctif passé.
    SubjonctifB2,
    /// B2 tenses — sequence of tenses, exceptional participle agreement.
    B2Tenses,
    /// Participle vs verbal adjective pairs.
    ParticipeAdjectif,
    /// Interrogative lequel.
    B2Pronouns,
    /// B2 sentence patterns — au cas où, opposition, purpose & cause.
    B2Sentence,
    /// C1 tense cluster — passé simple & passé antérieur recognition,
    /// pronominal-verb agreement.
    C1Tenses,
    /// C1 mood cluster — subtle subjonctif, mood vs meaning,
    /// attenuating conditionnel, infinitive clauses, literary
    /// hypotheticals.
    C1Moods,
    /// C1 noun group — quelque/quel que, auquel/duquel, ellipsis.
    C1Nouns,
    /// C1 sentence patterns — avoir beau, formal connectors,
    /// inversion, expletive ne, impersonals, register.
    C1Sentence,
    /// C2 tense cluster — passé simple all persons, fine participle
    /// agreement.
    C2Tenses,
    /// C2 mood cluster — subjonctif imparfait, conditionnel passé 2e
    /// forme.
    C2Moods,
    /// C2 style cluster — adjective position, literary negations,
    /// word order, archaisms, register shifts.
    C2Style,
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
    // ── A2 tense & verb constructions ───────────────────────────
    Topic {
        slug: "futur-simple",
        title: "Futur simple",
        blurb: "Talking about tomorrow and later: endings -ai, -as, -a, -ons, -ez, -ont on the stem, irregulars included (je serai, j'irai, je ferai).",
        levels: &["A2"],
        family: Family::FuturSimple,
        section: Section::Tense,
    },
    Topic {
        slug: "verbs-prepositions",
        title: "Verbs with prepositions",
        blurb: "Which preposition joins the conjugated verb to its infinitive: aider à, apprendre à, réussir à, versus décider de, essayer de, oublier de.",
        levels: &["A2"],
        family: Family::VerbsPreps,
        section: Section::Mood,
    },
    Topic {
        slug: "comparative",
        title: "Comparative",
        blurb: "Comparing with plus/moins/aussi … que (le TGV est plus rapide que le vélo) — plus for more, moins for less, aussi for equal.",
        levels: &["A2"],
        family: Family::Compare,
        section: Section::Nouns,
    },
    Topic {
        slug: "superlative",
        title: "Superlative",
        blurb: "The extreme form: le/la/les plus + adjective agreeing with the noun (la plus belle), and bon → meilleur, mauvais → pire.",
        levels: &["A2"],
        family: Family::Compare,
        section: Section::Nouns,
    },
    // ── A2 pronouns ─────────────────────────────────────────────
    Topic {
        slug: "pronoun-en",
        title: "The pronoun en",
        blurb: "Replacing de + thing with en before the verb (j'en parle, tu en manges) — and the j'en elision trap.",
        levels: &["A2"],
        family: Family::PronounEn,
        section: Section::Nouns,
    },
    Topic {
        slug: "uses-of-on",
        title: "Uses of on",
        blurb: "Beyond on = nous: on for people in general (ici on parle français) — always with the 3rd-person singular form.",
        levels: &["A2"],
        family: Family::PronounBasics,
        section: Section::Nouns,
    },
    Topic {
        slug: "qui-que-ou",
        title: "Qui, que, où",
        blurb: "Joining clauses with relative pronouns: qui for the subject, que for the object, où for place and time (la ville où j'habite).",
        levels: &["A2"],
        family: Family::A2Pronouns,
        section: Section::Nouns,
    },
    Topic {
        slug: "possessive-pronouns",
        title: "Possessive pronouns",
        blurb: "The standing-alone possessives: le mien, la mienne, les nôtres — gender and number come from the thing, not the owner.",
        levels: &["A2"],
        family: Family::A2Pronouns,
        section: Section::Nouns,
    },
    Topic {
        slug: "tout-toute-tous",
        title: "Tout, toute, tous, toutes",
        blurb: "The four faces of tout: tout le temps, toute la journée, tous les amis, toutes les fenêtres — agreeing with what follows.",
        levels: &["A2"],
        family: Family::A2Pronouns,
        section: Section::Nouns,
    },
    Topic {
        slug: "ca-cela-ceci",
        title: "Ça, cela, ceci",
        blurb: "Pointing without a noun: ceci (this, near), cela/ça (that, general) — and when everyday ça replaces both.",
        levels: &["A2"],
        family: Family::A2Pronouns,
        section: Section::Nouns,
    },
    // ── A2 sentence ─────────────────────────────────────────────
    Topic {
        slug: "ne-plus-rien-personne",
        title: "Ne… plus, rien, personne",
        blurb: "Negation beyond ne … pas: ne … plus (not anymore), ne … rien (nothing), ne … personne (nobody) — same wrapping, stronger meanings.",
        levels: &["A2"],
        family: Family::Negation,
        section: Section::Sentence,
    },
    Topic {
        slug: "exclamations",
        title: "Exclamations",
        blurb: "Exclaiming: quel agreeing with its noun (quelle belle vue !) and que/comme as intensifiers (qu'il fait chaud !).",
        levels: &["A2"],
        family: Family::SentencesA2,
        section: Section::Sentence,
    },
    Topic {
        slug: "cause-consequence",
        title: "Cause & consequence",
        blurb: "Linking cause and result: parce que before the cause (parce qu'il pleut), donc before the result, à cause de + noun for blame.",
        levels: &["A2"],
        family: Family::SentencesA2,
        section: Section::Sentence,
    },
    Topic {
        slug: "purpose-opposition",
        title: "Purpose & opposition",
        blurb: "pour + infinitive for purpose (j'étudie pour réussir), and despite it all: malgré + noun, même si + clause.",
        levels: &["A2"],
        family: Family::SentencesA2,
        section: Section::Sentence,
    },
    Topic {
        slug: "sequencing-connectors",
        title: "Sequencing connectors",
        blurb: "Ordering a story: d'abord, ensuite, puis, enfin — first, then, finally.",
        levels: &["A2"],
        family: Family::SentencesA2,
        section: Section::Sentence,
    },
    // ── B1 tense & verb forms ───────────────────────────────────
    Topic {
        slug: "plus-que-parfait",
        title: "Plus-que-parfait",
        blurb: "The earlier of two past events: imparfait of the auxiliary + past participle — quand je suis arrivé, il avait déjà mangé.",
        levels: &["B1"],
        family: Family::B1Tenses,
        section: Section::Tense,
    },
    Topic {
        slug: "past-tenses-story",
        title: "Past tenses in a story",
        blurb: "Choosing between imparfait (background), passé composé (events) and plus-que-parfait (anteriority) inside a short narrative.",
        levels: &["B1"],
        family: Family::B1Tenses,
        section: Section::Tense,
    },
    Topic {
        slug: "conditionnel-forms",
        title: "Conditionnel présent: forms",
        blurb: "Futur stem + imparfait endings: il a promis qu'il viendrait — the future seen from the past.",
        levels: &["B1"],
        family: Family::Conditionnel,
        section: Section::Mood,
    },
    Topic {
        slug: "conditionnel-uses",
        title: "Conditionnel présent: uses",
        blurb: "Softening with the conditionnel: pourriez-vous (requests), je voudrais (wishes), tu devrais (advice).",
        levels: &["B1"],
        family: Family::Conditionnel,
        section: Section::Mood,
    },
    Topic {
        slug: "subjonctif-forms",
        title: "Subjonctif présent: forms",
        blurb: "The irregular subjonctif stems: que je sois, que j'aie, que j'aille, que je fasse, que je puisse.",
        levels: &["B1"],
        family: Family::SubjonctifB1,
        section: Section::Mood,
    },
    Topic {
        slug: "subjonctif-vs-indicatif",
        title: "Subjonctif vs indicatif",
        blurb: "Je veux qu'il vienne but je pense qu'il vient — wish and possibility take the subjonctif, belief and certainty keep the indicatif.",
        levels: &["B1"],
        family: Family::SubjonctifB1,
        section: Section::Mood,
    },
    Topic {
        slug: "passive-voice",
        title: "Passive voice",
        blurb: "être + past participle agreeing with the subject: la lettre a été envoyée — and the « on » shortcut to an agentless passive.",
        levels: &["B1"],
        family: Family::Passive,
        section: Section::Mood,
    },
    Topic {
        slug: "gerondif",
        title: "Gérondif",
        blurb: "en + participe présent for simultaneity and means: il maigrit en mangeant moins — with its spelling traps (mangeant, commençant, appelant).",
        levels: &["B1"],
        family: Family::Gerondif,
        section: Section::Mood,
    },
    // ── B1 noun group ───────────────────────────────────────────
    Topic {
        slug: "nominalization",
        title: "Nominalization",
        blurb: "Turning verbs into nouns: décider → la décision, réussir → la réussite — with the gender you must learn alongside.",
        levels: &["B1"],
        family: Family::Nominalisation,
        section: Section::Nouns,
    },
    Topic {
        slug: "double-pronouns",
        title: "Double object pronouns",
        blurb: "Two pronouns before the verb in a fixed order: je le lui donne — le/la/les first, lui/leur second.",
        levels: &["B1"],
        family: Family::B1Pronouns,
        section: Section::Nouns,
    },
    Topic {
        slug: "dont",
        title: "Dont",
        blurb: "The relative for verb + de: le film dont je parle, le projet dont on est fier — and the possessive dont l'histoire.",
        levels: &["B1"],
        family: Family::B1Relatives,
        section: Section::Nouns,
    },
    Topic {
        slug: "compound-relatives",
        title: "Compound relatives",
        blurb: "Preposition + lequel agreeing with its antecedent: la ville dans laquelle j'habite, les amis avec lesquels je sors.",
        levels: &["B1"],
        family: Family::B1Relatives,
        section: Section::Nouns,
    },
    Topic {
        slug: "indefinite-pronouns",
        title: "Indefinite pronouns",
        blurb: "quelqu'un, quelque chose, chacun, tous — including quelqu'un de gentil, where the adjective stays masculine.",
        levels: &["B1"],
        family: Family::B1Pronouns,
        section: Section::Nouns,
    },
    Topic {
        slug: "ce-qui-ce-que-ce-dont",
        title: "Ce qui, ce que, ce dont",
        blurb: "Relative pronouns without an antecedent noun: ce qui m'étonne, ce que je veux, ce dont j'ai besoin.",
        levels: &["B1"],
        family: Family::B1Relatives,
        section: Section::Nouns,
    },
    // ── B1 sentence ─────────────────────────────────────────────
    Topic {
        slug: "si-present",
        title: "Si + présent",
        blurb: "Real possibility: si tu viens, on ira — présent after si, futur in the result, never si + futur.",
        levels: &["B1"],
        family: Family::SiClauses,
        section: Section::Sentence,
    },
    Topic {
        slug: "si-imparfait",
        title: "Si + imparfait",
        blurb: "The unreal present: si j'avais le temps, je viendrais — imparfait after si, conditionnel in the result.",
        levels: &["B1"],
        family: Family::SiClauses,
        section: Section::Sentence,
    },
    Topic {
        slug: "reported-speech",
        title: "Reported speech (present)",
        blurb: "Reporting with present tenses: il dit que…, il demande si… — statements with que, yes/no questions with si.",
        levels: &["B1"],
        family: Family::B1Sentence,
        section: Section::Sentence,
    },
    Topic {
        slug: "restriction-ne-que",
        title: "Ne… que, aucun, ni… ni",
        blurb: "Restrictive negation: je ne mange que des légumes (only), aucun doute (not a single), ni café ni thé (neither).",
        levels: &["B1"],
        family: Family::B1Sentence,
        section: Section::Sentence,
    },
    Topic {
        slug: "cause-consequence-extended",
        title: "Cause & consequence (extended)",
        blurb: "Beyond parce que: grâce à vs à cause de, c'est pourquoi, si bien que, à force de.",
        levels: &["B1"],
        family: Family::B1Connectors,
        section: Section::Sentence,
    },
    Topic {
        slug: "pour-que-afin-de",
        title: "Pour que, afin de",
        blurb: "Purpose with a subject change: je répète pour que tu comprennes — same subject keeps pour + infinitive.",
        levels: &["B1"],
        family: Family::B1Connectors,
        section: Section::Sentence,
    },
    Topic {
        slug: "concession-opposition",
        title: "Concession & opposition",
        blurb: "Despite it all: bien que + subjonctif (bien qu'il soit malade), malgré + noun, même si + indicatif.",
        levels: &["B1"],
        family: Family::B1Connectors,
        section: Section::Sentence,
    },
    Topic {
        slug: "ce-qui-cest",
        title: "Ce qui…, c'est",
        blurb: "Emphasis with a cleft: ce qui m'étonne, c'est son courage — the written comma before c'est.",
        levels: &["B1"],
        family: Family::B1Sentence,
        section: Section::Sentence,
    },
    Topic {
        slug: "il-est-cest-adjectif-de",
        title: "Il est / c'est + adjective + de",
        blurb: "Impersonal judgement: il est facile d'apprendre le français — with the d' elision and the spoken c'est.",
        levels: &["B1"],
        family: Family::B1Sentence,
        section: Section::Sentence,
    },
    Topic {
        slug: "narrative-time-markers",
        title: "Narrative time markers",
        blurb: "Setting the rhythm of a story: soudain, pendant que, alors que, enfin, au moment où.",
        levels: &["B1"],
        family: Family::B1Connectors,
        section: Section::Sentence,
    },
    // ── B2 tense & verb forms ───────────────────────────────────
    Topic {
        slug: "futur-anterieur",
        title: "Futur antérieur",
        blurb: "The earlier future: quand tu seras arrivé, nous serons prêts — futur of the auxiliary + past participle.",
        levels: &["B2"],
        family: Family::Anteriorite,
        section: Section::Tense,
    },
    Topic {
        slug: "past-infinitive",
        title: "Past infinitive",
        blurb: "Anteriority in one clause: après avoir fini, après être arrivé — the same auxiliary as the passé composé.",
        levels: &["B2"],
        family: Family::Anteriorite,
        section: Section::Tense,
    },
    Topic {
        slug: "sequence-of-tenses",
        title: "Sequence of tenses",
        blurb: "A past main clause pulls the subordinate back: il a dit qu'il viendrait — present keeps the futur.",
        levels: &["B2"],
        family: Family::B2Tenses,
        section: Section::Tense,
    },
    Topic {
        slug: "exceptional-participle",
        title: "Exceptional participle agreement",
        blurb: "With avoir, the participle agrees with a preceding direct object: les fleurs que j'ai cueillies.",
        levels: &["B2"],
        family: Family::B2Tenses,
        section: Section::Tense,
    },
    Topic {
        slug: "conditionnel-passe",
        title: "Conditionnel passé",
        blurb: "The unreal past result: si tu m'avais écouté, j'aurais fini — avoir/être in the conditionnel + participle.",
        levels: &["B2"],
        family: Family::Conditionnel,
        section: Section::Mood,
    },
    Topic {
        slug: "subjonctif-verbs-impersonals",
        title: "Subjonctif after verbs & impersonals",
        blurb: "Beyond il faut: craindre que, redouter que, attendre que take the subjonctif; être sûr que, admettre que keep the indicatif.",
        levels: &["B2"],
        family: Family::SubjonctifB2,
        section: Section::Mood,
    },
    Topic {
        slug: "subjonctif-conjunctions",
        title: "Subjonctif after conjunctions",
        blurb: "The conjunction series: bien que, avant que, jusqu'à ce que, sans que, pourvu que — all subjonctif.",
        levels: &["B2"],
        family: Family::SubjonctifB2,
        section: Section::Mood,
    },
    Topic {
        slug: "subjonctif-passe",
        title: "Subjonctif passé",
        blurb: "A completed action inside the subjonctif: je suis content qu'il ait fini, qu'elle soit partie.",
        levels: &["B2"],
        family: Family::SubjonctifB2,
        section: Section::Mood,
    },
    Topic {
        slug: "participe-ou-adjectif",
        title: "Participle vs verbal adjective",
        blurb: "un film amusant (it amuses) vs des enfants amusés (they are amused); fatigant/fatigué and its traps.",
        levels: &["B2"],
        family: Family::ParticipeAdjectif,
        section: Section::Mood,
    },
    // ── B2 pronouns ─────────────────────────────────────────────
    Topic {
        slug: "lequel-interrogatif",
        title: "Interrogative lequel",
        blurb: "Asking with lequel: laquelle de ces robes — the question word agrees with the thing asked about.",
        levels: &["B2"],
        family: Family::B2Pronouns,
        section: Section::Nouns,
    },
    // ── B2 sentence ─────────────────────────────────────────────
    Topic {
        slug: "si-plus-que-parfait",
        title: "Si + plus-que-parfait",
        blurb: "The unreal past: si j'avais su, je serais venu — plus-que-parfait after si, conditionnel passé in the result.",
        levels: &["B2"],
        family: Family::SiClauses,
        section: Section::Sentence,
    },
    Topic {
        slug: "au-cas-ou-condition",
        title: "Au cas où, à condition que",
        blurb: "Contingency vs requirement: au cas où il pleuvrait (conditionnel), à condition que tu sois prêt (subjonctif).",
        levels: &["B2"],
        family: Family::B2Sentence,
        section: Section::Sentence,
    },
    Topic {
        slug: "opposition-avancee",
        title: "Advanced opposition & concession",
        blurb: "Formal opposition: néanmoins, toutefois, en revanche — and the extreme concessions quoique, quand bien même.",
        levels: &["B2"],
        family: Family::B2Sentence,
        section: Section::Sentence,
    },
    Topic {
        slug: "but-cause-avancee",
        title: "Advanced purpose & cause",
        blurb: "Formal purpose and cause: de peur que, de sorte que, afin de, vu que, étant donné que.",
        levels: &["B2"],
        family: Family::B2Sentence,
        section: Section::Sentence,
    },
    // ── C1 tense & verb forms ───────────────────────────────────
    Topic {
        slug: "passe-simple-recognize",
        title: "Passé simple (recognize)",
        blurb: "The tense of written narrative: il ouvrit la porte — spot the completed literary event, never confused with the imparfait again.",
        levels: &["C1"],
        family: Family::C1Tenses,
        section: Section::Tense,
    },
    Topic {
        slug: "passe-anterieur",
        title: "Passé antérieur (recognize)",
        blurb: "Literary anteriority: quand il eut fini, on partit — passé simple of the auxiliary + participle, mostly after quand and à peine.",
        levels: &["C1"],
        family: Family::C1Tenses,
        section: Section::Tense,
    },
    Topic {
        slug: "pronominal-agreement",
        title: "Agreement of pronominal verbs",
        blurb: "Elle s'est levée but elle s'est lavé les mains — whether se is direct or indirect decides the participle.",
        levels: &["C1"],
        family: Family::C1Tenses,
        section: Section::Tense,
    },
    // ── C1 moods ────────────────────────────────────────────────
    Topic {
        slug: "subjonctif-subtle",
        title: "Subtle subjonctif cases",
        blurb: "avant que + subjonctif but après que + indicatif; de peur que, sans que, loin que, autant que je sache.",
        levels: &["C1"],
        family: Family::C1Moods,
        section: Section::Mood,
    },
    Topic {
        slug: "subjonctif-meaning",
        title: "Subjonctif vs indicatif meaning",
        blurb: "un assistant qui ait (any will do) vs l'assistant qui a (the one who does) — the mood itself carries the meaning.",
        levels: &["C1"],
        family: Family::C1Moods,
        section: Section::Mood,
    },
    Topic {
        slug: "conditionnel-attenuation",
        title: "Attenuating conditionnel",
        blurb: "The hedging tense of journalists and polite people: il semblerait que, la décision aurait des conséquences, vous devriez…",
        levels: &["C1"],
        family: Family::C1Moods,
        section: Section::Mood,
    },
    Topic {
        slug: "infinitive-clauses",
        title: "Infinitive clauses",
        blurb: "faire réparer sa voiture, écouter les enfants chanter — perception and causative verbs with a controlled infinitive.",
        levels: &["C1"],
        family: Family::C1Moods,
        section: Section::Mood,
    },
    Topic {
        slug: "literary-hypotheticals",
        title: "Literary hypotheticals",
        blurb: "Qui l'eût cru ? — the literary conditionnel passé (eût fait = aurait fait) behind Maupassant's conditional world.",
        levels: &["C1"],
        family: Family::C1Moods,
        section: Section::Mood,
    },
    // ── C1 noun group ───────────────────────────────────────────
    Topic {
        slug: "quelque-quel-que",
        title: "Quelque, quel que",
        blurb: "One space or one letter decides: quelque paresseux qu'il soit, quel que soit le prix, quoi qu'il dise.",
        levels: &["C1"],
        family: Family::C1Nouns,
        section: Section::Nouns,
    },
    Topic {
        slug: "auquel-duquel",
        title: "Auquel, duquel & co.",
        blurb: "Compound relatives with à and de: le projet auquel je pense, le voisin duquel je connais la fille.",
        levels: &["C1"],
        family: Family::C1Nouns,
        section: Section::Nouns,
    },
    Topic {
        slug: "ellipsis-repetition",
        title: "Ellipsis & avoiding repetition",
        blurb: "celle de son frère, je les ai déjà pris — replacing nouns with celui/celle/le/la/les so prose stops repeating itself.",
        levels: &["C1"],
        family: Family::C1Nouns,
        section: Section::Nouns,
    },
    // ── C1 sentence ─────────────────────────────────────────────
    Topic {
        slug: "avoir-beau",
        title: "Avoir beau, quand bien même",
        blurb: "In-vain concession: il a beau pleuvoir, le pique-nique est maintenu — and the extreme quand bien même il pleuvrait.",
        levels: &["C1"],
        family: Family::C1Sentence,
        section: Section::Sentence,
    },
    Topic {
        slug: "formal-connectors",
        title: "Formal connectors",
        blurb: "The written connective tissue: en effet, par conséquent, en outre, pourtant, quant à, or (not hors!).",
        levels: &["C1"],
        family: Family::C1Sentence,
        section: Section::Sentence,
    },
    Topic {
        slug: "formal-inversion",
        title: "Formal inversion",
        blurb: "Peut-être viendra-t-il, rare sont les occasions — the stylistic inversion after fronted words, hyphen and all.",
        levels: &["C1"],
        family: Family::C1Sentence,
        section: Section::Sentence,
    },
    Topic {
        slug: "expletive-ne",
        title: "Expletive ne",
        blurb: "The ne that negates nothing: je crains qu'il ne vienne, plus qu'on ne le croit — pure literary polish.",
        levels: &["C1"],
        family: Family::C1Sentence,
        section: Section::Sentence,
    },
    Topic {
        slug: "impersonal-formal",
        title: "Impersonal formal style",
        blurb: "il convient de, il s'agit de, il importe que, il ne tient qu'à vous de — the impersonal machinery of formal French.",
        levels: &["C1"],
        family: Family::C1Sentence,
        section: Section::Sentence,
    },
    Topic {
        slug: "spoken-vs-written",
        title: "Spoken vs written French",
        blurb: "ça vs cela, on vs nous, y a vs il existe — the same thought, dressed for the chat or for the essay.",
        levels: &["C1"],
        family: Family::C1Sentence,
        section: Section::Sentence,
    },
    // ── C2 tense & verb forms ───────────────────────────────────
    Topic {
        slug: "passe-simple-all-persons",
        title: "Passé simple, all persons",
        blurb: "The full literary conjugation: je vins, tu fis, il courut, nous fûmes, vous vîtes, ils lurent.",
        levels: &["C2"],
        family: Family::C2Tenses,
        section: Section::Tense,
    },
    Topic {
        slug: "fine-participle-agreement",
        title: "Fine participle agreement",
        blurb: "les fleurs que je lui ai données but elle s'est rendu compte — the preceding-object rule with all its edge cases.",
        levels: &["C2"],
        family: Family::C2Tenses,
        section: Section::Tense,
    },
    // ── C2 moods ────────────────────────────────────────────────
    Topic {
        slug: "subjonctif-imparfait",
        title: "Subjonctif imparfait",
        blurb: "The literary past subjonctif: la reine souhaitait qu'il vînt, qui qu'il fût — recognise it, decode it, admire it.",
        levels: &["C2"],
        family: Family::C2Moods,
        section: Section::Mood,
    },
    Topic {
        slug: "conditionnel-passe-deuxieme",
        title: "Conditionnel passé 2e forme",
        blurb: "S'il avait su, il eût refusé — the literary second form of the past conditional behind qui l'eût cru.",
        levels: &["C2"],
        family: Family::C2Moods,
        section: Section::Mood,
    },
    // ── C2 noun group ───────────────────────────────────────────
    Topic {
        slug: "adjective-position-meaning",
        title: "Adjective position & meaning",
        blurb: "un grand homme vs un homme grand, mon ancien professeur vs des meubles anciens — position rewrites the meaning.",
        levels: &["C2"],
        family: Family::C2Style,
        section: Section::Nouns,
    },
    // ── C2 sentence ─────────────────────────────────────────────
    Topic {
        slug: "literary-negations",
        title: "Literary negations",
        blurb: "nul ne sait, point n'est besoin, elle ne parlait guère — the written negation family beyond ne… pas.",
        levels: &["C2"],
        family: Family::C2Style,
        section: Section::Sentence,
    },
    Topic {
        slug: "stylistic-word-order",
        title: "Stylistic word order",
        blurb: "Grande fut sa surprise, franchi le seuil, et voici que la pluie cessa — fronted attributes and participles in prose.",
        levels: &["C2"],
        family: Family::C2Style,
        section: Section::Sentence,
    },
    Topic {
        slug: "archaic-phrases",
        title: "Archaic fixed phrases",
        blurb: "il y a belle lurette, de par la loi, chose promise chose due, le cas échéant — formulas that time forgot.",
        levels: &["C2"],
        family: Family::C2Style,
        section: Section::Sentence,
    },
    Topic {
        slug: "register-shifts",
        title: "Register shifts",
        blurb: "Rewrite on a eu des soucis into nous avons rencontré des difficultés — moving a sentence across registers.",
        levels: &["C2"],
        family: Family::C2Style,
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
            assert!(t
                .levels
                .iter()
                .all(|l| ["A1", "A2", "B1", "B2", "C1", "C2"].contains(l)));
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
