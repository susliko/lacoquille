/**
 * Grammar map — a faithful build of the handoff prototype
 * (grammar-grid.html): six CEFR rows × four family columns, one
 * mini-square per taxonomy topic on the four-step indigo scale,
 * vermillon ring = time to review. Three openable scopes — a cell,
 * a whole level (row head), a whole family (column head) — each
 * opening the bottom sheet. Colours come from the design tokens;
 * every metric is the prototype's.
 */

import { createEffect, createMemo, createSignal, For, onCleanup, onMount, Show } from "solid-js";
import type { JSX } from "solid-js";
import { fetchTopics, type TopicsCatalog } from "./topicsApi";
import {
  GG_ABOUT,
  GG_CATS,
  GG_KEYS,
  GG_LEVELS,
  GG_SUBS,
  type FamKey,
} from "./taxonomy";
import {
  entriesIn,
  entryStates,
  levelStats,
  solidOf,
  TIER_LABELS,
  type EntryState,
} from "./tiers";

/** The column-head labels; the N column wraps onto two lines. */
const HEAD_LABEL: Record<FamKey, JSX.Element> = {
  T: <>Tense</>,
  M: <>Mood</>,
  N: (
    <>
      Noun
      <br />
      group
    </>
  ),
  S: <>Sentence</>,
};

const ChevRight = () => (
  <svg
    width="14"
    height="14"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2.6"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <path d="M9 6l6 6-6 6" />
  </svg>
);

const ChevDown = () => (
  <svg
    width="15"
    height="15"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2.6"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <path d="M6 9l6 6 6-6" />
  </svg>
);

const HandIcon = () => (
  <svg
    class="gg-hand"
    width="26"
    height="26"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="1.8"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <path d="M9 11V5.5a1.5 1.5 0 0 1 3 0V11" />
    <path d="M12 10.5V9a1.5 1.5 0 0 1 3 0v2" />
    <path d="M15 10.5a1.5 1.5 0 0 1 3 0V15a6 6 0 0 1-6 6h-.5a6 6 0 0 1-4.6-2.2L4.5 15.6a1.6 1.6 0 0 1 2.3-2.2L9 15" />
  </svg>
);

const CloseIcon = () => (
  <svg
    width="18"
    height="18"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2.2"
    stroke-linecap="round"
    aria-hidden="true"
  >
    <path d="M6 6l12 12M18 6L6 18" />
  </svg>
);

interface Sheet {
  lv?: string;
  fam?: FamKey;
}

interface Trigger {
  action: string;
  lv?: string;
  fam?: string;
}

const HINT_KEY = "grammarMapHintOff";

