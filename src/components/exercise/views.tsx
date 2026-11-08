/**
 * The six exercise views, one per designed screen:
 *   cloze (02/03/04) · choice (05) · tiles (06) · transform (07)
 *   error-spot (08) · dictation (09)
 *
 * Each view owns its answer → feedback phases and calls
 * `onResolve(correct)` when the learner taps Next. The Session mounts a
 * view fresh for every queue slot (keyed Show), so mount state is item
 * state — no reset effects needed.
 */

import { createSignal, For, Show } from "solid-js";
import type {
  ChoiceData,
  ClozeData,
  DictationData,
  ErrorSpotData,
  TilesData,
  TransformData,
} from "./data";
import { eqLoose, seqLoose } from "./grade";
import {
  AnswerField,
  EnterToContinue,
  ExerciseFoot,
  FeedbackPanel,
  Gap,
  noteFor,
} from "./ui";

export interface ViewProps<T> {
  item: T;
  /** True when this is the in-session retry of a missed item. */
  isRetry: boolean;
  onResolve: (correct: boolean) => void;
}

/**
 * Shared tail of every view: feedback panel + foot + Enter handling.
 * Props are accessors so the foot button and accent keys stay reactive
 * without the views re-rendering anything themselves.
 */
function Tail(props: {
  verdict: () => "ok" | "miss" | null;
  isRetry: boolean;
  answer?: string;
  rule: string[];
  canCheck: () => boolean;
  check: () => void;
  next: () => void;
  /** Provide for typing exercises — shows the accent keys while answering. */
  onAccent?: (c: string) => void;
}) {
  return (
    <>
      <Show when={props.verdict()}>
        <FeedbackPanel
          verdict={props.verdict()!}
          answer={props.answer}
          rule={props.rule}
          note={noteFor(props.verdict()!, props.isRetry)}
        />
      </Show>
      <ExerciseFoot
        accents={props.onAccent && !props.verdict() ? { onPick: props.onAccent } : undefined}
        button={
          props.verdict()
            ? {
                label: "Next",
                kind: props.verdict() === "ok" ? "next-ok" : "next-miss",
                onClick: props.next,
              }
            : {
                label: "Check",
                kind: props.canCheck() ? "check" : "disabled",
                onClick: props.check,
              }
        }
      />
      <EnterToContinue enabled={!!props.verdict()} onContinue={props.next} />
    </>
  );
}

/* ── Cloze (frames 02, 03, 04) ────────────────────────────────── */

export function ClozeView(props: ViewProps<ClozeData>) {
  const [value, setValue] = createSignal("");
  const [verdict, setVerdict] = createSignal<"ok" | "miss" | null>(null);

  const check = () => {
    if (!value().trim() || verdict()) return;
    setVerdict(eqLoose(value(), props.item.answer) ? "ok" : "miss");
  };
  const next = () => props.onResolve(verdict() === "ok");

  return (
    <div class="ex-view">
      <p class="ex-task">{props.item.task[0]}</p>
      <p class="ex-sentence">
        {props.item.head} <Gap answer={props.item.answer} filled={!!verdict()} /> {props.item.tail}
      </p>
      <p class="ex-hint">({props.item.infinitive})</p>

      <AnswerField
        value={value()}
        onInput={setValue}
        onEnter={check}
        disabled={!!verdict()}
      />
      <Tail
        verdict={verdict}
        isRetry={props.isRetry}
        answer={props.item.answer}
        rule={props.item.rule}
        canCheck={() => !!value().trim()}
        check={check}
        next={next}
        onAccent={(c) => setValue((v) => v + c)}
      />
    </div>
  );
}

/* ── Choice (frame 05) ────────────────────────────────────────── */

