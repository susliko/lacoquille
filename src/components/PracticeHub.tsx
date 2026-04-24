import { For } from "solid-js";

/* ── Practice tiles ────────────────────────────────────────────── */
const PRACTICE = [
  { id: "article-of-day", name: "Article of Day",  sub: "daily Maupassant story",  color: "#ff4757", path: "/stories/article-of-the-day" },
  { id: "typing",         name: "Typing Race",    sub: "speed conjugate drills",   color: "#ff6b35", path: "/practice/typing" },
  { id: "shadowing",      name: "Shadowing",      sub: "listen & repeat",          color: "#ff9f43", path: "/practice/shadowing" },
  { id: "vocabulary",     name: "Vocabulary Mining", sub: "mine texts for words", color: "#f7b731", path: "/practice/vocabulary" },
];

export default function PracticeHub() {
  return (
    <div class="practice-hub-root">
      <div class="practice-hub-grid">
        <For each={PRACTICE}>
          {(item, i) => (
            <a
              href={item.path}
              class={`practice-card practice-card-${i()}`}
              style={{ "--card-color": item.color }}
            >
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
