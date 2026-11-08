/**
 * Grading for the exercise session. The design's feedback has two states —
 * ✓ Correct and × Not quite — so unlike the drills there is no "almost".
 *
 * `eqLoose` accepts differences that don't matter in French typing:
 * case, accents (typed or not), apostrophe style, trailing punctuation
 * and repeated spaces. Everything else is a miss.
 */

import { norm } from "../drills/match";

function stripPunctuation(s: string): string {
  return s
    .replace(/[.,!?;:«»"“”‘’—–]/g, " ")
    .replace(/\s+/g, " ")
    .trim();
}

/** Accent-, case- and punctuation-insensitive equality. */
export function eqLoose(a: string, b: string): boolean {
  return norm(stripPunctuation(a)) === norm(stripPunctuation(b));
}

/** Word-by-word sequence equality (for the tiles exercise). */
export function seqLoose(a: string[], b: string[]): boolean {
  return (
    a.length === b.length && a.every((w, i) => norm(w) === norm(b[i]))
  );
}
