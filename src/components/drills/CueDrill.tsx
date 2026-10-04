/**
 * CueDrill · standalone wrapper for the cue-to-form flashcard. Uses the
 * shared CueView component. Manages which item is current, streak,
 * best streak, and shows a summary at the end of the session.
 */

import { createSignal, createMemo, Show } from "solid-js";
import { CUE_ITEMS, type CueItem } from "./data";
import type { Verdict } from "./match";
import { CueView } from "./views";
import {
  DrillHeader,
  DrillProgress,
  SummaryPanel,
} from "./ui";

export default function CueDrill() {
  const [idx, setIdx] = createSignal(0);
  const [phase, setPhase] = createSignal<"playing" | "done">("playing");
  const [streak, setStreak] = createSignal(0);
  const [bestStreak, setBestStreak] = createSignal(0);
  const [stats, setStats] = createSignal({ correct: 0, almost: 0, wrong: 0 });

  const item = createMemo<CueItem>(() => CUE_ITEMS[idx()]);

  const onComplete = (verdict: Verdict) => {
    setStats((s) => ({
      ...s,
      correct: s.correct + (verdict === "correct" ? 1 : 0),
      almost:  s.almost  + (verdict === "almost"  ? 1 : 0),
      wrong:   s.wrong   + (verdict === "wrong"   ? 1 : 0),
    }));
    if (verdict === "correct") {
      const ns = streak() + 1;
      setStreak(ns);
      setBestStreak(Math.max(bestStreak(), ns));
    } else {
      setStreak(0);
    }
    if (idx() + 1 >= CUE_ITEMS.length) {
      setPhase("done");
    } else {
      setIdx(idx() + 1);
    }
  };

  return (
    <div class="drill-root" style={{ "--accent": "#ff4757", "--accent-soft": "rgba(255, 71, 87, 0.10)" }}>
      <DrillHeader
        title="Cue-to-form"
        eyebrow={
          phase() === "done"
            ? "session complete"
            : `cue · ${idx() + 1}/${CUE_ITEMS.length}`
        }
      />
      <DrillProgress pct={idx() / CUE_ITEMS.length} />

      <div class="drill-bar" style="border-bottom:none; padding-top:0; padding-bottom:0.5rem; background:transparent">
        <span class="drill-spacer" />
        <span
          class="drill-eyebrow"
          style={{ color: streak() === 0 ? "var(--text-muted)" : "var(--coral)", "font-weight": "500" }}
        >
          🔥 streak <b>{streak()}</b>
        </span>
      </div>

      <main class="drill-stage">
        <div class="drill-card">
          <Show
            when={phase() !== "done"}
            fallback={
              <SummaryPanel
                title={`${stats().correct} of ${CUE_ITEMS.length} on the first try.`}
                stats={[
                  { label: "correct", value: stats().correct, color: "var(--emerald)" },
                  { label: "almost",  value: stats().almost,  color: "#b07500" },
                  { label: "wrong",   value: stats().wrong,   color: "var(--coral)" },
                  { label: "best streak", value: bestStreak(), color: "var(--coral)" },
                ]}
                note="Pure retrieval at speed. The streak counter creates a <em>don't break it</em> pressure — the strongest loop in the set."
                retryHref="/practice/cue"
                nextHref="/practice/conjugations"
                nextLabel="next: endless mix →"
              />
            }
          >
            <CueView item={item()} onComplete={onComplete} streak={streak()} />
          </Show>
        </div>
      </main>

      <footer class="drill-foot">
        Enter to submit · auto-advances on correct · missing accents count as <em>almost</em>, not wrong
      </footer>
    </div>
  );
}