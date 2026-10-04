/**
 * Per-type "View" components used by the 4 standalone drill pages AND by
 * the mixed ConjugationsDrill orchestrator.
 *
 * Each View:
 *   - Takes an `item` prop (the data for this question)
 *   - Takes an `onComplete(verdict)` callback (the view fires this when
 *     the user has finished with the item — verdict is the most recent
 *     "correct", "almost", or "wrong" judgement)
 *   - Manages its own phase state (answer → feedback → ...)
 *   - Resets state when `item` changes via `createEffect`
 *
 * The standalone drill components (ClozeDrill, CueDrill, ...) are thin
 * wrappers around these Views that manage which item is current and
 * show a summary at the end of a fixed-length session. The
 * ConjugationsDrill orchestrator mounts them in an endless stream and
 * never shows a summary until the user explicitly ends the session.
 */

import { createSignal, createEffect, Show, For } from "solid-js";
import { judge, judgeWithNegation, type Verdict } from "./match";
import type {
  ClozeItem,
  CueItem,
  TenseItem,
  Chain,
} from "./data";
import {
  ChipRow,
  SentenceCloze,
  AccentInput,
  FeedbackPanel,
} from "./ui";

/* ── Cloze view ──────────────────────────────────────────────── */

interface ClozeViewProps {
  item: ClozeItem;
  onComplete: (v: Verdict) => void;
}

export function ClozeView(props: ClozeViewProps) {
  type Phase = "answer" | "feedback" | "retype" | "retype-feedback";
  const [phase, setPhase] = createSignal<Phase>("answer");
  const [value, setValue] = createSignal("");
  const [retryValue, setRetryValue] = createSignal("");
  const [tried, setTried] = createSignal("");
  const [verdict, setVerdict] = createSignal<Verdict | null>(null);

  // Reset state when item changes (ConjugationsDrill reuses this view).
  createEffect(() => {
    props.item;
    setPhase("answer");
    setValue("");
    setRetryValue("");
    setTried("");
    setVerdict(null);
  });

  const submit = () => {
    if (!value().trim()) return;
    const v = judge(value(), props.item.answer);
    setTried(value().trim());
    setVerdict(v);
    setPhase(v === "wrong" ? "retype" : "feedback");
  };

  const retrySubmit = () => {
    if (!retryValue().trim()) return;
    const v = judge(retryValue(), props.item.answer);
    if (v === "correct") {
      setPhase("retype-feedback");
    } else {
      setRetryValue("");
    }
  };

  const continueForward = () => {
    props.onComplete(verdict() ?? "wrong");
  };

  return (
    <div>
      <ChipRow
        items={[
          { label: "verb",   value: props.item.hint.verb },
          { label: "person", value: props.item.hint.person },
          { label: "tense",  value: props.item.hint.tense },
        ]}
      />
      <SentenceCloze
        parts={props.item.parts}
        state={verdict()}
        answer={props.item.answer}
        stem={props.item.stem}
        ending={props.item.ending}
      />

      <Show when={phase() === "answer"}>
        <AccentInput
          value={value()}
          onInput={setValue}
          onSubmit={submit}
          placeholder="type the conjugated form"
        />
      </Show>

      <Show when={phase() === "feedback" || phase() === "retype"}>
        <FeedbackPanel
          verdict={verdict() ?? "wrong"}
          note={
            verdict() === "correct"
              ? "✓ correct"
              : verdict() === "almost"
                ? "almost — watch the accent"
                : "not quite — type the correct form below"
          }
          tried={tried()}
          answer={props.item.answer}
          decomp={props.item.stem ? { stem: props.item.stem, ending: props.item.ending } : undefined}
          rule={props.item.rule}
        />

        <Show when={phase() === "retype"}>
          <div class="drill-retry">
            <span class="drill-retry-label">
              Retype <b>{props.item.answer}</b> to continue ↓
            </span>
            <AccentInput
              value={retryValue()}
              onInput={setRetryValue}
              onSubmit={retrySubmit}
              placeholder={`type: ${props.item.answer}`}
              submitLabel="submit"
            />
          </div>
        </Show>

        <Show when={phase() === "feedback" && verdict() !== "wrong"}>
          <div class="drill-continue-row">
            <button type="button" class="drill-continue is-shown" onClick={continueForward}>
              continue <span class="drill-kbd">↵</span>
            </button>
          </div>
        </Show>
      </Show>

      <Show when={phase() === "retype-feedback"}>
        <FeedbackPanel
          verdict="correct"
          note="✓ nice recovery — you got the retype"
          answer={props.item.answer}
        />
        <div class="drill-continue-row">
          <button type="button" class="drill-continue is-shown" onClick={continueForward}>
            continue <span class="drill-kbd">↵</span>
          </button>
        </div>
      </Show>

      {/* Keyboard: Enter to continue when in feedback phase */}
      <EnterToContinue
        enabled={phase() === "feedback" || phase() === "retype-feedback"}
        onContinue={continueForward}
      />
    </div>
  );
}

/* ── Cue view ────────────────────────────────────────────────── */

