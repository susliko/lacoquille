/**
 * Progress tiers — the pure mapping from learner history onto the
 * grid's four-state scale (tokens.css --p0..--p3; five shades were
 * tested and dropped, see handoff docs/06-ux-findings.md):
 *
 *   0 Not started · 1 Learning · 2 Solid · 3 Mastered
 *
 * Thresholds are pedagogy, kept here in one place:
 *  - Solid:    ≥5 attempts at ≥80%.
 *  - Mastered: ≥10 attempts at ≥90%, recently (≤3 weeks) — the grid
 *    shows where you stand now, not where you once were.
 *  - Review dot (vermillon): items still due from the fixed session,
 *    or a topic drilled ≥3 times that still sits under 80%.
 *
 * State lands on taxonomy entries (taxonomy.ts), not raw catalog
 * topics: the grid draws the designed 121-topic skeleton, and the
 * catalog's 11 drillable topics light up the entries they cover.
 * History comes from two client-side stores — the endless feed
 * (lacoquille.feed.v1, keyed by slug) and the fixed session
 * (lacoquille.exercises.v1, keyed by curated title); the server's
 * curatedTitles map folds the latter into catalog slugs.
 */

import { feedTopicStats } from "../exercise/feedStore";
import { topicSummaries } from "../exercise/store";
import { GG_LEVELS, GG_KEYS, TAXONOMY, type FamKey, type TaxonomyEntry } from "./taxonomy";
import type { TopicsCatalog } from "./topicsApi";

export const SOLID_ATTEMPTS = 5;
export const SOLID_ACC = 0.8;
export const MASTERED_ATTEMPTS = 10;
export const MASTERED_ACC = 0.9;
export const MASTERED_WITHIN_DAYS = 21;
export const REVIEW_ATTEMPTS = 3;
export const REVIEW_ACC = 0.8;

export type Tier = 0 | 1 | 2 | 3;
export const TIER_LABELS = ["Not started", "Learning", "Solid", "Mastered"] as const;

export interface EntryState {
  entry: TaxonomyEntry;
  attempts: number;
  correct: number;
  /** 0..1, or -1 when never attempted. */
  accuracy: number;
  /** Epoch seconds of the most recent answer across both stores. */
  lastAt: number;
  tier: Tier;
  /** Vermillon dot — time to review. */
  review: boolean;
}

interface Aggregate {
  attempts: number;
  correct: number;
  due: number;
  lastAt: number;
}

/** The fixed session timestamps in ms; the feed store, in seconds. */
function toSeconds(at: number): number {
  return at > 1e12 ? Math.floor(at / 1000) : at;
}

function daysSince(sec: number): number {
  return (Date.now() / 1000 - sec) / 86400;
}

/** Fold both stores into per-slug aggregates. */
function slugAggregates(catalog: TopicsCatalog): Map<string, Aggregate> {
  const agg = new Map<string, Aggregate>();
  const merge = (slug: string, part: Partial<Aggregate>) => {
    const cur = agg.get(slug) ?? { attempts: 0, correct: 0, due: 0, lastAt: 0 };
    cur.attempts += part.attempts ?? 0;
    cur.correct += part.correct ?? 0;
    cur.due += part.due ?? 0;
    cur.lastAt = Math.max(cur.lastAt, part.lastAt ?? 0);
    agg.set(slug, cur);
  };

  for (const [slug, t] of Object.entries(feedTopicStats())) {
    merge(slug, { attempts: t.attempts, correct: t.correct, lastAt: t.lastAt });
  }
  for (const [title, s] of Object.entries(topicSummaries())) {
    const slug = catalog.curatedTitles[title];
    if (!slug) continue;
    merge(slug, { attempts: s.attempts, correct: s.correct, due: s.due, lastAt: toSeconds(s.lastAt) });
  }
  return agg;
}

/** Per-taxonomy-entry state: the full designed skeleton, filled
    where our drills cover it, honestly "Not started" elsewhere. */
export function entryStates(catalog: TopicsCatalog): Map<string, EntryState> {
  const agg = slugAggregates(catalog);
  const out = new Map<string, EntryState>();
  for (const entry of TAXONOMY) {
    let attempts = 0;
    let correct = 0;
    let due = 0;
    let lastAt = 0;
    for (const slug of entry.slugs) {
      const a = agg.get(slug);
      if (!a) continue;
      attempts += a.attempts;
      correct += a.correct;
      due += a.due;
      lastAt = Math.max(lastAt, a.lastAt);
    }
    const accuracy = attempts > 0 ? correct / attempts : -1;

    let tier: Tier = 1;
    if (attempts === 0) tier = 0;
    else if (
      attempts >= MASTERED_ATTEMPTS &&
      accuracy >= MASTERED_ACC &&
      daysSince(lastAt) <= MASTERED_WITHIN_DAYS
    ) tier = 3;
    else if (attempts >= SOLID_ATTEMPTS && accuracy >= SOLID_ACC) tier = 2;

    const review = due > 0 || (attempts >= REVIEW_ATTEMPTS && accuracy < REVIEW_ACC);

    out.set(entry.id, { entry, attempts, correct, accuracy, lastAt, tier, review });
  }
  return out;
}

/** Count of Solid-or-better entries in a list. */
export function solidOf(list: EntryState[]): number {
  return list.filter((t) => t.tier >= 2).length;
}

/** The entries of one scope — a cell (level+family), or a whole
    row/column when only one axis is given. */
export function entriesIn(
  states: Map<string, EntryState>,
  lv?: string,
  fam?: FamKey,
): EntryState[] {
  return [...states.values()].filter(
    (t) => (!lv || t.entry.lv === lv) && (!fam || t.entry.cat === fam),
  );
}

export interface LevelStat {
  id: string;
  total: number;
  solid: number;
  pct: number;
}

/** One row head's numbers: solid share of the whole level. */
export function levelStats(states: Map<string, EntryState>): LevelStat[] {
  return GG_LEVELS.map((id) => {
    const list = entriesIn(states, id);
    const solid = solidOf(list);
    return { id, total: list.length, solid, pct: Math.round((100 * solid) / list.length) };
  });
}

/** Sanity for dev: the four columns × six rows all exist. */
export const GRID_SHAPE_OK =
  GG_LEVELS.length === 6 &&
  GG_KEYS.length === 4 &&
  TAXONOMY.length === 121;
