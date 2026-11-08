/**
 * Learner state for the endless feed, persisted in localStorage.
 * This is the ONLY thing the client sends to the feed endpoint —
 * its own context (level estimate, per-topic history, recent
 * activity). Everything pedagogical is attached server-side.
 *
 * Level estimate: a rolling correctness window. When a window of
 * ≥10 answers closes above 85% the level bumps up, below 40% it
 * drops one; the window then resets.
 */

import type { TopicStat } from "./feedApi";

const KEY = "lacoquille.feed.v1";
const LEVELS = ["A1", "A2", "B1", "B2"] as const;
export type Level = (typeof LEVELS)[number];

const RECENT_ITEMS_CAP = 40;
const RECENT_TOPICS_CAP = 4;
const WINDOW_MIN = 10;
const WINDOW_UP = 0.85;
const WINDOW_DOWN = 0.4;

interface TopicAgg {
  attempts: number;
  correct: number;
  lastAt: number;
}

interface FeedState {
  level: Level;
  topics: Record<string, TopicAgg>;
  recentItems: string[];
  recentTopics: string[];
  windowAttempts: number;
  windowCorrect: number;
}

function fresh(): FeedState {
  return {
    level: "A1",
    topics: {},
    recentItems: [],
    recentTopics: [],
    windowAttempts: 0,
    windowCorrect: 0,
  };
}

function load(): FeedState {
  try {
    const raw = localStorage.getItem(KEY);
    if (!raw) return fresh();
    const parsed = JSON.parse(raw) as FeedState;
    if (typeof parsed !== "object" || parsed === null) return fresh();
    return { ...fresh(), ...parsed, topics: parsed.topics ?? {} };
  } catch {
    return fresh();
  }
}

function save(s: FeedState): void {
  try {
    localStorage.setItem(KEY, JSON.stringify(s));
  } catch {
    // Best-effort only.
  }
}

let state: FeedState | null = null;

function mut(fn: (s: FeedState) => void): FeedState {
  if (!state) state = load();
  fn(state);
  save(state);
  return state;
}

function read(): FeedState {
  if (!state) state = load();
  return state;
}

function bumpLevel(s: FeedState, dir: 1 | -1): void {
  const i = LEVELS.indexOf(s.level);
  const next = Math.min(LEVELS.length - 1, Math.max(0, i + dir));
  s.level = LEVELS[next];
  s.windowAttempts = 0;
  s.windowCorrect = 0;
}

/** Record one graded answer; updates topic stats, rings, level window. */
export function recordFeed(topicSlug: string, itemId: string, correct: boolean): void {
  mut((s) => {
    const t = s.topics[topicSlug] ?? { attempts: 0, correct: 0, lastAt: 0 };
    t.attempts += 1;
    t.correct += correct ? 1 : 0;
    t.lastAt = Math.floor(Date.now() / 1000);
    s.topics[topicSlug] = t;

    s.recentItems = [itemId, ...s.recentItems.filter((x) => x !== itemId)].slice(0, RECENT_ITEMS_CAP);
    s.recentTopics = [topicSlug, ...s.recentTopics.filter((x) => x !== topicSlug)].slice(0, RECENT_TOPICS_CAP);

    s.windowAttempts += 1;
    s.windowCorrect += correct ? 1 : 0;
    if (s.windowAttempts >= WINDOW_MIN) {
      const rate = s.windowCorrect / s.windowAttempts;
      if (rate >= WINDOW_UP && s.level !== "B2") bumpLevel(s, 1);
      else if (rate <= WINDOW_DOWN && s.level !== "A1") bumpLevel(s, -1);
      else {
        s.windowAttempts = 0;
        s.windowCorrect = 0;
      }
    }
  });
}

/** The whole wire payload — learner context and nothing else. */
export function feedContext(): { level: string; topics: TopicStat[]; recentTopics: string[]; recentItems: string[] } {
  const s = read();
  return {
    level: s.level,
    topics: Object.entries(s.topics).map(([slug, t]) => ({
      slug,
      attempts: t.attempts,
      correct: t.correct,
      lastAt: t.lastAt,
    })),
    recentTopics: [...s.recentTopics],
    recentItems: [...s.recentItems],
  };
}

/** Topic aggregates for the sitting summary (UI-only). */
export function feedTopicStats(): Record<string, TopicAgg> {
  return read().topics;
}

export function currentLevel(): Level {
  return read().level;
}
