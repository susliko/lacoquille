# Round 5 Recon: Dogfood Fixes

## ISSUE-005: Verb diagram "durée" edge label is illegibly small

### Location
- `src/components/VerbDiagram.tsx` — edge rendering loop (lines ~479–508 mobile, ~582–609 desktop)
- `src/content/verbs/diagram.json` — edge definitions (line ~204: the mood-swap edge)

### Current code
**`diagram.json` (line 204–207):**
```json
{
  "type": "mood-swap",
  "from": "present-indicatif",
  "to": "subjonctif-present",
  "label": "same time, different mood"
}
```

**`VerbDiagram.tsx` — edge group renderer (desktop, lines ~582–609):**
```tsx
<For each={visibleEdges()}>
  {edge => {
    const from = dkPosMap().get(edge.from);
    const to   = dkPosMap().get(edge.to);
    if (!from || !to) return null;
    const isActive = () => activeEdge() === edge;
    const edgeColor = EDGE_COLOR[edge.type];
    const d = edgePath(from, to, edge);
    return (
      <g class="diagram-edge"
        onClick={() => setActiveEdge(isActive() ? null : edge)}
        style={{ cursor: "pointer" }}
        role="button" tabIndex={0}
        aria-label={`${edge.type}: ${edge.label}`}
        onKeyDown={e => e.key === "Enter" && setActiveEdge(isActive() ? null : edge)}
      >
        <path d={d} fill="none" stroke={edgeColor}
          stroke-width={isActive() ? 3 : 2}
          stroke-dasharray={EDGE_DASH[edge.type]}
          stroke-linecap="round"
          opacity={isActive() ? 1 : 0.55}
          marker-end={...}
        />
        <path d={d} fill="none" stroke="transparent" stroke-width="16" />
      </g>
    );
  }}
</For>
```

### Analysis
The edge `label` field from `diagram.json` is only used in the `aria-label` attribute — **there is no `<text>` SVG element that renders the label visually on the diagram**. The "durée" label does not exist in the current `diagram.json`; the mood-swap edge says "same time, different mood". The label either needs to be (a) added as a new short label in `diagram.json` for the mood-swap edge, and (b) rendered as an SVG `<text>` element at the edge midpoint.

### Fix approach
Two-file change:

1. **`src/content/verbs/diagram.json`** — update the mood-swap edge label to a short French label:
   ```json
   "label": "durée"
   ```

2. **`src/components/VerbDiagram.tsx`** — inside the desktop and mobile edge `<g>` blocks, after the `<path>` lines, add a `<text>` element that renders `edge.label` at the midpoint of the bezier path. For desktop (cross-lane mood-swap edges), compute the midpoint along the bezier; for same-lane edges use `(from.cx + to.cx) / 2` and `from.cy + yOff`:
   ```tsx
   {/* Edge label */}
   <text
     x={(from.cx + to.cx) / 2}
     y={(from.cy + to.cy) / 2 - 6}
     text-anchor="middle"
     font-size="11"
     font-style="italic"
     fill={edgeColor}
     opacity={isActive() ? 1 : 0.85}
     style={{ pointerEvents: "none" }}
   >
     {edge.label}
   </text>
   ```
   The `font-size="11"` gives a readable label (vs the implied ~7px that makes it illegible). `fill={edgeColor}` gives the label the same color as the edge line (coral/violet/amber) for contrast. Add the same pattern to the mobile SVG block.

### Risk
- Adding a `<text>` element on cross-lane bezier edges (mood-swap, stem-share) requires computing the bezier midpoint correctly — the `edgePath()` function returns a path string, so either parse the control points or render the label at the straight-line midpoint and accept slight inaccuracy for diagonal edges.
- The label sits "on the line" — for busy areas of the diagram, it may overlap with node cards. `pointer-events: none` ensures it doesn't block clicks. Low risk.

---

## ISSUE-008: Active-token highlight is too subtle

