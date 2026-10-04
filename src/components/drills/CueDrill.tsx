/**
 * CueDrill · cue-to-form flashcard.
 *
 * - Three chips (verb · person · tense), one large input.
 * - Auto-advance on correct after 1.5s (with a countdown); wrong/almost
 *   pauses for 2.5s and shows the correct form.
 * - Streak counter (most distinct element) — Von Restorff Effect.
 * - Per the spec: session-local, no progress tracking.
 */

import { createSignal, createMemo, onCleanup, Show } from "solid-js";
import { CUE_ITEMS, type CueItem } from "./data";
import { judge, type Verdict } from "./match";
import {
  DrillHeader,
  DrillProgress,
  ChipRow,
  AccentInput,
  SummaryPanel,
} from "./ui";

type Phase = "answer" | "feedback" | "done";

export default function CueDrill() {
  const [idx, setIdx] = createSignal(0);
  const [phase, setPhase] = createSignal<Phase>("answer");
  const [verdict, setVerdict] = createSignal<Verdict | null>(null);
  const [value, setValue] = createSignal("");
  const [streak, setStreak] = createSignal(0);
  const [bestStreak, setBestStreak] = createSignal(0);
  const [stats, setStats] = createSignal({ correct: 0, almost: 0, wrong: 0 });
  const [countdown, setCountdown] = createSignal<number | null>(null);

  const item = createMemo<CueItem>(() => CUE_ITEMS[idx()]);

  /** Auto-advance timer — held in a ref-like signal so we can cancel it. */
  let advanceTimer: number | undefined;

  const clearAdvance = () => {
    if (advanceTimer !== undefined) {
      clearTimeout(advanceTimer);
      advanceTimer = undefined;
    }
    setCountdown(null);
  };

  onCleanup(clearAdvance);

  const scheduleAdvance = (delayMs: number) => {
    clearAdvance();
    setCountdown(delayMs / 1000);
    const startedAt = Date.now();
    advanceTimer = window.setInterval(() => {
      const remaining = Math.max(0, delayMs - (Date.now() - startedAt));
      setCountdown(remaining / 1000);
      if (remaining <= 0) {
        clearAdvance();
        advanceTimer = undefined;
      }
    }, 100);
    advanceTimer = window.setTimeout(() => {
      next();
    }, delayMs);
  };

  const onSubmit = () => {
    if (!value().trim()) return;
    const v = judge(value(), item().answer);
    setVerdict(v);
    setStats((s) => ({
      ...s,
      correct: s.correct + (v === "correct" ? 1 : 0),
      almost: s.almost + (v === "almost" ? 1 : 0),
      wrong: s.wrong + (v === "wrong" ? 1 : 0),
    }));
    if (v === "correct") {
      const ns = streak() + 1;
      setStreak(ns);
      setBestStreak(Math.max(bestStreak(), ns));
    } else {
      setStreak(0);
    }
    setPhase("feedback");
    scheduleAdvance(v === "correct" ? 1500 : 2500);
  };

  const next = () => {
    clearAdvance();
    if (idx() + 1 >= CUE_ITEMS.length) {
      setPhase("done");
      return;
    }
    setIdx(idx() + 1);
    setPhase("answer");
    setVerdict(null);
    setValue("");
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
                nextHref="/practice/tense"
                nextLabel="next: spot the tense →"
              />
            }
          >
            <ChipRow
              items={[
                { label: "verb",   value: item().verb },
                { label: "person", value: item().person },
                { label: "tense",  value: item().tense },
              ]}
              separator="×"
              align="center"
            />

            <AccentInput
              value={value()}
              onInput={setValue}
              onSubmit={onSubmit}
              placeholder="?"
              big
              state={verdict() ?? undefined}
              submitLabel={phase() === "answer" ? "check" : "next"}
              submitKbd={countdown() !== null ? countdown()!.toFixed(1) + "s" : "↵"}
            />

            <Show when={phase() === "feedback" && verdict() !== null}>
              <div class="drill-feedback" classList={{ [`drill-feedback-${verdict()}`]: true }}>
                <div class="drill-feedback-verdict">
                  {verdict() === "correct" && "✓"}
                  {verdict() === "almost" && "almost — watch the accent"}
                  {verdict() === "wrong" && "✗"}
                </div>
                <Show when={verdict() !== "correct"}>
                  <div class="drill-feedback-diff">
                    <span class="drill-feedback-theirs">{item().answer}</span>
                  </div>
                </Show>
              </div>
            </Show>
          </Show>
        </div>
      </main>

      <footer class="drill-foot">
        Enter to submit · auto-advances on correct · missing accents count as <em>almost</em>, not wrong
      </footer>
    </div>
  );
}