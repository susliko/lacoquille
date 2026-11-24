/**
 * Anki → lacoquille mapping.
 *
 * One app card per Anki note; the schedule comes from the note's
 * first card. Anki schedules with SM-2 variants or FSRS, ours is an
 * SM-2 flavour — the conversion is honest about being an estimate:
 * intervals and eases carry over, exact future due dates will not
 * match Anki's (the import screen says this in so many words).
 *
 * Cloze notes are the one structural surprise: our cards have no
 * blanks, so the first deletion becomes the card — front shows the
 * sentence with that deletion masked, back reveals it. Extra
 * deletions stay revealed on the front (as Anki shows them).
 */

import type { AnkiCollection, AnkiCardState, AnkiNotetypeInfo } from "./collection";

export interface FieldChoice {
  front: number;
  back: number;
}

/** Notetype id → chosen fields. */
export type FieldMapping = Record<number, FieldChoice>;

export interface PlannedCard {
  front: string;
  back: string;
  due: number;
  interval: number;
  ease: number;
  reps: number;
  lapses: number;
}

export interface PlannedDeck {
  name: string;
  cards: PlannedCard[];
}

export interface ImportStats {
  notes: number;
  /** Notes whose notetype has more than one card template. */
  multiCard: number;
  cloze: number;
  /** Notes dropped for empty front after cleaning. */
  empty: number;
  /** Notes in Anki decks we can't name (data drifted). */
  orphanDecks: number;
}

const EASE_MIN = 1.3;
const EASE_MAX = 3.0;
const DAY = 86400;
/** Year 2100 in epoch seconds — a due past this must be milliseconds. */
const MS_CUTOFF = 4102444800;

// ─── defaults ───────────────────────────────────────────────────────

/** Field 0 = front, field 1 = back is only right for a plain Basic
    note type. When the templates say which fields show on which
    side, believe them — vocab decks love 3-field layouts
    ([French, French example, English]) where "back = field 1" puts
    French on both sides of the card. Cloze maps the text field. */
export function defaultMapping(col: AnkiCollection): FieldMapping {
  const m: FieldMapping = {};
  for (const nt of col.notetypes) {
    if (nt.cloze) {
      m[nt.id] = { front: 0, back: 0 };
      continue;
    }
    const front = firstRefIndex(nt.frontRefs, nt.fields) ?? 0;
    // first answer-side field that isn't the front; else whatever
    // follows it; single-field note types map the field to both sides
    let back = -1;
    for (const ref of nt.backRefs) {
      const i = nt.fields.indexOf(ref);
      if (i !== -1 && i !== front) {
        back = i;
        break;
      }
    }
    if (back === -1) back = nt.fields.length > 1 ? (front === 0 ? 1 : 0) : 0;
    m[nt.id] = { front, back };
  }
  return m;
}

function firstRefIndex(refs: string[], fields: string[]): number | null {
  for (const f of fields) {
    if (refs.includes(f)) return fields.indexOf(f);
  }
  return null;
}

// ─── field cleaning ─────────────────────────────────────────────────

const CLOZE_RE = /\{\{c(\d+)::(.*?)(?:::([^}]*))?\}\}/gs;
const SOUND_RE = /\[sound:[^\]]*\]/g;