### Location
`src/components/ArticleOfTheDay.tsx` — inline `<style>` block (lines ~155–170)

### Current code
```css
.token.active {
  background-color: rgba(59, 130, 246, 0.2);
  outline: 2px solid var(--accent, #3b82f6);
  outline-offset: 1px;
}
.en-token.active {
  background-color: rgba(59, 130, 246, 0.2);
}
.fr-token.active {
  background-color: rgba(59, 130, 246, 0.2);
}
```

### Fix approach
Bump the alpha from `0.2` to `0.4` and strengthen the outline on `.token.active` and `.en-token.active`:

```css
.token.active {
  background-color: rgba(59, 130, 246, 0.4);
  outline: 3px solid var(--accent, #3b82f6);
  outline-offset: 1px;
}
.en-token.active {
  background-color: rgba(59, 130, 246, 0.4);
  outline: 2px solid var(--accent, #3b82f6);
  outline-offset: 1px;
}
.fr-token.active {
  background-color: rgba(59, 130, 246, 0.4);
}
```

The `.fr-token.active` gets its highlight from the shared `.token.active` rule; only the `.token.active` and `.en-token.active` rules need updating.

### Risk
- `rgba(59, 130, 246, 0.4)` is still a light blue — it won't look jarring on white. Low visual risk.
- No structural changes; purely CSS. Zero runtime risk.

---

## ISSUE-012: "Roman (1888)" metadata confusing for English readers

### Location
`src/components/ArticleOfTheDay.tsx` — `SOURCE_LABELS` map (line 11) and header template (line ~226)

### Current code
```tsx
const SOURCE_LABELS: Record<string, string> = {
  "Roman": "Novel",
  "Nouvelles": "Short Stories",
  "Contes du jour et de la nuit": "Short Stories",
  "Les Rougon-Macquart": "Novel Cycle",
};
// ...
<p class="article-meta">
  {AUTHOR} · {SOURCE_LABELS[data().source] ?? data().source} · ...
</p>
```

### Analysis
**ISSUE-012 is already resolved.** `SOURCE_LABELS` was added in a prior round (ISSUE-013 work). The page header now renders "Novel" instead of "Roman" for English readers. The `AUTHOR` constant ("Guy de Maupassant") is also displayed, addressing the "no author" part of the issue.

### Fix approach
No code change needed. Confirm via `curl https://lacoquille.fly.dev/api/article-of-the-day | jq -r '.source'` that the API still returns `"Roman"` (not `"Novel"`), so the `SOURCE_LABELS` lookup is still necessary and working.

### Risk
None — no change.

---

## ISSUE-014: "durée" label not explained in the diagram Key

### Location
`src/components/VerbDiagram.tsx` — `Legend` component (lines ~319–358)

### Current code
```tsx
const Legend = () => (
  <details class="diagram-legend">
    <summary>Key</summary>
    <div class="legend-section">
      <p class="legend-section-title">Difficulty</p>
      <div class="legend-grid">
        {/* Présent start-here + literary lock */}
      </div>
    </div>
    <div class="legend-section">
      <p class="legend-section-title">Connections</p>
      <div class="legend-grid">
        {(Object.entries(EDGE_COLOR) as ...).map(([type, color]) => (
          <div class="legend-item">
            <svg ...>...</svg>
            <span>{type.replace(/-/g, " ")}</span>
          </div>
        ))}
      </div>
    </div>
  </details>
);
```

### Analysis
The Key has two sections: **Difficulty** and **Connections**. Neither mentions the "durée" edge label. The Connections section lists all 5 edge types by their `EDGE_COLOR` key names, but the `mood-swap` entry says "mood swap" while the actual edge label is "durée" (after the fix for ISSUE-005). English readers have no way to connect the French word to its meaning.

### Fix approach
Add a third `<div class="legend-section">` inside the `Legend` component, after the Connections section:

