/**
 * Shared SolidJS UI primitives for the four drill components.
 *
 * Each drill (ClozeDrill, CueDrill, TenseDrill, ChainDrill) keeps its own
 * state and its own copy of the per-exercise styling. These primitives are
 * the boring bits — header, chip row, accent input, verdict chip — that
 * would otherwise be duplicated four times.
 *
 * The accent colour is supplied by the parent as a CSS variable on the
 * root container (`.drill-root { --accent: #ffb703 }`). Children use
 * `var(--accent)` for any colour that needs to track the exercise.
 */

import { type JSX, For, Show } from "solid-js";
import type { Verdict } from "./match";

/* ── Drill header (top bar) ─────────────────────────────────────── */

interface DrillHeaderProps {
  title: string;
  /** Short eyebrow label, e.g. "Cloze · 3/8". */
  eyebrow: string;
}

export function DrillHeader(props: DrillHeaderProps) {
  return (
    <header class="drill-bar">
      <a class="drill-back" href="/practice">← practice</a>
      <span class="drill-title">{props.title}</span>
      <span class="drill-spacer" />
      <span class="drill-eyebrow">{props.eyebrow}</span>
    </header>
  );
}

/* ── Progress strip ─────────────────────────────────────────────── */

interface DrillProgressProps {
  /** 0..1 */
  pct: number;
}

export function DrillProgress(props: DrillProgressProps) {
  return (
    <div class="drill-progress">
      <div class="drill-progress-fill" style={{ width: `${Math.round(props.pct * 100)}%` }} />
    </div>
  );
}

/* ── Hint chips (verb · person · tense) ────────────────────────── */

interface ChipRowItem {
  label: string;
  value: string;
}

interface ChipRowProps {
  items: ChipRowItem[];
  /** Tiny punctuation between chips ("·", "×", etc.). */
  separator?: string;
  align?: "start" | "center";
}

export function ChipRow(props: ChipRowProps) {
  const sep = props.separator ?? "·";
  const align = props.align ?? "start";
  return (
    <div class="drill-chips" classList={{ "drill-chips-center": align === "center" }}>
      <For each={props.items}>
        {(item, i) => (
          <>
            <Show when={i() > 0}>
              <span class="drill-chip-sep">{sep}</span>
            </Show>
            <span class="drill-chip">
              <span class="drill-chip-k">{item.label}</span>
              <span class="drill-chip-v">{item.value}</span>
            </span>
          </>
        )}
      </For>
    </div>
  );
}

/* ── Sentence with blank (three-state reveal) ──────────────────── */

interface SentenceClozeProps {
  parts: [string, string];
  /** null = not yet answered. */
  state: Verdict | null;
  /** The revealed answer. */
  answer: string;
  /** Optional stem/ending for colour-coding (indigo stem + coral ending). */
  stem?: string;
  ending?: string;
  /** Optional cue word to highlight in the surrounding sentence. */
  cue?: string;
}

export function SentenceCloze(props: SentenceClozeProps) {
  /** Split the answer into stem + ending for colour-coding. */
  const formatted = () => {
    const { answer, ending } = props;
    if (ending) {
      const idx = answer.toLowerCase().lastIndexOf(ending.toLowerCase());
      if (idx >= 0) {
        return (
          <>
            <span class="drill-blank-stem">{answer.slice(0, idx)}</span>
            <span class="drill-blank-ending">{answer.slice(idx)}</span>
          </>
        );
      }
    }
    return <>{answer}</>;
  };

  const renderSentence = (blank: JSX.Element) => {
    let before = props.parts[0];
    const after = props.parts[1];
    if (props.cue) {
      // Highlight the cue word(s) in the surrounding text.
      const re = new RegExp(`(${escapeRe(props.cue)})`, "g");
      before = before.replace(re, '<mark class="drill-cue">$1</mark>');
    }
    return (
      <p class="drill-sentence">
        <span>{before}</span>
        {blank}
        <span>{after}</span>
      </p>
    );
  };

  const blankClass = () => {
    if (!props.state) return "drill-blank";
    return `drill-blank drill-blank-${props.state}`;
  };

  return (
    <>
      <Show when={!props.state}>{renderSentence(<span class={blankClass()}>?</span>)}</Show>
      <Show when={props.state}>
        {renderSentence(<span class={blankClass()}>{formatted()}</span>)}
      </Show>
    </>
  );
}