function decodeEntities(s: string): string {
  return s
    .replace(/&nbsp;/gi, " ")
    .replace(/&amp;/gi, "&")
    .replace(/&lt;/gi, "<")
    .replace(/&gt;/gi, ">")
    .replace(/&quot;/gi, '"')
    .replace(/&(?:apos|#39|#8217);/gi, "'")
    .replace(/&#(\d+);/g, (_, d: string) => String.fromCodePoint(Number(d)));
}

/** Fields arrive as HTML fragments; our cards are plain text. Block
    ends become "; " (meaning lists read right), everything else is
    stripped, then entities decoded. */
function cleanHtml(raw: string): string {
  return decodeEntities(
    raw
      .replace(SOUND_RE, "")
      .replace(/<br\s*\/?\s*>/gi, "; ")
      .replace(/<\/(?:p|div|li|h[1-6]|tr)>/gi, "; ")
      .replace(/<[^>]+>/g, "")
      .replace(/\{\{[^}]*\}\}/g, ""),
  )
    .replace(/[ \t]+/g, " ")
    .replace(/\s*;\s*(?:;\s*)+/g, "; ")
    .trim()
    .replace(/^[;,.\s]+/, "");
}

/** Front for a cloze note: chosen deletion masked, others revealed. */
function clozeSides(raw: string): { front: string; back: string } | null {
  const matches = [...raw.matchAll(CLOZE_RE)];
  if (matches.length === 0) return null;
  const first = matches.reduce((a, b) => (Number(a[1]) <= Number(b[1]) ? a : b));
  const answer = first[2].trim();
  const hint = first[3]?.trim();
  let done = 0;
  const front = raw.replace(CLOZE_RE, (_m, n: string, inner: string, h?: string) => {
    if (n === first[1]) {
      done += 1;
      return `[${h?.trim() ? h.trim() : "…"}]`;
    }
    return inner.trim() || (h?.trim() ? `[${h.trim()}]` : "");
  });
  return { front: cleanHtml(front), back: cleanHtml(answer + (hint ? ` (${hint})` : "")) };
}

// ─── scheduling conversion ──────────────────────────────────────────

const clamp = (v: number, lo: number, hi: number) => Math.min(hi, Math.max(lo, v));

/** Learning due arrives as epoch seconds (or milliseconds in some
    exports, or a stray day number); coerce to sane "now-ish". */
function normSeconds(d: number, now: number): number {
  if (d > MS_CUTOFF) d = Math.floor(d / 1000);
  if (d <= 0 || d > now + 3650 * DAY) return now;
  return clamp(d, now - 3650 * DAY, now + 3650 * DAY);
}

function reviewDue(days: number, crt: number, now: number): number {
  // exported without scheduling → due resets to 0 anyway; guard the
  // rest against corrupt day numbers rather than trust them
  const d = crt + days * DAY;
  if (!Number.isFinite(d) || d > now + 3650 * DAY) return now;
  return clamp(d, now - 3650 * DAY, now + 3650 * DAY);
}

function toPlanned(card: AnkiCardState, crt: number, now: number): PlannedCard {
  const q = card.queue;
  const t = card.type;
  const isLearning = q === 1 || q === 3 || (t === 1 && q >= 0);
  const isReview = !isLearning && (t === 2 || (q >= 2 && card.ivl > 0) || (q < 0 && card.ivl > 0));

  if (isLearning) {
    return {
      front: "",
      back: "",
      due: normSeconds(card.due, now),
      interval: 0,
      ease: 2.5,
      reps: Math.max(1, card.reps),
      lapses: card.lapses,
    };
  }
  if (isReview) {
    return {
      front: "",
      back: "",
      due: reviewDue(card.due, crt, now),
      interval: clamp(card.ivl, 1, 36500),
      ease: clamp((card.factor || 2500) / 1000, EASE_MIN, EASE_MAX),
      reps: Math.max(1, card.reps),
      lapses: card.lapses,
    };
  }
  // new, suspended-new, buried-new — everything else starts fresh
  return { front: "", back: "", due: now, interval: 0, ease: 2.5, reps: 0, lapses: card.lapses };
}

// ─── the plan ───────────────────────────────────────────────────────

export interface ImportPlan {
  decks: PlannedDeck[];
  stats: ImportStats;
}

export function buildPlan(col: AnkiCollection, mapping: FieldMapping): ImportPlan {
  const now = Math.floor(Date.now() / 1000);
  const fallback = defaultMapping(col);
  const byId = new Map(col.notetypes.map((nt) => [nt.id, nt]));
  const deckNames = new Map(col.decks.map((d) => [d.id, d.name]));
  const stats: ImportStats = { notes: col.notes.length, multiCard: 0, cloze: 0, empty: 0, orphanDecks: 0 };
  const perDeck = new Map<string, PlannedCard[]>();

  for (const note of col.notes) {
    const nt: AnkiNotetypeInfo | undefined = byId.get(note.mid);
    const first = note.cards.find((c) => c.queue !== undefined); // already ord-sorted
    if (!nt || !first) {
      stats.orphanDecks += 0; // no deck assignment possible; counted as skipped below
      stats.empty += 1;
      continue;
    }
    const deckName = deckNames.get(first.did) ?? "";
    if (!deckName) {
      stats.orphanDecks += 1;
      continue;
    }
    if (nt.templates > 1 || note.cards.length > 1) stats.multiCard += 1;

    const choice = mapping[note.mid] ?? fallback[note.mid] ?? { front: 0, back: 0 };
    let front = "";
    let back = "";
    if (nt.cloze) {
      stats.cloze += 1;
      const sides = clozeSides(note.fields[choice.front] ?? "");
      if (sides) {
        front = sides.front;
        back = sides.back;
      } else {
        // a "cloze" notetype with no deletions — fall back to plain
        front = cleanHtml(note.fields[choice.front] ?? "");
        back = cleanHtml(note.fields[choice.back] ?? "");
      }
    } else {
      front = cleanHtml(note.fields[choice.front] ?? "");
      back = cleanHtml(note.fields[choice.back] ?? "");
    }
    if (!front || !back) {
      stats.empty += 1;
      continue;
    }

    const planned = toPlanned(first, col.crt, now);
    planned.front = front;
    planned.back = back;
    if (!perDeck.has(deckName)) perDeck.set(deckName, []);
    perDeck.get(deckName)!.push(planned);
  }

  const decks: PlannedDeck[] = [...perDeck.entries()]
    .map(([name, cards]) => ({ name, cards }))
    .sort((a, b) => a.name.localeCompare(b.name));
  return { decks, stats };
}

/** A live "what the first card will look like" sample for the mapping
    screen — the cheapest way to see that a mapping is wrong (French
    on both sides) before 2,000 cards commit it. Uses the first note
    of this note type that produces a non-empty pair. */
export function sampleFor(
  col: AnkiCollection,
  mapping: FieldMapping,
  nt: AnkiNotetypeInfo,
): { front: string; back: string } | null {
  const choice = mapping[nt.id] ?? defaultMapping(col)[nt.id];
  if (!choice) return null;
  for (const note of col.notes) {
    if (note.mid !== nt.id) continue;
    let sides: { front: string; back: string } | null = null;
    if (nt.cloze) {
      sides = clozeSides(note.fields[choice.front] ?? "");
      if (!sides) {
        const front = cleanHtml(note.fields[choice.front] ?? "");
        const back = cleanHtml(note.fields[choice.back] ?? "");
        if (front && back) sides = { front, back };
      }
    } else {
      const front = cleanHtml(note.fields[choice.front] ?? "");
      const back = cleanHtml(note.fields[choice.back] ?? "");
      if (front && back) sides = { front, back };
    }
    if (sides) return sides;
  }
  return null;
}
