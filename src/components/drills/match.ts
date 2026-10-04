/**
 * Pure matching logic for the conjugation drills. Used by ClozeDrill,
 * CueDrill, TenseDrill, and ChainDrill. No DOM, no Solid imports.
 *
 * Three-state judgement:
 *  - "correct" : exact match (case-insensitive)
 *  - "almost"  : a 1-character Levenshtein edit OR identical after
 *           stripping diacritics (the "missing accent" case)
 *  - "wrong"   : anything else
 *
 * The "almost" state is critical for French — learners often type "parlé"
 * without the accent, or "parler" instead of "parlait". Treating those as
 * "wrong" is hostile. Treating them as "correct" hides the difference.
 * "Almost" with a gentle note is the right answer.
 */

export type Verdict = "correct" | "almost" | "wrong";

/**
 * Normalise a string for accent/apostrophe-insensitive comparison.
 * - lowercase
 * - strip diacritics (NFD decomposition)
 * - normalise curly apostrophes to straight
 */
export function norm(s: string): string {
  return s
    .toLowerCase()
    .normalize("NFD")
    .replace(/[\u0300-\u036f]/g, "")
    .replace(/[''‛]/g, "'")
    .trim();
}

/**
 * Strip "ne" and "pas" from a string, then collapse whitespace. Used by
 * ChainDrill where the learner is asked to transform a form into its negative
 * ("elle n'est pas allée"). Stripping the negation wrappers makes "was" etc.
 * structurally equal.
 */
export function normNoNegation(s: string): string {
  return norm(s).replace(/\bne\b|\bpas\b/g, "").replace(/\s+/g, "");
}

/**
 * True if `a` and `b` differ by at most one edit (Levenshtein distance ≤ 1).
 * Used as the "almost" gate after the accent-stripped equality check.
 */
export function levenshteinLeq1(a: string, b: string): boolean {
  if (a === b) return true;
  if (Math.abs(a.length - b.length) > 1) return false;

  const m = a.length;
  const n = b.length;
  // Allocate the dp matrix as a flat array — V8 handles this well.
  const dp = new Array<number>((m + 1) * (n + 1));
  for (let i = 0; i <= m; i++) dp[i * (n + 1)] = i;
  for (let j = 0; j <= n; j++) dp[j] = j;

  for (let i = 1; i <= m; i++) {
    for (let j = 1; j <= n; j++) {
      const cost = a[i - 1] === b[j - 1] ? 0 : 1;
      const a1 = dp[(i - 1) * (n + 1) + j] + 1;
      const a2 = dp[i * (n + 1) + (j - 1)] + 1;
      const a3 = dp[(i - 1) * (n + 1) + (j - 1)] + cost;
      dp[i * (n + 1) + j] = Math.min(a1, a2, a3);
    }
  }

  return dp[m * (n + 1) + n] <= 1;
}

/**
 * Judge a learner's input against the expected answer.
 * Empty input is "wrong" (no verdict for empty answers).
 */
export function judge(input: string, answer: string): Verdict {
  const v = input.trim();
  if (v === "") return "wrong";
  if (v.toLowerCase() === answer) return "correct";
  if (levenshteinLeq1(norm(v), norm(answer))) return "almost";
  return "wrong";
}

/**
 * Negation-tolerant judgement for ChainDrill steps that require a negation
 * transformation ("elle n'est pas allée"). Tries the basic first, then falls
 * back to the no-negation comparison.
 */
export function judgeWithNegation(input: string, answer: string): Verdict {
  const basic = judge(input, answer);
  if (basic === "correct") return basic;
  const a = normNoNegation(input);
  const b = normNoNegation(answer);
  if (a === b) return "correct";
  if (levenshteinLeq1(a, b)) return "almost";
  return "wrong";
}