interface CueViewProps {
  item: CueItem;
  onComplete: (v: Verdict) => void;
  /** Streak counter shown above the input. The orchestrator manages it. */
  streak: number;
}

export function CueView(props: CueViewProps) {
  const [phase, setPhase] = createSignal<"answer" | "feedback">("answer");
  const [value, setValue] = createSignal("");
  const [verdict, setVerdict] = createSignal<Verdict | null>(null);

  createEffect(() => {
    props.item;
    setPhase("answer");
    setValue("");
    setVerdict(null);
  });

  const submit = () => {
    if (!value().trim()) return;
    const v = judge(value(), props.item.answer);
    setVerdict(v);
    setPhase("feedback");
    props.onComplete(v);
  };

  return (
    <div>
      <ChipRow
        items={[
          { label: "verb",   value: props.item.verb },
          { label: "person", value: props.item.person },
          { label: "tense",  value: props.item.tense },
        ]}
        separator="×"
        align="center"
      />

      <AccentInput
        value={value()}
        onInput={setValue}
        onSubmit={submit}
        placeholder="?"
        big
        hidden={phase() !== "answer"}
      />

      <Show when={phase() === "feedback" && verdict()}>
        <div class="drill-feedback" classList={{ [`drill-feedback-${verdict()}`]: true }}>
          <div class="drill-feedback-verdict">
            {verdict() === "correct" && "✓"}
            {verdict() === "almost" && "almost — watch the accent"}
            {verdict() === "wrong" && "✗"}
          </div>
          <Show when={verdict() !== "correct"}>
            <div class="drill-feedback-diff">
              <span class="drill-feedback-theirs">{props.item.answer}</span>
            </div>
          </Show>
        </div>
      </Show>
    </div>
  );
}

/* ── Tense view ──────────────────────────────────────────────── */

interface TenseViewProps {
  item: TenseItem;
  onComplete: (v: Verdict) => void;
}

export function TenseView(props: TenseViewProps) {
  const [phase, setPhase] = createSignal<"answer" | "feedback">("answer");
  const [picked, setPicked] = createSignal<number | null>(null);

  createEffect(() => {
    props.item;
    setPhase("answer");
    setPicked(null);
  });

  const onPick = (i: number) => {
    if (phase() !== "answer") return;
    setPicked(i);
    setPhase("feedback");
    const v: Verdict = i === props.item.correct ? "correct" : "wrong";
    props.onComplete(v);
  };

  // Arrow keys for picking.
  createEffect(() => {
    if (phase() !== "answer") return;
    const handler = (e: KeyboardEvent) => {
      if (e.key === "ArrowLeft")  { e.preventDefault(); onPick(0); }
      if (e.key === "ArrowRight") { e.preventDefault(); onPick(1); }
    };
    document.addEventListener("keydown", handler);
    const cleanup = () => document.removeEventListener("keydown", handler);
    // Auto-cleanup via tracking — when phase changes away from "answer",
    // the effect re-runs and we tear down. We attach a tag to the
    // document so we can find and remove the matching listener.
    (handler as any).__cleanup = cleanup;
  });

  // Render the sentence with the cue highlighted.
  let sentence = props.item.sentence.replace("<blank>", "{BLANK}");
  if (props.item.cue) {
    const re = new RegExp(`(${escapeRe(props.item.cue)})`, "g");
    sentence = sentence.replace(re, '<mark style="background:var(--amber-soft); color:#b07500; padding:0.05em 0.3em; border-radius:4px; font-weight:600">$1</mark>');
  }
  const [before, after] = sentence.split("{BLANK}");

  const isRight = () => picked() === props.item.correct;
  const feedbackVerdict = (): Verdict => (isRight() ? "correct" : "wrong");

  return (
    <div>
      <div class="drill-sentence">
        <span innerHTML={before} />
        <span
          class="drill-blank"
          classList={{
            "drill-blank-correct": phase() === "feedback" && isRight(),
            "drill-blank-wrong":   phase() === "feedback" && !isRight(),
          }}
        >
          {phase() === "feedback" ? props.item.options[props.item.correct].form : "?"}
        </span>
        <span innerHTML={after} />
      </div>

      <p class="drill-gloss">{props.item.gloss}</p>

      <div class="drill-choice-grid">
        {props.item.options.map((opt, i) => {
          let state = "";
          if (phase() === "answer") state = "";
          else if (i === props.item.correct) state = "is-right";
          else if (i === picked()) state = "is-wrong";
          else state = "is-dimmed";
          return (
            <button
              type="button"
              class="drill-choice"
              classList={{ [state]: true }}
              disabled={phase() === "feedback"}
              onClick={() => onPick(i)}
            >
              <div class="drill-choice-tense">{opt.tense}</div>
              <div class="drill-choice-form">{opt.form}</div>
              <div class="drill-choice-gloss">{opt.gloss}</div>
            </button>
          );
        })}
      </div>

      <Show when={phase() === "feedback"}>
        <div class="drill-feedback" classList={{ [`drill-feedback-${feedbackVerdict()}`]: true }}>
          <div class="drill-feedback-verdict">
            {isRight() ? "✓ correct" : `the right form is ${props.item.options[props.item.correct].form}`}
          </div>
          <div class="drill-feedback-rule" innerHTML={props.item.rule} />
        </div>
      </Show>
    </div>
  );
}

