# La Coquille — Design System

**La Coquille** is a free, immersive French language learning website. The centerpiece is an interactive verb-tense diagram that connects all tenses by semantic relations (aspect pairs, auxiliary composition, stem sharing, mood swaps, anteriority). The vision is bold: not another boring drill site, but a living, visual, reference-quality experience — completely free, forever.

**Source repository:** https://github.com/susliko/lacoquille  
**Stack:** Astro 5 · SolidJS islands · Custom CSS (migrated away from Pico.css)  
**Deploy:** Cloudflare Pages

---

## Product Overview

There is one product surface: a **web application / reference site**. Key pages:

| Route | Description |
|---|---|
| `/` | Topic grid — all grammar areas (Verbs live; rest "coming soon") |
| `/verbs` | Verb hub — the interactive Grammar Map diagram + tense index |
| `/verbs/tenses/[slug]` | Per-tense reference page |
| `/verbs/choice/[slug]` | Cross-cutting tense-choice guides (imparfait vs PC, etc.) |

The **Grammar Map** is a shared kit of diagram primitives (canvas, node cards, edges, aspect icons) reused across all grammar topics. The verb diagram is the reference implementation.

---

## Content Fundamentals

**Voice & Tone**
- Second-person, direct, warm but not condescending: "When to use" not "You should use"
- Written in English first (French toggle planned as a later iteration)
- Assumes an intermediate learner — no hand-holding, but jargon is always explained on first use
- Concise and precise: one-line rules ("Ongoing, habitual, or descriptive past actions.") before prose explanation
- No marketing language, no hype — the quality of the content speaks for itself

**Casing**
- Tense names: French typographic conventions — lowercase within sentences (*le passé composé*)
- UI labels (nav, tags, controls): uppercase spaced tracking for macro-level labels; sentence case for body text
- Category tags (e.g. VERBS, INDICATIF): uppercase, tracked, small — used as eyebrow/badge labels

**Examples**
- French sentences use `<v data-tense="...">` markup to tokenize verb forms for interactive linking
- English translation follows immediately, lighter style
- Example: *Il parlait doucement.* → "He was speaking softly."

**Emoji & Symbols**
- No emoji used anywhere in the UI
- Unicode used sparingly: `→` for arrows, `/` as breadcrumb separator, `+`/`−` for disclosure triangles
- Aspect icons are custom SVG glyphs (filled dot = completed, wave = ongoing/habitual, two dots = anterior)

**Vibe**
- Scholarly but alive. The site feels like a beautifully typeset reference book that happens to be interactive.
- The gold accent is the only warm note in an otherwise cool, dark palette — use it to signal what matters.

---

## Visual Foundations

### Colors
Dark-only color scheme. No light mode.

| Token | Value | Usage |
|---|---|---|
| `--bg` | `#0d1117` | Page background |
| `--surface` | `#131923` | Card / panel fills |
| `--surface-2` | `#192030` | Hover state fills, control backgrounds |
| `--surface-3` | `#1e2840` | Deep inset elements (code bg, active table rows) |
| `--border` | `#243048` | Standard border |
| `--border-subtle` | `#1a2438` | Low-emphasis dividers |
| `--border-bright` | `#3a5080` | Hover/focus borders |
| `--text` | `#ddd8cf` | Primary text — warm off-white |
| `--text-2` | `#7a8a9d` | Secondary text, labels, placeholders |
| `--text-muted` | `#374455` | Very muted — icons, separators |
| `--gold` | `#c4913a` | Primary accent: brand name, active states, highlights |
| `--gold-soft` | `rgba(196,145,58,0.15)` | Checked toggle fills |
| `--gold-glow` | `rgba(196,145,58,0.25)` | Glow effects |

**Mood palette** (Grammar Map diagram):

| Token | Value | Mood |
|---|---|---|
| `--indicatif` | `#4d8eff` | Indicatif — blue |
| `--conditionnel` | `#9d6cf0` | Conditionnel — purple |
| `--subjonctif` | `#f0a030` | Subjonctif — amber |
| `--imperatif` | `#2dbe96` | Impératif — teal |

**Edge palette** (Grammar Map diagram):

| Token | Value | Edge type |
|---|---|---|
| `--edge-auxiliary` | `#4d8eff` | Auxiliary-compound — built from |
| `--edge-aspect` | `#f97316` | Aspect pair — ongoing vs completed |
| `--edge-stem` | `#c4913a` | Stem share — same infinitive stem |
| `--edge-mood` | `#9d6cf0` | Mood swap — same time, different mood |
| `--edge-anteriority` | `#6b8ba4` | Anteriority — temporal sequence |

### Typography
Three font families, each with a distinct role:

| Role | Family | Weights | Usage |
|---|---|---|---|
| Display | Cormorant Garamond | 400, 500, 600 (+ italics) | All headings h1–h4, brand name, panel titles, one-line rules |
| Body | DM Sans | 300, 400, 500 | All body copy, navigation, UI labels |
| Mono | JetBrains Mono | 400, 500 | Code, verb forms, conjugation tables, example sentences |

**Scale:**
- `h1`: `clamp(2rem, 5vw, 3rem)` — weight 500
- `h2`: `clamp(1.3rem, 3vw, 1.7rem)` — weight 500
- `h3`: `1.2rem` — weight 500
- Body: `1rem / 1.65` — weight 300
- Small labels: `0.75–0.875rem`
- Eyebrow tags: `0.7–0.75rem`, uppercase, tracked `0.08–0.15em`

