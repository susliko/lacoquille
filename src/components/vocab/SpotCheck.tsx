/**
 * Vocabulary size — the spot-check (pushed screen).
 *
 * Three states: what this is, ~70 quick judgments, the estimate.
 *
 * The judgments are stratified by frequency band with deck words
 * excluded (the deck is counted as a census, not sampled), and
 * invented words mixed in without a marker. Disclosing that the
 * invented words exist is deliberate — instructions in the yes/no
 * test tradition do — people still can't reliably tell, and the
 * false-alarm rate stays informative.
 *
 * "I know it" is recognition: you'd understand the word reading it
 * in a sentence. Nothing is written until the last judgment lands;
 * walking away keeps the previous calibration untouched.
 */

import { createSignal, For, onMount, Show } from "solid-js";
import { buildCalibration, drawSample, computeEstimate, deckCensus, type SampleItem, type EstimateResult } from "./estimate";
import { readCalibration, writeCalibration, type Calibration } from "./calibration";

const Back = () => (
  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
    <path d="M15 6l-6 6 6 6" />
  </svg>
);

type Phase = "intro" | "run" | "done";

interface Answer {
  band: number;
  pseudo: boolean;
  known: boolean;
}

/** Rough CEFR anchor for a passive-vocabulary estimate — fuzzy by
 *  nature, shown as "roughly", never as a grade. */
function cefr(v: number): string {
  const scale: Array<[number, string]> = [
    [16_000, "C2"],
    [8_000, "C1"],
    [4_000, "B2"],
    [2_000, "B1"],
    [1_000, "A2"],
    [400, "A1"],
  ];
  for (const [n, label] of scale) if (v >= n) return label;
  return "A1";
}

const fmt = (n: number) => Math.round(n).toLocaleString("en");

/** Recompute the detail view from a persisted snapshot — same math
 *  as the live run, no second source of truth. */
function estimateOf(c: Calibration): EstimateResult {
  return computeEstimate(
    c.bands.map((b) => ({ from: b.from, to: b.to, yes: b.yes, no: b.no, deckInBand: b.deckInBand })),
    c.falseAlarms,
    c.pseudoAsked,
    c.deckKnown,
  );
}

/** The estimate read-out: point, interval, band bars, and the
 *  honest footnotes. Shared by the result screen and the intro's
 *  "your last estimate" block. */
function resultView(r: EstimateResult, c: Calibration) {
  return (
    <>
      <div class="ve-big">≈ {fmt(r.estimate)}</div>
      <div class="ve-sub">of the 20,000 most frequent French words</div>
      <div class="ve-ci">
        95% range {fmt(r.low)} – {fmt(r.high)} · roughly {cefr(r.estimate)} territory
      </div>

      <div class="ve-bands">
        <For each={r.bands}>
          {(b) => (
            <Show when={b.asked > 0}>
              <div class="ve-band">
                <span class="k">{b.label}</span>
                <div class="ve-bbar" aria-hidden="true">
                  <i style={{ width: `${Math.round(b.p * 100)}%` }} />
                </div>
                <span class="v">{Math.round(b.p * 100)}%</span>
              </div>
            </Show>
          )}
        </For>
      </div>

      <Show when={c.falseAlarms > 0}>
        <p class="ai-note">
          You marked {c.falseAlarms} of {c.pseudoAsked} invented words as known — the bands
          above are corrected for that optimism.
        </p>
      </Show>
      <p class="ai-note">
        {fmt(c.deckKnown)} words counted directly from your decks · {c.wordsAsked} sampled.
      </p>
    </>
  );
}

