/**
 * Deck manager — one deck, inspected and edited (pushed screen,
 * /vocab/deck/?id=…).
 *
 * What a deck owns is visible: its cards (front/back) and what's
 * waiting. Removal is honest: a card goes with an undo toast; a
 * whole deck asks first, in words — "N cards and their schedules
 * go with it" — because a schedule is months of reviewing and
 * there is no undo for it. The card list renders in chunks so a
 * 2,000-card deck stays a phone, not a spreadsheet.
 */

import { createSignal, For, onMount, Show } from "solid-js";
import {
  readVocab,
  removeCard,
  removeDeck,
  restoreCard,
  type VocabCard,
} from "./vocabStore";

const Back = () => (
  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
    <path d="M15 6l-6 6 6 6" />
  </svg>
);

const X = () => (
  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.4" stroke-linecap="round" aria-hidden="true">
    <path d="M6 6l12 12M18 6L6 18" />
  </svg>
);

const Trash = () => (
  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
    <path d="M4 6.5h16M9.5 6.5V4.8a1 1 0 0 1 1-1h3a1 1 0 0 1 1 1v1.7M6.5 6.5l1 13a1 1 0 0 0 1 .9h7a1 1 0 0 0 1-.9l1-13" />
  </svg>
);

const CHUNK = 150;

let toastTimer: ReturnType<typeof setTimeout> | undefined;

export default function DeckManager() {
  const [deckId, setDeckId] = createSignal("");
  const [deckName, setDeckName] = createSignal("");
  const [isAnki, setIsAnki] = createSignal(false);
  const [cards, setCards] = createSignal<VocabCard[]>([]);
  const [shown, setShown] = createSignal(CHUNK);
  const [confirming, setConfirming] = createSignal(false);
  const [removed, setRemoved] = createSignal<VocabCard | null>(null);
  const [toast, setToast] = createSignal("");
  const [now, setNow] = createSignal(Math.floor(Date.now() / 1000));

  const refresh = () => {
    const id = new URLSearchParams(location.search).get("id") ?? "";
    const s = readVocab();
    const deck = s.decks.find((d) => d.id === id);
    setDeckId(id);
    setDeckName(deck?.name ?? "");
    setIsAnki(deck?.src === "anki");
    setCards(
      Object.values(s.cards)
        .filter((c) => c.deckId === id)
        .sort((a, b) => a.front.localeCompare(b.front, "fr")),
    );
  };

  onMount(refresh);

  const due = () => cards().filter((c) => c.due <= now()).length;

  const dropCard = (c: VocabCard) => {
    removeCard(c.id);
    setRemoved(c);
    setToast(`Removed “${c.front}”`);
    setCards(cards().filter((x) => x.id !== c.id));
    clearTimeout(toastTimer);
    toastTimer = setTimeout(() => {
      setToast("");
      setRemoved(null);
    }, 6000);
  };

  const undoRemove = () => {
    const c = removed();
    if (!c) return;
    restoreCard(c);
    setRemoved(null);
    setToast("");
    setCards([...cards(), c].sort((a, b) => a.front.localeCompare(b.front, "fr")));
  };

  const destroyDeck = () => {
    removeDeck(deckId());
    location.href = "/practice/";
  };

  return (
    <div class="ai-root">
      <header class="ai-bar">
        <a class="ai-back" href="/practice/" aria-label="Back">
          <Back />
        </a>
        <div class="ai-title">{deckName() || "Deck"}</div>
        {/* deletion lives in the bar — reachable above any list
            length, which is exactly where you want it for a
            2,000-card deck */}
        <button
          type="button"
          class="dm-trash"
          aria-label="Delete this deck"
          onClick={() => setConfirming(true)}
        >
          <Trash />
        </button>
      </header>

      <div class="ai-scroll">
        <Show
          when={deckName()}
          fallback={
            <p class="dm-gone">This deck doesn't exist — it may have been deleted.</p>
          }
        >
          {/* deck facts + the scoped study link */}
          <div class="dm-head">
            <div class="dm-facts">
              <b>
                {cards().length} {cards().length === 1 ? "card" : "cards"}
              </b>
              <span>
                {due() > 0
                  ? `${due()} waiting for review`
                  : cards().length > 0
                    ? "nothing waiting right now"
                    : ""}
              </span>
            </div>
            <Show when={isAnki()}>
              <span class="anki-badge">Anki</span>
            </Show>
          </div>

          <Show when={cards().length > 0}>
            <a class="dm-study" href={`/vocab/drill/?deck=${deckId()}`}>
              <span>
                <b>Study this deck</b>
                <span>{due() > 0 ? `${due()} cards waiting` : "nothing waiting — come back later"}</span>
              </span>
            </a>
          </Show>

          <Show
            when={cards().length > 0}
            fallback={
              <p class="dm-empty">
                No cards here yet. Add words from Practice — <b>Paste something</b> lets you
                pick this deck, or import an Anki deck with this name.
              </p>
            }
          >
            <div class="dm-cards">
              <For each={cards().slice(0, shown())}>
                {(c) => (
                  <div class="dm-card">
                    <span class="dm-faces">
                      <b>{c.front}</b>
                      <span>{c.back}</span>
                    </span>
                    <button
                      type="button"
                      class="dm-x"
                      aria-label={`Remove ${c.front}`}
                      onClick={() => dropCard(c)}
                    >
                      <X />
                    </button>
                  </div>
                )}
              </For>
            </div>
            <Show when={shown() < cards().length}>
              <button type="button" class="dm-more" onClick={() => setShown(shown() + CHUNK)}>
                Show more ({cards().length - shown()} left)
              </button>
            </Show>
          </Show>
        </Show>
      </div>

      {/* the one destructive act, asked in words first — a sheet,
          because the ask must not live at the end of a long list */}
      <Show when={confirming()}>
        <div class="dm-sheet" onClick={() => setConfirming(false)}>
          <div class="dm-sheetcard" onClick={(e) => e.stopPropagation()}>
            <b>Delete {deckName()}?</b>
            <p>
              {cards().length} {cards().length === 1 ? "card" : "cards"} and their review
              schedules go with it — this can't be undone.
            </p>
            <div class="dm-delrow">
              <button type="button" class="dm-keep" onClick={() => setConfirming(false)}>
                Keep it
              </button>
              <button type="button" class="dm-del2" onClick={destroyDeck}>
                Delete forever
              </button>
            </div>
          </div>
        </div>
      </Show>

      <Show when={toast()}>
        <div class="v-toast dm-toast" role="status">
          {toast()}
          <Show when={removed()}>
            <button type="button" class="dm-undo" onClick={undoRemove}>
              Undo
            </button>
          </Show>
        </div>
      </Show>
    </div>
  );
}