**Letter spacing:** slight negative on headings (`-0.01em`); positive on uppercase labels (`0.05–0.15em`).

### Spacing & Layout
- Container max-width: `1200px`, padding `0 1.5rem`
- Main padding: `2.5rem 1.5rem 4rem`
- Consistent gap increments: `0.25`, `0.5`, `0.75`, `1`, `1.25`, `1.5`, `2`, `2.5rem`
- Grid gaps use `1px` for tight ruled grid layouts (topic grid, tense lists)

### Border Radius
- Standard: `8px` (`--radius`)
- Small: `4px` (`--radius-sm`)
- Pill: `999px` (tags, toggles, related-tense links)

### Shadows & Elevation
No box-shadow system. Elevation is expressed through background-color stepping (`--bg` → `--surface` → `--surface-2` → `--surface-3`). Borders are the primary separation mechanism.

### Backgrounds
- Solid dark fills only — no gradients, no textures, no images in UI
- The hero section uses a ghost watermark: giant display-font text at `rgba(255,255,255,0.025)` behind the hero content
- Nav uses `backdrop-filter: blur(12px)` over `rgba(13,17,23,0.92)` — the only use of blur

### Animation
- Duration: `0.18s ease` (`--transition`) for micro-interactions (hover, color shifts)
- Panel entry: `panel-in` keyframe — `opacity 0 → 1`, `translateY(6px → 0)`, same `0.18s ease`
- Diagram nodes: `brightness(1.15)` on hover via CSS filter
- `prefers-reduced-motion`: all motion must degrade to static color underline
- No bounces, springs, or dramatic easing — everything is subtle and fast

### Hover & Press States
- **Links/nav items:** color shifts to `--text` (from `--text-2`), background to `--surface-2`
- **Active nav links:** color `--gold`
- **Topic cards:** background `--surface-2`, title color `--gold`
- **Buttons/pills with active state:** `--gold` background, `--bg` text
- **Bordered interactive elements (tags, back-links):** border shifts to `--gold` on hover, text to `--gold`
- No scale/shrink press states; no opacity fades

### Cards
- Background: `--surface`
- Border: `1px solid var(--border-subtle)`
- Radius: `8px`
- No drop shadow
- Hover: background → `--surface-2`
- Panel cards (node/edge detail): `animation: panel-in`, close button top-right

### Navigation
- Sticky, height `52px`, `z-index 100`
- Background: `rgba(13,17,23,0.92)` + `backdrop-filter: blur(12px)`
- Brand: "LA COQUILLE" in `--font-display`, gold, `1.15rem`, `letter-spacing: 0.08em`, uppercase
- Links: `0.8rem` DM Sans, `--text-2`, hover → `--text`; active → `--gold`
- Breadcrumb inline with nav; mobile: hamburger drawer replaces nav links

### Iconography
- No icon font or icon library
- All icons are inline SVG or unicode characters
- Aspect icons: custom SVG glyphs (see `AspectIcon.tsx`) — filled dot, wave line, two dots
- Arrow `→` used inline for card arrows, drawer items

---

## Iconography

La Coquille uses **no external icon library**. Icons are:
1. **Aspect glyphs** — custom SVG, defined in `src/components/GrammarMap/AspectIcon.tsx`. Four variants: `completed` (filled circle), `ongoing`/`habitual` (wave path), `anterior` (two circles). These are the design system's only custom icons and are central to the Grammar Map kit.
2. **Unicode symbols** — `→`, `/`, `+`, `−` used as inline text characters
3. **No PNG icons, no icon fonts, no emoji**

**Substitution note:** The favicon is an SVG (`public/favicon.svg`). No other image assets exist in the repository — the site is intentionally asset-free, relying on typography and color for visual richness.

---

## File Index

```
README.md                    ← this file
SKILL.md                     ← skill definition for Claude agents
colors_and_type.css          ← all CSS custom properties + base styles
assets/
  favicon.svg                ← site favicon (SVG)
preview/
  colors-base.html           ← bg/surface/border color tokens
  colors-text.html           ← text color tokens
  colors-mood.html           ← mood palette (Grammar Map)
  colors-edge.html           ← edge palette (Grammar Map)
  colors-gold.html           ← gold accent scale
  type-display.html          ← Cormorant Garamond specimens
  type-body.html             ← DM Sans specimens
  type-mono.html             ← JetBrains Mono specimens
  type-scale.html            ← full heading scale
  spacing-tokens.html        ← radius, transition, spacing tokens
  components-nav.html        ← navigation bar
  components-cards.html      ← topic cards + surface cards
  components-controls.html   ← toggles, selects, pills
  components-table.html      ← conjugation table
  components-badges.html     ← badges, tags, related-tense pills
  components-panel.html      ← detail panel (node/edge)
  brand-aspect-icons.html    ← Grammar Map aspect glyph set
  brand-edge-vocab.html      ← edge type vocabulary
ui_kits/
  website/
    README.md
    index.html               ← interactive website prototype
    Nav.jsx
    HomePage.jsx
    VerbsPage.jsx
    TensePage.jsx
```