```tsx
<div class="legend-section">
  <p class="legend-section-title">Labels</p>
  <div class="legend-grid">
    <div class="legend-item">
      <svg width="32" height="10" aria-hidden="true">
        <line x1="0" y1="5" x2="32" y2="5"
          stroke="var(--edge-mood)" stroke-width="2.5"
          stroke-dasharray="11 5" stroke-linecap="round" />
      </svg>
      <span><em>durée</em> — ongoing action; same form across moods</span>
    </div>
  </div>
</div>
```

This uses the existing `--edge-mood` CSS variable (violet) and the `11 5` dash pattern from `EDGE_DASH["mood-swap"]` to render a representative line. The explanation links the French word to its English meaning.

### Risk
- `legend-grid` already uses `flex-wrap: wrap`, so the new item flows naturally. Low risk.
- No changes to the SVG rendering or data model.

---

## ISSUE-015: Mobile tooltip overlaps previous line of text

### Location
`src/components/ArticleOfTheDay.tsx` — mobile CSS `@media (max-width: 768px)` block (lines ~169–186)

### Current code
```css
@media (max-width: 768px) {
  /* ... */
  .fr-token.active::after {
    content: attr(data-trans);
    display: block;
    position: absolute;
    bottom: calc(100% + 4px);
    left: 50%;
    transform: translateX(-50%);
    background: var(--accent, #3b82f6);
    color: white;
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 0.8rem;
    z-index: 100;
    pointer-events: none;
    max-width: 200px;
    white-space: normal;
    text-align: center;
    box-shadow: 0 2px 8px rgba(0,0,0,0.2);
  }
}
```

### Fix approach
The simplest CSS fix: flip the tooltip to appear below the word instead of above, using `top: 100%` with `margin-top: 4px` instead of `bottom: calc(100% + 4px)`. This naturally positions the tooltip below the token and avoids the viewport-top overflow problem entirely:

```css
.fr-token.active::after {
  content: attr(data-trans);
  display: block;
  position: absolute;
  top: 100%;            /* was: bottom: calc(100% + 4px) */
  left: 50%;
  transform: translateX(-50%);
  margin-top: 4px;       /* gap between word and tooltip */
  background: var(--accent, #3b82f6);
  color: white;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 0.8rem;
  z-index: 100;
  pointer-events: none;
  max-width: 200px;
  white-space: normal;
  text-align: center;
  box-shadow: 0 2px 8px rgba(0,0,0,0.2);
}
```

For words near the bottom of the viewport, the tooltip will still overflow. A more robust fix would add a JS `onClick` handler that checks `getBoundingClientRect()` and flips to `bottom: 100%` if `rect.top < 120` — but that requires adding state to the component. The CSS-only fix solves the primary issue (tooltip floating over the heading) with zero JS complexity.

### Risk
- For words near the bottom of the viewport, the tooltip will now extend below the fold. This is a known trade-off for CSS-only tooltip positioning. The tooltip is dismissible by clicking the word again (or any other token), so users can recover. Acceptable risk.
- `transform: translateX(-50%)` combined with `left: 50%` still works correctly with `top: 100%`. No layout change.

---

## ISSUE-016: Verb dropdown has no keyboard navigation or proper ARIA

### Location
`src/components/VerbDiagram.tsx` — Controls component (lines ~368–395)

### Current code
```tsx
<button
  class="verb-dropdown-trigger"
  type="button"
  aria-expanded={verbDropdownOpen()}
  onClick={() => setVerbDropdownOpen(o => !o)}
>
  {VERB_OPTIONS.find(o => o.value === selectedVerb())?.label ?? selectedVerb()}
  <span class="verb-dropdown-arrow" aria-hidden="true">▾</span>
</button>
<Show when={verbDropdownOpen()}>
  <div class="verb-dropdown-menu" role="listbox">
    {VERB_OPTIONS.map(opt => (
      <button
        class={`verb-dropdown-item${selectedVerb() === opt.value ? " active" : ""}`}
        role="option"
        aria-selected={selectedVerb() === opt.value}
        type="button"
        onClick={() => {
          setSelectedVerb(opt.value);
          setVerbDropdownOpen(false);
        }}
      >
        {opt.label}
      </button>
    ))}
  </div>
</Show>
```

