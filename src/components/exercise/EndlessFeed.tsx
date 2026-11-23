/**
 * Endless feed — the continuous, coach-driven practice surface.
 *
 * Unlike the fixed session (a fetched batch with pips and a done
 * screen), the feed never runs out: the server's coach picks a topic
 * per run (level + history in, drill plan out) and a generator family
 * fills it. The next run is prefetched in the background while the
 * learner works, so topic switches feel instant. All learner state
 * stays in localStorage (feedStore); the wire payload is exactly that
 * context.
 *
 * In-run semantics match the session: a miss re-queues the item at
 * the end of the current run for an immediate retry.
 */

import { createMemo, createSignal, For, Match, onCleanup, onMount, Show, Switch } from "solid-js";
import type { ExerciseItem } from "./data";
import { fetchFeed, takeWarmRun, type DrillPlan, type FeedRun } from "./feedApi";
import { feedContext, recordFeed } from "./feedStore";
import { isTaught, markTaught, peekFor } from "./peek";
import { readSettings } from "../settings/settingsStore";
import { Pips, TopicChip, trackKeyboard, type PipState } from "./ui";
import {
  ChoiceView,
  ClozeView,
  DictationView,
  ErrorSpotView,
  TilesView,
  TransformView,
} from "./views";

interface Slot {
  item: ExerciseItem;
  retry: boolean;
}

interface Sitting {
  answered: number;
  correct: number;
  /** slug → { title, correct, missed } */
  byTopic: Record<string, { title: string; correct: number; missed: number }>;
}

const EMPTY_SITTING: Sitting = { answered: 0, correct: 0, byTopic: {} };

/** Run boundary: with how many items left to start prefetching. */
const PREFETCH_AT = 2;

/**
 * Time ends a session, not item count (docs/01-decisions.md): a typed
 * cloze takes four times as long as a tap, so items are a bad clock —
 * and time cannot accumulate into a backlog. ~5 min matches the hero's
 * "about 5 min" forecast.
 */
const SESSION_MS = 5 * 60 * 1000;

