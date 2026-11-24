/**
 * Rule-sheet content for the peek sheet + the teaching card, from
 * src/data/knowledge/rules.json (keyed by taxonomy entry id) — the
 * same material the Theory tab renders at /theory/t/<id>, shown
 * as-is: every rule with its own worked example, plus the exception
 * when the sheet carries one. No editorial slicing: the full sheet
 * is the content (2026-11 simplification).
 */

import rulesJson from "../../data/knowledge/rules.json";
import { TAXONOMY } from "../progress/taxonomy";

export interface PeekRule {
  h?: string;
  /** Rule text; may carry <em> from the source sheets. */
  p: string;
  /** Worked example, <em> kept. */
  ex?: { fr: string; en: string };
}

export interface PeekContent {
  entryId: string;
  title: string;
  /** The complete rule sheet, as authored. */
  rules: PeekRule[];
  /** The sheet's exception note, when it has one. */
  exc?: string;
}

const RULES = rulesJson as Record<
  string,
  { rules: Array<{ h?: string; p: string; ex?: { fr: string; en: string } }>; exc?: string }
>;

/** slug → taxonomy entry (one slug may map to two rows; first wins). */
const BY_SLUG = new Map<string, { id: string; name: string }>();
for (const e of TAXONOMY) for (const s of e.slugs) if (!BY_SLUG.has(s)) BY_SLUG.set(s, { id: e.id, name: e.name });

export function peekFor(slug: string | undefined): PeekContent | null {
  if (!slug) return null;
  const entry = BY_SLUG.get(slug);
  if (!entry) return null;
  const sheet = RULES[entry.id];
  if (!sheet?.rules?.length) return null;
  return { entryId: entry.id, title: entry.name, rules: sheet.rules, exc: sheet.exc };
}

// ---- taught-topics ledger (teaching card shows once per topic) ----

const TAUGHT_KEY = "lacoquille.teach.v1";

export function isTaught(slug: string): boolean {
  try {
    const led = JSON.parse(localStorage.getItem(TAUGHT_KEY) ?? "{}") as Record<string, 1>;
    return !!led[slug];
  } catch {
    return false;
  }
}

export function markTaught(slug: string): void {
  try {
    const led = JSON.parse(localStorage.getItem(TAUGHT_KEY) ?? "{}") as Record<string, 1>;
    led[slug] = 1;
    localStorage.setItem(TAUGHT_KEY, JSON.stringify(led));
  } catch {
    /* private mode: teaching card just shows every session */
  }
}
