/**
 * The designed grammar taxonomy — the full topic skeleton from the
 * handoff prototype (grammar-grid.html, itself built on
 * docs/02-topics.md): 121 topics, each at exactly ONE level and
 * family, across all six CEFR rows A1–C2.
 *
 * Families: T tense, M mood, N noun group, S sentence.
 * N carries three subgroups: n nouns & articles, a adjectives &
 * adverbs, p pronouns.
 *
 * `slugs` wires drillable catalog topics (slugs are the wire
 * contract, never renamed) onto their taxonomy entries. An entry may
 * carry two slugs when one drill family covers two taxonomy rows;
 * an entry without slugs is pure structure — it renders "Not
 * started" until the catalog grows to cover it. The prototype's
 * demo tiers (raw 0–4 and "!" markers) are ignored: real learner
 * state comes from tiers.ts.
 */

export type FamKey = "T" | "M" | "N" | "S";

/** Grid rows — all six CEFR levels, as designed. */
export const GG_LEVELS = ["A1", "A2", "B1", "B2", "C1", "C2"] as const;
/** Grid columns — the four families, in column order. */
export const GG_KEYS: FamKey[] = ["T", "M", "N", "S"];

export const GG_CATS: Record<FamKey, string> = {
  T: "Tense",
  M: "Mood",
  N: "Noun group",
  S: "Sentence",
};

/** Family blurbs, shown in the sheet's "about" box on column open. */
export const GG_ABOUT: Record<FamKey, string> = {
  T: "When things happen: the present, past and future forms of verbs.",
  M: "How the speaker sees an action: commands, wishes, hypotheses and doubt, plus forms like the passive and the gérondif.",
  N: "A noun and everything around it: the articles in front of it, the adjectives and adverbs that describe it, and the pronouns that replace it.",
  S: "How words come together into sentences: negation, questions, linking words and reported speech.",
};

/** The N column's subgroups, in sheet order. */
export const GG_SUBS = [
  { key: "n", name: "Nouns & articles" },
  { key: "a", name: "Adjectives & adverbs" },
  { key: "p", name: "Pronouns" },
] as const;

export interface TaxonomyEntry {
  id: string;
  lv: string;
  cat: FamKey;
  /** N subgroup: n/a/p; "" outside N. */
  sub: string;
  name: string;
  /** Catalog slugs whose drills light this entry up. */
  slugs: string[];
}

/* The taxonomy, verbatim from the prototype. Levels are in order
   A1→C2; each line is level | family[subgroup] | (ignored demo tier)
   | name. */