### Analysis
Good news: `role="listbox"`, `role="option"`, and `aria-selected` are already present. The missing pieces are:
1. `aria-haspopup="listbox"` on the trigger button
2. `aria-controls` on the trigger pointing to the menu `id`
3. Keyboard navigation: arrow keys move focus through options, `Enter` selects, `Escape` closes
4. On open, focus should move to the first item (or last focused item)

### Fix approach
**Step 1 — Add ARIA attributes to trigger and menu:**
```tsx
<button
  class="verb-dropdown-trigger"
  type="button"
  aria-expanded={verbDropdownOpen()}
  aria-haspopup="listbox"
  aria-controls="verb-dropdown-menu"   // add id to menu div first
  onClick={() => setVerbDropdownOpen(o => !o)}
  onKeyDown={handleTriggerKeyDown}
>
```
```tsx
<div id="verb-dropdown-menu" class="verb-dropdown-menu" role="listbox">
```

**Step 2 — Add keyboard handler in the component (or as a helper function):**
```tsx
function handleTriggerKeyDown(e: KeyboardEvent) {
  if (e.key === "ArrowDown" || e.key === "ArrowUp") {
    e.preventDefault();
    if (!verbDropdownOpen()) setVerbDropdownOpen(true);
    // move focus to first/last item
  }
  if (e.key === "Escape") setVerbDropdownOpen(false);
}
```

**Step 3 — Add `onKeyDown` to each menu item:**
```tsx
<button
  ...
  onKeyDown={(e) => {
    if (e.key === "Escape") { setVerbDropdownOpen(false); /* return focus to trigger */ }
    if (e.key === "ArrowDown") { e.preventDefault(); /* focus next item */ }
    if (e.key === "ArrowUp") { e.preventDefault(); /* focus prev item */ }
  }}
>
```

### Risk
- This is a multi-part a11y change. The current basic click-to-select behavior must be preserved. The `onClick` handler on items remains unchanged; keyboard handlers are additive.
- The `onMount` click-outside handler already closes the dropdown — `Escape` should do the same. No new state needed.
- **Deferral note**: The keyboard navigation logic (tracking focused index, moving focus programmatically) is non-trivial in SolidJS. Consider deferring to a dedicated "a11y polish" PR as the issue notes.

---

## ISSUE-018: Browser tab title has "La Coquille" twice

### Location
- `src/layouts/Base.astro` (line 13: `<title>{title} · La Coquille</title>`)
- `src/pages/stories/article-of-the-day.astro` (line 6: `title="Article of the Day — La Coquille"`)
- `src/pages/practice/typing.astro` (line 6: `title="Typing Race — La Coquille"`)
- `src/pages/practice/shadowing.astro` (line 6: `title="Shadowing Practice — La Coquille"`)
- `src/pages/index.astro` (line 6: `title="La Coquille"`)

### Current code
**`Base.astro` (line 13):**
```astro
<title>{title} · La Coquille</title>
```

**`article-of-the-day.astro` (line 6):**
```astro
<Base title="Article of the Day — La Coquille">
```

### Fix approach
Add a title-cleaning helper in `Base.astro` that strips the site name before appending it as suffix:

```astro
---
interface Props {
  title: string;
  description?: string;
}
const { title, description = "An interactive French grammar reference." } = Astro.props;
const cleanTitle = title.replace(/ — La Coquille$/, "").replace(/ · La Coquille$/, "");
---
<!doctype html>
<html lang="en">
  <head>
    ...
    <title>{cleanTitle} · La Coquille</title>
    ...
  </head>
```

