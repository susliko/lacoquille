/**
 * Practice screen — the second tab (regroup 2026-11).
 *
 * Everything you DO lives here; Progress holds where you stand.
 * Two sections:
 *
 *   Grammar — the one button (the hero absorbs endless feed +
 *   exercise session) plus the subordinate drill rows.
 *   Vocabulary — review (the Anki-style flip drill), paste-in
 *   mining, and the decks, absorbed from the old Vocab tab.
 *
 * Grammar and vocabulary practice are twin navy buttons of the
 * same style — "Learn grammar" (the endless feed) and "Learn
 * vocabulary" (the Anki-style review over your decks). The deck
 * waiting line under the vocab button is the only due count on
 * the page — the review loop's own screen owns its state.
 */

import { createSignal, For, onCleanup, onMount, Show } from "solid-js";
import { feedTopicStats } from "./exercise/feedStore";
import { warmFeed } from "./exercise/feedApi";
import { topicSummaries } from "./exercise/store";
import { buildAnkiExport, downloadExport } from "../lib/anki/export";
import {
  addCards,
  addDeck,
  contextFor,
  deckCounts,
  dueQueue,
  mineWords,
  readVocab,
} from "./vocab/vocabStore";

const GRAMMAR_DRILLS = [
  {
    t: "Conjugations",
    s: "one verb at a time, untimed",
    href: "/practice/conjugations",
    tag: "",
  },
  {
    t: "Typing race",
    s: "the same drill, against a clock",
    href: "/practice/typing",
    tag: "",
  },
  {
    t: "Article of the day",
    s: "Maupassant, one story a day",
    href: "/stories/article-of-the-day",
    tag: "New",
  },
];

const Flame = () => (
  <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
    <path d="M12 2s1 3-1.5 5.5S7 12 7 14.5A5 5 0 0 0 17 15c0-3-2-4.5-2-7 0 0-1.5 1-2 2.5C12.5 8 12 5 12 2z" />
  </svg>
);

const Play = () => (
  <svg width="22" height="22" viewBox="0 0 24 24" fill="#fff" aria-hidden="true">
    <path d="M8 5.2v13.6a.6.6 0 0 0 .92.5l10.4-6.8a.6.6 0 0 0 0-1l-10.4-6.8A.6.6 0 0 0 8 5.2z" />
  </svg>
);

const Doc = () => (
  <svg width="19" height="19" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.1" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
    <path d="M6.5 3h7l4.5 4.5V21h-11.5z" />
    <path d="M13 3v5h5M9.5 13h5M9.5 17h5" />
  </svg>
);

const Deck = () => (
  <svg width="19" height="19" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.1" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
    <rect x="3.5" y="6.5" width="13" height="14" rx="2" />
    <path d="M7.5 3.5h11a2 2 0 0 1 2 2v11" />
  </svg>
);

const Down = () => (
  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.1" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
    <path d="M12 3.5v12M7 11l5 5 5-5M4.5 20.5h15" />
  </svg>
);

type Mined = { front: string; cx: string; keep: boolean; meaning: string };