const RAW = `A1|T|4|Present: être & avoir
A1|T|4|Present: -er verbs
A1|T|3|Present: common irregular verbs
A1|T|3|Present: pronominal verbs
A1|T|4|Être en train de
A1|T|4|Futur proche
A1|T|3|Passé récent
A1|T|3!|Passé composé: first steps
A1|M|4|Imperative
A1|M|4|Polite conditionnel
A1|M|3|Pouvoir, vouloir, devoir + infinitive
A1|M|4|Il faut + infinitive
A1|M|3|Pour + infinitive
A1|Nn|4|Gender of nouns
A1|Nn|4|Plural of nouns
A1|Nn|4|Definite & indefinite articles
A1|Nn|3!|Partitive articles
A1|Na|4|Adjective agreement
A1|Na|3|Adjective position
A1|Na|4|Possessive adjectives
A1|Na|3|Demonstrative adjectives
A1|Na|3|Quantity & intensity adverbs
A1|Np|4|Subject pronouns & on
A1|Np|3|Stressed pronouns
A1|S|4|Ne… pas, ne… jamais
A1|S|4|Three question forms
A1|S|3|Question words
A1|S|4|Il y a, c'est, voilà
A1|S|2!|C'est vs il/elle est
A1|S|3|Dans, depuis, il y a
A1|S|4|Prepositions of place
A1|S|3|Prepositions of time
A1|S|4|Et, ou, mais, alors
A2|T|3|Passé composé with avoir
A2|T|2!|Passé composé with être
A2|T|2|Imparfait
A2|T|1|Passé composé vs imparfait
A2|T|2|Futur simple
A2|M|1|Verbs with prepositions
A2|Na|3|Comparative
A2|Na|2|Superlative
A2|Np|2!|Direct object pronouns
A2|Np|2|Indirect object pronouns
A2|Np|1|En
A2|Np|2|Y
A2|Np|3|Uses of on
A2|Np|3|Qui, que, où
A2|Np|0|Possessive pronouns
A2|Np|2|Tout, toute, tous
A2|Np|1|Ça, cela, ceci
A2|S|3|Ne… plus, rien, personne
A2|S|2|Exclamations
A2|S|3!|Cause & consequence
A2|S|1|Purpose & opposition
A2|S|3|Sequencing connectors
B1|T|0|Plus-que-parfait
B1|T|1|Past tenses in a story
B1|T|0|Agreement with preceding object
B1|M|1|Conditionnel présent: forms
B1|M|0|Conditionnel présent: uses
B1|M|1|Subjonctif présent: forms
B1|M|0|Subjonctif after feelings & il faut
B1|M|0|Subjonctif vs indicatif
B1|M|0|Passive voice
B1|M|0|Gérondif
B1|Nn|0|Nominalization
B1|Np|0|Double object pronouns
B1|Np|1|Dont
B1|Np|0|Compound relatives
B1|Np|0|Indefinite pronouns
B1|Np|0|Ce qui, ce que, ce dont
B1|S|0|Si + présent
B1|S|0|Si + imparfait
B1|S|0|Reported speech (present)
B1|S|0|Ne… que, aucun, ni… ni
B1|S|0|Cause & consequence (extended)
B1|S|1|Pour que, afin de
B1|S|0|Concession & opposition
B1|S|0|Ce qui…, c'est
B1|S|0|Il est / c'est + adjective + de
B1|S|0|Narrative time markers
B2|T|0|Futur antérieur
B2|T|0|Past infinitive
B2|T|0|Sequence of tenses
B2|T|0|Exceptional participle agreement
B2|M|0|Conditionnel passé
B2|M|0|Subjonctif after verbs & impersonals
B2|M|0|Subjonctif after conjunctions
B2|M|0|Subjonctif passé
B2|M|0|Participle vs verbal adjective
B2|Np|0|Interrogative lequel
B2|S|0|Si + plus-que-parfait
B2|S|0|Au cas où, à condition que
B2|S|0|Advanced opposition & concession
B2|S|0|Advanced purpose & cause
C1|T|0|Passé simple (recognize)
C1|T|0|Passé antérieur (recognize)
C1|T|0|Agreement of pronominal verbs
C1|M|0|Subtle subjonctif cases
C1|M|0|Subjonctif vs indicatif meaning
C1|M|0|Attenuating conditionnel
C1|M|0|Infinitive clauses
C1|M|0|Literary hypotheticals
C1|Na|0|Quelque, quel que
C1|Np|0|Auquel, duquel & co.
C1|Np|0|Ellipsis & avoiding repetition
C1|S|0|Avoir beau, quand bien même
C1|S|0|Formal connectors
C1|S|0|Formal inversion
C1|S|0|Expletive ne
C1|S|0|Impersonal formal style
C1|S|0|Spoken vs written French
C2|T|0|Passé simple, all persons
C2|T|0|Fine participle agreement
C2|M|0|Subjonctif imparfait
C2|M|0|Conditionnel passé 2e forme
C2|Na|0|Adjective position & meaning
C2|S|0|Literary negations
C2|S|0|Stylistic word order
C2|S|0|Archaic fixed phrases
C2|S|0|Register shifts`;

/* Which catalog slugs light which entry up, keyed
   "level|family+subgroup|name". pronouns-cod-coi covers both COD
   and COI rows: its drills mix the two, so both fill together. */
