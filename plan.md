# La Coquille — French Grammar Reference Site (v1)

## Context

A public, mobile-first website for learning French grammar. The user is personally frustrated with tense selection/conjugation and finds existing resources boring and flat. Goal: a well-designed reference site with a distinctive *interactive verb tense diagram* as its centerpiece, plus a growing encyclopedic reference for the rest of French grammar (adjectives, pronouns, etc.). Must be fast on mobile and usable by anyone on the Internet without login.

**Design principles (pedagogical):**
- Aspect (ongoing vs completed vs anterior) should be visible, not just named.
- Chunks and real examples beat bare rules.
- Reference pages are the fast lane; visuals are the wow moment.

## Scope (user-confirmed)

- **Layer 1:** Full grammar reference covering verbs, nouns, adjectives, pronouns (all types), articles/determiners, adverbs, prepositions, negation, interrogation, syntax/word order.
- **Centerpiece:** A single interactive verb-tense diagram using the **time-axis + mood-lanes** metaphor.
- **Interactivity v1:** Reference + diagrams only. *No* quizzes, SRS, or accounts in v1. (Add later.)
- **Languages:** English explanations first; French toggle added in a later iteration (design with i18n in mind from day one).
- **Sequencing:** Ship the verb diagram + full verb reference first, then expand to other topics.
- **Tech stack:** **Astro** + **Pico.css** (classless defaults for typography/spacing/dark mode) + **SolidJS islands** for the reactive diagram. SolidJS is the default island framework because its fine-grained reactivity is the sharpest fit for the Tier-1 *reactive parameter* (one selector change → every card re-renders without a full re-run). If during prototyping Solid proves to be overkill for the ~18 node diagram, we fall back to vanilla JS inside an Astro island.
- **Hosting & repo:** GitHub monorepo, deployed to **Cloudflare Pages**. Preview deploys on every branch/PR.
- **Content authoring:** AI-drafted grammar content (tense pages, choice pages, example sentences), reviewed and edited by the user before publishing. I write first-pass Markdown; user corrects accuracy, tone, and any pedagogical emphasis.
- **Conjugation approach:** **Hand-authored rules per model verb**. No conjugation library shipped. Each model verb and each displayed irregular has a small hand-written rule set that produces the 6 person forms for each of the v1 tenses. Data lives in content (not code) so it's reviewable like prose.

## Information architecture

- **`/` (home)** — a clean **topic grid** listing every grammar area (Verbs, Pronouns, Adjectives, Articles & Determiners, Adverbs, Prepositions, Negation, Interrogation, Syntax). Feels like a well-designed reference index. In v1, only the Verbs tile fully lights up; the rest render as "coming soon" placeholders linking to a roadmap note so the site doesn't feel broken.
- **`/verbs`** — the centerpiece. The interactive tense diagram sits at the top of this page. Below it: an index of tense reference pages and the five "which tense?" choice pages.
- **`/verbs/tenses/<tense>`** — one page per tense.
- **`/verbs/choice/<topic>`** — the cross-cutting choice pages.
- Top-of-page navigation is a minimal breadcrumb + topic menu, consistent across every page. No drawer, no modal nav on mobile — just a compact top bar.

## Non-functional requirements

- **Fast first paint on 3G-class mobile.** Each reference page should be pre-rendered HTML, no blocking JS. Interactive widgets (the diagram) are lazy-loaded only on pages that need them.
- **Mobile-friendly.** Touch targets ≥44px, no hover-only interactions, the diagram must degrade gracefully to a horizontally-scrollable view on narrow screens.
- **No login, no tracking wall.** Public content; everything works anonymously.
- **Accessible.** Semantic HTML for all reference content; diagram should expose a textual fallback (list of tenses with relations).
- **i18n-ready.** Content strings separated from markup so the FR toggle can be added without a rewrite.

## Diagram design language ("Grammar Map" kit)

Every interactive diagram on the site — verbs now, pronouns/adjectives/articles/negation/syntax later — must be a different instance of the **same kit**. This turns individual diagrams into a transferable reading habit. The verb diagram in v1 is the **reference implementation** of the kit; every primitive below has to be finalized as we build it, because future topics inherit it.

