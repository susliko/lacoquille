/**
 * Exercise session orchestrator — the frame-02 shell (pips, close, topic
 * chip) hosting one of the six views, plus the frame-10 done screen.
 *
 * Items come from the backend generator (`GET /api/exercises`); if the
 * API is unreachable (frontend-only dev, offline) it falls back to the
 * curated local session in data.ts. All learner state — results, retry
 * queue, due counts — lives on the client (store.ts → localStorage).
 *
 * Queue semantics (from the design):
 *  - a miss on a first attempt re-queues the item for an in-session retry
 *    (frame 03: "Back in this session in a few minutes")
 *  - the done screen counts first-try correct / missed / recovered-on-retry
 *
 * Pip colours (progress scale + vermillon):
 *   current → p3 · correct → p2 · retried → p1 · missed → review
 */

import { createSignal, For, Match, onMount, Show, Switch } from "solid-js";
import {
  SESSION,
  countWord,
  type ExerciseItem,
} from "./data";
import { fetchSession } from "./api";
import { dueInfo, recordResult } from "./store";
import { Pips, TopicChip, type PipState } from "./ui";
import {
  ChoiceView,
  ClozeView,
  DictationView,
  ErrorSpotView,
  TilesView,
  TransformView,
} from "./views";

interface Slot {
  /** Index into the current item list. */
  i: number;
  retry: boolean;
}

interface Attempt {
  first: "correct" | "miss";
  retry?: "correct" | "miss";
}

export default function ExerciseSession() {
  const [items, setItems] = createSignal<ExerciseItem[]>(SESSION);
  const [queue, setQueue] = createSignal<Slot[]>([]);
  const [pos, setPos] = createSignal(0);
  const [results, setResults] = createSignal<(Attempt | undefined)[]>([]);
  /** null until the first batch (API or fallback) is in. */
  const [ready, setReady] = createSignal(false);

  const reset = (list: ExerciseItem[]) => {
    setItems(list);
    setQueue(list.map((_, i) => ({ i, retry: false })));
    setPos(0);
    setResults(Array<Attempt | undefined>(list.length).fill(undefined));
  };

  /** Load a batch: API first, curated fallback second. */
  const load = async (count?: number) => {
    setReady(false);
    const batch = await fetchSession({ count });
    reset(batch ? batch.items : SESSION);
    setReady(true);
  };

  onMount(() => {
    reset(SESSION);
    void load(7);
  });

  const done = () => pos() >= queue().length;
  const currentSlot = () => queue()[pos()];

  const resolve = (correct: boolean) => {
    const slot = currentSlot();
    if (!slot) return;
    const item = items()[slot.i];
    setResults((prev) => {
      const next = prev.slice();
      next[slot.i] = slot.retry
        ? { first: prev[slot.i]?.first ?? "miss", retry: correct ? "correct" : "miss" }
        : { first: correct ? "correct" : "miss" };
      return next;
    });
    if (!correct && !slot.retry) {
      setQueue((q) => [...q, { i: slot.i, retry: true }]);
    }
    // Learner state is client-side by design — record after the queue
    // bookkeeping so a retry correct later clears the due again.
    recordResult(item.id, item.topic, correct);
    setPos((p) => p + 1);
  };

  const pipState = (i: number): PipState => {
    if (!done() && currentSlot()?.i === i) return "current";
    const r = results()[i];
    if (!r) return "";
    if (r.first === "correct") return "ok";
    if (r.retry === "correct") return "retried";
    return "miss";
  };

  return (
    <div class="ex-root">
      <div class="ex-app">
        <Show
          when={ready()}
          fallback={
            <div class="ex-loading">
              <p>Preparing your session…</p>
            </div>
          }
        >
          <Show
            when={!done()}
            fallback={
              <DoneScreen
                items={items()}
                results={results()}
                onAgain={() => void load(7)}
              />
            }
          >
            <header class="ex-top">
              <Pips total={items().length} stateFor={pipState} />
              <a class="ex-close" href="/practice" aria-label="Close session">×</a>
            </header>

            <Show when={currentSlot()} keyed>
              {(slot) => {
                const item: ExerciseItem = items()[slot.i];
                return (
                  <>
                    <TopicChip level={item.level} topic={item.topic} />
                    <Switch>
                      <Match when={item.kind === "cloze" && item}>
                        {(cloze) => <ClozeView item={cloze()} isRetry={slot.retry} onResolve={resolve} />}
                      </Match>
                      <Match when={item.kind === "choice" && item}>
                        {(choice) => <ChoiceView item={choice()} isRetry={slot.retry} onResolve={resolve} />}
                      </Match>
                      <Match when={item.kind === "tiles" && item}>
                        {(tiles) => <TilesView item={tiles()} isRetry={slot.retry} onResolve={resolve} />}
                      </Match>
                      <Match when={item.kind === "transform" && item}>
                        {(tr) => <TransformView item={tr()} isRetry={slot.retry} onResolve={resolve} />}
                      </Match>
                      <Match when={item.kind === "error-spot" && item}>
                        {(es) => <ErrorSpotView item={es()} isRetry={slot.retry} onResolve={resolve} />}
                      </Match>
                      <Match when={item.kind === "dictation" && item}>
                        {(d) => <DictationView item={d()} isRetry={slot.retry} onResolve={resolve} />}
                      </Match>
                    </Switch>
                  </>
                );
              }}
            </Show>
          </Show>
        </Show>
      </div>
    </div>
  );
}