const SLUG_MAP: Record<string, string[]> = {
  // A1
  "A1|T|Present: être & avoir": ["present-etre-avoir"],
  "A1|T|Present: -er verbs": ["present-er-verbs"],
  "A1|T|Present: common irregular verbs": ["present-irregulars"],
  "A1|T|Present: pronominal verbs": ["present-pronominal"],
  "A1|T|Être en train de": ["etre-en-train-de"],
  "A1|T|Futur proche": ["futur-proche"],
  "A1|T|Passé récent": ["passe-recent"],
  "A1|T|Passé composé: first steps": ["pc-aux"],
  "A1|M|Imperative": ["imperative"],
  "A1|M|Polite conditionnel": ["polite-conditionnel"],
  "A1|M|Pouvoir, vouloir, devoir + infinitive": ["modal-verbs-infinitive"],
  "A1|M|Il faut + infinitive": ["il-faut-infinitive"],
  "A1|M|Pour + infinitive": ["pour-infinitive"],
  "A1|Nn|Gender of nouns": ["gender-of-nouns"],
  "A1|Nn|Plural of nouns": ["plural-of-nouns"],
  "A1|Nn|Definite & indefinite articles": ["definite-indefinite-articles"],
  "A1|Nn|Partitive articles": ["partitive-articles"],
  "A1|Na|Adjective agreement": ["adjective-agreement"],
  "A1|Na|Adjective position": ["adjective-position"],
  "A1|Na|Possessive adjectives": ["possessive-adjectives"],
  "A1|Na|Demonstrative adjectives": ["demonstrative-adjectives"],
  "A1|Na|Quantity & intensity adverbs": ["quantity-adverbs"],
  "A1|Np|Subject pronouns & on": ["subject-pronouns"],
  "A1|Np|Stressed pronouns": ["stressed-pronouns"],
  "A1|S|Ne… pas, ne… jamais": ["negation"],
  "A1|S|Three question forms": ["three-question-forms"],
  "A1|S|Question words": ["question-words"],
  "A1|S|Il y a, c'est, voilà": ["il-ya-cest-voila"],
  "A1|S|C'est vs il/elle est": ["cest-vs-il-est"],
  "A1|S|Dans, depuis, il y a": ["dans-depuis-il-ya"],
  "A1|S|Prepositions of place": ["prepositions-place"],
  "A1|S|Prepositions of time": ["prepositions-time"],
  "A1|S|Et, ou, mais, alors": ["connectors-basic"],
  // A2
  "A2|T|Passé composé with avoir": ["pc-avoir"],
  "A2|T|Passé composé with être": ["pc-etre"],
  "A2|T|Imparfait": ["imparfait"],
  "A2|T|Passé composé vs imparfait": ["imparfait-vs-pc"],
  "A2|Np|Direct object pronouns": ["pronouns-cod-coi"],
  "A2|Np|Indirect object pronouns": ["pronouns-cod-coi"],
  "A2|Np|Y": ["pronoun-y"],
  "A2|T|Futur simple": ["futur-simple"],
  "A2|M|Verbs with prepositions": ["verbs-prepositions"],
  "A2|Na|Comparative": ["comparative"],
  "A2|Na|Superlative": ["superlative"],
  "A2|Np|En": ["pronoun-en"],
  "A2|Np|Uses of on": ["uses-of-on"],
  "A2|Np|Qui, que, où": ["qui-que-ou"],
  "A2|Np|Possessive pronouns": ["possessive-pronouns"],
  "A2|Np|Tout, toute, tous": ["tout-toute-tous"],
  "A2|Np|Ça, cela, ceci": ["ca-cela-ceci"],
  "A2|S|Ne… plus, rien, personne": ["ne-plus-rien-personne"],
  "A2|S|Exclamations": ["exclamations"],
  "A2|S|Cause & consequence": ["cause-consequence"],
  "A2|S|Purpose & opposition": ["purpose-opposition"],
  "A2|S|Sequencing connectors": ["sequencing-connectors"],
  // B1
  "B1|T|Agreement with preceding object": ["accord-antepose"],
  "B1|M|Subjonctif after feelings & il faut": ["subjonctif"],
};

/** The 121 entries, in taxonomy order (row by row, left to right). */
export const TAXONOMY: TaxonomyEntry[] = RAW.trim()
  .split("\n")
  .map((line, i) => {
    const [lv, code, , name] = line.trim().split("|");
    const cat = code[0] as FamKey;
    const sub = code[1] ?? "";
    const slugs = SLUG_MAP[`${lv}|${code}|${name}`] ?? [];
    return { id: `t${i}`, lv, cat, sub, name, slugs };
  });
