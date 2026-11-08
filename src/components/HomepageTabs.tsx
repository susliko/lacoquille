import { createSignal, For, onMount } from "solid-js";

import ProgressGrid from "./progress/ProgressGrid";

/* ── Knowledge tiles ───────────────────────────────────────────
   Quiet paper menu — colour is meaning-reserved in this design
   system (indigo = progress, vermillon = review/miss), so tiles
   are white cards with ink names. See global.css. */
const REGIONS = [
  { id: "adjectifs", name: "Adjectifs",  sub: "accord · ordre",      path: "/reference/grammar/adjectifs" },
  { id: "adverbes",  name: "Adverbes",   sub: "formation",           path: "/reference/grammar/adverbs" },
  { id: "articles",  name: "Articles",   sub: "défini · partitif",   path: "/reference/grammar/articles" },
  { id: "negation",  name: "Négation",   sub: "ne…pas · jamais",     path: "/reference/grammar/negation" },
  { id: "pronoms",   name: "Pronoms",    sub: "personal · relative", path: "/reference/grammar/pronouns" },
  { id: "preps",     name: "Prép.",      sub: "à · de · en",         path: "/reference/grammar/prepositions" },
  { id: "syntaxe",   name: "Syntaxe",    sub: "word order",          path: "/reference/grammar/syntax" },
  { id: "verbs",     name: "Verbes",     sub: "tenses & conjugation", path: "/reference/verbs" },
];

/* ── Practice tiles ───────────────────────────────────────────── */
const PRACTICE = [
  { id: "article",    name: "Article of Day",    sub: "daily Maupassant story", path: "/stories/article-of-the-day" },
  { id: "typing",     name: "Typing Race",       sub: "speed conjugate drills", path: "/practice/typing" },
  { id: "shadowing",  name: "Shadowing",         sub: "listen & repeat",        path: "/practice/shadowing" },
  { id: "vocabulary", name: "Vocabulary Mining", sub: "mine texts for words",   path: "/practice/vocabulary" },
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

      {/* Knowledge tab */}
      <div
        role="tabpanel"
        class="homepage-tab-panel"
        style={{ display: tab() === "knowledge" ? "block" : "none" }}
      >
        <div class="poly-grid">
          <For each={REGIONS}>
            {(region, i) => (
              <a href={region.path} class={`poly-card poly-card-${i()}`}>
                <div class="poly-card-name">{region.name}</div>
                <div class="poly-card-sub">{region.sub}</div>
                <div class="poly-card-cta">explore →</div>
              </a>
            )}
          </For>
        </div>
        <p style="margin-top: var(--s-3); text-align: center;">
          <a href="/reference/grammar" style="font-size: var(--t-sm); color: var(--muted);">
            See all grammar →
          </a>
        </p>
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
