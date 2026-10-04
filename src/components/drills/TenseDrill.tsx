/**
 * TenseDrill · spot the tense (imparfait vs passé composé).
 *
 * - Story context with a gap, two tappable candidate cards.
 * - Cue word highlighted in amber (Von Restorff).
 * - After tapping, the explanation reveals (the "aha" moment).
 * - Arrow keys (← →) to pick; Enter to continue.
 *
 * Per the spec: session-local, no progress tracking.
 */

import { createSignal, createMemo, createEffect, onCleanup, Show } from "solid-js";
import { TENSE_ITEMS, type TenseItem } from "./data";
import {
  DrillHeader,
  DrillProgress,
  SummaryPanel,
} from "./ui";

export default function TenseDrill() {
  const [idx, setIdx] = createSignal(0);
  const [phase, setPhase] = createSignal<"answer" | "feedback" | "done">("answer");
  const [picked, setPicked] = createSignal<number | null>(null);
  const [stats, setStats] = createSignal({ right: 0, wrong: 0 });

  const item = createMemo<TenseItem>(() => TENSE_ITEMS[idx()]);

  const onPick = (i: number) => {
    if (phase() !== "answer") return;
    setPicked(i);
    setStats((s) => ({
      right: s.right + (i === item().correct ? 1 : 0),
      wrong: s.wrong + (i === item().correct ? 0 : 1),
    }));
    setPhase("feedback");
  };

  const next = () => {
    if (idx() + 1 >= TENSE_ITEMS.length) {
      setPhase("done");
      return;
    }
    setIdx(idx() + 1);
    setPicked(null);
    setPhase("answer");
  };

  // Arrow keys for picking (Hick: minimise hand travel).
  createEffect(() => {
    if (phase() !== "answer") return;
    const handler = (e: KeyboardEvent) => {
      if (e.key === "ArrowLeft")  { e.preventDefault(); onPick(0); }
      if (e.key === "ArrowRight") { e.preventDefault(); onPick(1); }
    };
    document.addEventListener("keydown", handler);
    onCleanup(() => document.removeEventListener("keydown", handler));
  });

  // Enter to continue once feedback is shown.
  createEffect(() => {
    if (phase() !== "feedback") return;
    const handler = (e: KeyboardEvent) => {
      if (e.key === "Enter" && !(e.target as HTMLElement)?.matches("input, button")) {
        e.preventDefault();
        next();
      }
    };
    document.addEventListener("keydown", handler);
    onCleanup(() => document.removeEventListener("keydown", handler));
  });

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
                  { label: "right",    value: stats().right, color: "var(--emerald)" },
                  { label: "to revisit", value: stats().wrong, color: "var(--coral)" },
                ]}
                note={`${stats().right} of ${TENSE_ITEMS.length} right. The cue words (<em>quand, pendant que, soudain, tous les jours</em>) tell you which lens to use — imparfait is the camera's wide shot; passé composé is the cut to a specific action.`}
                retryHref="/practice/tense"
                nextHref="/practice/chain"
                nextLabel="next: transformation chain →"
              />
            }
          >
            <TenseCard
              item={item()}
              phase={phase() === "done" ? "answer" : (phase() as "answer" | "feedback")}
              picked={picked() ?? 0}
              onPick={onPick}
              onContinue={next}
            />
          </Show>
        </div>
      </main>

      <footer class="drill-foot">
        Tap a form, or use ← → keys. Enter to continue. The <mark style="background:var(--amber-soft); color:#b07500; padding:0.05em 0.3em; border-radius:4px; font-weight:600">cue word</mark> tells you which tense fits.
      </footer>
    </div>
  );
}

/* ── Single tense card ────────────────────────────────────────── */

interface TenseCardProps {
  item: TenseItem;
  phase: "answer" | "feedback";
  picked: number;
  onPick: (i: number) => void;
  onContinue: () => void;
}

function TenseCard(props: TenseCardProps) {
  const renderSentence = (blank: any) => {
    let sentence = props.item.sentence.replace("<blank>", "{BLANK}");
    if (props.item.cue) {
      const re = new RegExp(`(${escapeRe(props.item.cue)})`, "g");
      sentence = sentence.replace(re, '<mark style="background:var(--amber-soft); color:#b07500; padding:0.05em 0.3em; border-radius:4px; font-weight:600">$1</mark>');
    }
    const [before, after] = sentence.split("{BLANK}");
    return (
      <p class="drill-sentence">
        <span innerHTML={before} />
        {blank}
        <span innerHTML={after} />
      </p>
    );
  };

  const isRight = () => props.picked === props.item.correct;

  return (
    <div>
      {renderSentence(
        <span
          class="drill-blank"
          classList={{
            "drill-blank-correct": props.phase === "feedback" && isRight(),
            "drill-blank-wrong":   props.phase === "feedback" && !isRight(),
          }}
        >
          {props.phase === "feedback" ? props.item.options[props.item.correct].form : "?"}
        </span>
      )}

      <p class="drill-gloss">{props.item.gloss}</p>

      <div class="drill-choice-grid">
        {props.item.options.map((opt, i) => {
          const state =
            props.phase === "answer"
              ? ""
              : i === props.item.correct
                ? "is-right"
                : i === props.picked
                  ? "is-wrong"
                  : "is-dimmed";
          return (
            <button
              type="button"
              class="drill-choice"
              classList={{ [state]: true }}
              disabled={props.phase === "feedback"}
              onClick={() => props.onPick(i)}
            >
              <div class="drill-choice-tense">{opt.tense}</div>
              <div class="drill-choice-form">{opt.form}</div>
              <div class="drill-choice-gloss">{opt.gloss}</div>
            </button>
          );
        })}
      </div>

      <Show when={props.phase === "feedback"}>
        <div class="drill-feedback" classList={{ [`drill-feedback-${isRight() ? "correct" : "wrong"}`]: true }}>
          <div class="drill-feedback-verdict">
            {isRight() ? "✓ correct" : `the right form is ${props.item.options[props.item.correct].form}`}
          </div>
          <div class="drill-feedback-rule" innerHTML={props.item.rule} />
        </div>
        <div class="drill-continue-row">
          <button type="button" class="drill-continue is-shown" onClick={props.onContinue}>
            continue <span class="drill-kbd">↵</span>
          </button>
        </div>
      </Show>
    </div>
  );
}

function escapeRe(s: string): string {
  return s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}