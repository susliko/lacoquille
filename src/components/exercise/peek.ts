/**
 * Peek + teaching-card content, from the compact rule sheets
 * (src/data/knowledge/rules.json, keyed by taxonomy entry id).
 *
 * The peek is the rule over the feed in a dismissible sheet: two or
 * three sentences and a small table of worked examples — not the full
 * page, too much to read inside a feed. The teaching card is the same
 * content shown before a topic's first item ever (retrieval practice
 * assumes the material has been met).
 */

import rulesJson from "../../data/knowledge/rules.json";
import { TAXONOMY } from "../progress/taxonomy";

export interface PeekBlock {
  h?: string;
  /** Rule text; may carry <em> from the source sheets. */
  p: string;
}

export interface PeekContent {
  entryId: string;
  title: string;
  blocks: PeekBlock[];
  /** [french, english] pairs, <em> kept. */
  examples: Array<[string, string]>;
}

const RULES = rulesJson as Record<
  string,
  { rules: Array<{ h?: string; p: string; ex?: { fr: string; en: string } }> }
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
  const blocks: PeekBlock[] = sheet.rules.slice(0, 3).map((r) => ({ h: r.h, p: r.p }));
  const examples: Array<[string, string]> = [];
  for (const r of sheet.rules) {
    if (r.ex && examples.length < 4) examples.push([r.ex.fr, r.ex.en]);
  }
  return { entryId: entry.id, title: entry.name, blocks, examples };
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
