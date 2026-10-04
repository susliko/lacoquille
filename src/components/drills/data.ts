/**
 * Curated exercise data for the four drill types. Static for now — no
 * scheduler, no progress. Each array is hand-tuned to mix difficulty and
 * cover a useful spread of verbs and tenses.
 *
 * NOTE: the prototypes directory (`/prototypes/`) had the same data inline
 * in each HTML file. This file is the single source of truth for the
 * production drills.
 */

/* ── Cloze (three-state + retyping) ───────────────────────────── */

export interface ClozeHint {
  verb: string;
  person: string;
  tense: string;
}

export interface ClozeItem {
  /** e.g. "Quand j'étais petit, " + "_" + " beaucoup avec mes amis." */
  parts: [string, string];
  hint: ClozeHint;
  answer: string;
  /** For colour-coding the reveal (indigo stem, coral ending). */
  stem?: string;
  ending?: string;
  /** Optional rule shown only on wrong/almost. Inline HTML allowed. */
  rule?: string;
}

export const CLOZE_ITEMS: ClozeItem[] = [
  {
    parts: ["Quand j'étais petit, ", " beaucoup avec mes amis."],
    hint: { verb: "parler", person: "je", tense: "imparfait" },
    answer: "parlais", stem: "parl", ending: "ais",
    rule: "<code>-er</code> imparfait: stem + <code>-ais · -ait · -ions · -iez · -aient</code>",
  },
  {
    parts: ["Chaque matin, il ", " à l'école à pied."],
    hint: { verb: "aller", person: "il", tense: "imparfait" },
    answer: "allait", stem: "all", ending: "ait",
    rule: "<code>aller</code> imparfait: <code>all-</code> + endings (not <code>aller-</code>).",
  },
  {
    parts: ["Tu ", " toujours tes devoirs avant de jouer."],
    hint: { verb: "finir", person: "tu", tense: "présent" },
    answer: "finis", stem: "fin", ending: "is",
    rule: "<code>-ir</code> verbs like <code>finir</code>: <code>-is · -is · -it · -issons · -issez · -issent</code>.",
  },
  {
    parts: ["Hier soir, nous ", " au restaurant italien."],
    hint: { verb: "manger", person: "nous", tense: "passé composé" },
    answer: "avons mangé", stem: "avons mang", ending: "é",
    rule: "passé composé with <code>avoir</code>: <code>avoir</code> + past participle. <code>manger → mangé</code>.",
  },
  {
    parts: ["Elle ", " très timide quand elle était enfant."],
    hint: { verb: "être", person: "elle", tense: "imparfait" },
    answer: "était", stem: "ét", ending: "ait",
    rule: "<code>être</code> imparfait: <code>j'étais, tu étais, il était…</code> — circumflex on the stem.",
  },
  {
    parts: ["J'", " faim, on mange quelque chose ?"],
    hint: { verb: "avoir", person: "je", tense: "présent" },
    answer: "ai", stem: "ai", ending: "",
    rule: "<code>avoir</code> présent: <code>j'ai, tu as, il a, nous avons, vous avez, ils ont</code>.",
  },
  {
    parts: ["Qu'est-ce que vous ", " ce week-end ?"],
    hint: { verb: "faire", person: "vous", tense: "passé composé" },
    answer: "avez fait", stem: "avez f", ending: "ait",
    rule: "<code>faire</code> past participle is <code>fait</code> (irregular). passé composé: <code>avoir</code> + <code>fait</code>.",
  },
  {
    parts: ["Les étés, ils ", " souvent chez leurs grands-parents."],
    hint: { verb: "venir", person: "ils", tense: "imparfait" },
    answer: "venaient", stem: "ven", ending: "aient",
    rule: "<code>venir</code> imparfait: <code>ven-</code> + imparfait endings.",
  },
];

/* ── Cue-to-form (flashcard) ───────────────────────────────────── */

export interface CueItem {
  verb: string;
  person: string;
  tense: string;
  answer: string;
}

export const CUE_ITEMS: CueItem[] = [
  { verb: "parler",  person: "je",   tense: "présent",   answer: "parle" },
  { verb: "finir",   person: "tu",   tense: "présent",   answer: "finis" },
  { verb: "vendre",  person: "il",   tense: "présent",   answer: "vend" },
  { verb: "être",    person: "nous", tense: "imparfait", answer: "étions" },
  { verb: "avoir",   person: "vous", tense: "présent",   answer: "avez" },
  { verb: "aller",   person: "elle", tense: "imparfait", answer: "allait" },
  { verb: "faire",   person: "je",   tense: "présent",   answer: "fais" },
  { verb: "venir",   person: "ils",  tense: "imparfait", answer: "venaient" },
  { verb: "parler",  person: "vous", tense: "passé composé", answer: "avez parlé" },
  { verb: "finir",   person: "il",   tense: "passé composé", answer: "a fini" },
  { verb: "se lever", person: "je",  tense: "présent",   answer: "me lève" },
  { verb: "prendre", person: "tu",   tense: "présent",   answer: "prends" },
];

/* ── Spot the tense (imparfait vs passé composé) ───────────────── */

export interface TenseOption {
  form: string;
  tense: string;
  gloss: string;
}

export interface TenseItem {
  /** Sentence string with `<blank>` for the verb slot. */
  sentence: string;
  gloss: string;
  options: [TenseOption, TenseOption];
  /** Index into `options` — 0 or 1. */
  correct: 0 | 1;
  /** The cue word to highlight in the sentence (e.g. "quand j'ai vu"). */
  cue: string;
  /** Explanation shown after the answer. Inline HTML allowed. */
  rule: string;
}

