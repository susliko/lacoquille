/**
 * Anki import (handoff docs/04-vocab-and-anki.md, design/vocab-anki.html).
 *
 * A pushed screen with three states: pick a file, "What's in it",
 * done. The point of the middle screen is honesty — History is a
 * first-class, colour-coded fact (green when review history came
 * across, orange when the export was made without scheduling), and
 * every lossy conversion we do is said out loud before the commit:
 *
 *   · one card per Anki note, first card's schedule kept
 *   · cloze notes: the first deletion becomes the card
 *   · the two schedulers differ, so future dates won't match Anki's
 *
 * All parsing happens on-device (src/lib/anki) — nothing about the
 * file leaves the browser.
 */

import { createMemo, createSignal, For, Show } from "solid-js";
import { readAnkiContainer, AnkiError, type AnkiContainer } from "../../lib/anki/container";
import { openAnkiCollection, type AnkiCollection } from "../../lib/anki/collection";
import { buildPlan, defaultMapping, type FieldMapping, type ImportPlan } from "../../lib/anki/import";
import { commitAnkiImport } from "./vocabStore";

const Back = () => (
  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
    <path d="M15 6l-6 6 6 6" />
  </svg>
);

type Phase = "pick" | "parsing" | "ready" | "importing" | "done" | "error";

const fmt = (n: number) => n.toLocaleString("en");