### Shared primitives
- **Canvas.** Each diagram has one semantic primary axis + one secondary axis (swim lanes). For verbs: *time × mood*. Future topics: articles → *definiteness × specificity*, pronoun placement → *word-order position × pronoun category*, adjectives → *gender × number*, etc. Same visual structure, different axes.
- **Node card.** Shared anatomy across all topics: title, one-line rule, one example sentence, "learn more" link. Identical sizing and padding across topics.
- **Edge vocabulary** (shared palette, reused wherever the semantic fits):
  - **solid** = *built from / composed of* (aux-compound for verbs; composite forms generally)
  - **double** = *duality / contrast pair* (imparfait ↔ PC; masc ↔ fem; definite ↔ indefinite)
  - **dotted** = *shares stem / shares form*
  - **dashed** = *anteriority / sequence*
  - **arrow** = *triggers* (subjunctive triggers; preposition-verb pairings)
- **Aspect/status icon set.** A small shared glyph library: filled dot = completed/definite; wave = ongoing/habitual; dot-before-dot = anterior; empty slot = partitive/uncountable. Same glyph means the same thing across topics.
- **Color system.** Category hues (one hue per mood/category) stay consistent whenever the same category reappears; saturation/lightness encodes recency or focus, not meaning.
- **Gesture set.** Tap = open detail panel. Long-press = open legend. Swipe horizontally = move to sibling topic. Pinch/scroll = zoom on desktop; filter chips replace zoom on mobile.
- **Textual fallback.** Every diagram ships with a structured `<dl>` outline using the same template — for screen readers, no-JS clients, and users who prefer linear text.

### Tier-1 interactive techniques (must exist in every diagram built under this kit, starting with the verb diagram)
1. **Overview → filter → detail.** The diagram opens at full-system view. Filters (e.g. literary tenses off, single mood) compress the view. Tap a node for the detail panel.
2. **Reactive parameter.** Changing one global control live-updates every node simultaneously. For the verb diagram: a subject-pronoun selector (*je / tu / il / nous / vous / ils*) re-conjugates every tense card at once. For future topics: a noun selector re-agrees every adjective; a gender/number toggle re-forms every article.
3. **Linking (brushing).** Example sentences inside detail panels have tappable words — tapping a verb in a sentence highlights the tense node it belongs to; tapping a node highlights every occurrence of that tense in visible example sentences. Bridges *form* and *meaning*.
4. **Animated morph.** Verb endings, adjective agreements, and article choices animate from base form to selected form when opened. Motion encodes the formation rule. Respects `prefers-reduced-motion`.

### Scaffolded disclosure
Training wheels (edge labels visible, legend chip pinned, colored hints) on by default for first-time visitors. After 3 detail-panel opens (tracked in localStorage), labels fade to hover/tap-only. Users can re-pin them from the legend.

## The Verb Tense Diagram (centerpiece)

The verb diagram is the reference implementation of the Grammar Map kit defined above. Canvas axes, card anatomy, edge palette, icons, gestures, and all four Tier-1 interactions must be finished and polished here.


### Layout
Horizontal **time axis**: past ← → future. Vertical **mood lanes**: Indicatif, Conditionnel, Subjonctif, Impératif. Non-finite forms (infinitif, participe présent/passé, gérondif) sit in a side panel, not on the lanes.

### Nodes (tense cards)
- **Indicatif:** plus-que-parfait, imparfait, passé composé, passé simple, présent, futur simple, futur antérieur *(passé simple and passé antérieur hidden behind a "literary tenses" toggle to reduce visual noise on mobile)*
- **Conditionnel:** conditionnel passé, conditionnel présent
- **Subjonctif:** subjonctif plus-que-parfait, subjonctif passé, subjonctif présent, subjonctif imparfait *(imparfait/p-q-p behind the literary toggle)*
- **Impératif:** impératif présent, impératif passé

### Edge types (typed, styled distinctly)
1. **Auxiliary-compound** — every compound tense is wired to the simple tense that provides its auxiliary form. *Présent* → *passé composé* (avoir/être au présent + PP). *Imparfait* → *plus-que-parfait*. *Futur* → *futur antérieur*. *Cond. présent* → *cond. passé*. *Subj. présent* → *subj. passé*. *Subj. imparfait* → *subj. plus-que-parfait*.
2. **Aspect pair** — imparfait ↔ passé composé (ongoing/habitual vs completed). Highlighted because this is the #1 learner pain point.
3. **Stem share** — futur simple ↔ conditionnel présent (same stem, different endings).
4. **Mood swap** — indicatif présent ↔ subjonctif présent (same time, different mood).
5. **Anteriority** — plus-que-parfait → passé composé; futur antérieur → futur; cond. passé → cond. présent.

