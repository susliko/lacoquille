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

import initSqlJs from "sql.js";
import type { AnkiContainer, CollectionVariant } from "./container";

export interface AnkiNotetypeInfo {
  id: number;
  name: string;
  fields: string[];
  cloze: boolean;
  templates: number;
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

export async function openAnkiCollection(
  container: AnkiContainer,
  /** Test hook only — the node harness feeds the wasm binary
      directly; the browser always uses the default /vendor/ path. */
  sqlConfig?: { wasmBinary?: ArrayBuffer },
): Promise<AnkiCollection> {
  // The wasm lives next to the app as a static asset; only this page
  // ever pulls it in (dynamic import, ~660 KB gzipped, cached). The
  // name is pinned: the browser glue asks for sql-wasm-browser.wasm,
  // the node harness for sql-wasm.wasm — same bytes either way.
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
    const notes = readNotes(db);
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
    const tmplCount = new Map<number, number>();
    for (const [ntid, n] of rows(db, "SELECT ntid, COUNT(*) FROM templates GROUP BY ntid")) {
      tmplCount.set(Number(ntid), Number(n));
    }
    const out: AnkiNotetypeInfo[] = [];
    for (const [id, name, config] of rows(db, "SELECT id, name, config FROM notetypes ORDER BY id")) {
      const cfg = safeJson(String(config ?? "{}")) as { type?: number } | null;
      const fields = [...(fieldNames.get(Number(id))?.values() ?? [])];
      out.push({
        id: Number(id),
        name: String(name),
        fields,
        cloze: (cfg?.type ?? 0) === 1 || /cloze/i.test(String(name)),
        templates: tmplCount.get(Number(id)) ?? 1,
      });
    }
    return out;
  }

  // schema 11 (anki2 / anki21): models JSON in the col row
  const modelsJson = scalar(db, "SELECT models FROM col LIMIT 1");
  const models = safeJson(String(modelsJson ?? "{}")) as
    | Record<string, { id?: number; name?: string; type?: number; flds?: Array<{ name?: string }>; tmpls?: unknown[] }>
    | null;
  if (!models) return [];
  return Object.values(models).map((m) => ({
    id: Number(m.id ?? 0),
    name: String(m.name ?? "?"),
    fields: (m.flds ?? []).map((f) => String(f.name ?? "")),
    cloze: (m.type ?? 0) === 1 || /cloze/i.test(String(m.name ?? "")),
    templates: Array.isArray(m.tmpls) ? m.tmpls.length : 1,
  }));
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

function readNotes(db: Db): AnkiNoteRec[] {
  // cards grouped by note, sorted by ord — first card owns the schedule
  const cardsByNote = new Map<number, AnkiCardState[]>();
  for (const row of trySelect(
    db,
    "nid, ord, did, type, queue, due, ivl, factor, reps, lapses",
    "cards",
    "ORDER BY nid, ord",
  )) {
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
  }

  const out: AnkiNoteRec[] = [];
  for (const row of trySelect(db, "id, guid, mid, tags, flds", "notes", "ORDER BY id")) {
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