export default function AnkiImport() {
  const [phase, setPhase] = createSignal<Phase>("pick");
  const [error, setError] = createSignal("");
  const [fileName, setFileName] = createSignal("");
  const [fileSize, setFileSize] = createSignal(0);
  const [container, setContainer] = createSignal<AnkiContainer | null>(null);
  const [col, setCol] = createSignal<AnkiCollection | null>(null);
  const [mapping, setMapping] = createSignal<FieldMapping>({});
  const [result, setResult] = createSignal<{ added: number; decks: number; skipped: number } | null>(null);

  let input: HTMLInputElement | undefined;

  const plan = createMemo<ImportPlan | null>(() => {
    const c = col();
    if (!c) return null;
    return buildPlan(c, mapping());
  });

  const totalCards = () => plan()?.decks.reduce((n, d) => n + d.cards.length, 0) ?? 0;

  const notetypesUsed = createMemo(() => {
    const c = col();
    if (!c) return [];
    const used = new Set(c.notes.map((n) => n.mid));
    return c.notetypes.filter((nt) => used.has(nt.id));
  });

  /** Reset and open the native picker. */
  const pick = () => {
    setError("");
    input?.click();
  };

  const load = async (file: File) => {
    setFileName(file.name);
    setFileSize(file.size);
    setPhase("parsing");
    try {
      const ct = await readAnkiContainer(file);
      const c = await openAnkiCollection(ct);
      setContainer(ct);
      setCol(c);
      setMapping(defaultMapping(c));
      setPhase("ready");
    } catch (e) {
      const msg =
        e instanceof AnkiError
          ? e.message
          : "This file couldn't be opened — if it came from Anki, try exporting it again as .apkg.";
      setError(msg);
      setPhase("error");
    }
  };

  const onFile = (e: Event) => {
    const f = (e.currentTarget as HTMLInputElement).files?.[0];
    if (f) void load(f);
    (e.currentTarget as HTMLInputElement).value = ""; // re-picking the same file must still fire
  };

  const setField = (mid: number, side: "front" | "back", ord: number) =>
    setMapping({ ...mapping(), [mid]: { ...mapping()[mid], [side]: ord } });

  const runImport = () => {
    const p = plan();
    if (!p || p.decks.length === 0) return;
    setPhase("importing");
    try {
      const total = p.decks.reduce((n, d) => n + d.cards.length, 0);
      const r = commitAnkiImport(p.decks);
      setResult({ added: r.added, decks: r.decksCreated, skipped: total - r.added });
      setPhase("done");
      setContainer(null);
      setCol(null);
    } catch {
      // private mode / quota — the store write failed session-only
      setError("The import couldn't be saved — browser storage refused the write.");
      setPhase("error");
    }
  };

  const history = createMemo(() => {
    const c = col();
    if (!c) return null;
    if (c.revlog.count > 0 && c.revlog.earliest) {
      const when = new Date(c.revlog.earliest).toLocaleDateString("en", { month: "long", year: "numeric" });
      return { ok: true, label: "Included", detail: `${fmt(c.revlog.count)} reviews, back to ${when}` };
    }
    // Anki strips the revlog when scheduling is left out of the export
    const scheduled = c.notes.some((n) => n.cards.some((cd) => cd.ivl > 0 || cd.reps > 0));
    return scheduled
      ? { ok: true, label: "Included", detail: "card intervals carried across (no review log in this export)" }
      : {
          ok: false,
          label: "Not included",
          detail: "This export was made without scheduling information",
        };
  });

  return (
    <div class="ai-root">
      <header class="ai-bar">
        <a class="ai-back" href="/practice/" aria-label="Back">
          <Back />
        </a>
        <div class="ai-title">Import a deck</div>
      </header>

      <div class="ai-scroll">
        <Show when={phase() === "pick" || phase() === "parsing"}>
          <button type="button" class="ai-drop" onClick={pick} disabled={phase() === "parsing"}>
            <Show
              when={phase() === "parsing"}
              fallback={
                <>
                  <b>Choose an .apkg or .colpkg</b>
                  <span>exported from Anki — parsed here, on your device</span>
                </>
              }
            >
              <b>Opening {fileName()}…</b>
              <span>large decks take a moment</span>
            </Show>
          </button>
          <Show when={phase() === "error"}>
            <p class="ai-error">{error()}</p>
          </Show>
          <p class="ai-note">
            Anki writes deck files with your review history when you export with “include
            scheduling information” switched on.
          </p>
        </Show>

        <Show when={(phase() === "ready" || phase() === "importing") && plan()}>
          <button type="button" class="ai-drop" onClick={pick}>
            <b>{fileName()}</b>
            <span>
              {(fileSize() / 1024 / 1024).toLocaleString("en", { maximumFractionDigits: 1 })} MB ·
              tap to choose a different file
            </span>
          </button>

          <section class="ai-sec">
            <div class="ai-sechead">
              <h2>What's in it</h2>
            </div>
            <div class="ai-facts">
              <div class="ai-fact">
                <span class="k">Decks</span>
                <span class="v">
                  {plan()!.decks.length === 1 ? plan()!.decks[0].name : `${plan()!.decks.length} decks`}
                  <em>{plan()!.decks.length > 1 ? plan()!.decks.map((d) => d.name).slice(0, 3).join(" · ") : ""}</em>
                </span>
              </div>
              <div class="ai-fact">
                <span class="k">Cards</span>
                <span class="v">{fmt(totalCards())}</span>
              </div>
              <div class="ai-fact">
                <span class="k">Note type</span>
                <span class="v">
                  {notetypesUsed().length === 1
                    ? `${notetypesUsed()[0].name} — ${notetypesUsed()[0].fields.length} field${notetypesUsed()[0].fields.length === 1 ? "" : "s"}`
                    : `${notetypesUsed().length} note types`}
                </span>
              </div>
              <div class="ai-fact">
                <span class="k">Media</span>
                <span class="v">{container()?.mediaCount === null ? "—" : `${fmt(container()?.mediaCount ?? 0) || "none"}`}</span>
              </div>
              <Show when={history()}>
                <div class="ai-fact">
                  <span class="k">History</span>
                  <span class={`v ${history()!.ok ? "ok" : "warn"}`}>
                    {history()!.label}
                    <em>{history()!.detail}</em>
                  </span>
                </div>
              </Show>
            </div>
            <p class="ai-note">
              <Show
                when={history()!.ok}
                fallback="Without scheduling information every card starts as new. The words come across; the history does not."
              >
                Intervals and ease carry across; future dates will not match Anki's exactly,
                because the two schedulers differ.
              </Show>
            </p>
            <Show when={plan()!.stats.multiCard > 0}>
              <p class="ai-note">
                {fmt(plan()!.stats.multiCard)} notes carry several card types — we keep the
                first card's schedule.
              </p>
            </Show>
            <Show when={plan()!.stats.cloze > 0}>
              <p class="ai-note">Cloze notes — the first deletion of each becomes the card.</p>
            </Show>
            <Show when={plan()!.stats.empty > 0}>
              <p class="ai-note">{fmt(plan()!.stats.empty)} notes had nothing to put on a card — skipped.</p>
            </Show>
          </section>

          <section class="ai-sec">
            <div class="ai-sechead">
              <h2>Which field is which</h2>
            </div>
            <div class="ai-facts">
              <For each={notetypesUsed()}>
                {(nt) => (
                  <Show
                    when={nt.cloze}
                    fallback={
                      <>
                        <div class="ai-map">
                          <span class="k">Front</span>
                          <select
                            class="ai-sel"
                            value={mapping()[nt.id]?.front ?? 0}
                            onChange={(e) => setField(nt.id, "front", Number(e.currentTarget.value))}
                          >
                            <For each={nt.fields}>
                              {(f, i) => <option value={i()}>{f || `field ${i() + 1}`}</option>}
                            </For>
                          </select>
                        </div>
                        <div class="ai-map">
                          <span class="k">Back</span>
                          <select
                            class="ai-sel"
                            value={mapping()[nt.id]?.back ?? (nt.fields.length > 1 ? 1 : 0)}
                            onChange={(e) => setField(nt.id, "back", Number(e.currentTarget.value))}
                          >
                            <For each={nt.fields}>
                              {(f, i) => <option value={i()}>{f || `field ${i() + 1}`}</option>}
                            </For>
                          </select>
                        </div>
                      </>
                    }
                  >
                    <div class="ai-map">
                      <span class="k">Cloze text</span>
                      <select
                        class="ai-sel"
                        value={mapping()[nt.id]?.front ?? 0}
                        onChange={(e) => setField(nt.id, "front", Number(e.currentTarget.value))}
                      >
                        <For each={nt.fields}>
                          {(f, i) => <option value={i()}>{f || `field ${i() + 1}`}</option>}
                        </For>
                      </select>
                    </div>
                  </Show>
                )}
              </For>
            </div>
            <p class="ai-note">
              Cards are drilled the way Anki drills them: see the front, reveal the back, rate
              yourself.
            </p>
          </section>

          <section class="ai-sec">
            <button type="button" class="ai-cta" disabled={phase() === "importing" || totalCards() === 0} onClick={runImport}>
              {phase() === "importing" ? "Importing…" : totalCards() === 0 ? "Nothing to import" : `Import ${fmt(totalCards())} cards`}
            </button>
          </section>
        </Show>

        <Show when={phase() === "done" && result()}>
          <section class="ai-sec ai-done">
            <div class="ai-done-big">{fmt(result()!.added)} cards added</div>
            <p class="ai-note">
              {result()!.decks === 1 ? "1 deck" : `${fmt(result()!.decks)} decks`} created
              <Show when={result()!.skipped > 0}>
                {" · "}
                {fmt(result()!.skipped)} already in your decks — skipped
              </Show>
              . Nothing left this device at any point.
            </p>
            <a class="ai-cta" href="/vocab/drill/">
              Start reviewing
            </a>
            <a class="ai-after" href="/practice/">
              Back to practice
            </a>
          </section>
        </Show>

        <Show when={phase() === "error"}>
          <section class="ai-sec">
            <p class="ai-error">{error()}</p>
            <button type="button" class="ai-cta" onClick={pick}>
              Choose another file
            </button>
          </section>
        </Show>

        <input
          ref={input}
          type="file"
          accept=".apkg,.colpkg,application/zip,application/octet-stream"
          style="display:none"
          onChange={onFile}
        />
      </div>
    </div>
  );
}