### Interactions (specialized applications of the Tier-1 kit)
- **Global subject-pronoun selector** (Tier-1: reactive parameter). A persistent control lets you pick *je / tu / il-elle-on / nous / vous / ils-elles*. Every visible tense card re-conjugates live using a model regular verb, plus a small set of high-frequency irregulars the user can toggle (*être, avoir, aller, faire*).
- **Filters** (Tier-1: overview → filter → detail). Literary-tenses toggle (off by default on mobile). Per-mood toggles. "Simple tenses only" preset.
- **Tap a node** → slide-up detail panel with: full conjugation table (all 6 persons), formation rule, *when to use* bullet list, 3 example sentences with translation. Clicking a verb token inside any example sentence highlights the corresponding tense node (Tier-1: linking). Link to the full reference page for that tense.
- **Tap an edge** → mini-explanation of the relation ("*Passé composé* is built by conjugating *avoir* or *être* in the *présent*, then adding the past participle."). Edges are the *why* of the whole site.
- **Opening a detail panel animates the ending** onto the stem (Tier-1: animated morph). Respects `prefers-reduced-motion` — motion becomes a static color underline on the changed suffix.
- **Reduced-motion fallback** respected throughout; no essential info conveyed by animation alone.

### Rendering
SVG-based, one component. On narrow viewports, degrade to a horizontal scroll container with snap-points on each time column. Provide a plain-HTML `<dl>` fallback inside the same component for screen readers and pre-JS paint.

## Verb reference pages (per tense)

Each tense gets a dedicated page:
- **What it is** (one paragraph, plain language)
- **When to use** (bullet list of contexts, with real-world examples)
- **How to form** (rule + two animated conjugation tables: regular model + one key irregular)
- **Common pitfalls** (e.g. for PC: auxiliary choice, agreement with preceding direct object)
- **Cross-references** to related tenses (the edges from the diagram become hyperlinks here)
- **See it in the wild** — 3–5 short authentic sentences from songs/news/film *if copyright allows*, else hand-crafted realistic sentences

Plus cross-cutting pages for the big *choice* problems, linked from multiple tense pages:
- Imparfait vs passé composé
- Subjonctif triggers (a curated list; mark the ones that genuinely require it vs stylistic)
- Si-clauses (the three canonical patterns)
- Futur vs conditionnel (including journalistic conditional)
- Reported speech / sequence of tenses

## Content model

A simple content-first structure (exact shape pending tech choice, but the contract is):

```
content/
  verbs/
    tenses/
      present-indicatif.{md,json}     # prose + structured fields
      passe-compose.{md,json}
      ...
    choice/
      imparfait-vs-passe-compose.md
      subjonctif-triggers.md
      ...
    diagram.json                      # nodes + edges for the mind map
  pronouns/, adjectives/, ...         # scaffolded, filled iteratively post-v1
  i18n/
    en/ ...                           # English strings
    fr/ ...                           # French strings, wired up later
```

Keeping content as flat files (Markdown + small JSON for structured bits like conjugation tables) makes the site pre-renderable regardless of framework.

## Out of scope for v1 (explicit)

- Accounts, progress tracking, SRS, streaks
- Quizzes/drills embedded in pages
- Real-content ingestion (paste-a-YouTube-link analyzer)
- LLM-powered grammar explanations
- Audio/shadowing mode
- French-language UI (ship EN first; FR toggle is a follow-up)

These are all good ideas previously discussed; they return as candidates after v1 ships.

## Roadmap after v1

1. Add remaining reference topics (pronouns → adjectives → articles → negation → interrogation → syntax).
2. Wire up French toggle using the already-separated strings.
3. Add light inline quizzes on reference pages.
4. Mastery tracking via localStorage (still no account).
5. Real-content reader (LLM-assisted) — only if there's appetite.

## Open decisions (safe to resolve while building)

