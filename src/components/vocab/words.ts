/**
 * Word-level view of the vocab store.
 *
 * The store keeps cards; a reversed import drills one word through
 * two cards (FR→EN and EN→FR) with separate schedules — the import
 * stamps a note-level `word` on both so they collapse here. A
 * vocabulary *size* counts words, and the word is as strong as its
 * strongest card.
 *
 * Passive-vs-active: these numbers estimate *recognition* vocabulary
 * — what you'd understand when reading. That does not decay with
 * the review clock the way active recall does, so the proxy ignores
 * elapsed time: a word once held at a 21-day interval is counted as
 * passively known, full stop. The drill is where active recall is
 * measured; this is not that.
 */

import { readVocab, type VocabCard } from "./vocabStore";
import { wordKey } from "../../lib/word";

/** Anki's convention: interval 21 days or more = mature. */
export const MATURE_DAYS = 21;

export { wordKey };

export interface WordSplit {
  /** Unique words across all decks. */
  total: number;
  mature: number;
  young: number;
  learning: number;
  fresh: number;
  /** Σ passive-known proxy over all words (the deck term of the
      vocabulary-size estimate). */
  known: number;
  /** Normalized word keys — for the spot-check to exclude. */
  keys: Set<string>;
}

/** Passive-known proxy of one card. The young tier is the honest
    uncertain middle: reviewed to days-not-weeks, recognized in
    context more often than not. */
export function passiveProxy(c: VocabCard): number {
  if (c.reps === 0) return 0;
  if (c.interval >= MATURE_DAYS) return 1;
  if (c.interval >= 1) return 0.7;
  return 0.2;
}

/** Collapse all cards into words. */
export function wordSplit(): WordSplit {
  const cards = Object.values(readVocab().cards);
  const byWord = new Map<string, VocabCard>();
  for (const c of cards) {
    // imports stamp a note-level word on every direction of a note
    // ("Mouse" → "La souris" shares the word with its French twin);
    // mining cards carry it from creation; anything older falls back
    // to the front
    const key = c.word?.trim() || wordKey(c.front) || wordKey(c.back) || "·";
    const cur = byWord.get(key);
    if (!cur || proxyOrder(c) > proxyOrder(cur)) byWord.set(key, c);
  }
  let mature = 0;
  let young = 0;
  let learning = 0;
  let fresh = 0;
  let known = 0;
  for (const c of byWord.values()) {
    if (c.reps === 0) fresh += 1;
    else if (c.interval >= MATURE_DAYS) mature += 1;
    else if (c.interval >= 1) young += 1;
    else learning += 1;
    known += passiveProxy(c);
  }
  return {
    total: byWord.size,
    mature,
    young,
    learning,
    fresh,
    known,
    keys: new Set(byWord.keys()),
  };
}

/** Which card represents the word: the strongest schedule. Mature
    (2) > young (1) > learning (0.5) > fresh (0). */
function proxyOrder(c: VocabCard): number {
  if (c.reps === 0) return 0;
  if (c.interval >= MATURE_DAYS) return 2;
  if (c.interval >= 1) return 1;
  return 0.5;
}
