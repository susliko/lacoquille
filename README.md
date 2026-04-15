# Frenchlation

An interactive French grammar reference site. The centerpiece is a single SVG diagram connecting all verb tenses by their semantic relations (auxiliary composition, aspect pairs, stem sharing, mood swaps, anteriority). Built with Astro, SolidJS islands, and Pico.css.

## Prerequisites

- **Node.js 22+** — the project requires Node 22. If you use nvm:
  ```sh
  nvm install 22
  nvm alias default 22   # make 22 the default so you don't have to switch each time
  ```

## Setup

```sh
npm install
```

## Development

```sh
npm run dev
```

Opens at `http://localhost:4321/`. Pages hot-reload on save. The SolidJS diagram island reloads without losing scroll position.

## Build & preview

```sh
npm run build       # outputs to ./dist/
npm run preview     # serves ./dist/ locally to verify before deploy
```

## Type-check

```sh
npx astro check
```

Runs Astro's TypeScript checker across `.astro`, `.tsx`, and `.ts` files. Run this before committing if you touched components or content schemas.

## Project layout

```
src/
  components/
    GrammarMap/         # Shared kit primitives (types, AspectIcon) — reused by all topic diagrams
    VerbDiagram.tsx     # SolidJS island: the interactive tense diagram
    ConjugationTable.astro
  content/
    verbs/
      tenses/           # One .md per tense — prose + structured frontmatter
      choice/           # Tense-choice guide pages (imparfait vs PC, si-clauses, …)
      diagram.json      # Nodes + edges for the verb diagram (single source of truth)
  content.config.ts     # Astro Content Collections schema
  data/
    conjugations/       # Hand-authored verb forms (parler, être, avoir, aller, faire)
  layouts/
    Base.astro          # Shared layout: nav, breadcrumb, Pico.css import
  pages/
    index.astro         # Topic grid home
    verbs/
      index.astro       # Verb hub — diagram island + tense index
      tenses/[slug].astro
      choice/[slug].astro
  styles/
    global.css          # Pico.css import + Grammar Map kit CSS custom properties
design/
  grammar-map-kit.md    # Living spec for the shared diagram design language
plan.md                 # Project plan (mirrors ~/.claude/plans/…)
```

## Adding a tense page

1. Create `src/content/verbs/tenses/<slug>.md` with the required frontmatter:
   ```yaml
   ---
   title: "Imparfait"
   mood: indicatif          # indicatif | conditionnel | subjonctif | imperatif
   timePosition: past       # far-past | past | near-past | present | near-future | future | far-future
   aspect: habitual         # completed | ongoing | anterior | habitual | none
   literary: false
   oneLineRule: "Ongoing, habitual, or descriptive past actions."
   examples:
     - fr: "Il <v data-tense=\"imparfait\">parlait</v> doucement."
       en: "He was speaking softly."
   relatedTenses:
     - passe-compose
   ---
   ```
2. Write the body in Markdown (What it is / When to use / How to form / Pitfalls).
3. The tense appears automatically in the diagram if its `id` matches a node in `diagram.json`, and at `/verbs/tenses/<slug>`.

## Adding a conjugation verb

1. Create `src/data/conjugations/<verb>.ts` following the pattern in `parler.ts`.
2. Export it from `src/data/conjugations/index.ts` and add it to the `VERBS` map.
3. The verb becomes available in the diagram's verb selector immediately.

## Tech stack

| Layer | Choice |
|---|---|
| Framework | [Astro 5](https://astro.build) — static-first, near-zero JS on reference pages |
| Interactive islands | [SolidJS](https://solidjs.com) — fine-grained reactivity for the diagram |
| CSS | [Pico.css](https://picocss.com) — classless, dark-mode-aware, mobile-first |
| Diagram | Hand-authored SVG inside the SolidJS island |
| Content | Astro Content Collections (Markdown + JSON) |
| Deploy | Cloudflare Pages (CI via GitHub Actions) |
