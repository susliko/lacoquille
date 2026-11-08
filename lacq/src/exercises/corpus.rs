//! Curated French exercise corpus. Hand-authored templates — the quality
//! bar for generated exercises is the same as for the designed static
//! session (Penpot frames 02–10). Extend by appending to the pools; IDs
//! must stay stable because the client keys its per-item history on them.

pub const ALL_LEVELS: &[&str] = &["A1", "A2", "B1", "B2"];

/* ── Cloze: subject-swappable sentences ─────────────────────────── */

pub struct ClozePerson {
    /// Stable key used in the item id ("nous", "je", …).
    pub key: &'static str,
    pub head: &'static str,
    pub tail: &'static str,
    pub answer: &'static str,
}

pub struct ClozeTemplate {
    pub id: &'static str,
    pub level: &'static str,
    pub topic: &'static str,
    pub task: &'static [&'static str],
    pub infinitive: &'static str,
    pub rule: &'static [&'static str],
    pub persons: &'static [ClozePerson],
}

pub const CLOZE: &[ClozeTemplate] = &[
    ClozeTemplate {
        id: "etre-pc-rester",
        level: "A2",
        topic: "Passé composé with être",
        task: &["Put the verb in the passé composé."],
        infinitive: "rester",
        rule: &[
            "Rester takes être in the passé composé, and the",
            "participle agrees with the subject.",
        ],
        persons: &[
            ClozePerson { key: "nous", head: "Hier soir, nous", tail: "à la maison.", answer: "sommes restés" },
            ClozePerson { key: "je", head: "Hier soir, je", tail: "à la maison.", answer: "suis resté" },
            ClozePerson { key: "elles", head: "Hier soir, elles", tail: "à la maison.", answer: "sont restées" },
        ],
    },
    ClozeTemplate {
        id: "subjonctif-sentiment-venir",
        level: "B1",
        topic: "Subjonctif after feelings",
        task: &["Put the verb in the right form."],
        infinitive: "venir",
        rule: &[
            "Être content que expresses a feeling, so the",
            "clause takes the subjonctif.",
        ],
        persons: &[
            ClozePerson { key: "tu", head: "Je suis content que tu", tail: ".", answer: "viennes" },
            ClozePerson { key: "il", head: "Je suis content qu'il", tail: ".", answer: "vienne" },
            ClozePerson { key: "vous", head: "Je suis content que vous", tail: ".", answer: "veniez" },
        ],
    },
    ClozeTemplate {
        id: "imparfait-parler",
        level: "A1",
        topic: "Imparfait of -er verbs",
        task: &["Put the verb in the imparfait."],
        infinitive: "parler",
        rule: &[
            "-er verbs take one imparfait stem for everyone:",
            "parl- + -ais · -ait · -ions · -iez · -aient.",
        ],
        persons: &[
            ClozePerson { key: "je", head: "Le dimanche, je", tail: "au téléphone.", answer: "parlais" },
            ClozePerson { key: "nous", head: "Le dimanche, nous", tail: "au téléphone.", answer: "parlions" },
            ClozePerson { key: "elles", head: "Le dimanche, elles", tail: "au téléphone.", answer: "parlaient" },
        ],
    },
    ClozeTemplate {
        id: "avoir-pc-manger",
        level: "A1",
        topic: "Passé composé with avoir",
        task: &["Put the verb in the passé composé."],
        infinitive: "manger",
        rule: &[
            "Passé composé with avoir: avoir in the présent +",
            "the past participle — manger → mangé.",
        ],
        persons: &[
            ClozePerson { key: "nous", head: "Hier, nous", tail: "au restaurant italien.", answer: "avons mangé" },
            ClozePerson { key: "ils", head: "Hier, ils", tail: "au restaurant italien.", answer: "ont mangé" },
            ClozePerson { key: "je", head: "Hier, je", tail: "au restaurant italien.", answer: "ai mangé" },
        ],
    },
    ClozeTemplate {
        id: "imparfait-etre",
        level: "A1",
        topic: "Imparfait of être",
        task: &["Put the verb in the imparfait."],
        infinitive: "être",
        rule: &[
            "Être imparfait: j'étais, tu étais, il était…",
            "— the circumflex sits on the stem.",
        ],
        persons: &[
            ClozePerson { key: "elle", head: "Elle", tail: "très timide quand elle était enfant.", answer: "était" },
            ClozePerson { key: "nous", head: "Nous", tail: "très timides quand nous étions enfants.", answer: "étions" },
        ],
    },
    ClozeTemplate {
        id: "imparfait-venir",
        level: "A2",
        topic: "Imparfait of venir",
        task: &["Put the verb in the imparfait."],
        infinitive: "venir",
        rule: &["Venir imparfait: ven- + imparfait endings."],
        persons: &[
            ClozePerson { key: "ils", head: "Les étés, ils", tail: "souvent chez leurs grands-parents.", answer: "venaient" },
            ClozePerson { key: "nous", head: "Les étés, nous", tail: "souvent chez nos grands-parents.", answer: "venions" },
        ],
    },
];

