/**
 * Client for the exercise generator API (`lacq` backend,
 * GET /api/exercises). The endpoint is a stateless generator — all
 * learner state lives here in the client (see store.ts).
 *
 * `fetchSession` never throws: on any failure it resolves to null and
 * the caller falls back to the curated local session, so the exercise
 * experience works even frontend-only.
 */

import type { ExerciseItem } from "./data";

export interface SessionBatch {
  items: ExerciseItem[];
  seed: number;
  count: number;
}

export interface FetchSessionOptions {
  count?: number;
  /** Comma-separated: cloze, choice, tiles, transform, error-spot, dictation. */
  types?: string;
  /** A1 · A2 · B1 · B2, or omit for mixed. */
  level?: string;
  /** Pass a previous seed to replay the same session. */
  seed?: number;
  signal?: AbortSignal;
}

export async function fetchSession(
  opts: FetchSessionOptions = {},
): Promise<SessionBatch | null> {
  const params = new URLSearchParams();
  if (opts.count != null) params.set("count", String(opts.count));
  if (opts.types) params.set("types", opts.types);
  if (opts.level) params.set("level", opts.level);
  if (opts.seed != null) params.set("seed", String(opts.seed));

  try {
    const res = await fetch(`/api/exercises?${params.toString()}`, {
      signal: opts.signal,
      headers: { Accept: "application/json" },
    });
    if (!res.ok) return null;
    const data: unknown = await res.json();
    if (typeof data !== "object" || data === null) return null;
    const raw = data as { items?: unknown; seed?: unknown };
    if (!Array.isArray(raw.items)) return null;
    const items = raw.items.filter(isExerciseItem);
    if (items.length === 0) return null;
    return {
      items,
      seed: typeof raw.seed === "number" ? raw.seed : 0,
      count: items.length,
    };
  } catch {
    return null;
  }
}

/* ── Runtime validation — the API contract is the TS union ───────── */

const str = (v: unknown): v is string => typeof v === "string" && v.length > 0;
const strArr = (v: unknown): v is string[] =>
  Array.isArray(v) && v.length > 0 && v.every((x) => typeof x === "string");
const base = (v: Record<string, unknown>): boolean => str(v.id) && strArr(v.task) && strArr(v.rule);

export function isExerciseItem(v: unknown): v is ExerciseItem {
  if (typeof v !== "object" || v === null) return false;
  const o = v as Record<string, unknown>;
  switch (o.kind) {
    case "cloze":
      return base(o) && str(o.head) && str(o.tail) && str(o.infinitive) && str(o.answer);
    case "choice":
      return (
        base(o) && str(o.sentence) && strArr(o.options) &&
        typeof o.correct === "number" && o.correct >= 0 && o.correct < (o.options as string[]).length
      );
    case "tiles":
      return (
        base(o) && typeof o.prompt === "string" && strArr(o.target) && strArr(o.bank) &&
        (o.bank as string[]).length >= (o.target as string[]).length
      );
    case "transform":
      return base(o) && str(o.source) && str(o.answer);
    case "error-spot":
      return (
        base(o) && strArr(o.words) &&
        typeof o.wrong === "number" && o.wrong >= 0 && o.wrong < (o.words as string[]).length &&
        str(o.fix) && str(o.fixTask)
      );
    case "dictation":
      return base(o) && str(o.speech) && str(o.answer);
    default:
      return false;
  }
}
