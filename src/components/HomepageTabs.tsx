import { createSignal, For } from "solid-js";

/* ── Knowledge tiles (from PolygonHero) ───────────────────────── */
const REGIONS = [
  { id: "adjectifs", name: "Adjectifs",  sub: "accord · ordre",       color: "#d97706" },
  { id: "adverbes",  name: "Adverbes",   sub: "formation",             color: "#16a34a" },
  { id: "articles",  name: "Articles",  sub: "défini · partitif",    color: "#059669" },
  { id: "negation",  name: "Négation",  sub: "ne…pas · jamais",      color: "#0891b2" },
  { id: "pronoms",   name: "Pronoms",    sub: "personal · relative",  color: "#7c3aed" },
  { id: "preps",     name: "Prép.",     sub: "à · de · en",          color: "#0d9488" },
  { id: "syntaxe",   name: "Syntaxe",   sub: "word order",           color: "#6366f1" },
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



/* ── Practice tiles ────────────────────────────────────────────── */
const PRACTICE = [
  { id: "article-of-day", name: "Article of Day", sub: "daily Maupassant story",  color: "#ff4757", path: "/stories/article-of-the-day" },
  { id: "typing",         name: "Typing Race",    sub: "speed conjugate drills",   color: "#ff6b35", path: "/practice/typing" },
  { id: "shadowing",      name: "Shadowing",      sub: "listen & repeat",          color: "#ff9f43", path: "/practice/shadowing" },
  { id: "vocabulary",    name: "Vocabulary Mining", sub: "mine texts for words",   color: "#f7b731", path: "/practice/vocabulary" },
];

/* ── Component ─────────────────────────────────────────────────── */
export default function HomepageTabs() {
  const [tab, setTab] = createSignal<"knowledge" | "practice">("knowledge");

  return (
    <div class="homepage-tabs-root">
      {/* Tab bar */}
      <div class="homepage-tab-bar" role="tablist">
        <button
          role="tab"
          aria-selected={tab() === "knowledge"}
          class="homepage-tab-btn"
          classList={{ active: tab() === "knowledge", inactive: tab() !== "knowledge" }}
          onClick={() => setTab("knowledge")}
          style={{ "font-family": "'DM Serif Display', serif !important" }}
        >
          Knowledge
        </button>
        <button
          role="tab"
          aria-selected={tab() === "practice"}
          class="homepage-tab-btn"
          classList={{ active: tab() === "practice", inactive: tab() !== "practice" }}
          onClick={() => setTab("practice")}
          style={{ "font-family": "'DM Serif Display', serif !important" }}
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
        <div class="practice-grid">
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