- **Diagram rendering.** Vanilla SVG (likely) vs a minimal layout helper. The graph is small and fixed (~18 nodes), so handwritten SVG coordinates are viable and avoid shipping d3.
- **Model regular verb(s).** One `-er` verb (*parler*) vs all three groups (*parler / finir / vendre*) visible in cards. Starting with *parler* + the four irregulars (*être, avoir, aller, faire*); extend if the diagram has room.
- **Example-sentence tokenization.** For Tier-1 linking, verbs in examples need tagging. Manual Markdown frontmatter for v1 (low volume, high control); automate later only if the catalog grows.
- **Choice-page list.** Current five: PC-vs-imparfait, subjonctif triggers, si-clauses, futur-vs-conditionnel, reported speech. Candidates to add if scope permits: gerund vs infinitive, *dont/que/qui*, *depuis/il y a/pendant*.
- **Copyright path for example sentences.** Hand-authored realistic sentences for v1; curated quotes with attribution can be introduced later.
- **Domain name.** Placeholder `lacoquille.pages.dev` for v1; custom domain whenever.
- **Analytics.** Plausible or Umami (self-host or SaaS) vs none. Default: none until there's a reason.
- **License.** MIT for code; CC-BY-SA for grammar content is the reasonable default.
- **Testing bar.** Minimum: Lighthouse in CI. Unit tests for the conjugation rules (easy, worth it because it's hand-written). Diagram behaviour covered by a small Playwright smoke test.

## Verification (how we'll know v1 works)

- Lighthouse mobile score ≥90 for Performance, Accessibility, Best Practices, SEO on the topic-grid home page and a representative verb page.
- The verb diagram is usable on a 375px-wide viewport (iPhone SE) end-to-end: every node tappable, every edge explainable, literary toggle reachable.
- **All four Tier-1 kit interactions demonstrably work in the verb diagram**: overview→filter→detail (literary toggle + mood filters), reactive parameter (subject-pronoun selector re-conjugates all visible cards in <100ms), linking (tapping a verb in an example sentence highlights its tense node and vice versa), animated morph (ending animates onto the stem when detail opens).
- The Grammar Map kit primitives (canvas, card, edges, icons, gestures, fallback) are documented in a single source file readable as a design reference for future topics.
- Every tense in the diagram deep-links to a fully-written reference page.
- `curl` of any reference page returns fully rendered HTML (SSG/SSR working; content visible without JS).
- All interactive affordances have keyboard equivalents; diagram has an ordered-list fallback readable by screen readers.
- "Back/forward" navigation between diagram and reference pages preserves scroll/zoom state on the diagram.

## Critical files / artefacts to be created (shape, not tech-specific)

Astro project layout (using Astro Content Collections for the grammar corpus):

- `astro.config.mjs` — Astro + Solid integration, Pico.css imported globally.
- `src/styles/global.css` — Pico overrides (scale, mobile spacing) and kit CSS custom properties (colors per mood, edge stroke widths, etc.).
- `src/content/config.ts` — Content Collections schema (tense, choice page, diagram).
- `src/content/verbs/tenses/<tense>.md` — one Markdown per tense (~10–16 files). Frontmatter carries structured fields (lane, time position, aspect icon, linked tokens).
- `src/content/verbs/choice/<topic>.md` — five cross-cutting choice pages.
- `src/content/verbs/diagram.json` — nodes + edges for the verb diagram, in kit-native vocabulary.
- `src/data/conjugations/<verb>.ts` — hand-authored conjugation rules per model/irregular verb (*parler, être, avoir, aller, faire*). Pure TS object exports, unit-testable.
- `src/components/GrammarMap/` (`.tsx`, SolidJS) — reusable primitives: `Canvas`, `NodeCard`, `Edge`, `AspectIcon`, `DetailPanel`, `Legend`, `TextualFallback`. Built *before* the verb diagram composes them.
- `src/components/VerbDiagram.tsx` — thin Solid island composing GrammarMap primitives + the verb-specific subject-pronoun selector.
- `src/components/ConjugationTable.astro` — static table used on every tense reference page; reused inside diagram detail panels.
- `src/pages/index.astro` — topic grid home.
- `src/pages/verbs/index.astro` — Verbs hub with the diagram island at top.
- `src/pages/verbs/tenses/[slug].astro` — tense reference page (static).
- `src/pages/verbs/choice/[slug].astro` — choice page (static).
- `design/grammar-map-kit.md` — canonical spec of shared canvas, card, edges, icons, gestures, and fallback. Living document, updated as we lock kit decisions.
- `.github/workflows/ci.yml` — Lighthouse CI + unit tests on PRs; Cloudflare Pages handles deploys.

## Scope reality check

The Tier-1 interactions are non-trivial — especially *reactive parameter* (a live conjugation engine) and *linking* (bidirectional highlight between example sentences and diagram nodes). Building these as generalized kit primitives adds upfront cost but pays off from the second diagram onward. If v1 scope feels too large in execution, the agreed-upon cut is to ship the verb diagram with Tier-1 interactions polished and **defer additional grammar topics** rather than to ship a diluted verb diagram.
