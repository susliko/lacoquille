/**
 * Grading for the exercise feed. Three verdicts, per the handoff:
 * ✓ Correct · ✓ Right — mind the accent · × Not quite.
 *
 * A missing accent is a near-miss: accepted with a nudge (lenient is
 * the default; strict remains available for DELF candidates). Marking
 * it wrong tests the keyboard, not the grammar.
 *
 * eqLoose stays for tiles/choice sequence comparisons.
 */

import { norm } from "../drills/match";
import { readSettings } from "../settings/settingsStore";

function stripPunctuation(s: string): string {
  return s
    .replace(/[.,!?;:«»"“”‘’—–]/g, " ")
    .replace(/\s+/g, " ")
    .trim();
}

function foldApostrophes(s: string): string {
  return s.replace(/[''‛]/g, "'");
}

/** Case-, punctuation- and apostrophe-insensitive — but accents kept. */
function eqStrict(a: string, b: string): boolean {
  return (
    foldApostrophes(stripPunctuation(a).toLowerCase()) ===
    foldApostrophes(stripPunctuation(b).toLowerCase())
  );
}

/** Accent-, case- and punctuation-insensitive equality. */
export function eqLoose(a: string, b: string): boolean {
  return norm(stripPunctuation(a)) === norm(stripPunctuation(b));
}

export type Grade = "ok" | "accent" | "miss";

/** The single entry point for typed answers. */
export function gradeAnswer(input: string, answer: string): Grade {
  if (eqStrict(input, answer)) return "ok";
  if (eqLoose(input, answer)) {
    return readSettings().accents === "lenient" ? "accent" : "miss";
  }
  return "miss";
}

/** Word-by-word sequence equality (for the tiles exercise). */
export function seqLoose(a: string[], b: string[]): boolean {
  return (
    a.length === b.length && a.every((w, i) => norm(w) === norm(b[i]))
  );
}
