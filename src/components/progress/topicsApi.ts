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
  const res = await fetch("/api/exercises/topics");
  if (!res.ok) throw new Error(`topics catalog: HTTP ${res.status}`);
  return (await res.json()) as TopicsCatalog;
}
