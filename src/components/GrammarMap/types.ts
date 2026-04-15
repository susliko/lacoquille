export type MoodId = "indicatif" | "conditionnel" | "subjonctif" | "imperatif";
export type EdgeType = "auxiliary-compound" | "aspect-pair" | "stem-share" | "mood-swap" | "anteriority";
export type AspectId = "completed" | "ongoing" | "anterior" | "habitual" | "none";
export type TimePosition = "far-past" | "past" | "near-past" | "present" | "near-future" | "future" | "far-future";

export interface DiagramNode {
  id: string;           // tense slug, matches content collection
  lane: MoodId;
  timePosition: TimePosition;
  aspect: AspectId;
  literary: boolean;
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

// Maps time positions to column indices (left = past, right = future)
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

// CSS hue for each mood (matches global.css --mood-* vars)
export const MOOD_HUE: Record<MoodId, number> = {
  indicatif:    210,
  conditionnel: 265,
  subjonctif:   30,
  imperatif:    160,
};

export const EDGE_DASH: Record<EdgeType, string> = {
  "auxiliary-compound": "none",
  "aspect-pair":        "6 3",
  "stem-share":         "3 3",
  "mood-swap":          "8 4",
  "anteriority":        "4 2",
};
