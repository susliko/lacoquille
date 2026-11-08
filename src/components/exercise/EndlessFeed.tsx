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

import { createSignal, For, Match, onCleanup, onMount, Show, Switch } from "solid-js";
import type { ExerciseItem } from "./data";
import { fetchFeed, type DrillPlan, type FeedRun } from "./feedApi";
import { feedContext, recordFeed } from "./feedStore";
import { TopicChip } from "./ui";
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

export default function EndlessFeed() {
  const [phase, setPhase] = createSignal<"boot" | "active" | "buffering" | "error">("boot");
  const [plan, setPlan] = createSignal<DrillPlan | null>(null);
  const [queue, setQueue] = createSignal<Slot[]>([]);
  const [pos, setPos] = createSignal(0);
  const [sitting, setSitting] = createSignal<Sitting>(EMPTY_SITTING);
  const [exited, setExited] = createSignal(false);

  let nextRun: FeedRun | null = null;
  let prefetching = false;
  let aborted = false;
  let controller: AbortController | null = null;

  const topicSlugOf = (item: ExerciseItem): string =>
    (item as { topicSlug?: string }).topicSlug ??
    item.topic.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/(^-|-$)/g, "");

  const adopt = (run: FeedRun) => {
    setPlan(run.plan);
    setQueue(run.items.map((item) => ({ item, retry: false })));
    setPos(0);
    setPhase("active");
  };

  const load = async (adoptNow: boolean) => {
    prefetching = true;
    controller = new AbortController();
    const run = await fetchFeed(feedContext(), controller.signal);
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

    if (!correct && !slot.retry) {
      setQueue((q) => [...q, { item: slot.item, retry: true }]);
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

  return (
    <div class="ex-root">
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
            fallback={<FeedSummary sitting={sitting()} onResume={() => setExited(false)} />}
          >
            <header class="ex-top ex-feed-top">
              <Show when={plan()} fallback={<span class="ex-streak" />}>
                {(p) => (
                  <div class="ex-feed-focus">
                    <TopicChip level={p().level} topic={p().topicTitle} />
                    <span class="ex-streak">{sitting().answered} answered</span>
                  </div>
                )}
              </Show>
              <button type="button" class="ex-close" aria-label="Finish the feed" onClick={() => setExited(true)}>
                ×
              </button>
            </header>

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
          </Show>
        </Show>
      </div>
    </div>
  );
}

/* ── Sitting summary (exit screen) ─────────────────────────────── */

function FeedSummary(props: { sitting: Sitting; onResume: () => void }) {
  const rate = () =>
    props.sitting.answered === 0
      ? 0
      : Math.round((props.sitting.correct / props.sitting.answered) * 100);

  const rows = () => Object.entries(props.sitting.byTopic);

  return (
    <div class="ex-done">
      <h1 class="ex-done-title">Nice sitting</h1>
      <p class="ex-done-sub">
        {props.sitting.answered} items, {rate()}% correct.
      </p>

      <Show when={rows().length > 0}>
        <div class="ex-miss-rows">
          <For each={rows()}>
            {([slug, t]) => (
              <div class="ex-miss-row" data-topic={slug}>
                <span class="ex-miss-topic">{t.title}</span>
                <span class="ex-miss-note">
                  {t.correct} correct · {t.missed} missed
                </span>
              </div>
            )}
          </For>
        </div>
      </Show>

      <div class="ex-next-card">
        <div class="ex-next-label">Next visit</div>
        <div class="ex-next-value">The coach will pick up from here</div>
      </div>

      <div class="ex-done-actions">
        <a class="ex-btn" href="/practice">Back to the map</a>
        <button type="button" class="ex-done-again" onClick={() => props.onResume()}>
          keep going
        </button>
      </div>
    </div>
  );
}