export function ChoiceView(props: ViewProps<ChoiceData>) {
  const [picked, setPicked] = createSignal<number | null>(null);
  const [verdict, setVerdict] = createSignal<"ok" | "miss" | null>(null);

  const check = () => {
    if (picked() === null || verdict()) return;
    setVerdict(picked() === props.item.correct ? "ok" : "miss");
  };
  const next = () => props.onResolve(verdict() === "ok");

  const stateFor = (i: number) => {
    if (!verdict()) return picked() === i ? "is-picked" : "";
    if (i === props.item.correct) return "is-right";
    if (picked() === i) return "is-wrong";
    return "";
  };

  return (
    <div class="ex-view">
      <p class="ex-task">{props.item.task[0]}</p>
      <p class="ex-sentence">{props.item.sentence}</p>

      <div class="ex-choices" role="radiogroup" aria-label="Answer options">
        <For each={props.item.options}>
          {(opt, i) => (
            <button
              type="button"
              role="radio"
              aria-checked={picked() === i()}
              class="ex-choice"
              classList={{ [stateFor(i())]: true }}
              disabled={!!verdict()}
              onClick={() => !verdict() && setPicked(i())}
            >
              {opt}
            </button>
          )}
        </For>
      </div>

      <Tail
        verdict={verdict}
        isRetry={props.isRetry}
        answer={props.item.options[props.item.correct]}
        rule={props.item.rule}
        canCheck={() => picked() !== null}
        check={check}
        next={next}
      />
    </div>
  );
}

/* ── Tiles (frame 06) ─────────────────────────────────────────── */

export function TilesView(props: ViewProps<TilesData>) {
  /** Indices into item.bank, in placement order. */
  const [placed, setPlaced] = createSignal<number[]>([]);
  const [verdict, setVerdict] = createSignal<"ok" | "miss" | null>(null);

  const used = () => new Set(placed());
  const canCheck = () =>
    placed().length === props.item.target.length && !verdict();

  const check = () => {
    if (!canCheck()) return;
    const seq = placed().map((i) => props.item.bank[i]);
    setVerdict(seqLoose(seq, props.item.target) ? "ok" : "miss");
  };
  const next = () => props.onResolve(verdict() === "ok");

  const placeFromBank = (i: number) => {
    if (verdict() || used().has(i)) return;
    setPlaced((p) => [...p, i]);
  };
  const removeFromSlot = (slot: number) => {
    if (verdict()) return;
    setPlaced((p) => p.filter((_, k) => k !== slot));
  };

  return (
    <div class="ex-view">
      <p class="ex-task">{props.item.task[0]}</p>
      <Show when={props.item.task[1]}>
        <p class="ex-task ex-task-sub">{props.item.task[1]}</p>
      </Show>

      <div class="ex-slot" aria-label="Your sentence">
        <Show when={placed().length === 0}>
          <span class="ex-slot-hint">Tap the words in order</span>
        </Show>
        <For each={placed()}>
          {(bankIdx, slot) => (
            <button
              type="button"
              class="ex-tile is-placed"
              disabled={!!verdict()}
              onClick={() => removeFromSlot(slot())}
            >
              {props.item.bank[bankIdx]}
            </button>
          )}
        </For>
      </div>

      <div class="ex-bank">
        <For each={props.item.bank}>
          {(word, i) => (
            <button
              type="button"
              class="ex-tile"
              classList={{ "is-used": used().has(i()) }}
              disabled={!!verdict() || used().has(i())}
              onClick={() => placeFromBank(i())}
            >
              {word}
            </button>
          )}
        </For>
      </div>

      <Tail
        verdict={verdict}
        isRetry={props.isRetry}
        answer={props.item.target.join(" ")}
        rule={props.item.rule}
        canCheck={canCheck}
        check={check}
        next={next}
      />
    </div>
  );
}

/* ── Transform (frame 07) ─────────────────────────────────────── */

export function TransformView(props: ViewProps<TransformData>) {
  const [value, setValue] = createSignal("");
  const [verdict, setVerdict] = createSignal<"ok" | "miss" | null>(null);

  const check = () => {
    if (!value().trim() || verdict()) return;
    setVerdict(eqLoose(value(), props.item.answer) ? "ok" : "miss");
  };
  const next = () => props.onResolve(verdict() === "ok");

  return (
    <div class="ex-view">
      <p class="ex-task">{props.item.task[0]}</p>
      <Show when={props.item.task[1]}>
        <p class="ex-task ex-task-sub">{props.item.task[1]}</p>
      </Show>

      <div class="ex-source">
        <p>{props.item.source}</p>
      </div>

      <AnswerField
        value={value()}
        onInput={setValue}
        onEnter={check}
        disabled={!!verdict()}
      />
      <Tail
        verdict={verdict}
        isRetry={props.isRetry}
        answer={props.item.answer}
        rule={props.item.rule}
        canCheck={() => !!value().trim()}
        check={check}
        next={next}
        onAccent={(c) => setValue((v) => v + c)}
      />
    </div>
  );
}

