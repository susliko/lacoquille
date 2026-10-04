/**
 * ChainDrill · transformation chain.
 *
 * - Each chain is 3-4 linked steps; each new form is built from the last.
 * - Visible chain viz at the top drives completion (Zeigarnik).
 * - Wrong/almost triggers gentle feedback; no forced retyping per step
 *   (this would break the chain-flow). Wrong steps count against the
 *   first-try rate but the learner advances.
 * - The hardest step is last, so the peak comes near the end (Serial
 *   Position).
 *
 * Per the spec: session-local, no progress tracking.
 */

import { createSignal, createMemo, createEffect, onCleanup, Show, For } from "solid-js";
import { CHAINS, type Chain, type ChainStep } from "./data";
import { judgeWithNegation, type Verdict } from "./match";
import {
  DrillHeader,
  DrillProgress,
  AccentInput,
  FeedbackPanel,
  SummaryPanel,
} from "./ui";

export default function ChainDrill() {
  const [chainIdx, setChainIdx] = createSignal(0);
  const [stepIdx, setStepIdx] = createSignal(0);
  const [phase, setPhase] = createSignal<"answer" | "feedback" | "chain-done" | "done">("answer");
  const [verdict, setVerdict] = createSignal<Verdict | null>(null);
  const [value, setValue] = createSignal("");
  const [history, setHistory] = createSignal<Array<{ tried: string; verdict: Verdict }>>([]);
  const [stats, setStats] = createSignal({ firstTry: 0, total: 0 });

  const chain = createMemo<Chain>(() => CHAINS[chainIdx()]);
  const step = createMemo<ChainStep>(() => chain().steps[stepIdx()]);
  const isLastStep = () => stepIdx() >= chain().steps.length - 1;
  const isLastChain = () => chainIdx() >= CHAINS.length - 1;

  const onSubmit = () => {
    if (!value().trim()) return;
    const v = judgeWithNegation(value(), step().target);
    setVerdict(v);
    setStats((s) => ({
      firstTry: s.firstTry + (v === "correct" ? 1 : 0),
      total: s.total + 1,
    }));
    setHistory((prev) => {
      const next = prev.slice();
      next[stepIdx()] = { tried: value().trim(), verdict: v };
      return next;
    });
    setPhase("feedback");
  };

  const nextStep = () => {
    if (isLastStep()) {
      setPhase("chain-done");
      return;
    }
    setStepIdx(stepIdx() + 1);
    setPhase("answer");
    setVerdict(null);
    setValue("");
  };

  const nextChain = () => {
    if (isLastChain()) {
      setPhase("done");
      return;
    }
    setChainIdx(chainIdx() + 1);
    setStepIdx(0);
    setHistory([]);
    setPhase("answer");
    setVerdict(null);
    setValue("");
  };

  // Enter to continue.
  createEffect(() => {
    if (phase() !== "feedback" && phase() !== "chain-done") return;
    const handler = (e: KeyboardEvent) => {
      if (e.key === "Enter" && !(e.target as HTMLElement)?.matches("input, button")) {
        e.preventDefault();
        if (phase() === "feedback") nextStep();
        else nextChain();
      }
    };
    document.addEventListener("keydown", handler);
    onCleanup(() => document.removeEventListener("keydown", handler));
  });

  return (
    <div class="drill-root" style={{ "--accent": "#8b5cf6", "--accent-soft": "rgba(139, 92, 246, 0.10)" }}>
      <DrillHeader
        title="Transformation chain"
        eyebrow={
          phase() === "done"
            ? "session complete"
            : phase() === "chain-done"
              ? `chain ${chainIdx() + 1}/${CHAINS.length} done`
              : `chain · ${chainIdx() + 1}/${CHAINS.length} · step ${stepIdx() + 1}/${chain().steps.length}`
        }
      />
      <DrillProgress
        pct={
          (chainIdx() + (stepIdx() + (phase() === "feedback" ? 0.5 : 0)) / chain().steps.length) /
          CHAINS.length
        }
      />

      <main class="drill-stage">
        <div class="drill-card">
          <Show when={phase() !== "done"} fallback={<SessionSummary stats={stats()} />}>
            <Show when={phase() !== "chain-done"} fallback={<ChainDone chain={chain()} stats={history()} onContinue={nextChain} isLast={isLastChain()} />}>
              <ChainStepCard
                chain={chain()}
                step={step()}
                stepIdx={stepIdx()}
                value={value()}
                verdict={verdict()}
                tried={history()[stepIdx()]?.tried ?? ""}
                onChangeValue={setValue}
                onSubmit={onSubmit}
                onContinue={nextStep}
                isLastStep={isLastStep()}
              />
            </Show>
          </Show>
        </div>
      </main>

      <footer class="drill-foot">
        Each step transforms the previous answer. The chain is the practice — person &amp; tense switching is where real errors live.
      </footer>
    </div>
  );
}

