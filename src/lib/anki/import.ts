/**
 * Anki → lacoquille mapping.
 *
 * Scheduling conversion is honest about being an estimate: intervals
 * and eases carry over, exact future due dates will not match Anki's
 * (the import screen says this in so many words).
 *
 * Note structure → app cards:
 *   · one direction  → one app card, front/back from the screen's
 *     field mapping (defaults follow the template sides);
 *   · several directions ("Basic (and reversed card)") → one app
 *     card PER DIRECTION, front/back taken from each template's own
 *     sides, schedule from the matching Anki card (cards.ord = the
 *     template's ord). FR→EN and EN→FR are separate cards with
 *     separate schedules, exactly as Anki drills them.
 *   · cloze → the first deletion becomes the card.
 *
 * Side-aware cleaning: a field can embed a collapsed <details>
 * example (a popular vocab-deck pattern). On the answer side it
 * unwraps to its content — what Anki shows expanded. On the question
 * side it is stripped entirely — a reversed card must never show the
 * answer hidden in its own front.
 */

import type { AnkiCollection, AnkiCardState, AnkiNotetypeInfo, AnkiTemplateSides } from "./collection";

export interface FieldChoice {
  front: number;
  back: number;
}

/** Notetype id → chosen fields (single-direction note types only). */
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
  /** Notes that import more than one direction (reversed decks). */
  multiDir: number;
  cloze: number;
  /** Notes dropped for empty front/back after cleaning. */
  empty: number;
  /** Notes in Anki decks we can't name (data drifted). */
  orphanDecks: number;
}

const EASE_MIN = 1.3;
const EASE_MAX = 3.0;
const DAY = 86400;
/** Year 2100 in epoch seconds — a due past this must be milliseconds. */
const MS_CUTOFF = 4102444800;

// ─── defaults (single-direction note types) ──────────────────────────

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

// ─── field cleaning ──────────────────────────────────────────────────

const CLOZE_RE = /\{\{c(\d+)::(.*?)(?:::([^}]*))?\}\}/gs;
const SOUND_RE = /\[sound:[^\]]*\]/g;
const DETAILS_RE = /<details[^>]*>(?:\s*<summary[^>]*>[\s\S]*?<\/summary>)?([\s\S]*?)<\/details>/gi;

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

type Side = "front" | "back";

function cleanHtml(raw: string, side: Side): string {
  const noSound = raw.replace(SOUND_RE, "").replace(/<details[^>]*>\s*$/gi, "");
  // the collapsed-example pattern: answer side keeps the content
  // (what Anki shows expanded), question side drops it whole
  const noDetails =
    side === "front"
      ? noSound.replace(DETAILS_RE, "")
      : noSound.replace(DETAILS_RE, " $1 ");
  return decodeEntities(
    noDetails
      .replace(/<br\s*\/?\s*>/gi, "; ")
      .replace(/<\/(?:p|div|li|h[1-6]|tr)>/gi, "; ")
      .replace(/<[^>]+>/g, "")
      .replace(/\{\{[^}]*\}\}/g, ""),
  )
    .replace(/[ \t\r\n]+/g, " ")
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
  const front = raw.replace(CLOZE_RE, (_m, n: string, inner: string, h?: string) => {
    if (n === first[1]) {
      return `[${h?.trim() ? h.trim() : "…"}]`;
    }
    return inner.trim() || (h?.trim() ? `[${h.trim()}]` : "");
  });
  return { front: cleanHtml(front, "front"), back: cleanHtml(answer + (hint ? ` (${hint})` : ""), "back") };
}

/** Pick the first field by name with non-empty cleaned content. */
function firstContent(
  names: string[],
  fields: string[],
  nt: AnkiNotetypeInfo,
  side: Side,
): string {
  for (const name of names) {
    const idx = nt.fields.indexOf(name);
    if (idx !== -1 && idx < fields.length) {
      const v = cleanHtml(fields[idx] ?? "", side);
      if (v) return v;
    }
  }
  return "";
}

// ─── scheduling conversion ───────────────────────────────────────────

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

// ─── directions ──────────────────────────────────────────────────────

/** Unique question/answer side-pairs of a note type, in template
    order. "Basic (and reversed card)" yields two; two templates
    that show the same sides collapse into one. */
export function directionsOf(nt: AnkiNotetypeInfo): AnkiTemplateSides[] {
  const seen = new Set<string>();
  const out: AnkiTemplateSides[] = [];
  for (const t of nt.templateSides) {
    const key = `${t.q.join("\x1f")}|${t.a.join("\x1f")}`;
    if (seen.has(key)) continue;
    seen.add(key);
    out.push(t);
  }
  return out;
}

/** The plan ─────────────────────────────────────────────────────────── */

export interface ImportPlan {
  decks: PlannedDeck[];
  stats: ImportStats;
}

