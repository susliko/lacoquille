/**
 * Progress screen — the first tab (regroup 2026-11).
 *
 * Status of where you STAND, never of what you did (handoff
 * 01-decisions: no streaks, no practice measurement). Two grade
 * cards — grammar (the feed's rolling CEFR estimate) and vocabulary
 * (a word count from the SM-2 intervals) — then the full grammar
 * grid, moved here from the old Practice home so Progress is the one
 * route to it.
 *
 * Both cards carry honest zero states: nothing answered → no grade;
 * no cards → no estimate. The grid handles fresh learners itself.
 *
 * The vocabulary card is an estimate, not a count: the deck is a
 * census of words (word-level — reversed imports drill one word
 * through two cards), and the spot-check (/vocab/estimate/) adds
 * the stratified sample that sees words no deck ever carded. Until
 * it runs, the card shows the deck census with an invitation.
 */

import { createSignal, onMount, Show } from "solid-js";
import ProgressGrid from "./progress/ProgressGrid";
import { currentLevel, feedTopicStats } from "./exercise/feedStore";
import { topicSummaries } from "./exercise/store";
import { readCalibration } from "./vocab/calibration";
import { wordSplit, type WordSplit } from "./vocab/words";

const Gear = () => (
  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
    <circle cx="12" cy="12" r="3.2" />
    <path d="M19.4 14.6a1.6 1.6 0 0 0 .3 1.8l.1.1a2 2 0 1 1-2.8 2.8l-.1-.1a1.6 1.6 0 0 0-2.7 1.1v.3a2 2 0 1 1-4 0v-.2a1.6 1.6 0 0 0-2.8-1.1l-.1.1a2 2 0 1 1-2.8-2.8l.1-.1a1.6 1.6 0 0 0-1.1-2.7h-.3a2 2 0 1 1 0-4h.2a1.6 1.6 0 0 0 1.1-2.8l-.1-.1a2 2 0 1 1 2.8-2.8l.1.1a1.6 1.6 0 0 0 1.8.3h.1a1.6 1.6 0 0 0 1-1.5v-.3a2 2 0 1 1 4 0v.2a1.6 1.6 0 0 0 2.7 1.1l.1-.1a2 2 0 1 1 2.8 2.8l-.1.1a1.6 1.6 0 0 0 1.1 2.7h.3a2 2 0 1 1 0 4h-.2a1.6 1.6 0 0 0-1.5 1z" />
  </svg>
);

export default function ProgressHome() {
  // SSR-safe defaults: nothing drilled, no cards. onMount reads the
  // stores — all learner state is client-side.
  const [drilled, setDrilled] = createSignal(false);
  const [level, setLevel] = createSignal("A1");
  const [vocab, setVocab] = createSignal<WordSplit>({ total: 0, mature: 0, young: 0, learning: 0, fresh: 0, known: 0, keys: new Set() });
  const [cal, setCal] = createSignal<ReturnType<typeof readCalibration>>(null);

  onMount(() => {
    const feed = Object.values(feedTopicStats());
    const session = Object.values(topicSummaries());
    const any = feed.some((t) => t.attempts > 0) || session.some((s) => s.attempts > 0);
    setDrilled(any);
    if (any) setLevel(currentLevel());
    setVocab(wordSplit());
    setCal(readCalibration());
  });

  const pct = (n: number) => (vocab().total > 0 ? `${Math.round((n / vocab().total) * 100)}%` : "0%");
  /** Point estimate: the spot-check's snapshot once run, else the
   *  deck census alone (word-level — a reversed import drills one
   *  word through two cards, and cards aren't words). */
  const big = () => {
    const c = cal();
    if (c) return `≈${(Math.round(c.estimate / 10) * 10).toLocaleString("en")}`;
    return vocab().total > 0 ? `≈${vocab().total.toLocaleString("en")}` : "";
  };
  const spread = () => {
    const c = cal();
    if (!c) return "";
    return `±${(Math.round((c.high - c.low) / 2 / 10) * 10).toLocaleString("en")}`;
  };
  const when = () =>
    new Date((cal()?.at ?? 0) * 1000).toLocaleDateString("en", { month: "short", year: "numeric" });

  return (
    <div class="ph-root">
      <header class="ph-bar">
        <div class="ph-mark">
          La <span>Coquille</span>
        </div>
        <a class="ph-iconbtn" href="/settings/" aria-label="Settings">
          <Gear />
        </a>
      </header>

      {/* The two grades */}
      <section class="ph-sec">
        <div class="pg-cards">
          <div class="pg-card">
            <span class="pg-eyebrow">Grammar</span>
            <Show when={drilled()} fallback={<span class="pg-big pg-dim">—</span>}>
              <span class="pg-big">{level()}</span>
            </Show>
            <span class="pg-sub">
              {drilled()
                ? "estimated from your answers"
                : "start practicing to find your level"}
            </span>
          </div>

          <div class="pg-card">
            <span class="pg-eyebrow">Vocabulary</span>
            <Show when={big() !== ""} fallback={<span class="pg-big pg-dim">—</span>}>
              <span class="pg-big">{big()}</span>
            </Show>
            <Show when={vocab().total > 0}>
              <div class="pg-bar" aria-hidden="true">
                <i style={{ width: pct(vocab().mature), background: "var(--p2)" }} />
                <i style={{ width: pct(vocab().young), background: "var(--p1)" }} />
                <i style={{ width: pct(vocab().learning + vocab().fresh), background: "var(--p0)" }} />
              </div>
            </Show>
            <Show
              when={cal()}
              fallback={
                vocab().total > 0 ? (
                  <a class="pg-sub pg-cal" href="/vocab/estimate/">
                    in your decks · calibrate the full picture
                  </a>
                ) : (
                  <span class="pg-sub">add words in Practice</span>
                )
              }
            >
              <a class="pg-sub pg-cal" href="/vocab/estimate/" title="Run the spot-check again">
                of 20,000 common words · {spread()} · {when()}
              </a>
            </Show>
          </div>
        </div>
      </section>

      {/* Your grammar */}
      <section class="ph-sec ph-sec-grid">
        <div class="ph-sechead">
          <h2>Your grammar</h2>
          <a href="/theory/">All topics</a>
        </div>
        <ProgressGrid visible={true} titled={false} />
      </section>
    </div>
  );
}
