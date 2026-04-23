import { createSignal, onMount, For } from "solid-js";

const REGIONS = [
  { id: "adjectifs", name: "Adjectifs", sub: "accord · ordre",       color: "#d97706" },
  { id: "adverbes",  name: "Adverbes",  sub: "formation",            color: "#16a34a" },
  { id: "articles",  name: "Articles",  sub: "défini · partitif",    color: "#059669" },
  { id: "negation",  name: "Négation",  sub: "ne…pas · jamais",     color: "#0891b2" },
  { id: "pronoms",   name: "Pronoms",   sub: "personal · relative", color: "#7c3aed" },
  { id: "preps",     name: "Prép.",     sub: "à · de · en",         color: "#0d9488" },
  { id: "syntaxe",   name: "Syntaxe",   sub: "word order",          color: "#6366f1" },
  { id: "verbs",     name: "Verbes",    sub: "tenses & conjugation", color: "#2563eb" },
];

const BRAND = {
  name: "La Coquille",
  sub: "Grammaire française interactive",
};

const PATHS: Record<string, string> = {
  syntaxe: "/reference/grammar/syntax",
  verbs: "/reference/verbs",
  pronoms: "/reference/grammar/pronouns",
  adjectifs: "/reference/grammar/adjectifs",
  articles: "/reference/grammar/articles",
  negation: "/reference/grammar/negation",
  preps: "/reference/grammar/prepositions",
  adverbes: "/reference/grammar/adverbs",
};

export default function PolygonHero() {
  return (
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
      <a href="/" class="poly-card poly-brand">
        <div class="poly-brand-name">{BRAND.name}</div>
        <div class="poly-brand-sub">{BRAND.sub}</div>
        <div class="poly-brand-hint">Click any topic</div>
      </a>
    </div>
  );
}