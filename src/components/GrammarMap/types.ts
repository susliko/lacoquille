export type MoodId = "indicatif" | "conditionnel" | "subjonctif" | "imperatif";
export type EdgeType = "auxiliary-compound" | "aspect-pair" | "stem-share" | "mood-swap" | "anteriority";
export type AspectId = "completed" | "ongoing" | "anterior" | "habitual" | "none";
export type TimePosition = "far-past" | "past" | "near-past" | "present" | "near-future" | "future" | "far-future";

export interface DiagramNode {
  id: string;
  lane: MoodId;
  timePosition: TimePosition;
  aspect: AspectId;
  literary: boolean;
  title?: string;
}

export interface DiagramEdge {
  type: EdgeType;
  from: string;
  to: string;
  label: string;
  literary?: boolean;
}

export interface DiagramData {
  nodes: DiagramNode[];
  edges: DiagramEdge[];
}

export const TIME_COLUMNS: Record<TimePosition, number> = {
  "far-past":    0,
  "past":        1,
  "near-past":   2,
  "present":     3,
  "near-future": 4,
  "future":      5,
  "far-future":  6,
};

export const TIME_LABELS: Record<TimePosition, string> = {
  "far-past":    "anterior",
  "past":        "past",
  "near-past":   "recent past",
  "present":     "present",
  "near-future": "near future",
  "future":      "future",
  "far-future":  "future anterior",
};

export const MOOD_LABELS: Record<MoodId, string> = {
  indicatif:    "Indicatif",
  conditionnel: "Conditionnel",
  subjonctif:   "Subjonctif",
  imperatif:    "Impératif",
};

export const MOOD_ORDER: MoodId[] = ["indicatif", "conditionnel", "subjonctif", "imperatif"];

export const MOOD_HUE: Record<MoodId, number> = {
  indicatif:    210,
  conditionnel: 265,
  subjonctif:   30,
  imperatif:    160,
};

// Each edge type gets a distinct COLOR — primary differentiator.
// Dash pattern is kept as a secondary, subtle reinforcement only.
export const EDGE_COLOR: Record<EdgeType, string> = {
  "auxiliary-compound": "#4d8eff",  // blue  — "built from" (most common edge)
  "aspect-pair":        "#f97316",  // orange — ongoing vs completed (most important for learners)
  "stem-share":         "#c4913a",  // gold  — same stem/form
  "mood-swap":          "#9d6cf0",  // purple — crosses moods
  "anteriority":        "#6b8ba4",  // slate  — temporal sequence
};

export const EDGE_DASH: Record<EdgeType, string> = {
  "auxiliary-compound": "none",
  "aspect-pair":        "7 4",
  "stem-share":         "3 3",
  "mood-swap":          "11 5",
  "anteriority":        "4 3",
};

export const EDGE_LABEL: Record<EdgeType, string> = {
  "auxiliary-compound": "Auxiliary-compound — built from another tense via avoir/être + PP",
  "aspect-pair":        "Aspect pair — same time, different view: ongoing vs completed",
  "stem-share":         "Stem share — same infinitive stem, different endings",
  "mood-swap":          "Mood swap — same time position, different mood",
  "anteriority":        "Anteriority — action completed before another",
};
