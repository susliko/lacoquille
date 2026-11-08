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