/* ── Choice ─────────────────────────────────────────────────────── */

pub struct ChoiceTemplate {
    pub id: &'static str,
    pub level: &'static str,
    pub topic: &'static str,
    pub task: &'static [&'static str],
    pub sentence: &'static str,
    pub options: &'static [&'static str],
    /// Index into `options` before shuffling.
    pub correct: usize,
    pub rule: &'static [&'static str],
}

pub const CHOICE: &[ChoiceTemplate] = &[
    ChoiceTemplate {
        id: "cest-idee",
        level: "A1",
        topic: "C'est vs il/elle est",
        task: &["Choose the right opening."],
        sentence: "____ une très bonne idée !",
        options: &["C'est", "Il est", "Elle est"],
        correct: 0,
        rule: &[
            "C'est precedes a noun with its article: c'est une",
            "idée. Il est / elle est precede an adjective alone.",
        ],
    },
    ChoiceTemplate {
        id: "imparfait-marche",
        level: "A2",
        topic: "Imparfait vs passé composé",
        task: &["Choose the right form."],
        sentence: "Hier matin, je ____ au marché quand j'ai vu Marie.",
        options: &["j'allais", "je suis allé"],
        correct: 0,
        rule: &[
            "The ongoing background takes the imparfait; the",
            "interrupting event (j'ai vu) takes the passé composé.",
        ],
    },
    ChoiceTemplate {
        id: "imparfait-parc",
        level: "A1",
        topic: "Imparfait vs passé composé",
        task: &["Choose the right form."],
        sentence: "Quand j'étais petit, je ____ au parc tous les jours.",
        options: &["j'allais", "je suis allé"],
        correct: 0,
        rule: &[
            "A habit (tous les jours) is imparfait — not a",
            "single completed event.",
        ],
    },
    ChoiceTemplate {
        id: "pc-soudain",
        level: "A2",
        topic: "Imparfait vs passé composé",
        task: &["Choose the right form."],
        sentence: "Soudain, il ____ dans la rue.",
        options: &["est tombé", "tombait"],
        correct: 0,
        rule: &[
            "Soudain, tout à coup mark a punctual event —",
            "the passé composé.",
        ],
    },
    ChoiceTemplate {
        id: "imparfait-pendant",
        level: "A2",
        topic: "Imparfait vs passé composé",
        task: &["Choose the right form."],
        sentence: "Pendant que je ____, le téléphone a sonné.",
        options: &["dormais", "ai dormi"],
        correct: 0,
        rule: &[
            "Pendant que sets the ongoing background —",
            "imparfait against the passé composé event.",
        ],
    },
    ChoiceTemplate {
        id: "cest-evident",
        level: "A2",
        topic: "C'est vs il/elle est",
        task: &["Choose the right opening."],
        sentence: "— Il est malade ? — Oui, ____ évident.",
        options: &["c'est", "il est"],
        correct: 0,
        rule: &[
            "Before a standalone adjective (évident), French",
            "uses c'est: c'est évident.",
        ],
    },
];

/* ── Tiles ──────────────────────────────────────────────────────── */

pub struct TilesTemplate {
    pub id: &'static str,
    pub level: &'static str,
    pub topic: &'static str,
    pub task: &'static [&'static str],
    pub prompt: &'static str,
    pub target: &'static [&'static str],
    pub distractors: &'static [&'static str],
    pub rule: &'static [&'static str],
}

