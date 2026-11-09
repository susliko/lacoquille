//! Shared banks and small helpers used by several family modules.
//! Verbs and pronouns are static tables; agreement is computed, never
//! stored.

pub(crate) struct EtreVerb {
    pub(crate) inf: &'static str,
    /// Base (masculine singular) participle; agreement is computed.
    pub(crate) p: &'static str,
    pub(crate) complement: &'static str,
}

pub(crate) const ETRE_VERBS: &[EtreVerb] = &[
    EtreVerb { inf: "aller", p: "allé", complement: "au cinéma" },
    EtreVerb { inf: "arriver", p: "arrivé", complement: "en retard" },
    EtreVerb { inf: "partir", p: "parti", complement: "à midi" },
    EtreVerb { inf: "rester", p: "resté", complement: "à la maison" },
    EtreVerb { inf: "sortir", p: "sorti", complement: "avec des amis" },
    EtreVerb { inf: "tomber", p: "tombé", complement: "dans la rue" },
    EtreVerb { inf: "retourner", p: "retourné", complement: "à Paris" },
    EtreVerb { inf: "monter", p: "monté", complement: "au troisième étage" },
];

pub(crate) struct AvoirVerb {
    pub(crate) inf: &'static str,
    pub(crate) p: &'static str,
    pub(crate) complement: &'static str,
}

pub(crate) const AVOIR_VERBS: &[AvoirVerb] = &[
    AvoirVerb { inf: "manger", p: "mangé", complement: "au restaurant" },
    AvoirVerb { inf: "parler", p: "parlé", complement: "au téléphone" },
    AvoirVerb { inf: "regarder", p: "regardé", complement: "la télévision" },
    AvoirVerb { inf: "finir", p: "fini", complement: "le travail" },
    AvoirVerb { inf: "vendre", p: "vendu", complement: "sa voiture" },
    AvoirVerb { inf: "prendre", p: "pris", complement: "le train" },
];

pub(crate) struct Pron {
    pub(crate) p: &'static str,
    pub(crate) feminine: bool,
    pub(crate) plural: bool,
    /// être form for this pronoun.
    pub(crate) etre: &'static str,
    /// avoir form for this pronoun.
    pub(crate) avoir: &'static str,
}

pub(crate) const PRONS: &[Pron] = &[
    Pron { p: "il", feminine: false, plural: false, etre: "est", avoir: "a" },
    Pron { p: "elle", feminine: true, plural: false, etre: "est", avoir: "a" },
    Pron { p: "nous", feminine: false, plural: true, etre: "sommes", avoir: "avons" },
    Pron { p: "vous", feminine: false, plural: true, etre: "êtes", avoir: "avez" },
    Pron { p: "ils", feminine: false, plural: true, etre: "sont", avoir: "ont" },
    Pron { p: "elles", feminine: true, plural: true, etre: "sont", avoir: "ont" },
];

impl Pron {
    pub(crate) fn participle(&self, base: &str) -> String {
        let mut p = base.to_string();
        if self.feminine {
            p.push('e');
        }
        if self.plural {
            p.push('s');
        }
        p
    }
    pub(crate) fn agree_note(&self) -> &'static str {
        match (self.feminine, self.plural) {
            (false, false) => "no agreement mark",
            (true, false) => "add -e",
            (false, true) => "add -s",
            (true, true) => "add -es",
        }
    }
}

pub(crate) fn cap(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// Sentence words with the final word carrying the period.
pub(crate) fn punctuate(words: Vec<String>) -> Vec<String> {
    let mut w = words;
    if let Some(last) = w.last_mut() {
        last.push('.');
    }
    w
}

/// True when elision applies (n', j', l'): the next chunk starts with a
/// vowel or a vowel letter (é, ê…).
pub(crate) fn starts_vowel(s: &str) -> bool {
    matches!(
        s.chars().next().map(|c| c.to_ascii_lowercase()),
        Some('a' | 'e' | 'i' | 'o' | 'u' | 'é' | 'è' | 'ê' | 'à' | 'â' | 'î' | 'ô' | 'û')
    )
}

/// (subject, capitalized display, form index for present-tense banks)
pub(crate) const SUBJ_CAP: &[(&str, &str, usize)] = &[
    ("je", "Je", 0),
    ("tu", "Tu", 1),
    ("il", "Il", 2),
    ("elle", "Elle", 2),
    ("nous", "Nous", 3),
    ("vous", "Vous", 4),
    ("ils", "Ils", 5),
    ("elles", "Elles", 5),
];

pub(crate) fn avoir_pc(subj: &str) -> &'static str {
    match subj {
        "je" => "ai",
        "tu" => "as",
        "il" | "elle" => "a",
        "nous" => "avons",
        "vous" => "avez",
        _ => "ont",
    }
}

pub(crate) fn slugify(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .replace("--", "-")
        .trim_matches('-')
        .to_string()
}
