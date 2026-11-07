# La Coquille

An interactive French grammar reference site. The centerpiece is a single SVG diagram connecting all verb tenses by their semantic relations (auxiliary composition, aspect pairs, stem sharing, mood swaps, anteriority). Built with Astro, SolidJS islands, and a hand-authored CSS design system.

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

> **Known baseline errors (pre-existing, unrelated to your change):**
> `src/components/TypingRace.tsx` (`setErrorSet(new Set())` infers `Set<unknown>`) and
> `src/pages/index.astro` (passes a `hideNav` prop that `Base.astro` does not declare).
> Compare against this list rather than expecting a clean run.

## Project layout

```
src/
  components/
    GrammarMap/types.ts  # Shared diagram node/edge types
    drills/              # Practice drill islands + their shared data and views
    VerbDiagram.tsx      # SolidJS island: the interactive tense diagram
    GrammarDiagram.tsx   # SolidJS island: the generic grammar-topic diagram
    ConjugationTable.astro
  content/
    verbs/
      tenses/            # One .md per tense — prose + structured frontmatter
      choice/            # Tense-choice guide pages (imparfait vs PC, si-clauses, …)
      groups/            # Verb-group reference pages
      movement/          # Movement-verb reference pages
      non-finite/        # Infinitive, gerondif, participles
      diagram.json       # Nodes + edges for the verb diagram (single source of truth)
    grammar/             # Grammar reference pages by topic
  content.config.ts      # Astro Content Collections schema
  data/
    conjugations/        # Hand-authored verb forms (parler, être, avoir, aller, faire, …)
  layouts/
    Base.astro           # Shared layout: nav, breadcrumb, global.css import
  pages/
    index.astro          # Topic grid home
    reference/
      verbs/             # Verb hub, tense pages, choice/groups/movement/non-finite pages
      grammar/           # Grammar topic pages (adjectifs, adverbs, pronouns, …)
    practice/            # Practice hub and the individual drill pages
    stories/             # Article of the day
  styles/
    global.css           # Design tokens + base styles (the live palette)
    drills.css           # Practice drill styles
lacq/                    # Rust (Axum) backend — article of the day API
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
3. The tense appears automatically in the diagram if its `id` matches a node in `diagram.json`, and at `/reference/verbs/tenses/<slug>`.

## Adding a conjugation verb

1. Create `src/data/conjugations/<verb>.ts` following the pattern in `parler.ts`.
2. Export it from `src/data/conjugations/index.ts` and add it to the `VERBS` map.
3. The verb becomes available in the diagram's verb selector immediately.

## Tech stack

| Layer | Choice |
|---|---|
| Framework | [Astro 6](https://astro.build) — static-first, near-zero JS on reference pages |
| Interactive islands | [SolidJS](https://solidjs.com) — fine-grained reactivity for the diagram |
| CSS | Hand-authored CSS with custom-property tokens (`src/styles/global.css`) |
| Diagram | Hand-authored SVG inside the SolidJS island |
| Content | Astro Content Collections (Markdown + JSON) |
| Deploy | Fly.io — one Machine serves the Astro build and the Rust backend |
