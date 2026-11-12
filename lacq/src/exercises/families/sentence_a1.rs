//! Family: A1 sentence cluster — five topics. Il y a / c'est / voilà,
//! the dans/depuis/il y a time trio, prepositions of place, prepositions
//! of time (pendant/avant/après), and the basic connectors (et, ou,
//! mais, alors) — every item unambiguous by construction.

use crate::exercises::{ChoiceItem, ClozeItem, ErrorSpotItem, ExerciseItem, TransformItem};

use super::shared::slugify;

const LEVEL: &str = "A1";

/* ── il y a, c'est, voilà ───────────────────────────────────────── */

/// (place lead, noun phrase) — « Dans le jardin, il y a deux chiens. »
const SCENES: &[(&str, &str)] = &[
    ("Dans le jardin", "deux chiens"),
    ("Sur la table", "un livre"),
    ("Dans ma poche", "une clé"),
    ("Au mur", "une image"),
    ("Dans la rue", "beaucoup de monde"),
    ("Dans ma classe", "trente élèves"),
    ("Dans le frigo", "du lait"),
    ("Sur la table", "deux tasses"),
    ("Dans le parc", "des enfants"),
    ("Dans ma chambre", "un ordinateur"),
    ("À la porte", "le facteur"),
    ("Dans le ciel", "des nuages"),
    ("Dans mon sac", "un stylo"),
    ("Sur la table", "trois assiettes"),
];

fn ilya_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(place, np) in SCENES {
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("ilya:{}:{}:cl", slugify(place), slugify(np)),
            level: LEVEL,
            topic: "Il y a, c'est, voilà",
            topic_slug: Some("il-ya-cest-voila".into()),
            task: vec!["Complete with « il y a ».".into()],
            head: format!("{} ", place),
            tail: format!(" {}.", np),
            infinitive: "il y a",
            answer: "il y a".into(),
            rule: vec![
                format!("{} il y a {}.", place, np),
                "il y a = there is / there are — never « il a ».".into(),
            ],
        }));
        // Error-spot: the il a / il y a slip.
        let mut es_words: Vec<String> = vec!["Il".into(), "a".into()];
        es_words.push(np.to_string());
        for w in place.split(' ') {
            es_words.push(w.to_string());
        }
        let es_words = super::shared::punctuate(es_words);
        out.push(ExerciseItem::ErrorSpot(ErrorSpotItem {
            id: format!("ilya:{}:{}:es", slugify(place), slugify(np)),
            level: LEVEL,
            topic: "Il y a, c'est, voilà",
            topic_slug: Some("il-ya-cest-voila".into()),
            task: vec!["One word is wrong. Tap it.".into()],
            words: es_words,
            wrong: 1,
            fix: "y a".into(),
            fix_task: "Now write the correct form.",
            rule: vec![
                "il y a = there is — « il a » means he has.".into(),
                format!("Il y a {} {}.", np, place.to_lowercase()),
            ],
        }));
        // Choice: il y a vs c'est with a plural/count noun.
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("ilya:{}:{}:ch", slugify(place), slugify(np)),
            level: LEVEL,
            topic: "Il y a, c'est, voilà",
            topic_slug: Some("il-ya-cest-voila".into()),
            task: vec!["Choose the right form.".into()],
            sentence: format!("{} ____ {}.", place, np),
            options: vec!["il y a".into(), "c'est".into()],
            correct: 0, // shuffled in `generate`
            rule: vec![
                "A quantity in a place → il y a.".into(),
                format!("{} il y a {}.", place, np),
            ],
        }));
    }
    out
}

/* ── dans, depuis, il y a (time) ────────────────────────────────── */

/// (sentence with gap, right word, wrong word, meaning note)
const TIME_TRIO: &[(&str, &str, &str, &str)] = &[
    ("J'habite ici ____ trois ans.", "depuis", "dans", "since/for — action still true"),
    ("Le train part ____ dix minutes.", "dans", "depuis", "in (from now) — future"),
    ("J'ai déménagé ____ deux ans.", "il y a", "dans", "ago — completed past"),
    ("Elle travaille ____ lundi.", "depuis", "il y a", "since Monday — still true"),
    ("On mange ____ une heure.", "dans", "depuis", "in an hour — future"),
    ("Il est parti ____ cinq minutes.", "il y a", "depuis", "five minutes ago"),
    ("Nous attendons ____ vingt minutes.", "depuis", "dans", "we're still waiting"),
    ("Le film commence ____ dix heures.", "dans", "il y a", "in ten minutes"),
    ("Je joue du piano ____ dix ans.", "depuis", "il y a", "for ten years — still playing"),
    ("Elle a appelé ____ une heure.", "il y a", "dans", "an hour ago"),
    ("Il rentre ____ une semaine.", "dans", "depuis", "in a week — future"),
    ("Je te connais ____ toujours.", "depuis", "dans", "since forever"),
    ("On a fini ____ deux heures.", "il y a", "depuis", "two hours ago"),
    ("Le cours dure ____ deux heures.", "depuis", "il y a", "it started two hours ago"),
    ("Elle revient ____ un mois.", "dans", "il y a", "in a month — future"),
    ("Je lis ce livre ____ une semaine.", "depuis", "dans", "I'm still reading it"),
];