export default function EndlessFeed() {
  const [phase, setPhase] = createSignal<"boot" | "active" | "buffering" | "error">("boot");
  const [plan, setPlan] = createSignal<DrillPlan | null>(null);
  const [queue, setQueue] = createSignal<Slot[]>([]);
  const [pos, setPos] = createSignal(0);
  const [sitting, setSitting] = createSignal<Sitting>(EMPTY_SITTING);
  const [exited, setExited] = createSignal(false);
  /** Per-slot verdicts, parallel to the queue — the run's pips. */
  const [runPips, setRunPips] = createSignal<PipState[]>([]);
  /** Unseen topic → teaching card shows before the run's first item. */
  const [teachSlug, setTeachSlug] = createSignal<string | null>(null);

  let deadline = Date.now() + SESSION_MS;

  let nextRun: FeedRun | null = null;
  let prefetching = false;
  let aborted = false;
  let controller: AbortController | null = null;

  const topicSlugOf = (item: ExerciseItem): string =>
    (item as { topicSlug?: string }).topicSlug ??
    item.topic.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/(^-|-$)/g, "");

  /** Tap-based kinds when "Typing items" is off (settings). */
  const TAP_KINDS = new Set(["choice", "tiles"]);

  const adopt = (run: FeedRun) => {
    setPlan(run.plan);
    const typing = readSettings().typing;
    const items = typing ? run.items : run.items.filter((i) => TAP_KINDS.has(i.kind));
    const usable = items.length ? items : run.items; // never dead-end the feed
    setQueue(usable.map((item) => ({ item, retry: false })));
    setRunPips(usable.map(() => ""));
    setPos(0);
    const slug = topicSlugOf(usable[0]);
    setTeachSlug(!isTaught(slug) ? slug : null);
    setPhase("active");
  };

  const load = async (adoptNow: boolean) => {
    prefetching = true;
    controller = new AbortController();
    // The home screen may have warmed the first run already — consume
    // it instead of another round-trip (task: open drills right away).
    const warm = await takeWarmRun();
    const run = warm ?? (await fetchFeed(feedContext(), controller.signal));
    prefetching = false;
    if (aborted) return;
    if (!run) {
      if (adoptNow || phase() === "boot") setPhase("error");
      return;
    }
    if (adoptNow || phase() === "buffering" || phase() === "boot") {
      adopt(run);
    } else {
      nextRun = run;
    }
  };

  onMount(() => {
    void load(true);
  });

  onCleanup(() => {
    aborted = true;
    controller?.abort();
  });

  const current = () => queue()[pos()];

  const prefetchIfNeeded = () => {
    const remaining = queue().length - pos();
    if (remaining <= PREFETCH_AT && !nextRun && !prefetching && !exited()) {
      void load(false);
    }
  };

  const resolve = (correct: boolean) => {
    const slot = current();
    if (!slot) return;
    const slug = topicSlugOf(slot.item);
    recordFeed(slug, slot.item.id, correct);

    setSitting((s) => {
      const t = s.byTopic[slug] ?? { title: slot.item.topic, correct: 0, missed: 0 };
      return {
        answered: s.answered + 1,
        correct: s.correct + (correct ? 1 : 0),
        byTopic: {
          ...s.byTopic,
          [slug]: {
            title: t.title,
            correct: t.correct + (correct ? 1 : 0),
            missed: t.missed + (correct ? 0 : 1),
          },
        },
      };
    });

    setRunPips((p) => {
      const next = [...p];
      next[pos()] = correct ? "ok" : "miss";
      return next;
    });

    if (!correct && !slot.retry) {
      setQueue((q) => [...q, { item: slot.item, retry: true }]);
      setRunPips((p) => [...p, ""]);
    }
    if (Date.now() >= deadline) {
      // The clock ends the session — the end screen offers "keep going".
      setExited(true);
      return;
    }
    const next = pos() + 1;
    setPos(next);
    if (next >= queue().length) {
      // Run boundary: adopt the prefetched run or buffer for it.
      if (nextRun) {
        const run = nextRun;
        nextRun = null;
        adopt(run);
      } else {
        setPhase("buffering");
        if (!prefetching) void load(false);
      }
    } else {
      prefetchIfNeeded();
    }
  };

  let rootRef!: HTMLDivElement;
  onMount(() => onCleanup(trackKeyboard(rootRef)));

  return (
    <div class="ex-root" ref={rootRef}>
      <div class="ex-app">
        <Show
          when={phase() !== "boot" && phase() !== "error"}
          fallback={
            <div class="ex-loading">
              <Show when={phase() === "error"} fallback={<p>Preparing your feed…</p>}>
                <p>The feed needs the backend.</p>
                <button type="button" class="ex-done-again" onClick={() => { setPhase("boot"); void load(true); }}>
                  try again
                </button>
              </Show>
            </div>
          }
        >
          <Show
            when={!exited()}
            fallback={
              <FeedSummary
                sitting={sitting()}
                onResume={() => {
                  deadline = Date.now() + SESSION_MS;
                  setExited(false);
                }}
              />
            }
          >
            <header class="ex-top ex-feed-top">
              <Show when={plan()} fallback={<span class="ex-streak" />}>
                {(p) => (
                  <div class="ex-feed-focus">
                    <TopicChip level={p().level} topic={p().topicTitle} />
                    <Pips total={queue().length} stateFor={(i) => runPips()[i] ?? (i === pos() ? "current" : "")} />
                  </div>
                )}
              </Show>
              <button type="button" class="ex-close" aria-label="Finish the feed" onClick={() => setExited(true)}>
                ×
              </button>
            </header>

            <Show when={teachSlug()} keyed fallback={
              <Show
                when={phase() === "active" && current()}
                keyed
                fallback={<div class="ex-loading"><p>Preparing your next focus…</p></div>}
              >
                {(slot) => (
                  <Switch>
                    <Match when={slot.item.kind === "cloze" && slot.item}>
                      {(cloze) => <ClozeView item={cloze()} isRetry={slot.retry} onResolve={resolve} />}
                    </Match>
                    <Match when={slot.item.kind === "choice" && slot.item}>
                      {(choice) => <ChoiceView item={choice()} isRetry={slot.retry} onResolve={resolve} />}
                    </Match>
                    <Match when={slot.item.kind === "tiles" && slot.item}>
                      {(tiles) => <TilesView item={tiles()} isRetry={slot.retry} onResolve={resolve} />}
                    </Match>
                    <Match when={slot.item.kind === "transform" && slot.item}>
                      {(tr) => <TransformView item={tr()} isRetry={slot.retry} onResolve={resolve} />}
                    </Match>
                    <Match when={slot.item.kind === "error-spot" && slot.item}>
                      {(es) => <ErrorSpotView item={es()} isRetry={slot.retry} onResolve={resolve} />}
                    </Match>
                    <Match when={slot.item.kind === "dictation" && slot.item}>
                      {(d) => <DictationView item={d()} isRetry={slot.retry} onResolve={resolve} />}
                    </Match>
                  </Switch>
                )}
              </Show>
            }>
              {(slug) => (
                <TeachCard
                  slug={slug}
                  onDone={() => {
                    markTaught(slug);
                    setTeachSlug(null);
                  }}
                />
              )}
            </Show>
          </Show>
        </Show>
      </div>
    </div>
  );
}