export default function ProgressGrid(props: { visible: boolean }) {
  const [catalog, setCatalog] = createSignal<TopicsCatalog | null>(null);
  const [failed, setFailed] = createSignal(false);
  const [sheet, setSheet] = createSignal<Sheet | null>(null);
  const [hintOff, setHintOff] = createSignal(false);
  let lastTrigger: Trigger | null = null;
  let closeBtn: HTMLButtonElement | undefined;
  let inFlight = false;

  onMount(() => {
    try {
      setHintOff(localStorage.getItem(HINT_KEY) === "1");
    } catch {
      /* private mode: banner just stays dismissable for the session */
    }
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape" && sheet()) closeSheet();
    };
    window.addEventListener("keydown", onKey);
    onCleanup(() => window.removeEventListener("keydown", onKey));
  });

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

  const states = createMemo(() => {
    const c = catalog();
    return c ? entryStates(c) : null;
  });

  // The sheet's close button takes focus when the sheet opens.
  createEffect(() => {
    if (sheet() && closeBtn) closeBtn.focus();
  });

  const dismissHint = () => {
    setHintOff(true);
    try {
      localStorage.setItem(HINT_KEY, "1");
    } catch {
      /* ignore */
    }
  };

  const openSheet = (s: Sheet, trigger: HTMLElement) => {
    lastTrigger = {
      action: trigger.dataset.ggAction ?? "",
      lv: trigger.dataset.lv,
      fam: trigger.dataset.fam,
    };
    setSheet(s);
  };

  const closeSheet = () => {
    setSheet(null);
    if (lastTrigger) {
      const sel =
        `[data-gg-action="${lastTrigger.action}"]` +
        (lastTrigger.lv ? `[data-lv="${lastTrigger.lv}"]` : "") +
        (lastTrigger.fam ? `[data-fam="${lastTrigger.fam}"]` : "");
      const el = document.querySelector<HTMLElement>(sel);
      if (el) el.focus();
    }
  };

  // ---- scope predicates: a cell lights up when it, its row or its column is open
  const isRow = (lv: string) => sheet()?.lv === lv && !sheet()?.fam;
  const isCol = (k: FamKey) => sheet()?.fam === k && !sheet()?.lv;
  const isCell = (lv: string, k: FamKey) => sheet()?.lv === lv && sheet()?.fam === k;

  const cellAria = (lv: string, k: FamKey, list: EntryState[]) => {
    const s = solidOf(list);
    const rev = list.filter((t) => t.review).length;
    return `Level ${lv}, ${GG_CATS[k]}: ${s} of ${list.length} solid${
      rev ? `, ${rev} to review` : ""
    }. Open`;
  };

  return (
    <div class="gg-root">
      <Show
        when={!failed()}
        fallback={
          <div class="gg-status">
            <p>Couldn't load your progress.</p>
            <button class="gg-retry" onClick={() => void ensure()}>
              Try again
            </button>
          </div>
        }
      >
        <Show when={states()} fallback={<p class="gg-status">Preparing your grid…</p>}>
          {(states) => (
            <>
              <h2 class="gg-title">Grammar map</h2>

              <Show when={!hintOff()}>
                <div class="gg-hint">
                  <HandIcon />
                  <p>
                    Tap a block to see its topics. Tap a <b>level</b> or a{" "}
                    <b>column name</b> to see all of it.
                  </p>
                  <button type="button" data-gg-action="hint-off" onClick={dismissHint}>
                    Got it
                  </button>
                </div>
              </Show>

              <div class="gg-matrix">
                {/* column heads */}
                <div class="gg-mrow">
                  <div class="gg-corner" />
                  <div class="gg-cols">
                    <For each={GG_KEYS}>
                      {(k) => (
                        <button
                          type="button"
                          class="gg-colhead"
                          classList={{ on: isCol(k) }}
                          data-gg-action="col"
                          data-fam={k}
                          aria-label={`All ${GG_CATS[k]} topics`}
                          onClick={(e) => openSheet({ fam: k }, e.currentTarget)}
                        >
                          <span>{HEAD_LABEL[k]}</span>
                          <ChevDown />
                        </button>
                      )}
                    </For>
                  </div>
                </div>

                {/* one row per level */}
                <For each={GG_LEVELS}>
                  {(lv) => (
                    <div class="gg-mrow">
                      <LevelHead lv={lv} states={states()} on={isRow(lv)} onOpen={openSheet} />
                      <div class="gg-cols">
                        <For each={GG_KEYS}>
                          {(k) => {
                            const list = () => entriesIn(states(), lv, k);
                            return (
                              <button
                                type="button"
                                class="gg-cell"
                                classList={{ on: isCell(lv, k) || isRow(lv) || isCol(k) }}
                                data-gg-action="cell"
                                data-lv={lv}
                                data-fam={k}
                                aria-label={cellAria(lv, k, list())}
                                onClick={(e) => openSheet({ lv, fam: k }, e.currentTarget)}
                              >
                                <span class="squares">
                                  <For each={list()}>
                                    {(t) => (
                                      <span
                                        class="gg-sq"
                                        data-tier={t.tier}
                                        classList={{ due: t.review }}
                                      />
                                    )}
                                  </For>
                                </span>
                                <span class="foot">
                                  <span class="count">
                                    {solidOf(list())}/{list().length}
                                  </span>
                                  <Show when={list().some((t) => t.review)}>
                                    <span class="gg-dot" />
                                  </Show>
                                  <ChevRight />
                                </span>
                              </button>
                            );
                          }}
                        </For>
                      </div>
                    </div>
                  )}
                </For>
              </div>

              {/* legend */}
              <div class="gg-legend">
                <For each={[0, 1, 2, 3]}>
                  {(tier) => (
                    <span>
                      <i class={`gg-sw gg-sw-${tier}`} />
                      {TIER_LABELS[tier]}
                    </span>
                  )}
                </For>
                <span>
                  <i class="gg-sw duedot" />
                  Time to review
                </span>
              </div>

              <Show when={sheet()}>
                {(s) => (
                  <>
                    <button
                      type="button"
                      class="gg-scrim"
                      data-gg-action="close"
                      aria-label="Close"
                      tabindex="-1"
                      onClick={closeSheet}
                    />
                    <section
                      class="gg-sheet"
                      role="dialog"
                      aria-modal="true"
                      aria-labelledby="gg-sheet-title"
                    >
                      <div class="gg-handle" aria-hidden="true" />
                      <SheetBody
                        scope={s()}
                        states={states()}
                        onClose={closeSheet}
                        closeRef={(el) => (closeBtn = el)}
                      />
                    </section>
                  </>
                )}
              </Show>
            </>
          )}
        </Show>
      </Show>
    </div>
  );
}

/** Row head: `A1 ›` with the level's solid percentage below. */
function LevelHead(props: {
  lv: string;
  states: Map<string, EntryState>;
  on: boolean;
  onOpen: (s: Sheet, trigger: HTMLElement) => void;
}) {
  const stat = () => levelStats(props.states).find((l) => l.id === props.lv)!;
  return (
    <button
      type="button"
      class="gg-rowhead"
      classList={{ on: props.on }}
      data-gg-action="row"
      data-lv={props.lv}
      aria-label={`All topics of level ${props.lv}`}
      onClick={(e) => props.onOpen({ lv: props.lv }, e.currentTarget)}
    >
      <span class="lvline">
        <span class="lv">{props.lv}</span>
        <ChevRight />
      </span>
      <span class="pct">{stat().pct}%</span>
    </button>
  );
}

