/**
 * Shared UI primitives for the exercise session — the pieces every
 * exercise view needs, styled by session.css on the design tokens
 * (Penpot frames 02–10).
 */

import { createEffect, For, Show, type JSX } from "solid-js";

/* ── Session pips (progress strip, frame 02) ──────────────────── */

export type PipState = "" | "current" | "ok" | "miss" | "retried";

export function Pips(props: { total: number; stateFor: (i: number) => PipState }) {
  return (
    <div class="ex-pips" role="progressbar" aria-label="Session progress">
      <For each={Array.from({ length: props.total })}>
        {(_, i) => {
          const state = () => props.stateFor(i());
          return <span class="ex-pip" classList={{ [`is-${state()}`]: state() !== "" }} />;
        }}
      </For>
    </div>
  );
}

/* ── Topic chip (level badge + topic, frame 02) ───────────────── */

export function TopicChip(props: { level: string; topic: string }) {
  return (
    <div class="ex-chip">
      <span class="ex-chip-badge">{props.level}</span>
      <span class="ex-chip-topic">{props.topic}</span>
    </div>
  );
}

/* ── Sentence gap (three states: empty / answered) ────────────── */

export function Gap(props: { answer: string; filled: boolean }) {
  return (
    <span class="ex-gap" classList={{ "is-empty": !props.filled }}>
      {props.filled ? props.answer : "?"}
    </span>
  );
}

/* ── Answer field ─────────────────────────────────────────────── */

export function AnswerField(props: {
  value: string;
  onInput: (v: string) => void;
  /** Enter key inside the field. */
  onEnter?: () => void;
  placeholder?: string;
  disabled?: boolean;
}) {
  return (
    <input
      class="ex-field"
      type="text"
      autocomplete="off"
      autocapitalize="off"
      autocorrect="off"
      spellcheck={false}
      enterkeyhint="go"
      placeholder={props.placeholder ?? "Your answer"}
      value={props.value}
      disabled={props.disabled}
      autofocus
      onInput={(e) => props.onInput(e.currentTarget.value)}
      onKeyDown={(e) => {
        if (e.key === "Enter" && props.onEnter) {
          e.preventDefault();
          props.onEnter();
        }
      }}
    />
  );
}

/* ── Accent keys (frame 02: é è ê à â ç î ô û ù) ──────────────── */

const ACCENTS = ["é", "è", "ê", "à", "â", "ç", "î", "ô", "û", "ù"] as const;

export function AccentKeys(props: { onPick: (c: string) => void }) {
  return (
    <div class="ex-accents">
      <For each={ACCENTS}>
        {(c) => (
          <button
            type="button"
            class="ex-accent"
            onMouseDown={(e) => e.preventDefault()}
            onClick={() => props.onPick(c)}
          >
            {c}
          </button>
        )}
      </For>
    </div>
  );
}

/* ── Foot: accent keys + the Check / Next button ──────────────── */

export type FootButton =
  | { label: "Check"; kind: "check" | "disabled"; onClick: () => void }
  | { label: "Next"; kind: "next-ok" | "next-miss"; onClick: () => void };

export function ExerciseFoot(props: {
  /** Omitted while showing feedback — the design hides the keys there. */
  accents?: { onPick: (c: string) => void };
  button: FootButton;
}) {
  return (
    <div class="ex-foot">
      <Show when={props.accents}>
        <AccentKeys onPick={props.accents!.onPick} />
      </Show>
      <button
        type="button"
        class="ex-btn"
        classList={{ "is-ok": props.button.kind === "next-ok", "is-disabled": props.button.kind === "disabled" }}
        disabled={props.button.kind === "disabled"}
        onClick={() => props.button.onClick()}
      >
        {props.button.label}
      </button>
    </div>
  );
}

/* ── Feedback panel (frames 03 & 04) ──────────────────────────── */

export type Verdict = "ok" | "miss";

export function FeedbackPanel(props: {
  verdict: Verdict;
  /** The correct answer, shown on a miss. */
  answer?: string;
  rule: string[];
  note: string;
}) {
  return (
    <div class="ex-feedback" classList={{ [`is-${props.verdict}`]: true }}>
      <div class="ex-feedback-title">
        {props.verdict === "ok" ? "✓ Correct" : "× Not quite"}
      </div>
      <Show when={props.verdict === "miss" && props.answer}>
        <div class="ex-feedback-answer">{props.answer}</div>
      </Show>
      <div class="ex-feedback-rule">
        <For each={props.rule}>{(line) => <p>{line}</p>}</For>
      </div>
      <div class="ex-feedback-note">{props.note}</div>
    </div>
  );
}

/** Scheduling copy under the rule (frames 03/04). */
export function noteFor(verdict: Verdict, isRetry: boolean): string {
  if (!isRetry) {
    return verdict === "ok"
      ? "Scheduled again in 4 days"
      : "Back in this session in a few minutes";
  }
  return verdict === "ok"
    ? "Good recovery — back tomorrow"
    : "It will come back tomorrow";
}

/* ── Keyboard: Enter → continue while in feedback ─────────────── */

export function EnterToContinue(props: { enabled: boolean; onContinue: () => void }) {
  createEffect(() => {
    if (!props.enabled) return;
    const handler = (e: KeyboardEvent) => {
      if (e.key === "Enter" && !(e.target as HTMLElement)?.matches("input, button, textarea, select")) {
        e.preventDefault();
        props.onContinue();
      }
    };
    document.addEventListener("keydown", handler);
    return () => document.removeEventListener("keydown", handler);
  });
  return null as JSX.Element;
}

/* ── Keyboard: keep the shell above the soft keyboard ─────────── */

/**
 * Keep the shell correctly sized against the soft keyboard and publish
 * its height as `--shell` (px) on the exercise root.
 *
 * - Keyboard: WebViews that honour `interactive-widget=resizes-content`
 *   shrink the layout viewport themselves (delta ≈ 0, a no-op); the rest
 *   keep `100dvh` fixed while the keyboard overlays the page — there the
 *   shell must shrink by the keyboard height or the input and the Check
 *   button end up underneath it.
 * - Broken surfaces: some embedded WebViews report `innerHeight`/dvh as 0
 *   until (and sometimes long after) their surface is measured, which
 *   collapses the shell into a crammed pile. When every viewport metric
 *   is 0, fall back to the physical screen — the only source that never
 *   lies. Real browsers always have working metrics and never hit it.
 */
export function trackKeyboard(root: HTMLElement): () => void {
  const vv = window.visualViewport;
  if (!vv) return () => {};
  const update = () => {
    const kb = Math.max(0, Math.round(window.innerHeight - vv.height - vv.offsetTop));
    root.style.setProperty("--kb", `${kb}px`);
    let h = Math.max(window.innerHeight, document.documentElement.clientHeight, Math.round(vv.height));
    if (h <= 0 && window.screen?.height) {
      const dpr = window.devicePixelRatio || 1;
      h = Math.round(window.screen.height / dpr);
    }
    if (h > 0) root.style.setProperty("--shell", `${h - kb}px`);
  };
  update();
  vv.addEventListener("resize", update);
  vv.addEventListener("scroll", update);
  window.addEventListener("resize", update);
  window.addEventListener("orientationchange", update);
  return () => {
    vv.removeEventListener("resize", update);
    vv.removeEventListener("scroll", update);
    window.removeEventListener("resize", update);
    window.removeEventListener("orientationchange", update);
  };
}