This handles all three patterns:
- `"La Coquille"` → `"La Coquille"` (home page, no change)
- `"Article of the Day — La Coquille"` → `"Article of the Day"` → `"Article of the Day · La Coquille"`
- `"Verbes"` → `"Verbes"` → `"Verbes · La Coquille"` (clean, no prior suffix)

### Risk
- The regex uses `replace()` (first match only), not `replaceAll()`, which is correct — each title should only have the site name appended once.
- The `replace(/ — La Coquille$/, "")` handles the em-dash variant used by content pages; the `·` variant is for Base.astro's own output. Low risk.
- Pages that don't include "La Coquille" in their title prop (e.g., verbs index: `title="Verbes"`) are unaffected — the clean title equals the original.

---

## ISSUE-020: No `<nav>` element, no skip link, no in-page links

### Location
`src/layouts/Base.astro` (lines 1–21)

### Current code
```astro
---
import "../styles/global.css";
interface Props { title: string; description?: string; }
const { title, description = "An interactive French grammar reference." } = Astro.props;
---
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="description" content={description} />
    <title>{title} · La Coquille</title>
    <link rel="icon" type="image/svg+xml" href="/favicon.svg" />
  </head>
  <body>
    <main>
      <slot />
    </main>
  </body>
</html>
```

### Analysis
`Base.astro` renders only a bare `<main>` inside `<body>`. There is no `<nav>`, no `<header>`, no skip link, and no in-page navigation. Screen-reader users and keyboard-only users have no landmark to jump to the main content.

### Fix approach
Add a skip link as the very first element in `<body>`, and a `<nav>` wrapper around the main content (or alongside it):

```astro
  <body>
    <!-- Skip to main content -->
    <a href="#main-content" class="skip-link">Skip to main content</a>

    <main id="main-content">
      <slot />
    </main>
  </body>
```

And in `global.css`, add the skip-link styles:
```css
.skip-link {
  position: absolute;
  top: -100%;
  left: 1rem;
  background: var(--accent, #3b82f6);
  color: white;
  padding: 0.5rem 1rem;
  border-radius: 0 0 0.5rem 0.5rem;
  font-weight: 600;
  text-decoration: none;
  z-index: 9999;
  transition: top 0.2s;
}
.skip-link:focus {
  top: 0;
}
```

The `<main>` element gets `id="main-content"` so the skip link's `href="#main-content"` resolves correctly. The skip link is visually hidden by default (`top: -100%`) and slides into view on focus.

### Risk
- The skip link appears on every page that uses `Base.astro`. This is correct WCAG behavior — all pages should have a skip link.
- `position: absolute` with `top: -100%` removes it from the document flow; it doesn't shift any content.
- No pages currently have a `#main-content` id, so there are no anchor conflicts. Low risk.

---

## Summary Table

| Issue | File(s) | Change Type | Effort |
|-------|---------|-------------|--------|
| ISSUE-005 | `VerbDiagram.tsx` + `diagram.json` | Add SVG `<text>` label rendering + short label in JSON | Small |
| ISSUE-008 | `ArticleOfTheDay.tsx` | CSS alpha bump + outline width | Tiny |
| ISSUE-012 | — | Already resolved by `SOURCE_LABELS` | None |
| ISSUE-014 | `VerbDiagram.tsx` | Add "Labels" section to Legend | Small |
| ISSUE-015 | `ArticleOfTheDay.tsx` | CSS `top: 100%` instead of `bottom: calc(100% + 4px)` | Tiny |
| ISSUE-016 | `VerbDiagram.tsx` | Add `aria-haspopup`, `aria-controls`, keyboard handlers | Medium |
| ISSUE-018 | `Base.astro` | Strip "La Coquille" suffix before re-appending | Tiny |
| ISSUE-020 | `Base.astro` + `global.css` | Add skip link + `id="main-content"` on `<main>` | Small |
