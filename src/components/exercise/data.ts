/**
 * Curated data for the exercise session — the experience designed in
 * Penpot frames 02–10 ("00 — lacoquille Design System", Screens — Mobile).
 *
 * One item per designed exercise type:
 *   cloze (02/03/04) · choice (05) · tiles (06) · transform (07)
 *   error-spot (08) · dictation (09)
 *
 * The session orchestrator mixes them, re-queues misses for an in-session
 * retry (frame 03: "Back in this session in a few minutes") and shows the
 * done screen (frame 10).
 */

export type Level = "A1" | "A2" | "B1" | "B2";

interface Base {
  id: string;
  kind: string;
  level: Level;
  topic: string;
  /** Instruction line(s) under the topic chip. */
  task: string[];
  /** Grammar rule shown in the feedback panel, as wrapped lines. */
  rule: string[];
}

export interface ClozeData extends Base {
  kind: "cloze";
  head: string;
  tail: string;
  infinitive: string;
  answer: string;
}

export interface ChoiceData extends Base {
  kind: "choice";
  sentence: string;
  options: string[];
  correct: number;
}

export interface TilesData extends Base {
  kind: "tiles";
  /** The English sentence to build, shown as a quote under the task. */
  prompt: string;
  /** The correct word sequence. */
  target: string[];
  /** Shuffled bank including distractors. */
  bank: string[];
}

export interface TransformData extends Base {
  kind: "transform";
  source: string;
  answer: string;
}

export interface ErrorSpotData extends Base {
  kind: "error-spot";
  words: string[];
  /** Index into `words` of the wrong word. */
  wrong: number;
  fix: string;
  fixTask: string;
}

export interface DictationData extends Base {
  kind: "dictation";
  /** What the speech synthesis says. */
  speech: string;
  answer: string;
}

export type ExerciseItem =
  | ClozeData
  | ChoiceData
  | TilesData
  | TransformData
  | ErrorSpotData
  | DictationData;

export const SESSION: ExerciseItem[] = [
  {
    id: "etre-pc-rester",
    kind: "cloze",
    level: "A2",
    topic: "Passé composé with être",
    task: ["Put the verb in the passé composé."],
    head: "Hier soir, nous",
    tail: "à la maison.",
    infinitive: "rester",
    answer: "sommes restés",
    rule: [
      "Rester is one of the être verbs, so the participle",
      "agrees with the subject: nous sommes restés.",
    ],
  },
  {
    id: "cest-vs-ilest",
    kind: "choice",
    level: "A1",
    topic: "C'est vs il/elle est",
    task: ["Choose the right opening."],
    sentence: "____ une très bonne idée !",
    options: ["C'est", "Il est", "Elle est"],
    correct: 0,
    rule: [
      "C'est precedes a noun with its article: c'est une",
      "idée. Il est / elle est precede an adjective alone.",
    ],
  },
  {
    id: "subjonctif-sentiment",
    kind: "cloze",
    level: "B1",
    topic: "Subjonctif after feelings",
    task: ["Put the verb in the right form."],
    head: "Je suis content que tu",
    tail: ".",
    infinitive: "venir",
    answer: "viennes",
    rule: [
      "Être content que expresses a feeling, so the",
      "clause takes the subjonctif: que tu viennes.",
    ],
  },
  {
    id: "double-object-tiles",
    kind: "tiles",
    level: "B1",
    topic: "Double object pronouns",
    task: ["Build the sentence:", "“He already gave it to me.”"],
    prompt: "",
    target: ["Il", "me", "l'a", "déjà", "donné"],
    bank: ["Il", "me", "l'a", "déjà", "lui", "le", "donné"],
    rule: [
      "The indirect object comes first — me l'a — then",
      "déjà, then the participle: Il me l'a déjà donné.",
    ],
  },
  {
    id: "accord-lieu-precedé",
    kind: "error-spot",
    level: "B1",
    topic: "Agreement with preceding object",
    task: ["One word is wrong. Tap it."],
    words: ["Les", "fleurs", "que", "j'ai", "acheté", "sont", "belles."],
    wrong: 4,
    fix: "achetées",
    fixTask: "Now write the correct form.",
    rule: [
      "The direct object (les fleurs) precedes the verb,",
      "so the participle agrees: que j'ai achetées.",
    ],
  },
  {
    id: "cod-transform",
    kind: "transform",
    level: "A2",
    topic: "Direct object pronouns",
    task: ["Rewrite it, replacing the object", "with a pronoun."],
    source: "Elle a acheté le livre.",
    answer: "Elle l'a acheté",
    rule: [
      "Le livre is masculine singular → l'. The pronoun",
      "goes before avoir: Elle l'a acheté.",
    ],
  },
  {
    id: "silent-agreements-dictation",
    kind: "dictation",
    level: "A2",
    topic: "Passé composé with être",
    task: ["Type what you hear."],
    speech: "Les filles sont allées.",
    answer: "Les filles sont allées.",
    rule: [
      "With être in the passé composé, the participle",
      "agrees with the feminine plural subject: allées.",
    ],
  },
];

/** Number words for the done-screen copy ("Six items, mixed across …"). */
const NUM_WORDS = [
  "Zero", "One", "Two", "Three", "Four", "Five", "Six",
  "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve",
];

export function countWord(n: number): string {
  return NUM_WORDS[n] ?? String(n);
}

export const SESSION_TOPICS = new Set(SESSION.map((i) => i.topic)).size;
