/**
 * ConjugationsDrill · endless stream of mixed verb conjugation exercises.
 *
 * This is the single entry point for verb conjugation practice. It picks
 * a random exercise type and a random item from that type's pool, shows
 * the appropriate View, and continues until the user explicitly ends the
 * session.
 *
 * Why mixed (not blocked): interleaving different exercise types transfers
 * better to real use than blocking ("all cloze, then all cue"). The user
 * has to context-switch, which is closer to how verb forms come up in
 * conversation.
 *
 * No progress is tracked (per spec). Each page load is a fresh session.
 */

import { createSignal, Show, Switch, Match } from "solid-js";
import {
  CLOZE_ITEMS,
  CUE_ITEMS,
  TENSE_ITEMS,
  CHAINS,
  type ClozeItem,
  type CueItem,
  type TenseItem,
  type Chain,
} from "./data";
import type { Verdict } from "./match";
import {
  DrillHeader,
  DrillProgress,
  SummaryPanel,
} from "./ui";
import { ClozeView, CueView, TenseView, ChainView } from "./views";

type Type = "cloze" | "cue" | "tense" | "chain";

interface Exercise {
  type: Type;
  item: ClozeItem | CueItem | TenseItem | Chain;
  /** Monotonic counter for header display ("#3", "#4", ...). */
  number: number;
}

const TYPES: Type[] = ["cloze", "cue", "tense", "chain"];

function pickRandom(num: number): Exercise {
  const type = TYPES[Math.floor(Math.random() * TYPES.length)];
  let item: ClozeItem | CueItem | TenseItem | Chain;
  switch (type) {
    case "cloze":  item = CLOZE_ITEMS[Math.floor(Math.random() * CLOZE_ITEMS.length)];  break;
    case "cue":    item = CUE_ITEMS[Math.floor(Math.random() * CUE_ITEMS.length)];      break;
    case "tense":  item = TENSE_ITEMS[Math.floor(Math.random() * TENSE_ITEMS.length)];  break;
    case "chain":  item = CHAINS[Math.floor(Math.random() * CHAINS.length)];            break;
  }
  return { type, item, number: num };
}

export default function ConjugationsDrill() {
  const [exercise, setExercise] = createSignal<Exercise>(pickRandom(1));
  const [streak, setStreak] = createSignal(0);
  const [stats, setStats] = createSignal({ correct: 0, almost: 0, wrong: 0 });
  const [count, setCount] = createSignal(0);
  const [ended, setEnded] = createSignal(false);

  const onComplete = (verdict: Verdict) => {
    setStats((s) => ({
      correct: s.correct + (verdict === "correct" ? 1 : 0),
      almost:  s.almost  + (verdict === "almost"  ? 1 : 0),
      wrong:   s.wrong   + (verdict === "wrong"   ? 1 : 0),
    }));
    if (verdict === "correct") {
      setStreak((s) => s + 1);
    } else {
      setStreak(0);
    }
    setCount((c) => c + 1);
    // Pick the next exercise — note we schedule the swap on the next
    // microtask so the current view's feedback renders first.
    setTimeout(() => {
      setExercise(pickRandom(count() + 1));
    }, 0);
  };

  const endSession = () => setEnded(true);
  const resume = () => {
    setEnded(false);
    setCount((c) => c + 1);
    setExercise(pickRandom(count() + 1));
  };

  // After the very first render, focus the input inside the View.
  // (Each view handles its own focus via autofocus on its input.)

  return (
    <div class="drill-root" style={{ "--accent": "#3d5af1", "--accent-soft": "rgba(61, 90, 241, 0.10)" }}>
      <DrillHeader
        title="Conjugations"
        eyebrow={ended() ? "session ended" : `#${count() + 1}`}
      />
      <DrillProgress
        pct={
          count() === 0
            ? 0
            : (stats().correct + stats().almost + stats().wrong) > 0
              ? Math.min(1, count() / (count() + 5))
              : 0
        }
      />

      <SessionStrip
        count={count()}
        streak={streak()}
        stats={stats()}
        onEnd={endSession}
      />

      <main class="drill-stage">
        <div class="drill-card">
          <Show
            when={!ended()}
            fallback={
              <SummaryPanel
                title="Session ended."
                stats={[
                  { label: "answered", value: count(), color: "var(--text)" },
                  { label: "correct",  value: stats().correct, color: "var(--emerald)" },
                  { label: "almost",   value: stats().almost, color: "#b07500" },
                  { label: "wrong",    value: stats().wrong, color: "var(--coral)" },
                  {
                    label: "accuracy",
                    value:
                      count() === 0
                        ? "—"
                        : Math.round(((stats().correct + stats().almost) / count()) * 100) + "%",
                    color: "var(--accent)",
                  },
                  { label: "best streak", value: streak(), color: "var(--accent)" },
                ]}
                note={`${count()} mixed exercises across cloze, cue-to-form, spot-the-tense, and transformation chains. Interleaving the types — even when it feels harder than blocking — is what transfers best to real use.`}
                retryHref="/practice/conjugations"
                nextHref="/practice"
                nextLabel="back to practice →"
              >
                {/* allow resuming */}
                <div style="display:flex; gap:0.5rem; justify-content:center; margin-top:1rem">
                  <button
                    class="drill-btn drill-btn-ghost"
                    onClick={resume}
                    type="button"
                  >
                    keep going
                  </button>
                </div>
              </SummaryPanel>
            }
          >
            {/* Render the right view based on exercise type. */}
            <Switch>
              <Match when={exercise().type === "cloze" && exercise()}>
                <ClozeView item={exercise().item as ClozeItem} onComplete={onComplete} />
              </Match>
              <Match when={exercise().type === "cue" && exercise()}>
                <CueView item={exercise().item as CueItem} onComplete={onComplete} streak={streak()} />
              </Match>
              <Match when={exercise().type === "tense" && exercise()}>
                <TenseView item={exercise().item as TenseItem} onComplete={onComplete} />
              </Match>
              <Match when={exercise().type === "chain" && exercise()}>
                <ChainView chain={exercise().item as Chain} onComplete={onComplete} />
              </Match>
            </Switch>
          </Show>
        </div>
      </main>

      <footer class="drill-foot">
        Endless stream — exercise type picked at random from cloze, cue-to-form, spot-the-tense, and chains. Tap <b>end session</b> when you're done.
      </footer>
    </div>
  );
}

/* ── Session strip (header stats + end button) ─────────────── */

interface SessionStripProps {
  count: number;
  streak: number;
  stats: { correct: number; almost: number; wrong: number };
  onEnd: () => void;
}

function SessionStrip(props: SessionStripProps) {
  return (
    <div class="drill-bar" style="border-bottom:none; padding-top:0; padding-bottom:0.5rem; background:transparent; display:flex; align-items:center; gap:0.75rem;">
      <span class="drill-eyebrow" style="font-variant-numeric:tabular-nums">
        <b style="color:var(--text)">{props.count}</b>&nbsp;answered
      </span>
      <span style="color:var(--text-muted)">·</span>
      <span
        class="drill-eyebrow"
        style={{
          color: props.streak === 0 ? "var(--text-muted)" : "var(--accent)",
          "font-weight": "500",
          "font-variant-numeric": "tabular-nums",
        }}
      >
        🔥 <b style={props.streak === 0 ? { color: "var(--text-muted)" } : { color: "var(--accent)" }}>{props.streak}</b>
      </span>
      <span class="drill-spacer" />
      <button
        type="button"
        class="drill-end-btn"
        onClick={props.onEnd}
      >
        end session
      </button>
    </div>
  );
}