/* ---- bottom sheet ---- */

interface Group {
  name: string;
  list: EntryState[];
  hideName?: boolean;
  practice?: boolean;
  showLevel?: boolean;
  big?: boolean;
}

/** A group is a titled list of topics; `practice` adds a small
    practice button to its heading. Ported from the prototype. */
function groupsFor(s: Sheet, ts: EntryState[]): Group[] {
  const g = (name: string, list: EntryState[], opts: Partial<Group> = {}): Group => ({
    name,
    list,
    ...opts,
  });
  const splitN = (list: EntryState[], opts: Partial<Group> = {}): Group[] =>
    GG_SUBS.map((sub) => g(sub.name, list.filter((t) => t.entry.sub === sub.key), { practice: true, ...opts })).filter(
      (x) => x.list.length,
    );

  if (s.lv && s.fam) {
    return s.fam === "N" ? splitN(ts) : [g("", ts, { hideName: true })];
  }
  if (s.lv) {
    return GG_KEYS.flatMap((k) => {
      const list = ts.filter((t) => t.entry.cat === k);
      if (!list.length) return [];
      if (k === "N") return splitN(list, { practice: false });
      return [g(GG_CATS[k], list)];
    });
  }
  // whole column
  if (s.fam === "N") return splitN(ts, { showLevel: true, big: true });
  return GG_LEVELS.map((l) => g(l, ts.filter((t) => t.entry.lv === l))).filter((x) => x.list.length);
}

function SheetBody(props: {
  scope: Sheet;
  states: Map<string, EntryState>;
  onClose: () => void;
  closeRef: (el: HTMLButtonElement) => void;
}) {
  const s = props.scope;
  const ts = () => entriesIn(props.states, s.lv, s.fam as FamKey | undefined);

  let eyebrow: string;
  let title: string;
  let cta: string;
  if (s.lv && s.fam) {
    eyebrow = `Level ${s.lv}`;
    title = GG_CATS[s.fam];
    cta = "Practice this block";
  } else if (s.lv) {
    eyebrow = "Everything in";
    title = `Level ${s.lv}`;
    cta = `Practice level ${s.lv}`;
  } else {
    eyebrow = "All levels";
    title = GG_CATS[s.fam as FamKey];
    cta = "Practice all of it";
  }

  const groups = () => groupsFor(s, ts());
  const segs = () =>
    [3, 2, 1, 0]
      .map((i) => ({ n: ts().filter((t) => t.tier === i).length, tier: i }))
      .filter((x) => x.n);
  const dueN = () => ts().filter((t) => t.review).length;

  return (
    <>
      <div class="gg-sheet-head">
        <div class="txt">
          <div class="gg-eyebrow">{eyebrow}</div>
          <h3 class="gg-sheet-title" id="gg-sheet-title">
            {title}
          </h3>
          <div class="gg-sheet-sub">
            {solidOf(ts())} of {ts().length} solid
            {dueN() ? ` · ${dueN()} to review` : ""}
          </div>
        </div>
        <button type="button" class="gg-close" data-gg-action="close" aria-label="Close" ref={props.closeRef} onClick={props.onClose}>
          <CloseIcon />
        </button>
      </div>

      <Show when={s.fam}>
        <p class="gg-about">{GG_ABOUT[s.fam as FamKey]}</p>
      </Show>

      {/* stacked tier bar — Solid → Mastered → Learning → Not started */}
      <div class="gg-bar" aria-hidden="true">
        <For each={segs()}>
          {(x) => <div class={`gg-sw-${x.tier}`} style={{ flex: `${x.n} 0 0` }} />}
        </For>
      </div>

      <div class="gg-list">
        <For each={groups()}>
          {(gr) => (
            <>
              <Show when={!gr.hideName}>
                <div class="gg-group-head" classList={{ big: !!gr.big }}>
                  <b>{gr.name}</b>
                  <span class="n">
                    {solidOf(gr.list)}/{gr.list.length}
                  </span>
                  <Show when={gr.practice}>
                    <a class="gg-mini-practice" href="/practice/feed" aria-label={`Practice ${gr.name}`}>
                      Practice
                    </a>
                  </Show>
                </div>
              </Show>
              <For each={gr.list}>
                {(t) => (
                  <div class="gg-topic">
                    <Show when={gr.showLevel}>
                      <span class="lvtag">{t.entry.lv}</span>
                    </Show>
                    <span class="name">{t.entry.name}</span>
                    <Show when={t.review}>
                      <span class="due">Review</span>
                    </Show>
                    <span class={`gg-chip gg-sw-${t.tier}`}>{TIER_LABELS[t.tier]}</span>
                  </div>
                )}
              </For>
            </>
          )}
        </For>
      </div>

      <a class="gg-cta" href="/practice/feed">
        {cta}
      </a>
    </>
  );
}
