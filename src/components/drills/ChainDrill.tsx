/**
 * ChainDrill · standalone wrapper for the transformation chain exercise.
 * Uses the shared ChainView.
 */

import { createSignal, createMemo, Show } from "solid-js";
import { CHAINS, type Chain } from "./data";
import type { Verdict } from "./match";
import { ChainView } from "./views";
import {
  DrillHeader,
  DrillProgress,
  SummaryPanel,
} from "./ui";

export default function ChainDrill() {
  const [chainIdx, setChainIdx] = createSignal(0);
  const [phase, setPhase] = createSignal<"playing" | "done">("playing");
  const [stats, setStats] = createSignal({ firstTry: 0, total: 0 });

  const chain = createMemo<Chain>(() => CHAINS[chainIdx()]);
  const isLastChain = () => chainIdx() >= CHAINS.length - 1;

  const onComplete = (verdict: Verdict) => {
    setStats((s) => ({
      firstTry: s.firstTry + (verdict === "correct" ? 1 : 0),
      total: s.total + 1,
    }));
    if (isLastChain()) {
      setPhase("done");
    } else {
      setChainIdx(chainIdx() + 1);
    }
  };

  return (
    <div class="drill-root" style={{ "--accent": "#8b5cf6", "--accent-soft": "rgba(139, 92, 246, 0.10)" }}>
      <DrillHeader
        title="Transformation chain"
        eyebrow={
          phase() === "done"
            ? "session complete"
            : `chain · ${chainIdx() + 1}/${CHAINS.length}`
        }
      />
      <DrillProgress pct={chainIdx() / CHAINS.length} />

      <main class="drill-stage">
        <div class="drill-card">
          <Show
            when={phase() !== "done"}
            fallback={
              <SummaryPanel
                title={`${stats().firstTry} of ${stats().total} chains right.`}
                stats={[
                  { label: "first-try", value: stats().firstTry, color: "var(--emerald)" },
                  { label: "retries",   value: stats().total - stats().firstTry, color: "var(--coral)" },
                ]}
                note="The chain exercise builds the kind of switching you'll need in real conversation — <em>elle a fini → vous finissiez → nous ne finissions pas</em>. Real errors happen at the seams between transformations, not in isolation."
                retryHref="/practice/chain"
                nextHref="/practice/conjugations"
                nextLabel="next: endless mix →"
              />
            }
          >
            <ChainView chain={chain()} onComplete={onComplete} />
          </Show>
        </div>
      </main>

      <footer class="drill-foot">
        Each step transforms the previous answer. The chain is the practice — person &amp; tense switching is where real errors live.
      </footer>
    </div>
  );
}