/**
 * Vocabulary store — lacoquille.vocab.v1 (handoff 2026-11, §6).
 *
 * A separate feed with its own queue: grammar and vocabulary do not
 * share a scheduler (an imported deck must never bury grammar).
 *
 * Scheduling is SM-2-flavoured: ease starts at 2.5, Again resets the
 * card to a ~10-minute rep, Hard/Good/Easy multiply the interval.
 * No interval previews in the UI — a scheduling readout was cut
 * everywhere (docs/01-decisions.md).
 *
 * Anki packages (.apkg / .colpkg) import client-side — parse, field
 * mapping and schedule conversion live in src/lib/anki; the commit
 * lands here as pre-scheduled cards. Decks carry an optional `src`
 * badge ("anki") so the decks list can tell imports from mining.
 */

import type { Level } from "../exercise/feedStore";

export type Rate = "again" | "hard" | "good" | "easy";

export interface VocabCard {
  id: string;
  deckId: string;
  front: string;
  back: string;
  /** Source sentence, kept from mining. May be empty. */
  cx: string;
  /** Epoch seconds when the card is next reviewable. */
  due: number;
  /** Current interval in days (0 = learning step). */
  interval: number;
  ease: number;
  reps: number;
  lapses: number;
  lastAt: number;
}

export interface VocabDeck {
  id: string;
  name: string;
  createdAt: number;
  /** "anki" = arrived through import (badged in the decks list). */
  src?: "anki";
  importedAt?: number;
}

export interface VocabState {
  decks: VocabDeck[];
  cards: Record<string, VocabCard>;
  /** Rated this sitting — for the minimal end screen, not a stat display. */
  session: { rated: number };
}

const KEY = "lacoquille.vocab.v1";

const MIN = 60;
const DAY = 86400;

export function readVocab(): VocabState {
  try {
    const raw = localStorage.getItem(KEY);
    if (raw) {
      const s = JSON.parse(raw) as VocabState;
      return { decks: s.decks ?? [], cards: s.cards ?? {}, session: s.session ?? { rated: 0 } };
    }
  } catch {
    /* fall through to empty */
  }
  return { decks: [], cards: {}, session: { rated: 0 } };
}

function write(s: VocabState): void {
  try {
    localStorage.setItem(KEY, JSON.stringify(s));
  } catch {
    /* private mode: session-only */
  }
}

let seq = 0;
function newId(): string {
  seq += 1;
  return `${Date.now().toString(36)}-${seq.toString(36)}`;
}

export function addDeck(name: string): VocabDeck {
  const s = readVocab();
  const deck: VocabDeck = { id: newId(), name, createdAt: Math.floor(Date.now() / 1000) };
  s.decks.push(deck);
  write(s);
  return deck;
}

export function deckName(id: string): string {
  return readVocab().decks.find((d) => d.id === id)?.name ?? "";
}

export function addCards(
  deckId: string,
  entries: Array<{ front: string; back: string; cx?: string }>,
): number {
  const s = readVocab();
  const now = Math.floor(Date.now() / 1000);
  // skip words already in any deck (same front, case-insensitive)
  const seen = new Set(Object.values(s.cards).map((c) => c.front.toLowerCase()));
  let added = 0;
  for (const e of entries) {
    const front = e.front.trim();
    if (!front || seen.has(front.toLowerCase())) continue;
    const id = newId();
    s.cards[id] = {
      id,
      deckId,
      front,
      back: e.back.trim(),
      cx: e.cx ?? "",
      due: now,
      interval: 0,
      ease: 2.5,
      reps: 0,
      lapses: 0,
      lastAt: 0,
    };
    seen.add(front.toLowerCase());
    added += 1;
  }
  write(s);
  return added;
}

/** Cards due right now, mixed across decks, learning cards first. */
export function dueQueue(deckId?: string): VocabCard[] {
  const now = Math.floor(Date.now() / 1000);
  const all = Object.values(readVocab().cards);
  const due = (deckId ? all.filter((c) => c.deckId === deckId) : all).filter((c) => c.due <= now);
  // deterministic-ish variety: shuffle, then sort learning cards first
  for (let i = due.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [due[i], due[j]] = [due[j], due[i]];
  }
  return due.sort((a, b) => (a.reps === 0 ? -1 : 1) - (b.reps === 0 ? -1 : 1));
}

export function deckCounts(): Array<{ deck: VocabDeck; n: number }> {
  const s = readVocab();
  return s.decks.map((d) => ({
    deck: d,
    n: Object.values(s.cards).filter((c) => c.deckId === d.id).length,
  }));
}

// ---- scheduling ----