function escapeRe(s: string): string {
  return s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/* ── Chain view ──────────────────────────────────────────────── */

interface ChainViewProps {
  chain: Chain;
  onComplete: (v: Verdict) => void;
}

export function ChainView(props: ChainViewProps) {
  const [stepIdx, setStepIdx] = createSignal(0);
  const [phase, setPhase] = createSignal<"answer" | "feedback">("answer");
  const [value, setValue] = createSignal("");
  const [verdict, setVerdict] = createSignal<Verdict | null>(null);
  const [tried, setTried] = createSignal("");
  const [history, setHistory] = createSignal<Verdict[]>([]);

  const totalSteps = () => props.chain.steps.length;
  const step = () => props.chain.steps[stepIdx()];
  const isLastStep = () => stepIdx() >= totalSteps() - 1;

  // Reset state when chain changes.
  createEffect(() => {
    props.chain;
    setStepIdx(0);
    setPhase("answer");
    setValue("");
    setVerdict(null);
    setTried("");
    setHistory([]);
  });

  const submit = () => {
    if (!value().trim()) return;
    const v = judgeWithNegation(value(), step().target);
    setTried(value().trim());
    setVerdict(v);
    setPhase("feedback");
    setHistory((prev) => {
      const next = prev.slice();
      next[stepIdx()] = v;
      return next;
    });
  };

  const advance = () => {
    if (isLastStep()) {
      // Compute the chain-level verdict: correct if all steps correct.
      const all = [...history()];
      const lastV = verdict() ?? "wrong";
      all[stepIdx()] = lastV;
      const correctCount = all.filter((x) => x === "correct").length;
      // Verdict passed back: "correct" if all correct, else "wrong".
      // "almost" is reserved for single-item grading.
      const final: Verdict = correctCount === totalSteps() ? "correct" : "wrong";
      props.onComplete(final);
    } else {
      setStepIdx(stepIdx() + 1);
      setPhase("answer");
      setValue("");
      setVerdict(null);
    }
  };

  return (
    <div>
      {/* Chain viz */}
      <div class="drill-chain">
        <For each={props.chain.steps}>
          {(s, i) => {
            let cls = "";
            if (i() < stepIdx()) cls = "is-done";
            else if (i() === stepIdx()) cls = "is-current";
            return (
              <>
                <Show when={i() > 0}>
                  <span class="drill-chain-arrow">→</span>
                </Show>
                <span class={`drill-chain-step ${cls}`}>
                  <span class="drill-chain-step-n">{i() + 1}·</span>
                  <span class="drill-chain-step-f">{s.target}</span>
                </span>
              </>
            );
          }}
        </For>
      </div>

      <div class="drill-step-card">
        <div class="drill-step-instr">
          step {stepIdx() + 1} of {totalSteps()} · {step().instruction}
        </div>

        <div class="drill-step-prev">
          <span class="drill-step-prev-label">from</span>
          <span class="drill-step-prev-form">{step().from}</span>
        </div>

        <p class="drill-step-prompt">
          take <b>{step().from}</b> and <em>{step().instruction}</em>
        </p>

        <AccentInput
          value={value()}
          onInput={setValue}
          onSubmit={submit}
          placeholder="type the new form"
          hidden={phase() !== "answer"}
        />

        <Show when={verdict()}>
          <FeedbackPanel
            verdict={verdict()!}
            note={
              verdict() === "correct"
                ? "✓ correct"
                : verdict() === "almost"
                  ? "almost"
                  : "✗ not quite"
            }
            tried={tried()}
            answer={step().target}
            rule={step().tip}
          />
          <div class="drill-continue-row">
            <button type="button" class="drill-continue is-shown" onClick={advance}>
              {isLastStep() ? "finish chain" : "next step"} <span class="drill-kbd">↵</span>
            </button>
          </div>
        </Show>

        {/* Keyboard: Enter to advance when in feedback */}
        <EnterToContinue enabled={phase() === "feedback"} onContinue={advance} />
      </div>
    </div>
  );
}

/* ── Helper: keyboard Enter → continue ───────────────────────── */

interface EnterToContinueProps {
  enabled: boolean;
  onContinue: () => void;
}

function EnterToContinue(props: EnterToContinueProps) {
  createEffect(() => {
    if (!props.enabled) return;
    const handler = (e: KeyboardEvent) => {
      // Don't intercept Enter if the user is typing in an input or has
      // just clicked a button (button has its own click handler).
      if (e.key === "Enter" && !(e.target as HTMLElement)?.matches("input, button, textarea, select")) {
        e.preventDefault();
        props.onContinue();
      }
    };
    document.addEventListener("keydown", handler);
    return () => document.removeEventListener("keydown", handler);
  });
  return null;
}