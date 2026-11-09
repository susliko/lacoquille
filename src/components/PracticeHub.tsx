import { For } from "solid-js";

/* ── Practice tiles ──────────────────────────────────────────────
   Colour carries meaning in this app (indigo = progress, vermillon
   = review/miss), so the menu itself stays quiet: white cards on
   paper, ink names, one indigo call to action. See global.css. */
const PRACTICE = [
  /* The endless feed: the coach picks a grammar topic per run from the
     learner's level + history, generator families fill it. The
     flagship practice — it leads the menu. */
  { id: "feed",          name: "Endless feed",      sub: "adaptive grammar · always ready", path: "/practice/feed" },

  /* The designed exercise session: six-plus mixed items with the
     ok/miss feedback system. tokens.css design system experience. */
  { id: "session",       name: "Exercise session",  sub: "mixed items · instant feedback", path: "/practice/session" },

  /* The single verb-conjugation entry. Mixes cloze, cue-to-form,
     spot-the-tense, and transformation chains in an endless stream. */
  { id: "conjugations", name: "Conjugations", sub: "endless mixed drills", path: "/practice/conjugations" },

  /* The other implemented practice. */
  { id: "article-of-day", name: "Article of Day",    sub: "daily Maupassant story",  path: "/stories/article-of-the-day" },
  { id: "typing",         name: "Typing Race",       sub: "speed conjugate drills",  path: "/practice/typing" },
];

export default function PracticeHub() {
  return (
    <div class="practice-hub-root">
      <div class="practice-hub-grid">
        <For each={PRACTICE}>
          {(item, i) => (
            <a href={item.path} class={`practice-card practice-card-${i()}`}>
              <div class="poly-card-name">{item.name}</div>
              <div class="poly-card-sub">{item.sub}</div>
              <div class="poly-card-cta">start →</div>
            </a>
          )}
        </For>
      </div>
    </div>
  );
}
