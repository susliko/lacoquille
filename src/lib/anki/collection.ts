/**
 * Inside the Anki package: the SQLite collection.
 *
 * Three database schemas exist across the three collection files:
 *
 *   · `collection.anki2` / `.anki21` — schema 11: note types and
 *     decks live as JSON blobs inside the single `col` row;
 *   · `collection.anki21b` — schema 18: they were promoted to real
 *     tables (`notetypes`, `fields`, `templates`, `decks`).
 *
 * notes / cards / revlog keep their columns across all of them, so
 * the reading of learner content is shared and only the metadata
 * branch differs. Everything is read into plain structures in one
 * pass and the database is closed — the wasm heap doesn't hold the
 * collection while the learner works.
 *
 * An app card maps to an Anki *note* (our model has one card per
 * front). The scheduling state comes from the note's first card
 * (lowest ord) — decks with recognition + production cards get the
 * first one's schedule, and the "What's in it" screen says so.
 */

import type { AnkiContainer, CollectionVariant } from "./container";

export interface AnkiNotetypeInfo {
  id: number;
  name: string;
  fields: string[];
  cloze: boolean;
  templates: number;
  /** Field names the card templates show on the question / answer
      side, in field order — the honest signal for defaulting the
      front↔back mapping ("Basic (and reversed)" decks, 3-field
      vocab layouts with audio columns, …). Empty when the export
      carries no readable templates; callers fall back to 0/1. */
  frontRefs: string[];
  backRefs: string[];
}

export interface AnkiDeckInfo {
  id: number;
  name: string;
}

export interface AnkiCardState {
  ord: number;
  did: number;
  type: number;
  queue: number;
  due: number;
  /** Days (review cards) — 0 or negative for learning. */
  ivl: number;
  /** Anki stores ease × 1000 (2500 = 2.5). */
  factor: number;
  reps: number;
  lapses: number;
}

export interface AnkiNoteRec {
  guid: string;
  mid: number;
  tags: string[];
  fields: string[];
  cards: AnkiCardState[];
}

export interface AnkiCollection {
  variant: CollectionVariant;
  /** Collection creation instant (epoch sec) — review due days count from it. */
  crt: number;
  notetypes: AnkiNotetypeInfo[];
  decks: AnkiDeckInfo[];
  notes: AnkiNoteRec[];
  /** Review history — the fact the import screen leads with. */
  revlog: { count: number; earliest: number | null };
}

const FLDS_SEP = "\x1f";

type Db = import("sql.js").Database;

/** Stage reports for the import screen's loading line. "decoder" is
    the wasm fetch — seconds on a phone, once; "cards" covers the
    note-reading loop on big decks. */
export type CollectionStage = "decoder" | "cards";

export async function openAnkiCollection(
  container: AnkiContainer,
  opts?: {
    /** Test hook — the node harness feeds the wasm binary directly;
        the browser always uses the default /vendor/ path. */
    sqlConfig?: { wasmBinary?: ArrayBuffer };
    onStage?: (s: CollectionStage) => void;
  },
): Promise<AnkiCollection> {
  // Dynamic import on purpose: the emscripten glue is ~60 KB of JS —
  // shipping it inside the import island made the page's first load
  // pay for it before the learner had picked a file. Only this page
  // ever pulls it, and only now, when a file is actually open.
  const { onStage, sqlConfig } = opts ?? {};
  onStage?.("decoder");
  const initSqlJs = (await import("sql.js")).default;
  // The wasm lives next to the app as a static asset. The name is
  // pinned: the browser glue asks for sql-wasm-browser.wasm, the
  // node harness for sql-wasm.wasm — same bytes either way.
  const SQL = await initSqlJs({
    locateFile: () => `/vendor/sql-wasm.wasm`,
    ...sqlConfig,
  });
  const db = new SQL.Database(container.collection);
  try {
    const tables = new Set(
      db
        .exec("SELECT lower(name) FROM sqlite_master WHERE type='table'")
        .flatMap((r) => r.values.map((v) => String(v[0]))),
    );

    const crt = Number(scalar(db, "SELECT crt FROM col LIMIT 1") ?? 0);
    const notetypes = readNotetypes(db, tables);
    const decks = readDecks(db, tables);
    onStage?.("cards");
    const notes = await readNotes(db);
    const revlog = tables.has("revlog")
      ? {
          count: Number(scalar(db, "SELECT COUNT(*) FROM revlog") ?? 0),
          earliest: (scalar(db, "SELECT MIN(id) FROM revlog") as number | null) ?? null,
        }
      : { count: 0, earliest: null };

    return { variant: container.variant, crt, notetypes, decks, notes, revlog };
  } finally {
    db.close();
  }
}