function escapeRe(s: string): string {
  return s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/* ── Accent buttons (clickable é, è, ê, à, ù, ç, ô, î, ë) ─────── */

const ACCENTS = ["é", "è", "ê", "à", "ù", "ç", "ô", "î", "ë"] as const;

interface AccentRowProps {
  onPick: (char: string) => void;
  /** Hide when disabled — used during feedback to remove a target. */
  hidden?: boolean;
}

export function AccentRow(props: AccentRowProps) {
  return (
    <Show when={!props.hidden}>
      <div class="drill-accents">
        <For each={ACCENTS}>
          {(c) => (
            <button
              type="button"
              class="drill-accent"
              onMouseDown={(e) => e.preventDefault()}
              onClick={() => props.onPick(c)}
            >
              {c}
            </button>
          )}
        </For>
      </div>
    </Show>
  );
}

/* ── Input with accent integration ─────────────────────────────── */

interface AccentInputProps {
  value: string;
  onInput: (v: string) => void;
  onSubmit: () => void;
  onPickAccent?: (c: string) => void;
  state?: Verdict;
  placeholder?: string;
  big?: boolean;
  disabled?: boolean;
  /** Hidden if the parent wants to render the input inline with other UI. */
  hidden?: boolean;
  /** Override the keyboard hint label ("submit" → "retype"). */
  submitLabel?: string;
  submitKbd?: string;
}

export function AccentInput(props: AccentInputProps) {
  let ref: HTMLInputElement | undefined;

  const insertAccent = (c: string) => {
    if (!ref) return;
    const start = ref.selectionStart ?? ref.value.length;
    const end = ref.selectionEnd ?? ref.value.length;
    const next = ref.value.slice(0, start) + c + ref.value.slice(end);
    props.onInput(next);
    ref.focus();
    requestAnimationFrame(() => {
      if (ref) ref.setSelectionRange(start + 1, start + 1);
    });
  };

  const handleKey = (e: KeyboardEvent) => {
    if (e.key === "Enter") {
      e.preventDefault();
      props.onSubmit();
    }
  };

  return (
    <Show when={!props.hidden}>
      <div class="drill-input-shell">
        <div class="drill-input-row">
          <input
            ref={ref}
            class="drill-input"
            classList={{
              "drill-input-big": !!props.big,
              [`drill-input-${props.state}`]: !!props.state,
            }}
            type="text"
            autocomplete="off"
            autocapitalize="off"
            autocorrect="off"
            spellcheck={false}
            placeholder={props.placeholder ?? "?"}
            value={props.value}
            disabled={props.disabled}
            onInput={(e) => props.onInput(e.currentTarget.value)}
            onKeyDown={handleKey}
          />
          <button
            type="button"
            class="drill-submit"
            onClick={props.onSubmit}
            disabled={props.disabled}
          >
            {props.submitLabel ?? "check"} <span class="drill-kbd">{props.submitKbd ?? "↵"}</span>
          </button>
        </div>
        <AccentRow onPick={insertAccent} hidden={props.disabled} />
      </div>
    </Show>
  );
}

/* ── Verdict chip (correct / almost / wrong) ───────────────────── */

interface VerdictChipProps {
  verdict: Verdict;
  text?: string;
}

export function VerdictChip(props: VerdictChipProps) {
  return (
    <span class={`drill-verdict drill-verdict-${props.verdict}`}>
      {props.text ?? props.verdict}
    </span>
  );
}

/* ── Feedback panel (used by ClozeDrill & ChainDrill) ──────────── */

interface FeedbackPanelProps {
  verdict: Verdict;
  /** Optional one-line note shown above the diff. */
  note?: string;
  /** Learner's typed answer (for wrong/almost diff). */
  tried?: string;
  /** The correct answer. */
  answer: string;
  /** Optional decomposition "<stem> + <ending>". */
  decomp?: { stem: string; ending?: string };
  /** Optional rule (inline HTML). Hidden on correct. */
  rule?: string;
}

export function FeedbackPanel(props: FeedbackPanelProps) {
  return (
    <div class={`drill-feedback drill-feedback-${props.verdict}`}>
      <div class="drill-feedback-verdict">
        {props.verdict === "correct" && "✓ correct"}
        {props.verdict === "almost" && (props.note ?? "almost")}
        {props.verdict === "wrong" && (props.note ?? "not quite")}
      </div>

      <Show when={props.verdict !== "correct" && props.tried}>
        <div class="drill-feedback-diff">
          <span class="drill-feedback-yours">{props.tried}</span>
          <span class="drill-feedback-arrow">→</span>
          <span class="drill-feedback-theirs">{props.answer}</span>
        </div>
      </Show>

      <Show when={props.decomp?.ending}>
        <div class="drill-feedback-decomp">
          <span class="drill-feedback-stem">{props.decomp!.stem}</span>
          <Show when={props.decomp!.ending}>
            <span class="drill-feedback-plus">+</span>
            <span class="drill-feedback-ending">-{props.decomp!.ending}</span>
          </Show>
        </div>
      </Show>

      <Show when={props.verdict !== "correct" && props.rule}>
        <div class="drill-feedback-rule" innerHTML={props.rule} />
      </Show>
    </div>
  );
}

/* ── Summary screen ─────────────────────────────────────────────── */

interface SummaryPanelProps {
  title: string;
  /** A list of stat blocks. */
  stats: Array<{ label: string; value: string | number; color?: string }>;
  note?: string;
  /** Tail of the deck — used by ChainDrill to show the completed chain. */
  trail?: string;
  /** Anchor hrefs for the action buttons. */
  retryHref: string;
  nextHref?: string;
  nextLabel?: string;
  /** Optional children rendered below the action buttons row
   *  (e.g. a "keep going" button after an explicit end-session). */
  children?: JSX.Element;
}

export function SummaryPanel(props: SummaryPanelProps) {
  return (
    <div class="drill-summary">
      <h2 class="drill-summary-title">{props.title}</h2>
      <Show when={props.trail}>
        <div class="drill-summary-trail" innerHTML={props.trail} />
      </Show>
      <div class="drill-summary-stats">
        <For each={props.stats}>
          {(s) => (
            <div class="drill-summary-stat">
              <span class="drill-summary-n" style={s.color ? { color: s.color } : undefined}>
                {s.value}
              </span>
              <span class="drill-summary-l">{s.label}</span>
            </div>
          )}
        </For>
      </div>
      <Show when={props.note}>
        <p class="drill-summary-note" innerHTML={props.note} />
      </Show>
      <div class="drill-summary-row">
        <a class="drill-btn drill-btn-primary" href={props.retryHref}>try again</a>
        <Show when={props.nextHref}>
          <a class="drill-btn drill-btn-ghost" href={props.nextHref}>
            {props.nextLabel ?? "next →"}
          </a>
        </Show>
      </div>
      <Show when={props.children}>{props.children}</Show>
    </div>
  );
}