pub const TILES: &[TilesTemplate] = &[
    TilesTemplate {
        id: "tiles-donne",
        level: "B1",
        topic: "Double object pronouns",
        task: &["Build the sentence:", "“He already gave it to me.”"],
        prompt: "He already gave it to me.",
        target: &["Il", "me", "l'a", "déjà", "donné"],
        distractors: &["lui", "le"],
        rule: &[
            "The indirect object comes first — me l'a — then",
            "déjà, then the participle: Il me l'a déjà donné.",
        ],
    },
    TilesTemplate {
        id: "tiles-ecrire",
        level: "A2",
        topic: "Object pronouns",
        task: &["Build the sentence:", "“I'm going to write to them tomorrow.”"],
        prompt: "I'm going to write to them tomorrow.",
        target: &["Je", "vais", "leur", "écrire", "demain"],
        distractors: &["eux", "la"],
        rule: &[
            "À + a person becomes leur, placed before the",
            "infinitive: Je vais leur écrire demain.",
        ],
    },
    TilesTemplate {
        id: "tiles-negation",
        level: "A1",
        topic: "Negation",
        task: &["Build the sentence:", "“She doesn't speak English.”"],
        prompt: "She doesn't speak English.",
        target: &["Elle", "ne", "parle", "pas", "anglais"],
        distractors: &["jamais", "rien"],
        rule: &[
            "Ne … pas wraps the conjugated verb:",
            "Elle ne parle pas anglais.",
        ],
    },
    TilesTemplate {
        id: "tiles-arrives",
        level: "A2",
        topic: "Passé composé with être",
        task: &["Build the sentence:", "“We arrived at eight o'clock.”"],
        prompt: "We arrived at eight o'clock.",
        target: &["Nous", "sommes", "arrivés", "à", "huit", "heures"],
        distractors: &["avons", "arrivée"],
        rule: &[
            "Arriver takes être: nous sommes arrivés — with",
            "the participle agreeing with nous.",
        ],
    },
    TilesTemplate {
        id: "tiles-subjonctif",
        level: "B1",
        topic: "Subjonctif after feelings",
        task: &["Build the sentence:", "“I'm glad that you're coming.”"],
        prompt: "I'm glad that you're coming.",
        target: &["Je", "suis", "content", "que", "tu", "viennes"],
        distractors: &["viens", "viendra"],
        rule: &[
            "Content que expresses a feeling, so the clause",
            "takes the subjonctif: que tu viennes.",
        ],
    },
];

/* ── Transform ──────────────────────────────────────────────────── */

pub struct TransformTemplate {
    pub id: &'static str,
    pub level: &'static str,
    pub topic: &'static str,
    pub task: &'static [&'static str],
    pub source: &'static str,
    pub answer: &'static str,
    pub rule: &'static [&'static str],
}

pub const TRANSFORM: &[TransformTemplate] = &[
    TransformTemplate {
        id: "cod-livre",
        level: "A2",
        topic: "Direct object pronouns",
        task: &["Rewrite it, replacing the object", "with a pronoun."],
        source: "Elle a acheté le livre.",
        answer: "Elle l'a acheté",
        rule: &[
            "Le livre is masculine singular → l'. The pronoun",
            "goes before avoir: Elle l'a acheté.",
        ],
    },
    TransformTemplate {
        id: "coi-soeur",
        level: "A2",
        topic: "Indirect object pronouns",
        task: &["Rewrite it, replacing the object", "with a pronoun."],
        source: "Tu téléphones à ta sœur.",
        answer: "Tu lui téléphones",
        rule: &[
            "À + a person becomes lui, placed before the",
            "conjugated verb: Tu lui téléphones.",
        ],
    },
    TransformTemplate {
        id: "cod-marie",
        level: "A2",
        topic: "Direct object pronouns",
        task: &["Rewrite it, replacing the object", "with a pronoun."],
        source: "Je connais Marie.",
        answer: "Je la connais",
        rule: &["Marie is feminine singular → la.", "Je la connais."],
    },
    TransformTemplate {
        id: "y-paris",
        level: "B1",
        topic: "The pronoun y",
        task: &["Rewrite it, replacing the place", "with y."],
        source: "Elle va à Paris.",
        answer: "Elle y va",
        rule: &[
            "À + a place becomes y, placed before the verb:",
            "Elle y va.",
        ],
    },
    TransformTemplate {
        id: "cod-fleurs",
        level: "A2",
        topic: "Direct object pronouns",
        task: &["Rewrite it, replacing the object", "with a pronoun."],
        source: "Nous avons acheté les fleurs.",
        answer: "Nous les avons achetées",
        rule: &[
            "Les fleurs is feminine plural → les, and the",
            "participle agrees: achetées.",
        ],
    },
];

/* ── Error spot ─────────────────────────────────────────────────── */

pub struct ErrorSpotTemplate {
    pub id: &'static str,
    pub level: &'static str,
    pub topic: &'static str,
    pub task: &'static [&'static str],
    pub words: &'static [&'static str],
    pub wrong: usize,
    pub fix: &'static str,
    pub fix_task: &'static str,
    pub rule: &'static [&'static str],
}

