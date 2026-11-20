/**
 * Vocab home (handoff 2026-11 §6): Review hero → Add words →
 * Your decks. Mining and drilling share the tab because they are
 * one loop. Deck rows show counts — size is descriptive; nothing
 * about what is waiting (that display was cut everywhere).
 *
 * Paste-in mining v1: extract surface forms on-device, let the
 * learner drop ones they know and (optionally) type a meaning;
 * unedited cards carry their source sentence as the back. The
 * designed triage screen is an open question — this is the
 * placeholder loop until it exists.
 */

import { createMemo, createSignal, For, onMount, Show } from "solid-js";
import {
  addCards,
  addDeck,
  contextFor,
  deckCounts,
  dueQueue,
  mineWords,
} from "./vocabStore";

const Play = () => (
  <svg width="22" height="22" viewBox="0 0 24 24" fill="#fff" aria-hidden="true">
    <path d="M8 5.2v13.6a.6.6 0 0 0 .92.5l10.4-6.8a.6.6 0 0 0 0-1l-10.4-6.8A.6.6 0 0 0 8 5.2z" />
  </svg>
);

const Book = () => (
  <svg width="19" height="19" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.1" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
    <path d="M12 7C10.5 5.5 8 5 4 5.5v12.8c4-.5 6.5 0 8 1.7 1.5-1.7 4-2.2 8-1.7V5.5C16 5 13.5 5.5 12 7z" />
    <path d="M12 7v12.5" />
  </svg>
);

const Doc = () => (
  <svg width="19" height="19" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.1" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
    <path d="M6.5 3h7l4.5 4.5V21h-11.5z" />
    <path d="M13 3v5h5M9.5 13h5M9.5 17h5" />
  </svg>
);

type Mined = { front: string; cx: string; keep: boolean; meaning: string };

export default function VocabHome() {
  const [decks, setDecks] = createSignal(deckCounts());
  const [ready, setReady] = createSignal(0);
  const [pasteOpen, setPasteOpen] = createSignal(false);
  const [text, setText] = createSignal("");
  const [mined, setMined] = createSignal<Mined[]>([]);
  const [toast, setToast] = createSignal("");

  const refresh = () => {
    setDecks(deckCounts());
    setReady(dueQueue().length);
  };
  onMount(refresh);

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

  const kept = createMemo(() => mined().filter((m) => m.keep));

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
    refresh();
    setTimeout(() => setToast(""), 3500);
  };

  const hasDecks = () => decks().some((d) => d.n > 0);

  return (
    <div class="v-root">
      <header class="ph-bar">
        <div class="ph-mark">
          La <span>Coquille</span>
        </div>
      </header>

      {/* Review hero — same shape as the grammar one */}
      <section class="ph-sec">
        <Show
          when={hasDecks()}
          fallback={
            <div class="v-empty">
              <Book />
              <p>Your decks live here.</p>
              <p class="v-empty-sub">
                Paste a text below — a song, an article — and the words you keep join a
                deck to review.
              </p>
            </div>
          }
        >
          <a class="ph-go" href="/vocab/drill/">
            <span>
              <span class="ph-eyebrow">Vocabulary</span>
              <span class="ph-big">Review</span>
              <span class="ph-meta">
                {ready() > 0 ? "cards ready from your decks" : "nothing waiting — add words below"}
              </span>
            </span>
            <span class="ph-disc" aria-hidden="true">
              <Play />
            </span>
          </a>
        </Show>
      </section>

      {/* Add words */}
      <section class="ph-sec">
        <div class="ph-sechead">
          <h2>Add words</h2>
        </div>
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
      </section>

      {/* Your decks */}
      <section class="ph-sec">
        <div class="ph-sechead">
          <h2>Your decks</h2>
        </div>
        <Show
          when={decks().length > 0}
          fallback={<p class="v-nodecks">Decks appear here once you add words.</p>}
        >
          <div class="set-rows">
            <For each={decks()}>
              {(d) => (
                <div class="set-row v-deckrow">
                  <span class="set-lb">
                    {d.deck.name}
                    <small>{d.n} {d.n === 1 ? "card" : "cards"}</small>
                  </span>
                </div>
              )}
            </For>
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