export const TENSE_ITEMS: TenseItem[] = [
  {
    sentence: "Hier matin, je <blank> au marché quand j'ai vu Marie.",
    gloss: "Yesterday morning, I ___ to the market when I saw Marie.",
    options: [
      { form: "j'allais",     tense: "imparfait",     gloss: "I was going (background)" },
      { form: "je suis allé", tense: "passé composé", gloss: "I went (specific event)" },
    ],
    correct: 0,
    cue: "quand j'ai vu",
    rule: "The <b>foreground event</b> (<code>j'ai vu</code>) takes the passé composé; the <b>ongoing background</b> takes the imparfait.",
  },
  {
    sentence: "Quand j'étais petit, je <blank> au parc tous les jours.",
    gloss: "When I was a child, I ___ to the park every day.",
    options: [
      { form: "j'allais",     tense: "imparfait",     gloss: "I used to go (habit)" },
      { form: "je suis allé", tense: "passé composé", gloss: "I went (single event)" },
    ],
    correct: 0,
    cue: "tous les jours",
    rule: "<b>Habitual past</b> is imparfait — <code>tous les jours, d'habitude, souvent</code>.",
  },
  {
    sentence: "Pendant que je <blank>, le téléphone a sonné.",
    gloss: "While I ___, the phone rang.",
    options: [
      { form: "dormais",   tense: "imparfait",     gloss: "was sleeping (ongoing)" },
      { form: "ai dormi",  tense: "passé composé", gloss: "slept (completed)" },
    ],
    correct: 0,
    cue: "pendant que",
    rule: "<code>pendant que / quand / tandis que</code> + imparfait sets up the <b>background</b> against which a foreground event (<code>a sonné</code>) happens.",
  },
  {
    sentence: "Soudain, il <blank> dans la rue.",
    gloss: "Suddenly, he ___ in the street.",
    options: [
      { form: "est tombé", tense: "passé composé", gloss: "fell (specific event)" },
      { form: "tombait",   tense: "imparfait",     gloss: "was falling (ongoing)" },
    ],
    correct: 0,
    cue: "soudain",
    rule: "<code>soudain, tout à coup, un jour</code> mark a <b>punctual, bounded event</b> — passé composé.",
  },
  {
    sentence: "Il pleuvait fort quand je <blank> de la maison.",
    gloss: "It was raining hard when I ___ the house.",
    options: [
      { form: "suis sorti", tense: "passé composé", gloss: "left (the foreground event)" },
      { form: "sortais",    tense: "imparfait",     gloss: "was leaving (ongoing)" },
    ],
    correct: 0,
    cue: "il pleuvait",
    rule: "<code>il pleuvait</code> is imparfait (description). The <b>interrupting event</b> is passé composé.",
  },
  {
    sentence: "À cette époque-là, nous <blank> dans un petit appartement.",
    gloss: "At that time, we ___ in a small apartment.",
    options: [
      { form: "habitions",   tense: "imparfait",     gloss: "were living (state, no end)" },
      { form: "avons habité", tense: "passé composé", gloss: "lived (bounded)" },
    ],
    correct: 0,
    cue: "à cette époque-là",
    rule: "<b>States and descriptions</b> (where someone lived, how they felt) are imparfait.",
  },
];

/* ── Transformation chain ──────────────────────────────────────── */

export interface ChainStep {
  /** The form the learner is transforming FROM. */
  from: string;
  /** Instruction shown as a chip (e.g. "switch to elle"). */
  instruction: string;
  /** The correct target form. */
  target: string;
  /** Optional tip shown after the answer. */
  tip?: string;
  /** True if this step introduces a negation transformation. */
  hasNegation?: boolean;
}

export interface Chain {
  /** Display name shown above the chain. */
  name: string;
  steps: ChainStep[];
}

export const CHAINS: Chain[] = [
  {
    name: "aller · passé composé · negative",
    steps: [
      { from: "je vais",                instruction: "switch to imparfait",     target: "j'allais" },
      { from: "j'allais",               instruction: "switch to elle",          target: "elle allait" },
      { from: "elle allait",            instruction: "now passé composé",       target: "elle est allée",
        tip: "passé composé of <em>aller</em> uses <em>être</em>, not avoir. Agreement in gender: allée (f.), allé (m.)." },
      { from: "elle est allée",         instruction: "make it negative",        target: "elle n'est pas allée",
        tip: "<em>ne … pas</em> wraps the auxiliary.",
        hasNegation: true },
    ],
  },
  {
    name: "finir · irregular stem · negation",
    steps: [
      { from: "je finis",               instruction: "passé composé, elle",     target: "elle a fini" },
      { from: "elle a fini",            instruction: "imparfait, nous",         target: "nous finissions",
        tip: "-ir verbs in the imparfait double the i: finiss- + endings." },
      { from: "nous finissions",        instruction: "make it negative",        target: "nous ne finissions pas",
        hasNegation: true },
      { from: "nous ne finissions pas", instruction: "switch to vous",          target: "vous ne finissiez pas",
        hasNegation: true },
    ],
  },
  {
    name: "avoir · être · chain",
    steps: [
      { from: "j'ai parlé",             instruction: "imparfait, je",           target: "j'avais parlé" },
      { from: "j'avais parlé",          instruction: "switch to ils",           target: "ils avaient parlé" },
      { from: "ils avaient parlé",      instruction: "switch to elle (f.)",     target: "elle avait parlé" },
    ],
  },
];

/* ── Persons (for cue-to-form chip) ────────────────────────────── */

export const PERSONS = ["je", "tu", "il", "nous", "vous", "ils"] as const;
export type Person = (typeof PERSONS)[number];