pub const ERROR_SPOT: &[ErrorSpotTemplate] = &[
    ErrorSpotTemplate {
        id: "fleurs-accord",
        level: "B1",
        topic: "Agreement with preceding object",
        task: &["One word is wrong. Tap it."],
        words: &["Les", "fleurs", "que", "j'ai", "acheté", "sont", "belles."],
        wrong: 4,
        fix: "achetées",
        fix_task: "Now write the correct form.",
        rule: &[
            "The direct object (les fleurs) precedes the verb,",
            "so the participle agrees: que j'ai achetées.",
        ],
    },
    ErrorSpotTemplate {
        id: "allee-accord",
        level: "A2",
        topic: "Passé composé with être",
        task: &["One word is wrong. Tap it."],
        words: &["Elle", "est", "allé", "au", "marché."],
        wrong: 2,
        fix: "allée",
        fix_task: "Now write the correct form.",
        rule: &[
            "With être, the participle agrees with the",
            "subject: elle est allée.",
        ],
    },
    ErrorSpotTemplate {
        id: "aux-partir",
        level: "A2",
        topic: "Auxiliary choice",
        task: &["One word is wrong. Tap it."],
        words: &["Il", "a", "parti", "à", "midi."],
        wrong: 1,
        fix: "est",
        fix_task: "Now write the correct form.",
        rule: &[
            "Partir is an être verb: il est parti,",
            "never il a parti.",
        ],
    },
    ErrorSpotTemplate {
        id: "arrivees-accord",
        level: "A2",
        topic: "Passé composé with être",
        task: &["One word is wrong. Tap it."],
        words: &["Elles", "sont", "arrivés", "en", "retard."],
        wrong: 2,
        fix: "arrivées",
        fix_task: "Now write the correct form.",
        rule: &[
            "Elles is feminine plural, so the participle",
            "agrees: elles sont arrivées.",
        ],
    },
    ErrorSpotTemplate {
        id: "subjonctif-doute",
        level: "B1",
        topic: "Subjonctif after doubt",
        task: &["One word is wrong. Tap it."],
        words: &["Je", "doute", "qu'il", "a", "raison."],
        wrong: 3,
        fix: "ait",
        fix_task: "Now write the correct form.",
        rule: &[
            "Douter que triggers the subjonctif:",
            "je doute qu'il ait raison.",
        ],
    },
];

/* ── Dictation ──────────────────────────────────────────────────── */

pub struct DictationTemplate {
    pub id: &'static str,
    pub level: &'static str,
    pub topic: &'static str,
    pub task: &'static [&'static str],
    pub speech: &'static str,
    pub answer: &'static str,
    pub rule: &'static [&'static str],
}

pub const DICTATION: &[DictationTemplate] = &[
    DictationTemplate {
        id: "dic-allées",
        level: "A2",
        topic: "Passé composé with être",
        task: &["Type what you hear."],
        speech: "Les filles sont allées.",
        answer: "Les filles sont allées.",
        rule: &[
            "With être, the participle agrees with the feminine",
            "plural subject: allées.",
        ],
    },
    DictationTemplate {
        id: "dic-mangé",
        level: "A1",
        topic: "Passé composé with avoir",
        task: &["Type what you hear."],
        speech: "Nous avons mangé au restaurant.",
        answer: "Nous avons mangé au restaurant.",
        rule: &[
            "Avoir in the présent + the past participle:",
            "manger → mangé.",
        ],
    },
    DictationTemplate {
        id: "dic-negation",
        level: "A1",
        topic: "Negation",
        task: &["Type what you hear."],
        speech: "Elle ne parle pas anglais.",
        answer: "Elle ne parle pas anglais.",
        rule: &["Ne … pas wraps the conjugated verb: ne parle pas."],
    },
    DictationTemplate {
        id: "dic-tombé",
        level: "A2",
        topic: "Imparfait vs passé composé",
        task: &["Type what you hear."],
        speech: "Il est tombé dans la rue.",
        answer: "Il est tombé dans la rue.",
        rule: &[
            "A completed event: être + participle, agreeing",
            "with il: tombé.",
        ],
    },
    DictationTemplate {
        id: "dic-sœur",
        level: "A1",
        topic: "Indirect object pronouns",
        task: &["Type what you hear."],
        speech: "Tu téléphones à ta sœur.",
        answer: "Tu téléphones à ta sœur.",
        rule: &[
            "Téléphoner à someone: the à marks the person",
            "you call.",
        ],
    },
    DictationTemplate {
        id: "dic-viennes",
        level: "B1",
        topic: "Subjonctif after feelings",
        task: &["Type what you hear."],
        speech: "Je suis content que tu viennes.",
        answer: "Je suis content que tu viennes.",
        rule: &["Content que + subjonctif: que tu viennes."],
    },
];