/* ── Teaching card (handoff §5) ──────────────────────────────────
   A topic's first appearance gets two or three sentences and one
   example, built from the compact rule sheets. Cold-testing a
   brand-new topic guarantees a failure — demoralising, not useful. */

function TeachCard(props: { slug: string; onDone: () => void }) {
  const content = createMemo(() => peekFor(props.slug));
  return (
    <div class="ex-view">
      <Show when={content()} fallback={
        <div class="ex-loading">
          <p>New topic coming up.</p>
          <button type="button" class="ex-btn" onClick={props.onDone}>Got it</button>
        </div>
      }>
        {(b) => (
          <>
            <div class="ex-teach-eyebrow">Before you start</div>
            <h2 class="ex-teach-title">{b().title}</h2>
            <For each={b().blocks.slice(0, 2)}>
              {(blk) => (
                <div class="ex-teach-block">
                  <Show when={blk.h}><b class="ex-teach-h">{blk.h}</b></Show>
                  <p innerHTML={blk.p} />
                </div>
              )}
            </For>
            <Show when={b().examples[0]}>
              {(ex) => (
                <div class="ex-teach-ex">
                  <span class="ex-teach-fr" innerHTML={ex()[0]} />
                  <span class="ex-teach-en">{ex()[1]}</span>
                </div>
              )}
            </Show>
            <div class="ex-teach-foot">
              <a class="ex-teach-link" href={`/theory/t/${b().entryId}`}>
                Full page
              </a>
              <button type="button" class="ex-btn" onClick={props.onDone}>
                Got it
              </button>
            </div>
          </>
        )}
      </Show>
    </div>
  );
}

/* ── Sitting summary (exit screen) ─────────────────────────────── */

function FeedSummary(props: { sitting: Sitting; onResume: () => void }) {
  const rows = () => Object.entries(props.sitting.byTopic);

  return (
    <div class="ex-done">
      <h1 class="ex-done-title">Nice sitting</h1>

      <Show when={rows().length > 0}>
        <div class="ex-miss-rows">
          <For each={rows()}>
            {([slug, t]) => (
              <div class="ex-miss-row" data-topic={slug}>
                <span class="ex-miss-topic">{t.title}</span>
                <span class="ex-miss-note">
                  {t.missed > t.correct ? "coming back soon" : "going well"}
                </span>
              </div>
            )}
          </For>
        </div>
      </Show>

      <p class="ex-done-gridline">Your grammar grid has caught up.</p>

      <div class="ex-done-actions">
        <a class="ex-btn" href="/">Done for now</a>
        <button type="button" class="ex-done-again" onClick={() => props.onResume()}>
          keep going
        </button>
      </div>
    </div>
  );
}