fn time_trio_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(sentence, right, wrong, note) in TIME_TRIO {
        let (head, tail) = sentence.split_once("____").unwrap();
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("trio:{}:cl", slugify(sentence)),
            level: LEVEL,
            topic: "Dans, depuis, il y a",
            topic_slug: Some("dans-depuis-il-ya".into()),
            task: vec!["Complete with dans, depuis or il y a.".into()],
            head: head.to_string(),
            tail: format!("{}.", tail.trim_start_matches(' ')),
            infinitive: "dans / depuis / il y a",
            answer: right.to_string(),
            rule: vec![format!("{} — {}.", right, note)],
        }));
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("trio:{}:ch", slugify(sentence)),
            level: LEVEL,
            topic: "Dans, depuis, il y a",
            topic_slug: Some("dans-depuis-il-ya".into()),
            task: vec!["Choose the right time word.".into()],
            sentence: sentence.replace("____", ""),
            options: vec![right.to_string(), wrong.to_string()],
            correct: 0, // shuffled in `generate`
            rule: vec![format!("{} — {}.", right, note)],
        }));
    }
    out
}

/* ── prepositions of place ──────────────────────────────────────── */

/// (sentence, preposition, distractor, note)
const PLACE_PREPS: &[(&str, &str, &str, &str)] = &[
    ("Je vais ____ Paris.", "à", "en", "cities take à"),
    ("Je vais ____ France.", "en", "à", "feminine countries take en"),
    ("Je vais ____ Japon.", "au", "en", "masculine countries take au"),
    ("Je vais ____ États-Unis.", "aux", "au", "plural places take aux"),
    ("Je vais ____ Marie.", "chez", "à", "chez + person = at their place"),
    ("Je vais ____ le médecin.", "chez", "dans", "chez le docteur"),
    ("Je suis ____ la cuisine.", "dans", "chez", "dans = inside an enclosed space"),
    ("Le livre est ____ le sac.", "dans", "sur", "dans = inside"),
    ("Le chat est ____ la table.", "sur", "dans", "sur = on top of"),
    ("Nous sommes ____ restaurant.", "au", "dans", "au restaurant (à + le = au)"),
    ("Elle est ____ la boulangerie.", "à la", "au", "la boulangerie is feminine → à la"),
    ("Il habite ____ Canada.", "au", "en", "masculine country → au"),
    ("Elle habite ____ Italie.", "en", "au", "feminine country → en"),
    ("Nous allons ____ ciné.", "au", "en", "au cinéma"),
    ("Elle est ____ l'hôpital.", "à", "dans", "à l'hôpital (elision)"),
    ("Ils sont ____ la gare.", "à", "chez", "à la gare"),
];

fn place_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(sentence, right, wrong, note) in PLACE_PREPS {
        let (head, tail) = sentence.split_once("____").unwrap();
        let plain = format!("{}{}{}", head, right, tail.replace("  ", " "));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("plc:{}:cl", slugify(sentence)),
            level: LEVEL,
            topic: "Prepositions of place",
            topic_slug: Some("prepositions-place".into()),
            task: vec!["Complete with the right preposition.".into()],
            head: head.to_string(),
            tail: tail.to_string(),
            infinitive: "preposition",
            answer: right.to_string(),
            rule: vec![format!("{} — {}.", plain, note)],
        }));
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("plc:{}:ch", slugify(sentence)),
            level: LEVEL,
            topic: "Prepositions of place",
            topic_slug: Some("prepositions-place".into()),
            task: vec!["Choose the right preposition.".into()],
            sentence: sentence.replace("____", "…"),
            options: vec![right.to_string(), wrong.to_string()],
            correct: 0, // shuffled in `generate`
            rule: vec![format!("{} — {}.", plain, note)],
        }));
    }
    out
}

/* ── prepositions of time ───────────────────────────────────────── */

/// (sentence, preposition, distractor, note)
const TIME_PREPS: &[(&str, &str, &str, &str)] = &[
    ("Je dors ____ le déjeuner.", "après", "avant", "after lunch"),
    ("Je me lève ____ le petit-déjeuner.", "avant", "après", "before breakfast"),
    ("Il travaille ____ la nuit.", "pendant", "après", "during the night"),
    ("____ le film, on dîne.", "Après", "Pendant", "after the film"),
    ("____ les vacances, je lis.", "Pendant", "Avant", "during the holidays"),
    ("Elle étudie ____ l'examen.", "avant", "pendant", "before the exam"),
    ("On se voit ____ le cours.", "après", "avant", "after the class"),
    ("Il dort ____ la réunion.", "pendant", "avant", "during the meeting"),
    ("Je bois un café ____ de travailler.", "avant", "après", "avant de + infinitive"),
    ("Elle se repose ____ avoir mangé.", "après", "avant", "après avoir + participle"),
    ("Nous skions ____ l'hiver.", "pendant", "après", "during winter"),
    ("Il se couche ____ minuit.", "avant", "pendant", "before midnight"),
    ("Je sors ____ le dîner.", "après", "pendant", "after dinner"),
    ("Elle court ____ la pluie.", "sous", "pendant", "sous la pluie = in the rain"),
    ("On se retrouve ____ le match.", "avant", "sous", "before the match"),
    ("Il marche ____ les cours.", "pendant", "sous", "during the lessons"),
];

