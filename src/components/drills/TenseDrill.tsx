/**
 * TenseDrill · standalone wrapper for the "spot the tense" exercise
 * (imparfait vs passé composé). Uses the shared TenseView.
 */

import { createSignal, createMemo, Show } from "solid-js";
import { TENSE_ITEMS, type TenseItem } from "./data";
import type { Verdict } from "./match";
import { TenseView } from "./views";
import {
  DrillHeader,
  DrillProgress,
  SummaryPanel,
} from "./ui";

export default function TenseDrill() {
  const [idx, setIdx] = createSignal(0);
  const [phase, setPhase] = createSignal<"playing" | "done">("playing");
  const [stats, setStats] = createSignal({ right: 0, wrong: 0 });

  const item = createMemo<TenseItem>(() => TENSE_ITEMS[idx()]);

  const onComplete = (verdict: Verdict) => {
    setStats((s) => ({
      right: s.right + (verdict === "correct" ? 1 : 0),
      wrong: s.wrong + (verdict === "wrong" ? 1 : 0),
    }));
    if (idx() + 1 >= TENSE_ITEMS.length) {
      setPhase("done");
    } else {
      setIdx(idx() + 1);
    }
  };

  return (
    <div class="drill-root" style={{ "--accent": "#00c48c", "--accent-soft": "rgba(0, 196, 140, 0.10)" }}>
      <DrillHeader
        title="Spot the tense"
        eyebrow={
          phase() === "done"
            ? "session complete"
            : `tense · ${idx() + 1}/${TENSE_ITEMS.length}`
        }
      />
      <DrillProgress pct={idx() / TENSE_ITEMS.length} />

      <main class="drill-stage">
        <div class="drill-card">
          <Show
            when={phase() !== "done"}
            fallback={
              <SummaryPanel
                title="The cue makes the call."
                stats={[
                  { label: "right",       value: stats().right, color: "var(--emerald)" },
                  { label: "to revisit",  value: stats().wrong, color: "var(--coral)" },
                ]}
                note={`${stats().right} of ${TENSE_ITEMS.length} right. The cue words (<em>quand, pendant que, soudain, tous les jours</em>) tell you which lens to use — imparfait is the camera's wide shot; passé composé is the cut to a specific action.`}
                retryHref="/practice/tense"
                nextHref="/practice/conjugations"
                nextLabel="next: endless mix →"
              />
            }
          >
            <TenseView item={item()} onComplete={onComplete} />
          </Show>
        </div>
      </main>

      <footer class="drill-foot">
        Tap a form, or use ← → keys. Enter to continue. The <mark style="background:var(--amber-soft); color:#b07500; padding:0.05em 0.3em; border-radius:4px; font-weight:600">cue word</mark> tells you which tense fits.
      </footer>
    </div>
  );
}