export default function SpotCheck() {
  const [phase, setPhase] = createSignal<Phase>("intro");
  const [items, setItems] = createSignal<SampleItem[]>([]);
  const [idx, setIdx] = createSignal(0);
  const [answers, setAnswers] = createSignal<Answer[]>([]);
  const [cal, setCal] = createSignal<Calibration | null>(null);

  // a previous run's snapshot — shown on the intro so the estimate
  // stays inspectable (band by band) after leaving the result screen
  onMount(() => setCal(readCalibration()));

  const start = () => {
    // deck words are drawn out of the population here — census, not
    // sample. Runs on tap only, so localStorage is never touched
    // during SSR/hydration.
    const sample = drawSample(deckCensus().keys);
    setItems(sample);
    setAnswers([]);
    setIdx(0);
    setPhase("run");
  };

  const answer = (known: boolean) => {
    const cur = items()[idx()];
    const next = [...answers(), { band: cur.band, pseudo: cur.pseudo, known }];
    setAnswers(next);
    if (idx() + 1 >= items().length) {
      finish(next);
    } else {
      setIdx(idx() + 1);
    }
  };

  const undo = () => {
    if (answers().length === 0) return;
    setAnswers(answers().slice(0, -1));
    setIdx(Math.max(0, idx() - 1));
  };

  const finish = (all: Answer[]) => {
    const c = buildCalibration(all);
    if (!c) {
      setPhase("intro");
      return;
    }
    writeCalibration(c);
    setCal(c);
    setPhase("done");
  };

  const result = () => {
    const c = cal();
    if (!c) return null;
    return estimateOf(c);
  };

  const when = () =>
    new Date((cal()?.at ?? 0) * 1000).toLocaleDateString("en", { month: "long", year: "numeric" });

  return (
    <div class="ai-root">
      <header class="ai-bar">
        <a class="ai-back" href="/" aria-label="Back">
          <Back />
        </a>
        <div class="ai-title">Vocabulary size</div>
      </header>

      <div class="ai-scroll">
        <Show when={phase() === "intro"}>
          <Show when={cal()} keyed>
            {(c) => (
              <section class="ai-sec">
                <div class="ai-sechead">
                  <h2>Your last estimate</h2>
                  <span class="ve-lastdate">{when()}</span>
                </div>
                {resultView(estimateOf(c), c)}
                <p class="ai-note">
                  Running it again draws a fresh sample — a second estimate landing close to
                  this one is the strongest signal the number is real.
                </p>
              </section>
            )}
          </Show>
          <section class="ai-sec">
            <div class="ve-lead">
              A two-minute check of what you actually know. Words are drawn from the 20,000
              most frequent French lemmas, spread across five frequency bands — plus a few
              invented ones.
            </div>
            <p class="ai-note">
              “I know it” means recognition: you'd understand the word reading it in a
              sentence. No half credit, no guessing — the estimate is statistical, and wrong
              taps average out.
            </p>
            <p class="ai-note">
              About ten of the words are invented, French-shaped non-words. Everyone
              over-claims a little; the invented ones measure how much, and the estimate
              corrects for it. They're indistinguishable on purpose.
            </p>
            <p class="ai-note">
              Words already in your decks are never asked — those are counted directly from
              your review schedules. Nothing is saved until the last answer.
            </p>
          </section>
          <section class="ai-sec">
            <button type="button" class="ai-cta" onClick={start}>
              {cal() ? "Run it again" : "Start the check"}
            </button>
          </section>
        </Show>

        <Show when={phase() === "run"}>
          <div class="ve-progress" aria-hidden="true">
            <i style={{ width: `${(idx() / items().length) * 100}%` }} />
          </div>
          <div class="ve-count">
            {idx() + 1} / {items().length}
          </div>
          <div class="ve-run">
            <div class="ve-word">{items()[idx()]?.word}</div>
            <div class="ve-buttons">
              <button type="button" class="ve-no" onClick={() => answer(false)}>
                I don't
              </button>
              <button type="button" class="ve-yes" onClick={() => answer(true)}>
                I know it
              </button>
            </div>
            <Show when={answers().length > 0}>
              <button type="button" class="ve-undo" onClick={undo}>
                undo
              </button>
            </Show>
          </div>
        </Show>

        <Show when={phase() === "done" && cal()} keyed>
          {(c) => (
            <section class="ai-sec ai-done">
              {resultView(estimateOf(c), c)}
              <p class="ai-note">Estimated {when()} — recalibrate in a month or two, vocabulary drifts.</p>
              <a class="ai-cta" href="/">
                Done
              </a>
            </section>
          )}
        </Show>
      </div>
    </div>
  );
}
