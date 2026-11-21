/**
 * Client for the endless-feed endpoint (`POST /api/exercises/feed`).
 * The request body is exactly the learner's context (see feedStore);
 * the server attaches the pedagogy context and its coach decides the
 * topic. The response is one run: a plan plus its items.
 *
 * Like `fetchSession`, this never throws — null means "feed
 * unavailable" and the UI offers a retry.
 */

import type { ExerciseItem } from "./data";
import { isExerciseItem } from "./api";
import { feedContext } from "./feedStore";

export interface TopicStat {
  slug: string;
  attempts: number;
  correct: number;
  lastAt: number;
}

export interface LearnerContext {
  level: string;
  topics: TopicStat[];
  recentTopics: string[];
  recentItems: string[];
}

export interface DrillPlan {
  topic: string;
  topicTitle: string;
  level: string;
  /** 0..1 — how settled the decision was. */
  confidence: number;
  source: "coach" | "built-in";
}

export interface FeedRun {
  plan: DrillPlan;
  items: ExerciseItem[];
  runLength: number;
}

export async function fetchFeed(ctx: LearnerContext, signal?: AbortSignal): Promise<FeedRun | null> {
  try {
    const res = await fetch("/api/exercises/feed", {
      method: "POST",
      headers: { "Content-Type": "application/json", Accept: "application/json" },
      body: JSON.stringify(ctx),
      signal,
    });
    if (!res.ok) return null;
    const data: unknown = await res.json();
    if (typeof data !== "object" || data === null) return null;
    const raw = data as { plan?: unknown; items?: unknown; runLength?: unknown };
    const plan = parsePlan(raw.plan);
    if (!plan) return null;
    if (!Array.isArray(raw.items) || raw.items.length === 0) return null;
    const items = raw.items.filter(isExerciseItem);
    if (items.length === 0) return null;
    return { plan, items, runLength: items.length };
  } catch {
    return null;
  }
}

/* ── Warm start: the home screen prefetches the first run ───────

Tapping "Continue" used to mean a dead "Preparing your feed…" screen
for the whole POST round-trip — worse when the Fly machine wakes from
auto-stop. The practice home fires the identical request while the
learner is still reading the hero; the feed consumes it on arrival. */

let warm: Promise<FeedRun | null> | null = null;

/** Fire the first-run request early (safe anywhere — never throws). */
export function warmFeed(): void {
  if (warm) return;
  let ctx: LearnerContext;
  try {
    ctx = feedContext();
  } catch {
    return; // store unavailable (private mode etc.) — feed fetches its own
  }
  const p = fetchFeed(ctx);
  warm = p;
  p.then((run) => {
    if (!run && warm === p) warm = null; // failed — feed fetches fresh
  }).catch(() => {
    if (warm === p) warm = null;
  });
}

/** Consume the warm run if one is in flight or done; null → fetch. */
export async function takeWarmRun(): Promise<FeedRun | null> {
  const p = warm;
  warm = null;
  return p;
}

function parsePlan(v: unknown): DrillPlan | null {
  if (typeof v !== "object" || v === null) return null;
  const p = v as Record<string, unknown>;
  if (typeof p.topic !== "string" || typeof p.topicTitle !== "string" || typeof p.level !== "string") {
    return null;
  }
  return {
    topic: p.topic,
    topicTitle: p.topicTitle,
    level: p.level,
    confidence: typeof p.confidence === "number" ? p.confidence : 0,
    source: p.source === "coach" ? "coach" : "built-in",
  };
}