export function buildPlan(col: AnkiCollection, mapping: FieldMapping): ImportPlan {
  const now = Math.floor(Date.now() / 1000);
  const fallback = defaultMapping(col);
  const byId = new Map(col.notetypes.map((nt) => [nt.id, nt]));
  const deckNames = new Map(col.decks.map((d) => [d.id, d.name]));
  const stats: ImportStats = { notes: col.notes.length, multiDir: 0, cloze: 0, empty: 0, orphanDecks: 0 };
  const perDeck = new Map<string, PlannedCard[]>();
  const push = (deckName: string, card: PlannedCard) => {
    if (!perDeck.has(deckName)) perDeck.set(deckName, []);
    perDeck.get(deckName)!.push(card);
  };

  for (const note of col.notes) {
    const nt = byId.get(note.mid);
    if (!nt) {
      stats.empty += 1;
      continue;
    }

    if (nt.cloze) {
      const deckName = deckNames.get(note.cards[0]?.did ?? -1) ?? "";
      if (!deckName) {
        stats.orphanDecks += 1;
        continue;
      }
      stats.cloze += 1;
      const choice = mapping[note.mid] ?? fallback[note.mid] ?? { front: 0, back: 0 };
      const raw = note.fields[choice.front] ?? "";
      const sides = clozeSides(raw) ?? {
        front: cleanHtml(raw, "front"),
        back: cleanHtml(note.fields[choice.back] ?? "", "back"),
      };
      if (!sides.front || !sides.back) {
        stats.empty += 1;
        continue;
      }
      const card = note.cards[0];
      const planned = toPlanned(
        card ?? { ord: 0, did: -1, type: 0, queue: 0, due: 0, ivl: 0, factor: 0, reps: 0, lapses: 0 },
        col.crt,
        now,
      );
      push(deckName, { ...planned, front: sides.front, back: sides.back });
      continue;
    }

    const dirs = directionsOf(nt);
    const deckName = deckNames.get(note.cards[0]?.did ?? -1) ?? "";

    if (dirs.length <= 1) {
      // single direction — the manual mapping decides, the schedule
      // comes from the note's first card
      const first = note.cards[0];
      if (!deckName || !first) {
        if (!deckName) stats.orphanDecks += 1;
        else stats.empty += 1;
        continue;
      }
      const choice = mapping[note.mid] ?? fallback[note.mid] ?? { front: 0, back: 0 };
      const front = cleanHtml(note.fields[choice.front] ?? "", "front");
      const back = cleanHtml(note.fields[choice.back] ?? "", "back");
      if (!front || !back) {
        stats.empty += 1;
        continue;
      }
      push(deckName, { ...toPlanned(first, col.crt, now), front, back });
      continue;
    }

    // several directions — one app card per direction, each with its
    // own template's sides and its own card's schedule
    if (!deckName) {
      stats.orphanDecks += 1;
      continue;
    }
    stats.multiDir += 1;
    let imported = 0;
    for (const dir of dirs) {
      // optional reversed cards: Anki doesn't generate the direction
      // when its trigger field is empty — respect that
      const state = note.cards.find((c) => c.ord === dir.ord);
      if (!state) continue;
      const front = firstContent(dir.q, note.fields, nt, "front");
      const back = firstContent(dir.a.filter((f) => !dir.q.includes(f)), note.fields, nt, "back");
      if (!front || !back) continue;
      push(deckName, { ...toPlanned(state, col.crt, now), front, back });
      imported += 1;
    }
    if (imported === 0) stats.empty += 1;
  }

  const decks: PlannedDeck[] = [...perDeck.entries()]
    .map(([name, cards]) => ({ name, cards }))
    .sort((a, b) => a.name.localeCompare(b.name));
  return { decks, stats };
}

/** Live "what the first cards will look like" samples for the mapping
    screen — the cheapest way to see that a mapping is wrong (French
    on both sides) before 2,000 cards commit it. One sample per
    direction, from the first note of this type that produces one. */
export function samplesFor(
  col: AnkiCollection,
  mapping: FieldMapping,
  nt: AnkiNotetypeInfo,
): Array<{ label: string; front: string; back: string }> {
  const fallback = defaultMapping(col);
  const dirs = nt.cloze ? [] : directionsOf(nt);
  const out: Array<{ label: string; front: string; back: string }> = [];

  if (dirs.length <= 1) {
    const choice = mapping[nt.id] ?? fallback[nt.id] ?? { front: 0, back: 0 };
    for (const note of col.notes) {
      if (note.mid !== nt.id) continue;
      const front = cleanHtml(note.fields[choice.front] ?? "", "front");
      const back = cleanHtml(note.fields[choice.back] ?? "", "back");
      if (front && back) {
        out.push({ label: nt.name, front, back });
        break;
      }
    }
    return out;
  }

  for (const dir of dirs) {
    for (const note of col.notes) {
      if (note.mid !== nt.id) continue;
      const front = firstContent(dir.q, note.fields, nt, "front");
      const back = firstContent(dir.a.filter((f) => !dir.q.includes(f)), note.fields, nt, "back");
      if (front && back) {
        out.push({ label: dir.name || `Card ${dir.ord + 1}`, front, back });
        break;
      }
    }
  }
  return out;
}