// ─── small helpers over sql.js' exec ────────────────────────────────

function scalar(db: Db, sql: string): string | number | null {
  try {
    const r = db.exec(sql);
    return r.length > 0 && r[0].values.length > 0 ? (r[0].values[0][0] as string | number) : null;
  } catch {
    return null;
  }
}

function rows(db: Db, sql: string): unknown[][] {
  try {
    const r = db.exec(sql);
    return r.length > 0 ? r[0].values : [];
  } catch {
    return [];
  }
}

const utf8 = new TextDecoder();
/** Schema 18 stores the notetype/template configs as BLOBs (Anki's
    own schema does too) — sql.js hands those back as Uint8Array, and
    String() on one produces digit soup, not JSON. */
function asText(v: unknown): string {
  return typeof v === "string" ? v : v instanceof Uint8Array ? utf8.decode(v) : String(v ?? "");
}

/** Column-safe select: old exports can lack a column (e.g. anki2 v1
    scheduler cards without `factor`) — missing columns come back null. */
function trySelect(db: Db, cols: string, table: string, extra = ""): unknown[][] {
  return rows(db, `SELECT ${cols} FROM ${table} ${extra}`);
}

// ─── metadata branches ──────────────────────────────────────────────

function readNotetypes(db: Db, tables: Set<string>): AnkiNotetypeInfo[] {
  if (tables.has("notetypes") && tables.has("fields")) {
    // schema 18 (.anki21b)
    const fieldNames = new Map<number, Map<number, string>>();
    for (const [ntid, ord, name] of rows(db, "SELECT ntid, ord, name FROM fields ORDER BY ntid, ord")) {
      const k = Number(ntid);
      if (!fieldNames.has(k)) fieldNames.set(k, new Map());
      fieldNames.get(k)!.set(Number(ord), String(name));
    }
    // the template config JSON carries qfmt/afmt — the question and
    // answer formats that say which fields show on which side
    const tmplSides = new Map<number, Array<{ q: string; a: string }>>();
    for (const [ntid, config] of rows(db, "SELECT ntid, config FROM templates")) {
      const cfg = safeJson(asText(config ?? "{}")) as { qfmt?: string; afmt?: string } | null;
      const k = Number(ntid);
      if (!tmplSides.has(k)) tmplSides.set(k, []);
      tmplSides.get(k)!.push({ q: cfg?.qfmt ?? "", a: cfg?.afmt ?? "" });
    }
    const out: AnkiNotetypeInfo[] = [];
    for (const [id, name, config] of rows(db, "SELECT id, name, config FROM notetypes ORDER BY id")) {
      const cfg = safeJson(asText(config ?? "{}")) as { type?: number } | null;
      const fields = [...(fieldNames.get(Number(id))?.values() ?? [])];
      const tmpls = tmplSides.get(Number(id)) ?? [];
      out.push({
        id: Number(id),
        name: String(name),
        fields,
        cloze: (cfg?.type ?? 0) === 1 || /cloze/i.test(String(name)),
        templates: Math.max(1, tmpls.length),
        ...sidesFor(fields, tmpls),
      });
    }
    return out;
  }

  // schema 11 (anki2 / anki21): models JSON in the col row
  const modelsJson = scalar(db, "SELECT models FROM col LIMIT 1");
  const models = safeJson(String(modelsJson ?? "{}")) as
    | Record<
        string,
        {
          id?: number;
          name?: string;
          type?: number;
          flds?: Array<{ name?: string }>;
          tmpls?: Array<{ q?: string; a?: string }>;
        }
      >
    | null;
  if (!models) return [];
  return Object.values(models).map((m) => {
    const fields = (m.flds ?? []).map((f) => String(f.name ?? ""));
    const tmpls = Array.isArray(m.tmpls)
      ? m.tmpls.map((t) => ({ q: String(t.q ?? ""), a: String(t.a ?? "") }))
      : [];
    return {
      id: Number(m.id ?? 0),
      name: String(m.name ?? "?"),
      fields,
      cloze: (m.type ?? 0) === 1 || /cloze/i.test(String(m.name ?? "")),
      templates: tmpls.length || 1,
      ...sidesFor(fields, tmpls),
    };
  });
}

/** Which fields a note type's templates put on the question vs the
    answer side. {{hint:Field}} and {{type:Field}} still name a field;
    {{FrontSide}}/{{Tags}}/cloze markers don't. Compared against the
    real field list, case-insensitively (Anki field refs are too). */
