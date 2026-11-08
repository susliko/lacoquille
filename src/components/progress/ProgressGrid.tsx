/**
 * Progress grid — the designed 6×4 matrix from the handoff
 * (docs/01-concept.md, 02-topics.md, 06-ux-findings.md): CEFR rows ×
 * four family columns (Tense · Mood · Noun group · Sentence), each
 * cell holding one mini-square per topic on the four-step indigo
 * scale, vermillon ring on minis = time to review. Tapping a cell
 * opens the topic sheet below it. No aggregate percentage —
 * user testing removed it.
 */

import { createEffect, createSignal, For, Show } from "solid-js";
import {
  fetchTopics,
  SECTION_LABELS,
  type Section,
  type TopicsCatalog,
} from "./topicsApi";
import {
  cellGrid,
  LEVELS,
  SECTIONS,
  TIER_LABELS,
  topicStates,
  type CellState,
  type TopicState,
} from "./tiers";

export default function ProgressGrid(props: { visible: boolean }) {
  const [catalog, setCatalog] = createSignal<TopicsCatalog | null>(null);
  const [failed, setFailed] = createSignal(false);
  const [open, setOpen] = createSignal<{ level: string; section: Section } | null>(null);
  let inFlight = false;

  const ensure = async () => {
    if (catalog() || inFlight) return;
    inFlight = true;
    try {
      setCatalog(await fetchTopics());
      setFailed(false);
    } catch {
      setFailed(true);
    } finally {
      inFlight = false;
    }
  };

  // Lazy: only hits the API when the tab is first shown.
  createEffect(() => {
    if (props.visible) void ensure();
  });

  const rows = (): CellState[][] | null => {
    const c = catalog();
    return c ? cellGrid(topicStates(c)) : null;
  };

  const totalAttempts = (): number => {
    const c = catalog();
    if (!c) return 0;
    return Object.values(topicStates(c)).reduce((n, t) => n + t.attempts, 0);
  };

  const openCell = (): CellState | null => {
    const o = open();
    const r = rows();
    if (!o || !r) return null;
    const li = (LEVELS as readonly string[]).indexOf(o.level);
    const si = SECTIONS.indexOf(o.section);
    return li >= 0 && si >= 0 ? r[li][si] : null;
  };

  return (
    <div class="pg-root">
      <Show
        when={!failed()}
        fallback={
          <div class="pg-status">
            <p>Couldn't load your progress.</p>
            <button class="pg-retry" onClick={() => void ensure()}>Try again</button>
          </div>
        }
      >
        <Show
          when={catalog()}
          fallback={<p class="pg-status">Preparing your grid…</p>}
        >
          <Show when={totalAttempts() === 0}>
            <div class="pg-hint">
              <p class="pg-hint-line">
                Your grid fills as you drill — each topic you practise lights up here.
              </p>
              <a class="pg-start" href="/practice/feed">Start the endless feed →</a>
            </div>
          </Show>

          <div class="pg-grid" role="table" aria-label="Progress by level and grammar family">
            {/* header row */}
            <span class="pg-head pg-corner" aria-hidden="true" />
            <For each={SECTIONS}>
              {(section) => (
                <span class="pg-head" role="columnheader">{SECTION_LABELS[section]}</span>
              )}
            </For>

            {/* one row per level */}
            <For each={rows()!}>
              {(row) => (
                <>
                  <span class="pg-level" role="rowheader">{row[0].level}</span>
                  <For each={row}>
                    {(cell) => {
                      const isOpen = () =>
                        open()?.level === cell.level && open()?.section === cell.section;
                      return (
                        <button
                          class="pg-cell"
                          disabled={cell.empty}
                          aria-expanded={isOpen() ? "true" : "false"}
                          aria-controls="pg-panel"
                          aria-label={`${cell.level} ${SECTION_LABELS[cell.section]}: ${cell.solid} of ${cell.total} solid${cell.review ? ", time to review" : ""}`}
                          onClick={() =>
                            setOpen(isOpen() ? null : { level: cell.level, section: cell.section })
                          }
                        >
                          <Show
                            when={!cell.empty}
                            fallback={<span class="pg-empty">—</span>}
                          >
                            <For each={cell.topics}>
                              {(t) => (
                                <span
                                  class="pg-mini"
                                  data-tier={t.tier}
                                  classList={{ "is-review": t.review }}
                                  aria-hidden="true"
                                />
                              )}
                            </For>
                          </Show>
                        </button>
                      );
                    }}
                  </For>
                </>
              )}
            </For>
          </div>

          <div class="pg-legend">
            <For each={[0, 1, 2, 3]}>
              {(tier) => (
                <span class="pg-legend-item">
                  <span class="pg-swatch" data-tier={tier} aria-hidden="true" />
                  {TIER_LABELS[tier]}
                </span>
              )}
            </For>
            <span class="pg-legend-item">
              <span class="pg-swatch-dot" aria-hidden="true" />
              Time to review
            </span>
          </div>

          <Show when={openCell()}>
            {(cell) => (
              <div class="pg-panel" id="pg-panel" role="region"
                aria-label={`${cell().level} · ${SECTION_LABELS[cell().section]}`}>
                <div class="pg-panel-head">
                  <span class="pg-panel-title">
                    {cell().level} · {SECTION_LABELS[cell().section]}
                  </span>
                  <button class="pg-close" aria-label="Close" onClick={() => setOpen(null)}>×</button>
                </div>
                <For each={cell().topics}>
                  {(t) => <TopicRow t={t} />}
                </For>
                <a class="pg-drill" href="/practice/feed">Drill a topic in the feed →</a>
              </div>
            )}
          </Show>
        </Show>
      </Show>
    </div>
  );
}

function TopicRow(props: { t: TopicState }) {
  const pct = () =>
    props.t.attempts > 0 ? Math.round(props.t.accuracy * 100) : null;
  return (
    <div class="pg-topic">
      <div class="pg-topic-main">
        <span class="pg-topic-name">{props.t.topic.title}</span>
        <Show when={props.t.review}>
          <span class="pg-review-tag">Time to review</span>
        </Show>
      </div>
      <div class="pg-topic-sub">
        <span class={`pg-tier-pill pg-tier-${props.t.tier}`}>
          {TIER_LABELS[props.t.tier]}
        </span>
        <span class="pg-topic-stat">
          {pct() === null ? "not attempted" : `${props.t.correct}/${props.t.attempts} · ${pct()}%`}
        </span>
      </div>
      <p class="pg-topic-blurb">{props.t.topic.blurb}</p>
    </div>
  );
}