fn time_prep_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(sentence, right, wrong, note) in TIME_PREPS {
        let (head, tail) = sentence.split_once("____").unwrap();
        let plain = format!("{}{}{}", head, right, tail.replace("  ", " "));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("ptm:{}:cl", slugify(sentence)),
            level: LEVEL,
            topic: "Prepositions of time",
            topic_slug: Some("prepositions-time".into()),
            task: vec!["Complete with the right preposition.".into()],
            head: head.to_string(),
            tail: tail.to_string(),
            infinitive: "preposition",
            answer: right.to_string(),
            rule: vec![format!("{} — {}.", plain, note)],
        }));
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("ptm:{}:ch", slugify(sentence)),
            level: LEVEL,
            topic: "Prepositions of time",
            topic_slug: Some("prepositions-time".into()),
            task: vec!["Choose the right preposition.".into()],
            sentence: sentence.replace("____", "…"),
            options: vec![right.to_string(), wrong.to_string()],
            correct: 0, // shuffled in `generate`
            rule: vec![format!("{} — {}.", plain, note)],
        }));
    }
    out
}

/* ── et, ou, mais, alors ────────────────────────────────────────── */

/// (sentence with gap, right connector, meaning note)
const CONNECTORS: &[(&str, &str, &str)] = &[
    ("J'aime le thé ____ le café.", "et", "and — adding"),
    ("Tu veux un thé ____ un café ?", "ou", "or — a choice"),
    ("Il pleut, ____ je prends mon parapluie.", "alors", "so — a consequence"),
    ("Je viens, ____ je suis fatigué.", "mais", "but — a contrast"),
    ("Marie parle français ____ anglais.", "et", "and — adding"),
    ("Tu préfères le vert ____ le bleu ?", "ou", "or — a choice"),
    ("Il est malade, ____ il reste au lit.", "alors", "so — a consequence"),
    ("Le film est long, ____ très bon.", "mais", "but — a contrast"),
    ("Nous dansons ____ chantons.", "et", "and — adding"),
    ("On part samedi ____ dimanche ?", "ou", "or — a choice"),
    ("Elle étudie, ____ elle réussit.", "alors", "so — a consequence"),
    ("Je veux venir, ____ je travaille.", "mais", "but — a contrast"),
    ("Il aime lire ____ écrire.", "et", "and — adding"),
    ("Tu sors ____ tu restes ?", "ou", "or — a choice"),
    ("Le train est parti, ____ on attend le suivant.", "alors", "so — a consequence"),
    ("C'est petit, ____ joli.", "mais", "but — a contrast"),
];

fn connector_items() -> Vec<ExerciseItem> {
    let mut out = Vec::new();
    for &(sentence, right, note) in CONNECTORS {
        let (head, tail) = sentence.split_once("____").unwrap();
        let plain = format!("{}{}{}", head, right, tail.replace("  ", " "));
        out.push(ExerciseItem::Cloze(ClozeItem {
            id: format!("cnx:{}:cl", slugify(sentence)),
            level: LEVEL,
            topic: "Et, ou, mais, alors",
            topic_slug: Some("connectors-basic".into()),
            task: vec!["Complete with et, ou, mais or alors.".into()],
            head: head.to_string(),
            tail: tail.to_string(),
            infinitive: "connector",
            answer: right.to_string(),
            rule: vec![format!("{} — {}.", plain, note)],
        }));
        let wrong = match right {
            "et" => "mais",
            "ou" => "et",
            "mais" => "alors",
            _ => "mais",
        };
        out.push(ExerciseItem::Choice(ChoiceItem {
            id: format!("cnx:{}:ch", slugify(sentence)),
            level: LEVEL,
            topic: "Et, ou, mais, alors",
            topic_slug: Some("connectors-basic".into()),
            task: vec!["Choose the right connector.".into()],
            sentence: sentence.replace("____", "…"),
            options: vec![right.to_string(), wrong.into()],
            correct: 0, // shuffled in `generate`
            rule: vec![format!("{} — {}.", plain, note)],
        }));
    }
    out
}

pub(crate) fn items(slug: &str) -> Vec<ExerciseItem> {
    match slug {
        "il-ya-cest-voila" => ilya_items(),
        "dans-depuis-il-ya" => time_trio_items(),
        "prepositions-place" => place_items(),
        "prepositions-time" => time_prep_items(),
        "connectors-basic" => connector_items(),
        _ => Vec::new(),
    }
}