/* ── Chain visualisation ──────────────────────────────────────── */

function ChainViz(props: {
  chain: Chain;
  stepIdx: number;
  highlight?: "all-done" | "current";
}) {
  return (
    <div class="drill-chain">
      <For each={props.chain.steps}>
        {(s, i) => {
          let cls = "";
          if (props.highlight === "all-done") cls = "is-done";
          else if (i() < props.stepIdx) cls = "is-done";
          else if (i() === props.stepIdx) cls = "is-current";
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
  );
}

/* ── Single step card ─────────────────────────────────────────── */

interface ChainStepCardProps {
  chain: Chain;
  step: ChainStep;
  stepIdx: number;
  value: string;
  verdict: Verdict | null;
  tried: string;
  onChangeValue: (v: string) => void;
  onSubmit: () => void;
  onContinue: () => void;
  isLastStep: boolean;
}

function ChainStepCard(props: ChainStepCardProps) {
  return (
    <div>
      <ChainViz chain={props.chain} stepIdx={props.stepIdx} />

      <div class="drill-step-card">
        <div class="drill-step-instr">
          step {props.stepIdx + 1} of {props.chain.steps.length} · {props.step.instruction}
        </div>

        <div class="drill-step-prev">
          <span class="drill-step-prev-label">from</span>
          <span class="drill-step-prev-form">{props.step.from}</span>
        </div>

        <p class="drill-step-prompt">
          take <b>{props.step.from}</b> and <em>{props.step.instruction}</em>
        </p>

        <AccentInput
          value={props.value}
          onInput={props.onChangeValue}
          onSubmit={props.onSubmit}
          placeholder="type the new form"
          hidden={props.verdict !== null}
        />

        <Show when={props.verdict}>
          <FeedbackPanel
            verdict={props.verdict!}
            note={
              props.verdict === "correct"
                ? "✓ correct"
                : props.verdict === "almost"
                  ? "almost"
                  : "✗ not quite"
            }
            tried={props.tried}
            answer={props.step.target}
            rule={props.step.tip}
          />
          <div class="drill-continue-row">
            <button type="button" class="drill-continue is-shown" onClick={props.onContinue}>
              {props.isLastStep ? "finish chain" : "next step"} <span class="drill-kbd">↵</span>
            </button>
          </div>
        </Show>
      </div>
    </div>
  );
}

/* ── Chain-done screen ────────────────────────────────────────── */

function ChainDone(props: {
  chain: Chain;
  stats: Array<{ tried: string; verdict: Verdict }>;
  onContinue: () => void;
  isLast: boolean;
}) {
  const correct = () => props.stats.filter((s) => s.verdict === "correct").length;
  return (
    <div>
      <ChainViz chain={props.chain} stepIdx={props.chain.steps.length} highlight="all-done" />
      <div class="drill-summary">
        <h2 class="drill-summary-title">
          Chain complete · {correct()} of {props.chain.steps.length} on the first try
        </h2>
        <div class="drill-continue-row" style="justify-content:center">
          <button type="button" class="drill-btn drill-btn-primary" onClick={props.onContinue}>
            {props.isLast ? "see summary" : "next chain"} <span class="drill-kbd" style="background:rgba(255,255,255,0.18); padding:0.1rem 0.4rem; border-radius:4px; margin-left:0.5rem">↵</span>
          </button>
        </div>
      </div>
    </div>
  );
}

/* ── Session summary ──────────────────────────────────────────── */

function SessionSummary(props: { stats: { firstTry: number; total: number } }) {
  const wrong = () => Math.max(0, props.stats.total - props.stats.firstTry);
  const pct = () =>
    props.stats.total === 0
      ? 0
      : Math.round((props.stats.firstTry / props.stats.total) * 100);
  return (
    <SummaryPanel
      title={`${props.stats.firstTry} of ${props.stats.total} chain steps right.`}
      stats={[
        { label: "first-try", value: props.stats.firstTry, color: "var(--emerald)" },
        { label: "retries",   value: wrong(),                color: "var(--coral)" },
        { label: "first-try %", value: `${pct()}%`,         color: "var(--accent)" },
      ]}
      note="The chain exercise builds the kind of switching you'll need in real conversation — <em>elle a fini → vous finissiez → nous ne finissions pas</em>. Real errors happen at the seams between transformations, not in isolation."
      retryHref="/practice/chain"
      nextHref="/practice"
      nextLabel="back to practice →"
    />
  );
}