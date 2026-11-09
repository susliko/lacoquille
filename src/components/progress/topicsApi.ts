/**
 * Client view of the topic catalog served by GET /api/exercises/topics.
 * Server-owned pedagogy: slugs, grid sections and level spans come from
 * the backend; slugs are the wire contract (never rename).
 */

export type Section = "tense" | "mood" | "nouns" | "sentence";

export const SECTION_LABELS: Record<Section, string> = {
  tense: "Tense",
  mood: "Mood",
  nouns: "Noun group",
  sentence: "Sentence",
};

export interface CatalogTopic {
  slug: string;
  title: string;
  blurb: string;
  /** Valid CEFR levels, low → high. A topic appears in one grid row per level. */
  levels: string[];
  section: Section;
}

export interface TopicsCatalog {
  topics: CatalogTopic[];
  /** Curated corpus title → catalog slug; folds fixed-session history into topics. */
  curatedTitles: Record<string, string>;
}

export async function fetchTopics(): Promise<TopicsCatalog> {
  // SPA navigations remount this island (Solid islands re-hydrate when
  // Astro swaps documents), and a cold grid shows empty cells until the
  // catalog lands. The catalog only changes on deploy, so a short-TTL
  // localStorage copy makes every remount paint instantly.
  try {
    const raw = localStorage.getItem(CACHE_KEY);
    if (raw) {
      const { at, catalog } = JSON.parse(raw) as { at: number; catalog: TopicsCatalog };
      if (Date.now() - at < CACHE_TTL_MS) return catalog;
    }
  } catch {
    /* corrupt or unavailable cache — fall through to the network */
  }

  const res = await fetch("/api/exercises/topics");
  if (!res.ok) throw new Error(`topics catalog: HTTP ${res.status}`);
  const catalog = (await res.json()) as TopicsCatalog;
  try {
    localStorage.setItem(CACHE_KEY, JSON.stringify({ at: Date.now(), catalog }));
  } catch {
    /* private mode: cache is best-effort */
  }
  return catalog;
}

const CACHE_KEY = "lacoquille.topics.v1";
const CACHE_TTL_MS = 10 * 60 * 1000;