/* ── Error spot (frame 08) ────────────────────────────────────── */

export function ErrorSpotView(props: ViewProps<ErrorSpotData>) {
  const [picked, setPicked] = createSignal<number | null>(null);
  const [fix, setFix] = createSignal("");
  const [verdict, setVerdict] = createSignal<"ok" | "miss" | null>(null);

  const canCheck = () => picked() !== null && !!fix().trim() && !verdict();
  const check = () => {
    if (!canCheck()) return;
    const ok = picked() === props.item.wrong && eqLoose(fix(), props.item.fix);
    setVerdict(ok ? "ok" : "miss");
  };
  const next = () => props.onResolve(verdict() === "ok");

  return (
    <div class="ex-view">
      <p class="ex-task">{props.item.task[0]}</p>

      <div class="ex-words">
        <For each={props.item.words}>
          {(word, i) => (
            <button
              type="button"
              class="ex-word"
              classList={{ "is-picked": picked() === i() }}
              disabled={!!verdict()}
              onClick={() => !verdict() && setPicked(i())}
            >
              {word}
            </button>
          )}
        </For>
      </div>

      <Show when={picked() !== null}>
        <p class="ex-task ex-task-fix">{props.item.fixTask}</p>
        <AnswerField
          value={fix()}
          onInput={setFix}
          onEnter={check}
          placeholder="the correct form"
          disabled={!!verdict()}
        />
      </Show>

      <Tail
        verdict={verdict}
        isRetry={props.isRetry}
        answer={props.item.fix}
        rule={props.item.rule}
        canCheck={canCheck}
        check={check}
        next={next}
        onAccent={(c) => setFix((v) => v + c)}
      />
    </div>
  );
}

/* ── Dictation (frame 09) ─────────────────────────────────────── */

/** Static waveform bars from the design (indigo family). */
const WAVE: Array<[height: number, color: string]> = [
  [6, "var(--p1)"], [12, "var(--accent)"], [20, "var(--p3)"],
  [14, "var(--accent)"], [8, "var(--p1)"], [18, "var(--p2)"],
  [10, "var(--p1)"],
];

function speakFr(text: string): boolean {
  if (typeof speechSynthesis === "undefined") return false;
  speechSynthesis.cancel();
  const u = new SpeechSynthesisUtterance(text);
  u.lang = "fr-FR";
  u.rate = 0.9;
  const voice = speechSynthesis.getVoices().find((v) => v.lang.startsWith("fr"));
  if (voice) u.voice = voice;
  speechSynthesis.speak(u);
  return true;
}

export function DictationView(props: ViewProps<DictationData>) {
  const [value, setValue] = createSignal("");
  const [verdict, setVerdict] = createSignal<"ok" | "miss" | null>(null);

  const check = () => {
    if (!value().trim() || verdict()) return;
    setVerdict(eqLoose(value(), props.item.answer) ? "ok" : "miss");
  };
  const next = () => props.onResolve(verdict() === "ok");

  const duration = () =>
    `0:0${Math.max(2, Math.ceil(props.item.speech.split(" ").length * 0.5))}`;

  return (
    <div class="ex-view">
      <p class="ex-task">{props.item.task[0]}</p>

      <div class="ex-audio">
        <button
          type="button"
          class="ex-play"
          aria-label="Play the audio"
          title={typeof speechSynthesis === "undefined" ? "Audio unavailable in this browser" : "Play"}
          onClick={() => speakFr(props.item.speech)}
        >
          ▶
        </button>
        <div class="ex-wave" aria-hidden="true">
          <For each={WAVE}>
            {([h, c]) => <i style={{ height: `${h}px`, background: c }} />}
          </For>
        </div>
        <span class="ex-dur">{duration()}</span>
      </div>

      <AnswerField
        value={value()}
        onInput={setValue}
        onEnter={check}
        placeholder="Type what you hear"
        disabled={!!verdict()}
      />
      <Tail
        verdict={verdict}
        isRetry={props.isRetry}
        answer={props.item.answer}
        rule={props.item.rule}
        canCheck={() => !!value().trim()}
        check={check}
        next={next}
        onAccent={(c) => setValue((v) => v + c)}
      />
    </div>
  );
}
