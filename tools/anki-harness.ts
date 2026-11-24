/**
 * Import harness — runs the real lib code (container → collection →
 * plan → commit) against the fixtures from tools/anki-fixtures.py.
 *
 *   python3 tools/anki-fixtures.py /tmp/anki-fixtures
 *   node_modules/.bin/esbuild tools/anki-harness.ts --bundle \
 *     --platform=node --format=esm --outfile=/tmp/anki-harness.mjs
 *   node /tmp/anki-harness.mjs /tmp/anki-fixtures
 *
 * Exit 0 = every check green.
 */

/* localStorage stub BEFORE vocabStore runs — the commit path writes
   through it, and node has no localStorage. */
const mem = new Map<string, string>();
(globalThis as Record<string, unknown>).localStorage = {
  getItem: (k: string) => mem.get(k) ?? null,
  setItem: (k: string, v: string) => void mem.set(k, v),
  removeItem: (k: string) => void mem.delete(k),
};

import { readFileSync } from "node:fs";
import { unzipSync } from "fflate";
import initSqlJs from "sql.js";
import { AnkiError, readAnkiContainer } from "../src/lib/anki/container";
import { openAnkiCollection } from "../src/lib/anki/collection";
import { buildPlan, defaultMapping } from "../src/lib/anki/import";
import { buildAnkiExport } from "../src/lib/anki/export";
import { commitAnkiImport, readVocab } from "../src/components/vocab/vocabStore";

const DIR = process.argv[2] ?? "/tmp/anki-fixtures";
const CRT = 1590000000;
const DAY = 86400;

let failures = 0;
let checks = 0;
function ok(cond: boolean, label: string, extra?: unknown) {
  checks += 1;
  if (!cond) {
    failures += 1;
    console.log(`  ✗ ${label}${extra !== undefined ? ` — ${JSON.stringify(extra)}` : ""}`);
  }
}

const bytes = (name: string) => new Uint8Array(readFileSync(`${DIR}/${name}`));

async function load(name: string) {
  const blob = new Blob([bytes(name) as BlobPart]);
  const ct = await readAnkiContainer(blob);
  const col = await openAnkiCollection(ct, {
    // node: feed the wasm directly (browser uses /vendor/sql-wasm.wasm)
    wasmBinary: readFileSync("public/vendor/sql-wasm.wasm").buffer as ArrayBuffer,
  });
  return { ct, col };
}