const TEMPLATE_FIELD_RE =
  /\{\{\s*(?:hint|type):\s*([^:#}]+?)\s*(?::[^}]*)?\}\}|\{\{\s*([^:#}]+?)\s*(?::[^}]*)?\}\}/g;
const NON_FIELD_REFS = new Set(["frontside", "tags", "cloze", "text"]);

function fieldRefs(fmt: string, fields: string[]): string[] {
  const lower = new Map(fields.map((f) => [f.toLowerCase(), f]));
  const hits = new Set<string>();
  for (const m of fmt.matchAll(TEMPLATE_FIELD_RE)) {
    const raw = (m[1] ?? m[2] ?? "").trim().toLowerCase();
    if (raw && !NON_FIELD_REFS.has(raw) && lower.has(raw)) hits.add(lower.get(raw)!);
  }
  // field order, not template order — the first-listed field is the
  // natural "front" when several appear on the question side
  return fields.filter((f) => hits.has(f));
}

function sidesFor(fields: string[], tmpls: Array<{ q: string; a: string }>): { frontRefs: string[]; backRefs: string[] } {
  const q = new Set<string>();
  const a = new Set<string>();
  for (const t of tmpls) {
    for (const f of fieldRefs(t.q, fields)) q.add(f);
    for (const f of fieldRefs(t.a, fields)) a.add(f);
  }
  return { frontRefs: fields.filter((f) => q.has(f)), backRefs: fields.filter((f) => a.has(f)) };
}

function readDecks(db: Db, tables: Set<string>): AnkiDeckInfo[] {
  if (tables.has("decks")) {
    // schema 18: decks became a table — but guard the columns anyway
    const rs = trySelect(db, "id, name", "decks");
    if (rs.length > 0) {
      return rs.map(([id, name]) => ({ id: Number(id), name: String(name) }));
    }
  }
  const decksJson = scalar(db, "SELECT decks FROM col LIMIT 1");
  const decks = safeJson(String(decksJson ?? "{}")) as Record<string, { id?: number; name?: string }> | null;
  if (!decks) return [];
  return Object.values(decks).map((d) => ({ id: Number(d.id ?? 0), name: String(d.name ?? "?") }));
}

// ─── learner content ────────────────────────────────────────────────

/** Paint between chunks: a 20k-note deck read synchronously freezes
    the screen with no feedback; yielding lets the staged "Reading
    cards…" line actually appear. */
const yieldPaint = (): Promise<void> =>
  new Promise((r) => requestAnimationFrame(() => setTimeout(r, 0)));

async function readNotes(db: Db): Promise<AnkiNoteRec[]> {
  // cards grouped by note, sorted by ord — first card owns the schedule
  const cardsByNote = new Map<number, AnkiCardState[]>();
  const cardRows = trySelect(
    db,
    "nid, ord, did, type, queue, due, ivl, factor, reps, lapses",
    "cards",
    "ORDER BY nid, ord",
  );
  for (let i = 0; i < cardRows.length; i++) {
    const row = cardRows[i];
    const nid = Number(row[0]);
    if (!cardsByNote.has(nid)) cardsByNote.set(nid, []);
    cardsByNote.get(nid)!.push({
      ord: Number(row[1]),
      did: Number(row[2]),
      type: Number(row[3]),
      queue: Number(row[4]),
      due: Number(row[5]),
      ivl: Number(row[6]),
      factor: Number(row[7]),
      reps: Number(row[8]),
      lapses: Number(row[9]),
    });
    if (i % 2000 === 1999) await yieldPaint();
  }

  const out: AnkiNoteRec[] = [];
  const noteRows = trySelect(db, "id, guid, mid, tags, flds", "notes", "ORDER BY id");
  for (let i = 0; i < noteRows.length; i++) {
    const row = noteRows[i];
    // cards.nid joins on notes.id (the rowid) — not the mid column,
    // which names the note *type* despite the prefix
    const id = Number(row[0]);
    const guid = String(row[1]);
    const mid = Number(row[2]);
    const tags = String(row[3] ?? "")
      .split(/\s+/)
      .filter(Boolean);
    const fields = String(row[4] ?? "").split(FLDS_SEP);
    out.push({ guid, mid, tags, fields, cards: cardsByNote.get(id) ?? [] });
    if (i % 2000 === 1999) await yieldPaint();
  }
  return out;
}

function safeJson(s: string): unknown {
  try {
    return JSON.parse(s);
  } catch {
    return null;
  }
}
