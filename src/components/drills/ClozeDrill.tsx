/**
 * ClozeDrill · standalone wrapper for the three-state cloze + retyping
 * exercise. Uses the shared ClozeView component (also used by
 * ConjugationsDrill). Manages which item is current and shows a
 * fixed-length session summary at the end.
 */

import { createSignal, createMemo, Show } from "solid-js";
import { CLOZE_ITEMS, type ClozeItem } from "./data";
import type { Verdict } from "./match";
import { ClozeView } from "./views";
import {
  DrillHeader,
  DrillProgress,
  SummaryPanel,
} from "./ui";

export default function ClozeDrill() {
  const [idx, setIdx] = createSignal(0);
  const [phase, setPhase] = createSignal<"playing" | "done">("playing");
  const [stats, setStats] = createSignal({ firstTry: 0, almost: 0, wrong: 0, retypeOk: 0 });

  const item = createMemo<ClozeItem>(() => CLOZE_ITEMS[idx()]);

  const onComplete = (verdict: Verdict) => {
    setStats((s) => {
      if (verdict === "correct") return { ...s, firstTry: s.firstTry + 1 };
      if (verdict === "almost")  return { ...s, almost: s.almost + 1 };
      // For "wrong" in standalone mode we count both the initial wrong
      // AND any successful retype.
      return { ...s, wrong: s.wrong + 1, retypeOk: s.retypeOk + 1 };
    });
    if (idx() + 1 >= CLOZE_ITEMS.length) {
      setPhase("done");
    } else {
      setIdx(idx() + 1);
    }
  };

  return (
    <div class="drill-root" style={{ "--accent": "#ffb703", "--accent-soft": "rgba(255, 183, 3, 0.14)" }}>
      <DrillHeader
        title="Cloze · three-state"
        eyebrow={
          phase() === "done"
            ? "session complete"
            : `cloze · ${idx() + 1}/${CLOZE_ITEMS.length}`
        }
      />
      <DrillProgress pct={idx() / CLOZE_ITEMS.length} />

      <main class="drill-stage">
        <div class="drill-card">
          <Show
            when={phase() !== "done"}
            fallback={
              <SummaryPanel
                title={`${stats().firstTry} of ${CLOZE_ITEMS.length} on the first try.`}
                stats={[
                  { label: "first-try", value: stats().firstTry, color: "var(--emerald)" },
                  { label: "almost",    value: stats().almost,    color: "#b07500" },
                  { label: "wrong",     value: stats().wrong,     color: "var(--coral)" },
                  { label: "retyped",   value: stats().retypeOk,  color: "var(--emerald)" },
                ]}
                note={
                  "First-try rate: <b>" +
                  Math.round((stats().firstTry / CLOZE_ITEMS.length) * 100) +
                  "%</b>. The retype step is a second retrieval — the items you retyped will be easier to recall tomorrow."
                }
                retryHref="/practice/cloze"
                nextHref="/practice/conjugations"
                nextLabel="next: endless mix →"
              />
            }
          >
            <ClozeView item={item()} onComplete={onComplete} />
          </Show>
        </div>
      </main>

      <footer class="drill-foot">
        Enter to submit · after a wrong answer, type the correct form to continue (it's a second retrieval)
      </footer>
    </div>
  );
}