async function main() {
  // sql.js sanity — the module must initialise at all
  await initSqlJs({ wasmBinary: readFileSync("public/vendor/sql-wasm.wasm").buffer as ArrayBuffer });

  // ── schema 11 ──────────────────────────────────────────────────
  {
    console.log("schema11.apkg");
    const { ct, col } = await load("schema11.apkg");
    ok(ct.variant === "anki2", "variant is anki2", ct.variant);
    ok(ct.kind === "zip", "container kind");
    ok(ct.mediaCount === 2, "media count from JSON map", ct.mediaCount);
    ok(col.notetypes.length === 1 && col.notetypes[0].name === "Basic", "notetype Basic");
    ok(
      JSON.stringify(col.notetypes[0].fields) === JSON.stringify(["Front", "Back"]),
      "fields Front/Back",
      col.notetypes[0].fields,
    );
    ok(col.notetypes[0].templates === 2, "template count from col JSON");
    ok(col.notetypes[0].cloze === false, "not cloze");
    ok(col.decks.some((d) => d.name === "French::Core 2k"), "deck name with :: path");
    ok(col.notes.length === 3, "three notes", col.notes.length);
    ok(col.revlog.count === 5, "revlog count", col.revlog);
    ok(col.revlog.earliest === 1591000000000, "revlog earliest (History fact)", col.revlog.earliest);

    const plan = buildPlan(col, defaultMapping(col));
    ok(plan.decks.length === 1, "one app deck", plan.decks.map((d) => d.name));
    ok(plan.decks[0].name === "French::Core 2k", "deck name carried");
    ok(plan.decks[0].cards.length === 3, "three cards planned");
    const [review, fresh, learning] = plan.decks[0].cards;
    ok(review.front === "une goéland" && review.back === "a gull", "review card fields");
    ok(review.interval === 12, "review interval carried (days)", review.interval);
    ok(review.ease === 2.5, "ease = factor/1000", review.ease);
    ok(review.due === CRT + 1234 * DAY, "review due = crt + days", review.due);
    ok(review.reps === 15 && review.lapses === 1, "reps/lapses carried");
    ok(fresh.interval === 0 && fresh.ease === 2.5 && fresh.reps === 0, "new card fresh state");
    ok(Math.abs(fresh.due - Math.floor(Date.now() / 1000)) < 5, "new card due now", fresh.due);
    ok(learning.interval === 0, "learning card stays in learning step");
    ok(learning.due === 1700000100, "learning due epoch-seconds kept", learning.due);
    ok(plan.stats.multiCard === 3, "multi-template disclosure counts", plan.stats);
    ok(plan.stats.empty === 0 && plan.stats.cloze === 0, "no empties, no cloze");
  }

  // ── schema 18 (.anki21b tables, protobuf media) ───────────────
  {
    console.log("schema18.apkg");
    const { ct, col } = await load("schema18.apkg");
    ok(ct.variant === "anki21b", "variant is anki21b", ct.variant);
    ok(ct.mediaCount === 2, "media count from protobuf map", ct.mediaCount);
    ok(col.notetypes.length === 2, "two notetypes", col.notetypes.map((n) => n.name));
    const cloze = col.notetypes.find((n) => n.cloze);
    ok(!!cloze && cloze.name === "Cloze French", "cloze flag from config type", cloze?.name);
    ok(cloze?.fields.length === 2, "cloze fields from fields table", cloze?.fields);

    const plan = buildPlan(col, defaultMapping(col));
    const basic = plan.decks[0]?.cards[0];
    ok(basic?.front === "un oiseau marin; seabird", "html + sound cleaned", basic?.front);
    ok(basic?.back === "a seabird", "back cleaned", basic?.back);
    ok(basic?.interval === 30 && basic?.ease === 2.35, "review state carried", basic);
    const clozeCard = plan.decks[0]?.cards[1];
    ok(
      clozeCard?.front === "Le [lighthouse] guide les navires.",
      "cloze front: first deletion masked, hint shown, others revealed",
      clozeCard?.front,
    );
    ok(clozeCard?.back === "phare (lighthouse)", "cloze back = answer + hint", clozeCard?.back);
    ok(plan.stats.cloze === 1, "cloze counted", plan.stats);
    ok(plan.stats.multiCard === 1, "multi-template only for the 2-tmpl notetype", plan.stats);
  }

  // ── export without scheduling ─────────────────────────────────
  {
    console.log("nohistory.apkg");
    const { col } = await load("nohistory.apkg");
    ok(col.revlog.count === 0, "no revlog");
    const scheduled = col.notes.some((n) => n.cards.some((c) => c.ivl > 0 || c.reps > 0));
    ok(!scheduled, "no card shows scheduled state — History: Not included");
    const plan = buildPlan(col, defaultMapping(col));
    ok(plan.decks[0].cards.every((c) => c.interval === 0 && c.reps === 0), "all cards start new");
  }

  // ── tar+zstd colpkg ───────────────────────────────────────────
  {
    console.log("colpkg.tar.zst");
    const { ct, col } = await load("colpkg.tar.zst");
    ok(ct.kind === "tar+zstd", "tar container", ct.kind);
    ok(ct.variant === "anki21b", "collection found inside tar");
    ok(col.notes.length === 2, "notes read through tar path", col.notes.length);
  }

  // ── corrupt package errors kindly ─────────────────────────────
  {
    console.log("corrupt.apkg");
    try {
      await readAnkiContainer(new Blob([bytes("corrupt.apkg") as BlobPart]));
      ok(false, "should have thrown");
    } catch (e) {
      ok(e instanceof AnkiError && e.kind === "badfile", "AnkiError with a friendly message", (e as Error).message);
    }
  }

  // ── export ────────────────────────────────────────────────────
  {
    console.log("export");
    const one = buildAnkiExport([
      { name: "French::Core 2k", cards: [{ front: "la marée", back: "the tide\nsecond line" }] },
    ]);
    ok(one.filename.endsWith(".txt"), "single deck → .txt", one.filename);
    const text = new TextDecoder().decode(one.data);
    ok(text.startsWith("#separator:tab\n#html:false\n#deck:French::Core 2k\n"), "anki import headers", text.slice(0, 60));
    ok(text.includes("la marée\tthe tide second line"), "newline flattened to space", text);

    const many = buildAnkiExport([
      { name: "Deck A", cards: [{ front: "a", back: "b" }] },
      { name: "Deck B", cards: [{ front: "c", back: "d" }] },
    ]);
    ok(many.filename.endsWith(".zip"), "multiple decks → zip", many.filename);
    const unzipped = unzipSync(many.data);
    ok(Object.keys(unzipped).length === 2, "one file per deck", Object.keys(unzipped));
    ok(
      new TextDecoder().decode(unzipped["Deck A.txt"]).includes("#deck:Deck A"),
      "per-file #deck header",
    );

    let threw = false;
    try {
      buildAnkiExport([{ name: "empty", cards: [] }]);
    } catch {
      threw = true;
    }
    ok(threw, "empty export throws");
  }

  // ── commit + badge + dedup ────────────────────────────────────
  {
    console.log("commit");
    const { col } = await load("schema11.apkg");
    const plan = buildPlan(col, defaultMapping(col));
    const r1 = commitAnkiImport(plan.decks);
    ok(r1.added === 3, "three cards committed", r1);
    ok(r1.decksCreated === 1, "one deck created", r1);
    const s = readVocab();
    ok(s.decks.length === 1 && s.decks[0].src === "anki", "deck badged anki", s.decks[0]);
    ok(s.decks[0].name === "French::Core 2k", "deck name stored");
    ok(Object.keys(s.cards).length === 3, "cards stored");
    const stored = Object.values(s.cards)[0];
    ok(typeof stored.due === "number" && typeof stored.ease === "number", "schedule persisted");

    // re-import the same package: every front is a duplicate
    const r2 = commitAnkiImport(plan.decks);
    ok(r2.added === 0 && r2.decksCreated === 0, "full dedup on re-import", r2);
    ok(readVocab().decks.length === 1, "no phantom deck for dup-only plan");
  }

  console.log(`\n${checks - failures}/${checks} checks passed`);
  if (failures > 0) process.exit(1);
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
