import { createSignal, For, onMount } from "solid-js";

import ProgressGrid from "./progress/ProgressGrid";
import taxonomy from "../data/knowledge/taxonomy.json";

/* ── Knowledge tab — Le Manuel entry ───────────────────────
   The manual itself lives at /knowledge/; the tab is a quiet
   level list into it. No tiles (retired design). */
const LEVELS = ["A1", "A2", "B1", "B2", "C1", "C2"];
const LEVEL_ROWS = LEVELS.map((lv) => ({
  lv,
  count: (taxonomy as any[]).filter((e) => e.lv === lv).length,
}));

/* ── Practice tiles ─────────────────────────────────────────────
   Implemented practices only — the feed is the flagship surface
   and leads the tab. Shadowing (feature-flagged off) and
   Vocabulary Mining (coming soon) stay out until they exist. */
const PRACTICE = [
  { id: "feed",         name: "Endless feed",     sub: "adaptive grammar · always ready", path: "/practice/feed" },
  { id: "session",      name: "Exercise session", sub: "mixed items · instant feedback",  path: "/practice/session" },
  { id: "conjugations", name: "Conjugations",     sub: "endless mixed drills",            path: "/practice/conjugations" },
  { id: "article",      name: "Article of Day",   sub: "daily Maupassant story",          path: "/stories/article-of-the-day" },
  { id: "typing",       name: "Typing Race",      sub: "speed conjugate drills",          path: "/practice/typing" },
];

/* ── Component ────────────────────────────────────────────────── */
type Tab = "knowledge" | "practice" | "progress";

export default function HomepageTabs() {
  const getTab = (): Tab => {
    if (typeof window === "undefined") return "knowledge";
    const p = new URLSearchParams(window.location.search).get("tab");
    return p === "practice" || p === "progress" ? p : "knowledge";
  };

  // Start at the SSR-safe default, then sync from the URL once mounted.
  // Initializing straight from the URL leaves hydration mismatched (the
  // panel switches but the buttons keep their server-rendered state).
  const [tab, setTab] = createSignal<Tab>("knowledge");

  onMount(() => {
    setTab(getTab());
    const handler = () => setTab(getTab());
    window.addEventListener("popstate", handler);
    return () => window.removeEventListener("popstate", handler);
  });

  const switchTab = (t: Tab) => {
    setTab(t);
    const url = new URL(window.location.href);
    if (t === "knowledge") url.searchParams.delete("tab");
    else url.searchParams.set("tab", t);
    history.replaceState(null, "", url.toString());
  };

  return (
    <div class="homepage-tabs-root">
      {/* Tab bar */}
      <div class="homepage-tab-bar" role="tablist">
        <button
          role="tab"
          aria-selected={tab() === "knowledge"}
          class={`homepage-tab-btn ${tab() === "knowledge" ? "homepage-tab-active" : "homepage-tab-inactive"}`}
          onClick={() => switchTab("knowledge")}
        >
          Knowledge
        </button>
        <button
          role="tab"
          aria-selected={tab() === "practice"}
          class={`homepage-tab-btn ${tab() === "practice" ? "homepage-tab-active" : "homepage-tab-inactive"}`}
          onClick={() => switchTab("practice")}
        >
          Practice
        </button>
        <button
          role="tab"
          aria-selected={tab() === "progress"}
          class={`homepage-tab-btn ${tab() === "progress" ? "homepage-tab-active" : "homepage-tab-inactive"}`}
          onClick={() => switchTab("progress")}
        >
          Progress
        </button>
      </div>

      {/* Knowledge tab — Le Manuel entry, quiet level list */}
      <div
        role="tabpanel"
        class="homepage-tab-panel"
        style={{ display: tab() === "knowledge" ? "block" : "none" }}
      >
        <div class="km-entry">
          <a href="/knowledge/" class="km-entry-open">
            Open the manual
            <span class="km-entry-sub">every lesson, A1–C2 — searchable</span>
          </a>
          <div class="km-entry-levels">
            <For each={LEVEL_ROWS}>
              {(row) => (
                <a href={`/knowledge/?l=${row.lv}`} class="km-entry-level">
                  <span class="lv">{row.lv}</span>
                  <span class="n">{row.count}</span>
                </a>
              )}
            </For>
          </div>
          <a href="/knowledge/verbs/" class="km-entry-verbs">Verb tables →</a>
        </div>
      </div>

      {/* Practice tab */}
      <div
        role="tabpanel"
        class="homepage-tab-panel"
        style={{ display: tab() === "practice" ? "block" : "none" }}
      >
        <div class="poly-grid">
          <For each={PRACTICE}>
            {(item, i) => (
              <a href={item.path} class={`poly-card poly-card-${i()}`}>
                <div class="poly-card-name">{item.name}</div>
                <div class="poly-card-sub">{item.sub}</div>
                <div class="poly-card-cta">start →</div>
              </a>
            )}
          </For>
        </div>
      </div>

      {/* Progress tab */}
      <div
        role="tabpanel"
        class="homepage-tab-panel"
        style={{ display: tab() === "progress" ? "block" : "none" }}
      >
        <ProgressGrid visible={tab() === "progress"} />
      </div>
    </div>
  );
}
