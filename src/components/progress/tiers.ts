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
 * History comes from two client-side stores: the endless feed
 * (lacoquille.feed.v1, keyed by slug) and the fixed session
 * (lacoquille.exercises.v1, keyed by curated title). The server's
 * curatedTitles map folds the latter into catalog topics.
 */

import { feedTopicStats } from "../exercise/feedStore";
import { topicSummaries } from "../exercise/store";
import type { CatalogTopic, Section, TopicsCatalog } from "./topicsApi";

export const SOLID_ATTEMPTS = 5;
export const SOLID_ACC = 0.8;
export const MASTERED_ATTEMPTS = 10;
export const MASTERED_ACC = 0.9;
export const MASTERED_WITHIN_DAYS = 21;
export const REVIEW_ATTEMPTS = 3;
export const REVIEW_ACC = 0.8;

/** Grid rows — our level model spans A1–B2 (see feedStore LEVELS). */
export const LEVELS = ["A1", "A2", "B1", "B2"] as const;
/** Grid columns — the four sections settled in the design (02-topics.md). */
export const SECTIONS: Section[] = ["tense", "mood", "nouns", "sentence"];

export type Tier = 0 | 1 | 2 | 3;
export const TIER_LABELS = ["Not started", "Learning", "Solid", "Mastered"] as const;

export interface TopicState {
  topic: CatalogTopic;
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

export type TopicStates = Record<string, TopicState>;

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

/** Fold both stores into per-slug aggregates keyed by catalog slug. */
function aggregates(catalog: TopicsCatalog): Map<string, Aggregate> {
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

export function topicStates(catalog: TopicsCatalog): TopicStates {
  const agg = aggregates(catalog);
  const out: TopicStates = {};
  for (const topic of catalog.topics) {
    const a = agg.get(topic.slug);
    const attempts = a?.attempts ?? 0;
    const correct = a?.correct ?? 0;
    const lastAt = a?.lastAt ?? 0;
    const accuracy = attempts > 0 ? correct / attempts : -1;

    let tier: Tier = 1;
    if (attempts === 0) tier = 0;
    else if (
      attempts >= MASTERED_ATTEMPTS &&
      accuracy >= MASTERED_ACC &&
      daysSince(lastAt) <= MASTERED_WITHIN_DAYS
    ) tier = 3;
    else if (attempts >= SOLID_ATTEMPTS && accuracy >= SOLID_ACC) tier = 2;

    const review = (a?.due ?? 0) > 0 || (attempts >= REVIEW_ATTEMPTS && accuracy < REVIEW_ACC);

    out[topic.slug] = { topic, attempts, correct, accuracy, lastAt, tier, review };
  }
  return out;
}

export interface CellState {
  level: string;
  section: Section;
  topics: TopicState[];
  total: number;
  solid: number;
  mastered: number;
  review: boolean;
  /** No catalog topic lives here yet; the cell renders hollow. */
  empty: boolean;
}

/**
 * The full matrix, rows × columns. The cell itself stays a quiet
 * container — each topic renders as its own mini-square, so the
 * per-topic state stays legible inside the cell.
 */
export function cellGrid(states: TopicStates): CellState[][] {
  return LEVELS.map((level) =>
    SECTIONS.map((section) => {
      const topics = Object.values(states).filter(
        (t) => t.topic.section === section && t.topic.levels.includes(level),
      );
      const solid = topics.filter((t) => t.tier >= 2);
      const mastered = topics.filter((t) => t.tier === 3);
      return {
        level,
        section,
        topics,
        total: topics.length,
        solid: solid.length,
        mastered: mastered.length,
        review: topics.some((t) => t.review),
        empty: topics.length === 0,
      };
    }),
  );
}