/* ── Done screen (frame 10) ───────────────────────────────────── */

function DoneScreen(props: {
  items: ExerciseItem[];
  results: (Attempt | undefined)[];
  onAgain: () => void;
}) {
  const stats = () => {
    let correct = 0, missed = 0, retried = 0;
    for (const r of props.results) {
      if (!r) continue;
      if (r.first === "correct") correct++;
      else if (r.retry === "correct") retried++;
      else missed++;
    }
    return { correct, missed, retried };
  };

  const missRows = () =>
    props.results
      .map((r, i) =>
        r && r.first === "miss"
          ? { topic: props.items[i]?.topic ?? "", recovered: r.retry === "correct" }
          : null,
      )
      .filter((x): x is { topic: string; recovered: boolean } => x !== null);

  const topicsCount = () => new Set(props.items.map((i) => i.topic)).size;

  /** Real client-side due count — misses not yet recovered. */
  const due = () => dueInfo();

  const dueLabel = () => {
    const n = due().count;
    return n === 0 ? "Nothing due — rest day" : `${n} item${n === 1 ? "" : "s"} due`;
  };

  return (
    <div class="ex-done">
      <h1 class="ex-done-title">Session complete</h1>
      <p class="ex-done-sub">
        {countWord(props.items.length)} items, mixed across {countWord(topicsCount()).toLowerCase()} topics.
      </p>

      <div class="ex-stats">
        <div class="ex-stat">
          <div class="ex-stat-n">{stats().correct}</div>
          <div class="ex-stat-l">CORRECT</div>
        </div>
        <div class="ex-stat">
          <div class="ex-stat-n">{stats().missed}</div>
          <div class="ex-stat-l">MISSED</div>
        </div>
        <div class="ex-stat">
          <div class="ex-stat-n">{stats().retried}</div>
          <div class="ex-stat-l">RETRIED</div>
        </div>
      </div>

      <Show when={missRows().length > 0}>
        <div class="ex-miss-rows">
          <For each={missRows()}>
            {(row) => (
              <div class="ex-miss-row">
                <span class="ex-miss-topic">{row.topic}</span>
                <span class="ex-miss-note">
                  {row.recovered ? "recovered on retry" : "review again"}
                </span>
              </div>
            )}
          </For>
        </div>
      </Show>

      <div class="ex-next-card">
        <div class="ex-next-label">Next session</div>
        <div class="ex-next-value">Tomorrow · {dueLabel()}</div>
      </div>

      <div class="ex-done-actions">
        <a class="ex-btn" href="/practice">Back to the map</a>
        <button type="button" class="ex-done-again" onClick={() => props.onAgain()}>
          practice again
        </button>
      </div>
    </div>
  );
}
