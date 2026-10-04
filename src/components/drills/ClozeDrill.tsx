/**
 * ClozeDrill · three-state feedback + retyping.
 *
 * - Submit answer via Enter.
 * - Verdict: "correct" → continue. "almost" → show diff + rule. "wrong" →
 *   show diff + rule, then force the learner to retype the correct form.
 * - Retyping is a second retrieval — the strongest memory-formation move.
 *
 * Progress and state are session-local; nothing persists between reloads
 * (per the spec: "for now without remembering progress").
 */

import { createSignal, createMemo, Show, createEffect, onMount, onCleanup } from "solid-js";
import { CLOZE_ITEMS, type ClozeItem } from "./data";
import { judge, type Verdict } from "./match";
import {
  DrillHeader,
  DrillProgress,
  SentenceCloze,
  AccentInput,
  FeedbackPanel,
  SummaryPanel,
} from "./ui";

type Phase = "answer" | "feedback" | "retype" | "retype-feedback" | "done";

interface HistoryEntry {
  tried: string;
  verdict: Verdict;
}

export default function ClozeDrill() {
  const [idx, setIdx] = createSignal(0);
  const [phase, setPhase] = createSignal<Phase>("answer");
  const [history, setHistory] = createSignal<HistoryEntry[]>([]);
  const [value, setValue] = createSignal("");
  const [retryValue, setRetryValue] = createSignal("");

  const item = createMemo<ClozeItem>(() => CLOZE_ITEMS[idx()]);
  const verdict = createMemo<Verdict | null>(() => {
    const h = history()[idx()];
    return h?.verdict ?? null;
  });

  const stats = createMemo(() => {
    const h = history();
    return {
      firstTry: h.filter((e) => e.verdict === "correct").length,
      almost: h.filter((e) => e.verdict === "almost").length,
      wrong: h.filter((e) => e.verdict === "wrong").length,
      retypeOk: phase() === "retype-feedback" ? 1 : 0,
    };
  });

  const onSubmit = () => {
    if (!value().trim()) return;
    const v = judge(value(), item().answer);
    setHistory((prev) => {
      const next = prev.slice();
      next[idx()] = { tried: value().trim(), verdict: v };
      return next;
    });
    setPhase(v === "wrong" ? "retype" : "feedback");
  };

  const onRetrySubmit = () => {
    if (!retryValue().trim()) return;
    const v = judge(retryValue(), item().answer);
    if (v === "correct") {
      setPhase("retype-feedback");
    } else {
      // bounce: select the retry value to encourage a re-attempt
      setRetryValue("");
    }
  };

  const next = () => {
    if (idx() + 1 >= CLOZE_ITEMS.length) {
      setPhase("done");
      return;
    }
    setIdx(idx() + 1);
    setPhase("answer");
    setValue("");
    setRetryValue("");
  };

  // Global Enter-to-continue once we're in feedback.
  createEffect(() => {
    const p = phase();
    if (p !== "feedback" && p !== "retype-feedback") return;
    const handler = (e: KeyboardEvent) => {
      if (e.key === "Enter" && !(e.target as HTMLElement)?.matches("input, button")) {
        e.preventDefault();
        next();
      }
    };
    document.addEventListener("keydown", handler);
    onCleanup(() => document.removeEventListener("keydown", handler));
  });

  // Reset value when entering "answer" phase.
  onMount(() => {
    // No-op; just here to ensure consistent setup.
  });

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
                nextHref="/practice/cue"
                nextLabel="next: cue-to-form →"
              />
            }
          >
            <ClozeCard
              item={item()}
              phase={phase()}
              verdict={verdict()}
              tried={history()[idx()]?.tried ?? ""}
              value={value()}
              retryValue={retryValue()}
              onChangeValue={setValue}
              onChangeRetry={setRetryValue}
              onSubmit={onSubmit}
              onRetrySubmit={onRetrySubmit}
              onContinue={next}
            />
          </Show>
        </div>
      </main>

      <footer class="drill-foot">
        Enter to submit · after a wrong answer, type the correct form to continue (it's a second retrieval)
      </footer>
    </div>
  );
}

/* ── Single cloze card ────────────────────────────────────────── */

interface ClozeCardProps {
  item: ClozeItem;
  phase: Phase;
  verdict: Verdict | null;
  tried: string;
  value: string;
  retryValue: string;
  onChangeValue: (v: string) => void;
  onChangeRetry: (v: string) => void;
  onSubmit: () => void;
  onRetrySubmit: () => void;
  onContinue: () => void;
}

function ClozeCard(props: ClozeCardProps) {
  const { item, phase } = props;

  return (
    <div>
      <div class="drill-chips">
        <span class="drill-chip"><span class="drill-chip-k">verb</span><span class="drill-chip-v">{item.hint.verb}</span></span>
        <span class="drill-chip-sep">·</span>
        <span class="drill-chip"><span class="drill-chip-k">person</span><span class="drill-chip-v">{item.hint.person}</span></span>
        <span class="drill-chip-sep">·</span>
        <span class="drill-chip"><span class="drill-chip-k">tense</span><span class="drill-chip-v">{item.hint.tense}</span></span>
      </div>

      <SentenceCloze
        parts={item.parts}
        state={props.verdict}
        answer={item.answer}
        stem={item.stem}
        ending={item.ending}
      />

      <Show when={phase === "answer"}>
        <AccentInput
          value={props.value}
          onInput={props.onChangeValue}
          onSubmit={props.onSubmit}
          placeholder="type the conjugated form"
        />
      </Show>

      <Show when={phase === "feedback" || phase === "retype"}>
        <FeedbackPanel
          verdict={props.verdict ?? "wrong"}
          note={
            props.verdict === "correct"
              ? "✓ correct"
              : props.verdict === "almost"
                ? "almost — watch the accent"
                : "not quite — type the correct form below"
          }
          tried={props.tried}
          answer={item.answer}
          decomp={item.stem ? { stem: item.stem, ending: item.ending } : undefined}
          rule={item.rule}
        />

        <Show when={phase === "retype"}>
          <div class="drill-retry">
            <span class="drill-retry-label">
              Retype <b>{item.answer}</b> to continue ↓
            </span>
            <AccentInput
              value={props.retryValue}
              onInput={props.onChangeRetry}
              onSubmit={props.onRetrySubmit}
              placeholder={`type: ${item.answer}`}
              submitLabel="submit"
            />
          </div>
        </Show>

        <Show when={phase === "feedback" && props.verdict !== "wrong"}>
          <div class="drill-continue-row">
            <button type="button" class="drill-continue is-shown" onClick={props.onContinue}>
              continue <span class="drill-kbd">↵</span>
            </button>
          </div>
        </Show>
      </Show>

      <Show when={phase === "retype-feedback"}>
        <FeedbackPanel
          verdict="correct"
          note="✓ nice recovery — you got the retype"
          answer={item.answer}
        />
        <div class="drill-continue-row">
          <button type="button" class="drill-continue is-shown" onClick={props.onContinue}>
            continue <span class="drill-kbd">↵</span>
          </button>
        </div>
      </Show>
    </div>
  );
}