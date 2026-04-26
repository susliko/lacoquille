import { createSignal, For, onMount } from "solid-js";

/* ── Knowledge tiles ─────────────────────────────────────────── */
const REGIONS = [
  { id: "adjectifs", name: "Adjectifs",  sub: "accord · ordre",      color: "#d97706" },
  { id: "adverbes",  name: "Adverbes",   sub: "formation",            color: "#16a34a" },
  { id: "articles",  name: "Articles",   sub: "défini · partitif",    color: "#059669" },
  { id: "negation",  name: "Négation",  sub: "ne…pas · jamais",      color: "#0891b2" },
  { id: "pronoms",   name: "Pronoms",    sub: "personal · relative",  color: "#7c3aed" },
  { id: "preps",     name: "Prép.",     sub: "à · de · en",          color: "#0d9488" },
  { id: "syntaxe",   name: "Syntaxe",   sub: "word order",            color: "#6366f1" },
  { id: "verbs",     name: "Verbes",     sub: "tenses & conjugation", color: "#2563eb" },
];

const PATHS: Record<string, string> = {
  syntaxe:   "/reference/grammar/syntax",
  verbs:     "/reference/verbs",
  pronoms:   "/reference/grammar/pronouns",
  adjectifs: "/reference/grammar/adjectifs",
  articles:  "/reference/grammar/articles",
  negation:  "/reference/grammar/negation",
  preps:     "/reference/grammar/prepositions",
  adverbes:  "/reference/grammar/adverbs",
};

/* ── Practice tiles ───────────────────────────────────────────── */
const PRACTICE = [
  { id: "typing",    name: "Typing Race",   sub: "speed conjugate drills", color: "#ff6b35", path: "/practice/typing" },
  { id: "shadowing", name: "Shadowing",     sub: "listen & repeat",       color: "#ff9f43", path: "/practice/shadowing" },
  { id: "article",   name: "Article of Day", sub: "daily Maupassant story", color: "#ff4757", path: "/stories/article-of-the-day" },
];

/* ── Component ────────────────────────────────────────────────── */
export default function HomepageTabs() {
  const getTab = (): "knowledge" | "practice" => {
    if (typeof window === "undefined") return "knowledge";
    const p = new URLSearchParams(window.location.search).get("tab");
    return p === "practice" ? "practice" : "knowledge";
  };

  const [tab, setTab] = createSignal<"knowledge" | "practice">(getTab());

  onMount(() => {
    const handler = () => setTab(getTab());
    window.addEventListener("popstate", handler);
    return () => window.removeEventListener("popstate", handler);
  });

  const switchTab = (t: "knowledge" | "practice") => {
    setTab(t);
    const url = new URL(window.location.href);
    if (t === "knowledge") url.searchParams.delete("tab");
    else url.searchParams.set("tab", "practice");
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
              <a
                href={PATHS[region.id]}
                class={`poly-card poly-card-${i()}`}
                style={{ "--card-color": region.color }}
              >
                <div class="poly-card-name">{region.name}</div>
                <div class="poly-card-sub">{region.sub}</div>
                <div class="poly-card-cta">explore →</div>
              </a>
            )}
          </For>
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
              <a
                href={item.path}
                class={`poly-card poly-card-${i()}`}
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
    </div>
  );
}
