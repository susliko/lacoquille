/**
 * The vocab drill (handoff 2026-11 §6) — Anki's loop with one change:
 * no reveal step. The four ratings are live from the moment the card
 * appears; the card flips on tap with a 3D rotate. Back face is a
 * different surface so there is never doubt which side is showing.
 * Undo in the header — a stray Easy buries a card for weeks.
 *
 * Implementation note from the prototype: flip by toggling a class on
 * the existing node, never re-render — a re-render replaces the
 * element and the transition never runs. Moving to the next card does
 * re-render, which is what you want.
 */

import { createMemo, createSignal, For, onCleanup, onMount, Show } from "solid-js";
import {
  clearSessionCounter,
  commitRating,
  deckName,
  dueQueue,
  rateCard,
  readVocab,
  undoRating,
  type Rate,
  type VocabCard,
} from "./vocabStore";

const RATES: Array<{ k: Rate; t: string; cls: string; key: string }> = [
  { k: "again", t: "Again", cls: "r1", key: "1" },
  { k: "hard", t: "Hard", cls: "r2", key: "2" },
  { k: "good", t: "Good", cls: "r3", key: "3" },
  { k: "easy", t: "Easy", cls: "r4", key: "4" },
];

const Back = () => (
  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
    <path d="M15 6l-6 6 6 6" />
  </svg>
);

const Undo = () => (
  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
    <path d="M9 14L4 9l5-5" />
    <path d="M4 9h10a6 6 0 0 1 0 12h-3" />
  </svg>
);

export default function FlipDrill() {
  const [queue, setQueue] = createSignal<VocabCard[]>([]);
  const [i, setI] = createSignal(0);
  const [flipped, setFlipped] = createSignal(false);
  const [undoCard, setUndoCard] = createSignal<VocabCard | null>(null);
  const [done, setDone] = createSignal(false);

  onMount(() => {
    clearSessionCounter();
    // a deck screen can launch a scoped session (/vocab/drill/?deck=…)
    const scoped = new URLSearchParams(location.search).get("deck") ?? undefined;
    setQueue(dueQueue(scoped || undefined));
    const onKey = (e: KeyboardEvent) => {
      if (done()) return;
      if (e.key === " ") {
        e.preventDefault();
        setFlipped(!flipped());
      } else {
        const r = RATES.find((x) => x.key === e.key);
        if (r) void judge(r.k);
      }
    };
    window.addEventListener("keydown", onKey);
    onCleanup(() => window.removeEventListener("keydown", onKey));
  });

  const card = createMemo(() => queue()[i()]);
  const deckChip = () => (card() ? deckName(card().deckId) : "");

  const judge = (k: Rate) => {
    const c = card();
    if (!c) return;
    setUndoCard(c);
    commitRating(c, rateCard(c, k));
    setFlipped(false);
    if (i() + 1 >= queue().length) setDone(true);
    else setI(i() + 1);
  };

  const undo = () => {
    const before = undoCard();
    if (!before) return;
    undoRating(before);
    setUndoCard(null);
    setDone(false);
    setI(i() - 1);
    setFlipped(false);
  };

  // Re-render on card change only — the flip itself is a class toggle.
  const front = () => card()?.front ?? "";
  const back = () => card()?.back ?? "";
  const cx = () => card()?.cx ?? "";

  return (
    <div class="vd-root">
      <header class="vd-bar">
        <a class="vd-close" href="/practice/" aria-label="Done">
          <Back />
        </a>
        <div class="vd-title">Review</div>
        <button
          type="button"
          class="vd-undo"
          disabled={!undoCard() || i() === 0}
          onClick={undo}
          aria-label="Undo last rating"
        >
          <Undo />
        </button>
      </header>

      <Show
        when={!done() && card()}
        fallback={
          <div class="vd-done">
            <p class="vd-done-big">Done for now.</p>
            <p class="vd-done-sub">
              {readVocab().session.rated > 0
                ? `${readVocab().session.rated} rated. They'll come back when they come back.`
                : "Nothing is waiting — add words from the Vocab tab."}
            </p>
            <a class="vd-done-link" href="/practice/">
              Back
            </a>
          </div>
        }
      >
        <div class="deckchip">{deckChip()}</div>

        {/* The card. Class toggle flips; re-render only advances. */}
        <div class="vd-stage">
          <button
            type="button"
            class={`flip${flipped() ? " is-back" : ""}`}
            onClick={() => setFlipped(!flipped())}
            aria-label={flipped() ? "Show front" : "Show back"}
          >
            <span class="face front-face">
              <span class="vfront">{front()}</span>
            </span>
            <span class="face back-face">
              <Show when={back()} fallback={<span class="vback vback-none">no meaning saved</span>}>
                <span class="vback">{back()}</span>
              </Show>
              <Show when={cx()}>
                <span class="vcx">…with <b>{cx()}</b></span>
              </Show>
              <span class="vfront-sm">{front()}</span>
            </span>
          </button>
        </div>

        <div class="vd-rates">
          <For each={RATES}>
            {(r) => (
              <button type="button" class={`vrate ${r.cls}`} onClick={() => judge(r.k)}>
                {r.t}
              </button>
            )}
          </For>
        </div>
        <p class="vd-kbd">tap card to flip · 1–4 to rate</p>
      </Show>
    </div>
  );
}