/** SM-2-flavoured rating. Returns the updated card. */
export function rateCard(card: VocabCard, rate: Rate): VocabCard {
  const now = Math.floor(Date.now() / 1000);
  const c = { ...card, lastAt: now, reps: card.reps + 1 };

  if (rate === "again") {
    c.lapses += 1;
    c.interval = 0;
    c.ease = Math.max(1.3, c.ease - 0.2);
    c.due = now + 10 * MIN;
    return c;
  }
  if (c.interval === 0) {
    // graduating step
    c.interval = rate === "easy" ? 3 : rate === "hard" ? 0.5 : 1;
  } else {
    if (rate === "hard") c.ease = Math.max(1.3, c.ease - 0.15);
    if (rate === "easy") c.ease = Math.min(3.0, c.ease + 0.15);
    const mult = rate === "hard" ? 1.2 : rate === "easy" ? c.ease * 1.3 : c.ease;
    c.interval = Math.max(1, c.interval * mult);
  }
  c.due = now + Math.round(c.interval * DAY);
  return c;
}

/** Persist a rating; returns the snapshot for undo. */
export function commitRating(before: VocabCard, after: VocabCard): void {
  const s = readVocab();
  s.cards[after.id] = after;
  s.session.rated += 1;
  write(s);
}

/** Undo restores the exact pre-rating card and pulls the count back. */
export function undoRating(before: VocabCard): void {
  const s = readVocab();
  s.cards[before.id] = before;
  s.session.rated = Math.max(0, s.session.rated - 1);
  write(s);
}

export function clearSessionCounter(): void {
  const s = readVocab();
  s.session.rated = 0;
  write(s);
}

// ---- Anki import (src/lib/anki) ----

export interface PlannedCard {
  front: string;
  back: string;
  due: number;
  interval: number;
  ease: number;
  reps: number;
  lapses: number;
}

/** Commit an import plan: one app deck per Anki deck (badged),
    cards keep their converted schedule, dedup is the same one rule
    everywhere — a front already in any deck is skipped. Returns
    counts for the result screen. */
export function commitAnkiImport(
  decks: Array<{ name: string; cards: PlannedCard[] }>,
): { added: number; decksCreated: number; deckIds: string[] } {
  const s = readVocab();
  const now = Math.floor(Date.now() / 1000);
  const seen = new Set(Object.values(s.cards).map((c) => c.front.toLowerCase()));
  let added = 0;
  const deckIds: string[] = [];

  for (const plan of decks) {
    const fresh: Array<{ p: PlannedCard; id: string }> = [];
    for (const p of plan.cards) {
      const front = p.front.trim();
      if (!front || seen.has(front.toLowerCase())) continue;
      const id = newId();
      fresh.push({ p, id });
      seen.add(front.toLowerCase());
    }
    // a deck whose cards were all duplicates never gets created
    if (fresh.length === 0) continue;
    const deck: VocabDeck = {
      id: newId(),
      name: plan.name,
      createdAt: now,
      src: "anki",
      importedAt: now,
    };
    s.decks.push(deck);
    deckIds.push(deck.id);
    for (const { p, id } of fresh) {
      s.cards[id] = {
        id,
        deckId: deck.id,
        front: p.front.trim(),
        back: p.back.trim(),
        cx: "",
        due: p.due,
        interval: p.interval,
        ease: p.ease,
        reps: p.reps,
        lapses: p.lapses,
        lastAt: 0,
      };
    }
    added += fresh.length;
  }
  write(s);
  return { added, decksCreated: deckIds.length, deckIds };
}

// ---- mining (paste-in v1) ----

/** Compact French stopword list — function words and the verbs that
    appear in every text. Lemmatisation is an open question; v1
    accepts surface forms and skips duplicates by exact form. */
const STOP = new Set(
  ("je tu il elle on nous vous ils elles me te se le la les l un une des de du d au aux à et ou mais donc or ni car " +
    "que qui quoi dont où ce cet cette ces c son sa ses leur leurs mon ma mes ton ta tes notre votre nos vos se s y en " +
    "est sont était étaient a ai as ont eu être été fait faire fais peu très trop bien mieux pire alors quand comme " +
    "si ne pas plus jamais rien personne tout tous toute toutes même aussi chaque quelques dans sur sous chez vers " +
    "pour par avec sans sans entre depuis avant après pendant encore déjà alors donc alors voici voilà cest")
    .split(" "),
);

/** Surface forms worth keeping: 2+ letters, not a stopword, not a number. */
export function mineWords(text: string): Array<{ front: string; back: string; cx: string }> {
  const words = text.toLowerCase().match(/[a-zà-öø-ÿœæ'’-]{2,}/g) ?? [];
  const seen = new Set<string>();
  const out: Array<{ front: string; back: string; cx: string }> = [];
  for (const w of words) {
    const front = w.replace(/^[dnl]'|['’]$/g, "").replace(/[’']/g, "'");
    if (front.length < 2 || STOP.has(front) || seen.has(front)) continue;
    seen.add(front);
    out.push({ front, back: "", cx: "" });
  }
  return out;
}

/** The context sentence for a word: the first line/sentence holding it. */
export function contextFor(text: string, word: string): string {
  const sentences = text.split(/(?<=[.!?…])\s+|\n+/);
  const hit = sentences.find((s) =>
    new RegExp(`(^|[^a-zà-öø-ÿœæ])${word.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}([^a-zà-öø-ÿœæ]|$)`, "i").test(s),
  );
  return (hit ?? "").trim().slice(0, 140);
}

export type { Level };
