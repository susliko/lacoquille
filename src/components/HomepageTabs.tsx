import { createSignal, For, onMount } from "solid-js";

import ProgressGrid from "./progress/ProgressGrid";

/* ── Knowledge is a link, not a tab: the button goes straight to
   /knowledge/ (the grammar manual). Only Practice / Progress are
   in-page tabs; Practice is the default view. */

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
type Tab = "practice" | "progress";

export default function HomepageTabs() {
  const getTab = (): Tab => {
    if (typeof window === "undefined") return "practice";
    const p = new URLSearchParams(window.location.search).get("tab");
    return p === "progress" ? "progress" : "practice";
  };

  // Start at the SSR-safe default, then sync from the URL once mounted.
  // Initializing straight from the URL leaves hydration mismatched (the
  // panel switches but the buttons keep their server-rendered state).
  const [tab, setTab] = createSignal<Tab>("practice");

  onMount(() => {
    setTab(getTab());
    const handler = () => setTab(getTab());
    window.addEventListener("popstate", handler);
    return () => window.removeEventListener("popstate", handler);
  });

  const switchTab = (t: Tab) => {
    setTab(t);
    const url = new URL(window.location.href);
    if (t === "practice") url.searchParams.delete("tab");
    else url.searchParams.set("tab", t);
    history.replaceState(null, "", url.toString());
  };

  return (
    <div class="homepage-tabs-root">
      {/* Tab bar — Knowledge navigates to the manual; not a panel */}
      <div class="homepage-tab-bar" role="tablist">
        <a href="/knowledge/" class="homepage-tab-btn homepage-tab-link">
          Knowledge
        </a>
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

      {/* Practice tab (default) */}
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
