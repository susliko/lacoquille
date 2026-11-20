/**
 * Practice screen — the homepage (handoff 2026-11, §2).
 *
 * One primary action (the hero) plus three subordinate drill rows,
 * then the grammar grid. "Primary action, then navigation, then
 * status" — the grid is motivating but 450px tall, so it goes last.
 *
 * The hero absorbs Endless feed + Exercise session: one entry, both
 * behaviours. Fresh users see "Start practicing", returning users
 * "Continue". No due count, no goal, no streak — deliberate.
 */

import { createSignal, onMount, Show } from "solid-js";
import ProgressGrid from "./progress/ProgressGrid";
import { feedTopicStats } from "./exercise/feedStore";
import { topicSummaries } from "./exercise/store";

const DRILLS = [
  {
    t: "Conjugations",
    s: "one verb at a time, untimed",
    href: "/practice/conjugations",
    tag: "",
  },
  {
    t: "Typing race",
    s: "the same drill, against a clock",
    href: "/practice/typing",
    tag: "",
  },
  {
    t: "Article of the day",
    s: "Maupassant, one story a day",
    href: "/stories/article-of-the-day",
    tag: "New",
  },
];

const Flame = () => (
  <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
    <path d="M12 2s1 3-1.5 5.5S7 12 7 14.5A5 5 0 0 0 17 15c0-3-2-4.5-2-7 0 0-1.5 1-2 2.5C12.5 8 12 5 12 2z" />
  </svg>
);

const Play = () => (
  <svg width="22" height="22" viewBox="0 0 24 24" fill="#fff" aria-hidden="true">
    <path d="M8 5.2v13.6a.6.6 0 0 0 .92.5l10.4-6.8a.6.6 0 0 0 0-1l-10.4-6.8A.6.6 0 0 0 8 5.2z" />
  </svg>
);

const Gear = () => (
  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
    <circle cx="12" cy="12" r="3.2" />
    <path d="M19.4 14.6a1.6 1.6 0 0 0 .3 1.8l.1.1a2 2 0 1 1-2.8 2.8l-.1-.1a1.6 1.6 0 0 0-2.7 1.1v.3a2 2 0 1 1-4 0v-.2a1.6 1.6 0 0 0-2.8-1.1l-.1.1a2 2 0 1 1-2.8-2.8l.1-.1a1.6 1.6 0 0 0-1.1-2.7h-.3a2 2 0 1 1 0-4h.2a1.6 1.6 0 0 0 1.1-2.8l-.1-.1a2 2 0 1 1 2.8-2.8l.1.1a1.6 1.6 0 0 0 1.8.3h.1a1.6 1.6 0 0 0 1-1.5v-.3a2 2 0 1 1 4 0v.2a1.6 1.6 0 0 0 2.7 1.1l.1-.1a2 2 0 1 1 2.8 2.8l-.1.1a1.6 1.6 0 0 0 1.1 2.7h.3a2 2 0 1 1 0 4h-.2a1.6 1.6 0 0 0-1.5 1z" />
  </svg>
);

export default function PracticeHome() {
  // Fresh until both stores hold history (SSR-safe default: fresh).
  const [fresh, setFresh] = createSignal(true);
  onMount(() => {
    const feed = Object.values(feedTopicStats());
    const session = Object.values(topicSummaries());
    setFresh(
      feed.every((t) => !t.attempts) && session.every((s) => !s.attempts),
    );
  });

  return (
    <div class="ph-root">
      <header class="ph-bar">
        <div class="ph-mark">
          La <span>Coquille</span>
        </div>
        <a class="ph-iconbtn" href="/settings/" aria-label="Settings">
          <Gear />
        </a>
      </header>

      {/* The one button */}
      <section class="ph-sec">
        <a class="ph-go" href="/practice/feed">
          <span>
            <span class="ph-eyebrow">
              Grammar
              <Show when={!fresh()}>
                <i class="ph-flame" aria-hidden="true">
                  <Flame />
                </i>
              </Show>
            </span>
            <span class="ph-big">{fresh() ? "Start practicing" : "Continue"}</span>
            <span class="ph-meta">
              {fresh()
                ? "we'll find your level as you go"
                : "mixed review · about 5 min"}
            </span>
          </span>
          <span class="ph-disc" aria-hidden="true">
            <Play />
          </span>
        </a>
      </section>

      {/* Other drills */}
      <section class="ph-sec">
        <div class="ph-sechead">
          <h2>Other drills</h2>
        </div>
        {DRILLS.map((d) => (
          <a class="ph-drill" href={d.href}>
            <span class="ph-tx">
              <span class="ph-tt">{d.t}</span>
              <span class="ph-ss">{d.s}</span>
            </span>
            <Show when={d.tag}>
              <span class="ph-tag">{d.tag}</span>
            </Show>
          </a>
        ))}
      </section>

      {/* Your grammar */}
      <section class="ph-sec ph-sec-grid">
        <div class="ph-sechead">
          <h2>Your grammar</h2>
          <a href="/knowledge/">All topics</a>
        </div>
        <ProgressGrid visible={true} titled={false} />
      </section>
    </div>
  );
}