export default function PracticeTab() {
  /* ── grammar ── */
  // Fresh until both stores hold history (SSR-safe default: fresh).
  const [fresh, setFresh] = createSignal(true);
  onMount(() => {
    const feed = Object.values(feedTopicStats());
    const session = Object.values(topicSummaries());
    setFresh(
      feed.every((t) => !t.attempts) && session.every((s) => !s.attempts),
    );
    // The hero leads to the feed — request its first run now so the
    // tap lands on exercises, not on a loading screen (also wakes a
    // cold backend while the learner reads the hero).
    warmFeed();
    refreshVocab();
    // A parked island is reattached without rehydration (Base.astro),
    // so the vocab section would show pre-import/pre-drill counts.
    // The swap layer taps this shoulder whenever the tab comes back.
    const onReactivate = () => refreshVocab();
    document.addEventListener("lacoquille:tab-reactivate", onReactivate);
    onCleanup(() => document.removeEventListener("lacoquille:tab-reactivate", onReactivate));
  });

  /* ── vocabulary ── */
  // SSR-safe defaults (empty), populated on mount — otherwise the
  // client's first render diverges from the SSR HTML and hydration
  // throws a mismatch for anyone with existing decks.
  const [decks, setDecks] = createSignal<ReturnType<typeof deckCounts>>([]);
  const [ready, setReady] = createSignal(0);
  const [pasteOpen, setPasteOpen] = createSignal(false);
  const [text, setText] = createSignal("");
  const [mined, setMined] = createSignal<Mined[]>([]);
  const [toast, setToast] = createSignal("");

  const refreshVocab = () => {
    setDecks(deckCounts());
    setReady(dueQueue().length);
  };

  const findWords = () => {
    const found = mineWords(text()).map((w) => ({
      ...w,
      cx: contextFor(text(), w.front),
      keep: true,
      meaning: "",
    }));
    setMined(found.slice(0, 120)); // triage stays scannable
  };

  const toggleKeep = (front: string) =>
    setMined(mined().map((m) => (m.front === front ? { ...m, keep: !m.keep } : m)));

  const setMeaning = (front: string, meaning: string) =>
    setMined(mined().map((m) => (m.front === front ? { ...m, meaning } : m)));

  const kept = () => mined().filter((m) => m.keep);

  const commitMined = () => {
    const today = new Date();
    const name = `Mined · ${today.getDate()} ${today.toLocaleString("en", { month: "short" })}`;
    const deck = addDeck(name);
    const entries = kept().map((m) => ({
      front: m.front,
      back: m.meaning,
      cx: m.cx,
    }));
    const added = addCards(deck.id, entries);
    setMined([]);
    setText("");
    setPasteOpen(false);
    setToast(`${added} words added to ${name}`);
    refreshVocab();
    setTimeout(() => setToast(""), 3500);
  };

  /** One-way export for Anki: .txt per deck (zipped when several),
      imported through Anki's own File → Import. Words travel;
      scheduling stays — the button's subtitle says so. */
  const exportAnki = () => {
    const s = readVocab();
    const cards = Object.values(s.cards);
    try {
      const f = buildAnkiExport(
        s.decks.map((d) => ({
          name: d.name,
          cards: cards.filter((c) => c.deckId === d.id).map((c) => ({ front: c.front, back: c.back })),
        })),
      );
      downloadExport(f);
      setToast("Deck files exported — import them from Anki");
    } catch {
      setToast("Nothing to export yet");
    }
    setTimeout(() => setToast(""), 3500);
  };

  return (
    <div class="ph-root">
      {/* ── Grammar ── */}
      <section class="ph-sec">
        <div class="ph-sechead">
          <h2>Grammar</h2>
        </div>

        {/* learn grammar — the endless feed */}
        <a class="ph-go" href="/practice/feed">
          <span>
            <span class="ph-big">
              Learn grammar
              <Show when={!fresh()}>
                <i class="ph-flame" aria-hidden="true">
                  <Flame />
                </i>
              </Show>
            </span>
            <span class="ph-meta">
              {fresh()
                ? "we'll find your level as you go"
                : "mixed review · about 5 min"}
            </span>
          </span>
          <span class="ph-disc" aria-hidden="true">
            <Play />
          </span>
        </a>

        <div class="pt-drills">
          {GRAMMAR_DRILLS.map((d) => (
            <a class="ph-drill" href={d.href}>
              <span class="ph-tx">
                <span class="ph-tt">{d.t}</span>
                <span class="ph-ss">{d.s}</span>
              </span>
              <Show when={d.tag}>
                <span class="ph-tag">{d.tag}</span>
              </Show>
            </a>
          ))}
        </div>
      </section>

      {/* ── Vocabulary ── */}
      <section class="ph-sec">
        <div class="ph-sechead">
          <h2>Vocabulary</h2>
        </div>

        {/* learn vocabulary — the review drill over your decks,
            same navy hero style as the grammar button */}
        <a class="ph-go" href="/vocab/drill/">
          <span>
            <span class="ph-big">Learn vocabulary</span>
            <span class="ph-meta">
              {ready() > 0
                ? `${ready()} ${ready() === 1 ? "card" : "cards"} waiting from your decks`
                : decks().some((d) => d.n > 0)
                  ? "nothing waiting from your decks — come back later"
                  : "nothing yet — add words below"}
            </span>
          </span>
          <span class="ph-disc" aria-hidden="true">
            <Play />
          </span>
        </a>

        {/* add words (paste-in mining; Anki import is a pushed
            screen — the parse can take a beat on big decks) */}
        <div class="v-add">
          <button type="button" class="ph-drill" onClick={() => setPasteOpen(!pasteOpen())}>
            <span class="ph-tx">
              <span class="ph-tt">Paste something</span>
              <span class="ph-ss">a text you read — words join a deck</span>
            </span>
            <span class="ph-ic" aria-hidden="true">
              <Doc />
            </span>
          </button>
          <a class="ph-drill" href="/vocab/import/">
            <span class="ph-tx">
              <span class="ph-tt">Import an Anki deck</span>
              <span class="ph-ss">.apkg or .colpkg, with your review history</span>
            </span>
            <span class="ph-ic" aria-hidden="true">
              <Deck />
            </span>
          </a>
        </div>

        <Show when={pasteOpen()}>
          <div class="v-paste">
            <textarea
              class="v-ta"
              placeholder="Paste lyrics, an article, a paragraph…"
              value={text()}
              onInput={(e) => setText(e.currentTarget.value)}
              rows={5}
            />
            <button
              type="button"
              class="v-btn"
              disabled={text().trim().length < 10}
              onClick={findWords}
            >
              Find words
            </button>
          </div>
        </Show>

        <Show when={mined().length > 0}>
          <div class="v-triage">
            <div class="v-triage-head">
              <span>{mined().length} words found — tap to drop the ones you know</span>
            </div>
            <div class="v-words">
              <For each={mined()}>
                {(m) => (
                  <Show
                    when={m.keep}
                    fallback={
                      <button
                        type="button"
                        class="v-word v-word-off"
                        onClick={() => toggleKeep(m.front)}
                      >
                        {m.front}
                      </button>
                    }
                  >
                    <div class="v-wordrow">
                      <button
                        type="button"
                        class="v-word"
                        onClick={() => toggleKeep(m.front)}
                        title="Drop this word"
                      >
                        {m.front}
                      </button>
                      <input
                        class="v-meaning"
                        placeholder="meaning (optional)"
                        value={m.meaning}
                        onInput={(e) => setMeaning(m.front, e.currentTarget.value)}
                      />
                    </div>
                  </Show>
                )}
              </For>
            </div>
            <Show when={kept().length > 0}>
              <button type="button" class="v-btn v-btn-primary" onClick={commitMined}>
                Add {kept().length} words
              </button>
            </Show>
          </div>
        </Show>

        {/* your decks — mined and imported in one list, Anki ones
            badged; then the one-way export for Anki */}
        <Show when={decks().length > 0}>
          <div class="pt-decks">
            <div class="pt-decks-head">Your decks</div>
            <div class="set-rows">
              <For each={decks()}>
                {(d) => (
                  <div class="set-row v-deckrow">
                    <span class="set-lb">
                      {d.deck.name}
                      <small>{d.n} {d.n === 1 ? "card" : "cards"}</small>
                    </span>
                    <Show when={d.deck.src === "anki"}>
                      <span class="anki-badge">Anki</span>
                    </Show>
                  </div>
                )}
              </For>
            </div>
            <button type="button" class="ph-drill" onClick={exportAnki}>
              <span class="ph-tx">
                <span class="ph-tt">Export for Anki</span>
                <span class="ph-ss">.txt files Anki imports — scheduling stays here</span>
              </span>
              <span class="ph-ic" aria-hidden="true">
                <Down />
              </span>
            </button>
          </div>
        </Show>
      </section>

      <Show when={toast()}>
        <div class="v-toast" role="status">
          {toast()}
        </div>
      </Show>
    </div>
  );
}
