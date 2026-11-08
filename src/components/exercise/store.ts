/**
 * Client-side learner state for the exercise session, persisted in
 * localStorage. The backend is a stateless generator by design; this
 * store is the single source of per-item history.
 *
 * Semantics (v1, deliberately simple):
 *  - Each item id keeps { last, misses, at }.
 *  - `last` is the most recent verdict — a miss sets "miss", any correct
 *    answer sets "correct" (a recovered retry clears the due).
 *  - `misses` is the lifetime miss count — it only ever grows, so future
 *    scheduling can weight hard items.
 *  - "Due" = items whose last answer was a miss. The done screen shows
 *    the live count; future sessions clear dues when the same template
 *    ids come back and are answered correctly.
 */

const KEY = "lacoquille.exercises.v1";

interface ItemStat {
  id: string;
  topic: string;
  last: "correct" | "miss";
  misses: number;
  at: number;
}

interface StoreShape {
  items: Record<string, ItemStat>;
}

function load(): StoreShape {
  try {
    const raw = localStorage.getItem(KEY);
    if (!raw) return { items: {} };
    const parsed = JSON.parse(raw) as StoreShape;
    if (typeof parsed !== "object" || parsed === null || typeof parsed.items !== "object") {
      return { items: {} };
    }
    return parsed;
  } catch {
    return { items: {} };
  }
}

function save(store: StoreShape): void {
  try {
    localStorage.setItem(KEY, JSON.stringify(store));
  } catch {
    // Private mode / quota — state is best-effort only.
  }
}

export function recordResult(id: string, topic: string, correct: boolean): void {
  const store = load();
  const prev = store.items[id];
  store.items[id] = {
    id,
    topic,
    last: correct ? "correct" : "miss",
    misses: (prev?.misses ?? 0) + (correct ? 0 : 1),
    at: Date.now(),
  };
  save(store);
}

export interface DueInfo {
  count: number;
  topics: string[];
}

/** Items whose last answer was a miss — what "tomorrow" will serve. */
export function dueInfo(): DueInfo {
  const { items } = load();
  const topics = new Set<string>();
  let count = 0;
  for (const stat of Object.values(items)) {
    if (stat.last === "miss") {
      count++;
      topics.add(stat.topic);
    }
  }
  return { count, topics: [...topics] };
}
