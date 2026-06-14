# ISSUE-001 Recon: Practice Hub Page Renders Blank on Production

## Files Retrieved

1. `src/pages/practice/index.astro` (lines 1–47) — the Astro page that mounts `PracticeHub` with `client:only`, and the only file in the project with a scoped `<style>` block targeting a `client:only` island.
2. `src/components/PracticeHub.tsx` (lines 1–29) — the SolidJS island that renders the 2×2 grid of practice cards.
3. `src/styles/global.css` (lines 1–490) — the global stylesheet; contains all `.poly-*` card classes (used by `HomepageTabs`) but does NOT contain any `.practice-*` classes.
4. `dist/practice/index.html` — the built output; proves the scoping mechanism and confirms `client:only` island behavior.
5. `src/components/HomepageTabs.tsx` (lines 1–98) — compared as a reference for a working `client:load` island.
6. `src/components/VerbDiagram.tsx` (lines 1–300+) — compared as a reference for a `client:load` island with no scoped styles.
7. `src/layouts/Base.astro` (lines 1–24) — the page shell; has no nav, no scoped styles.

---

## 1. The Page, the Component, and the Stylesheet

### `src/pages/practice/index.astro`

```astro
---
import Base from "../../layouts/Base.astro";
import PracticeHub from "../../components/PracticeHub";
---

<Base title="Practice Hub" section="practice">
  <main>
    <PracticeHub client:only="solid-js" />
  </main>
</Base>

<style>
  main { height: calc(100dvh - 64px); overflow: hidden; }   /* ← scoped */

  .practice-hub-root { height: 100%; width: 100%; max-width: 100dvw; overflow: hidden; }   /* ← scoped */
  .practice-hub-grid { display: grid; grid-template-columns: repeat(2, 1fr); grid-template-rows: repeat(2, 1fr); gap: 0; height: 100%; width: 100%; }   /* ← scoped */
  .practice-card { display: flex; flex-direction: column; align-items: center; justify-content: center; text-align: center; padding: 1.5rem 1rem; gap: 6px; text-decoration: none; background: var(--card-color, #ccc); transition: filter 0.15s ease; animation-fill-mode: both; }   /* ← scoped */
  .practice-card:active { filter: brightness(1.12); }   /* ← scoped */
  .practice-card-0 { animation: fly0  0.7s cubic-bezier(.16,1,.3,1) 0.00s both }   /* ← scoped */
  .practice-card-1 { animation: fly1  0.7s cubic-bezier(.16,1,.3,1) 0.07s both }   /* ← scoped */
  .practice-card-2 { animation: fly5  0.7s cubic-bezier(.16,1,.3,1) 0.14s both }   /* ← scoped */
  .practice-card-3 { animation: fly6  0.7s cubic-bezier(.16,1,.3,1) 0.07s both }   /* ← scoped */
</style>
```

### `src/components/PracticeHub.tsx`

```tsx
export default function PracticeHub() {
  return (
    <div class="practice-hub-root">                        {/* ← rendered by SolidJS, no scoped attr */}
      <div class="practice-hub-grid">                      {/* ← rendered by SolidJS, no scoped attr */}
        <For each={PRACTICE}>
          {(item, i) => (
            <a
              href={item.path}
              class={`practice-card practice-card-${i()}`}  {/* ← rendered by SolidJS */}
              style={{ "--card-color": item.color }}
            >
              <div class="poly-card-name">{item.name}</div>  {/* ← uses global .poly-card-name */}
              <div class="poly-card-sub">{item.sub}</div>     {/* ← uses global .poly-card-sub */}
              <div class="poly-card-cta">start →</div>       {/* ← uses global .poly-card-cta */}
            </a>
          )}
        </For>
      </div>
    </div>
  );
}
```

---

## 2. CSS Rules the Practice Cards Need and Where They Currently Live

| Rule | Selector | Currently in | Affects PracticeHub? |
|------|----------|--------------|---------------------|
| Grid layout | `.practice-hub-grid` | **Scoped** (`<style>` in `index.astro`) | ❌ No — SolidJS DOM has no scoped attr |
| Root fill viewport | `.practice-hub-root` | **Scoped** | ❌ No |
| Card flex layout | `.practice-card` | **Scoped** | ❌ No |
| Card background | `background: var(--card-color, #ccc)` | **Scoped** | ❌ No — fallback `#ccc` applies (transparent-ish) |
| Card active filter | `.practice-card:active` | **Scoped** | ❌ No |
| Card animations | `.practice-card-0` … `.practice-card-3` | **Scoped** | ❌ No |
| `main` height | `main` | **Scoped** | ✅ Yes — `<main>` is SSR'd by Base.astro |
| Card name text | `.poly-card-name` | **global.css** | ✅ Yes — works |
| Card sub text | `.poly-card-sub` | **global.css** | ✅ Yes — works |
| Card CTA text | `.poly-card-cta` | **global.css** | ✅ Yes — works |
| `@keyframes fly0…fly6` | — | **global.css** | ✅ Yes — keyframes are global |

The three `.poly-*` text classes work because they are defined in `global.css` without any attribute selector. The grid layout and all `.practice-*` card rules are scoped and silently fail.

---

## 3. Root Cause: Confirmed

The bug report's suspected root cause is **correct**.

**Mechanism:** Astro scopes `<style>` blocks by appending a unique attribute selector (e.g. `[data-astro-cid-xrrsn55w]`) to every rule. The built output proves this:

```html
<!-- dist/practice/index.html -->
<style>
  .practice-hub-grid[data-astro-cid-xrrsn55w]{display:grid;...}
  .practice-card[data-astro-cid-xrrsn55w]{display:flex;...}
  .practice-card-0[data-astro-cid-xrrsn55w]{animation:fly0...}
  ...
</style>
```

**Why it fails:** `PracticeHub` is mounted as `client:only="solid-js"`. This directive means:
- Astro does **not** SSR the component at all.
- The SolidJS runtime creates the entire `<div class="practice-hub-root">` … `<a class="practice-card practice-card-0">` DOM tree **client-side**, at hydration time.
- The client-side DOM elements carry **no** `data-astro-cid-xrrsn55w` attribute.
- CSS selectors like `.practice-hub-grid[data-astro-cid-xrrsn55w]` match **zero elements**.
- Result: `display: grid` never applies → `display: block` (default) → transparent backgrounds over nothing.

**Evidence from build output** (`dist/practice/index.html`):
```html
<main data-astro-cid-xrrsn55w>   <!-- ← SSR'd main HAS the scoped attr -->
  <astro-island ... client="only">  <!-- ← island tag, no inner DOM in HTML -->
</main>
```

The `astro-island` custom element has no children in the static HTML — its content is injected by JS at runtime. The injected elements have no scoped attribute.

**Secondary issue:** The `main` rule uses `calc(100dvh - 64px)` but `Base.astro` has no nav. The site-nav (52px) only exists on pages that include it. The `-64px` is therefore wrong for this page — it should be `100dvh` or `calc(100dvh - 52px)` if a nav is ever added. This is a separate bug but contributes to the visual blankness.

---

## 4. Similar Scoping-vs-Client-Only Issues Elsewhere

**Methodology:** grepped all `.astro` files for `client:only` and cross-referenced with `<style>` blocks.

| File | `client:*` directive | Has `<style>` block? | Same scoping issue? |
|------|---------------------|----------------------|---------------------|
| `src/pages/practice/typing.astro` | `client:only="solid-js"` | **No** | ❌ Safe |
| `src/pages/practice/shadowing.astro` | `client:only="solid-js"` | **No** | ❌ Safe |
| `src/pages/stories/article-of-the-day.astro` | `client:only="solid-js"` | **No** | ❌ Safe |
| `src/pages/reference/verbs/index.astro` | `client:only="solid-js"` | **No** | ❌ Safe |
| `src/pages/reference/grammar/pronouns/index.astro` | `client:only="solid-js"` | **No** | ❌ Safe |
| `src/pages/reference/grammar/syntax/index.astro` | `client:only="solid-js"` | **No** | ❌ Safe |
| `src/pages/reference/grammar/adjectifs/index.astro` | `client:only="solid-js"` | **No** | ❌ Safe |
| `src/pages/reference/grammar/adverbs/index.astro` | `client:only="solid-js"` | **No** | ❌ Safe |
| `src/pages/reference/grammar/negation/index.astro` | `client:only="solid-js"` | **No** | ❌ Safe |
| **`src/pages/practice/index.astro`** | `client:only="solid-js"` | **Yes** | ✅ **Only affected file** |

**`src/pages/index.astro`** — uses `client:load` (not `client:only`), so Astro SSR-renders `HomepageTabs` first. The SSR output carries the scoped attribute, and SolidJS hydrates it in-place. However, `HomepageTabs` uses class names (`.poly-card`, `.poly-grid`) that are **already global** in `global.css`, so scoping is irrelevant for it.

**`src/components/VerbDiagram.tsx`** — used with `client:load` on `src/pages/reference/verbs/index.astro`. The component uses SVG-rendered elements (no class names on HTML elements that need CSS). All diagram styling is via inline SVG attributes or CSS custom properties injected by JS. No scoped `<style>` risk.

**Conclusion:** `src/pages/practice/index.astro` is the **only file** in the project with a scoped `<style>` block adjacent to a `client:only` island. No similar bug exists elsewhere currently.

---

## 5. Available Fix Approaches

### A. Move the styles to `global.css`

**What:** Copy the 8 rules from the scoped `<style>` block into `src/styles/global.css`.

**Tradeoffs:**
- ✅ Simplest fix; no structural changes.
- ✅ No risk of regression; styles are always global.
- ✅ The keyframes (`fly0`–`fly6`) are already in `global.css` — consistent with existing pattern.
- ❌ Slight namespace pollution — these classes would be global. Currently they only exist on the practice page. Acceptable given the class names are prefixed with `practice-` and `poly-`.
- ❌ The `main` height rule (`calc(100dvh - 64px)`) would apply to ALL pages that use `Base.astro`, which is every page. This is wrong — the practice page needs special height treatment. The `main` rule should either stay scoped (but the `<main>` element IS SSR'd, so it would work) or be given a more specific selector like `.practice-page main`.

### B. Use `:global(...)` selectors in the scoped `<style>` block

**What:** Change the scoped `<style>` block to:
```astro
<style>
  :global(main) { height: calc(100dvh - 64px); overflow: hidden; }
  :global(.practice-hub-root) { ... }
  :global(.practice-hub-grid) { ... }
  :global(.practice-card) { ... }
  ...
</style>
```

**Tradeoffs:**
- ✅ Keeps styles co-located with the page file.
- ✅ Styles remain discoverable in context.
- ❌ Effectively identical to moving to `global.css` — the rules escape the scope anyway.
- ❌ `main` would still apply globally.
- ❌ Mixing scoped and `:global` in the same block can be confusing.

### C. Wrap the SolidJS island in a static Astro wrapper that owns the layout

**What:** Move the grid layout to a static Astro `<div>` wrapper in the `.astro` file, and pass the card data as props to `PracticeHub` (which only renders the `<a>` elements):

```astro
<main>
  <div class="practice-hub-root">
    <div class="practice-hub-grid">
      <PracticeHub client:only="solid-js" items={PRACTICE} />
    </div>
  </div>
</main>
```

**Tradeoffs:**
- ✅ Grid styles live in the `.astro` file's scoped block (or `global.css`) and apply to the static wrapper.
- ✅ `PracticeHub` becomes simpler — just renders cards.
- ✅ No CSS changes needed.
- ❌ Requires refactoring `PracticeHub.tsx` to accept `items` as a prop.
- ❌ The wrapper `<div>` must match the grid semantics — needs care with the `<For>` loop placement.
- ❌ `client:only` still means the inner `<a>` elements are client-side, but they only need the card-specific styles (`.practice-card`, `.practice-card-N`) which could be global.

### D. Use `is:global` directive on the `<style>` block

**What:**
```astro
<style is:global>
  .practice-hub-root { ... }
  .practice-hub-grid { ... }
  ...
</style>
```

**Tradeoffs:**
- ✅ One-line change.
- ✅ Styles remain in the page file.
- ❌ `is:global` removes scoping from the **entire block** — including the `main` rule. Same problem as A/B: `main` would apply globally.
- ❌ Equivalent to moving to `global.css` but less discoverable.

### Recommended Fix

**Approach A (move to `global.css`) + fix the `main` rule** is the most robust:

1. Move `.practice-hub-root`, `.practice-hub-grid`, `.practice-card`, `.practice-card:active`, and `.practice-card-0` through `.practice-card-3` to `global.css`.
2. Keep the `main` rule scoped (it applies to the SSR'd `<main>` from `Base.astro`).
3. Fix the `main` height from `calc(100dvh - 64px)` to `calc(100dvh - 52px)` to match the actual nav height, or `100dvh` if no nav is planned for this page.

---

## 6. Problematic Code — Exact Quotes

### The scoped `<style>` block (fails for `client:only` island):
```astro
<!-- src/pages/practice/index.astro lines 12–30 -->
<style>
  main {
    height: calc(100dvh - 64px);
    overflow: hidden;
  }

  .practice-hub-root {
    height: 100%;
    width: 100%;
    max-width: 100dvw;
    overflow: hidden;
  }

  .practice-hub-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: repeat(2, 1fr);
    gap: 0;
    height: 100%;
    width: 100%;
  }

  .practice-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 1.5rem 1rem;
    gap: 6px;
    text-decoration: none;
    background: var(--card-color, #ccc);
    transition: filter 0.15s ease;
    animation-fill-mode: both;
  }

  .practice-card:active {
    filter: brightness(1.12);
  }

  .practice-card-0 { animation: fly0  0.7s cubic-bezier(.16,1,.3,1) 0.00s both }
  .practice-card-1 { animation: fly1  0.7s cubic-bezier(.16,1,.3,1) 0.07s both }
  .practice-card-2 { animation: fly5  0.7s cubic-bezier(.16,1,.3,1) 0.14s both }
  .practice-card-3 { animation: fly6  0.7s cubic-bezier(.16,1,.3,1) 0.07s both }
</style>
```

### The island mount (generates DOM without scoped attribute):
```astro
<!-- src/pages/practice/index.astro line 8 -->
<PracticeHub client:only="solid-js" />
```

### The SolidJS component (renders class names that need the above CSS):
```tsx
// src/components/PracticeHub.tsx lines 17–24
<a
  href={item.path}
  class={`practice-card practice-card-${i()}`}
  style={{ "--card-color": item.color }}
>
  <div class="poly-card-name">{item.name}</div>
  <div class="poly-card-sub">{item.sub}</div>
  <div class="poly-card-cta">start →</div>
</a>
```

### Built output proving the scoped attribute (on `<main>` only, not on island DOM):
```html
<!-- dist/practice/index.html — built static HTML -->
<style>
  .practice-hub-grid[data-astro-cid-xrrsn55w]{display:grid;...}
  .practice-card[data-astro-cid-xrrsn55w]{display:flex;...}
  ...
</style>
...
<main data-astro-cid-xrrsn55w>    <!-- ← SSR'd element HAS the attr -->
  <astro-island ... client="only">  <!-- ← island, no inner DOM in static HTML -->
</main>
```

### Global rules that work (because they have no attribute selector):
```css
/* src/styles/global.css — these DO apply to PracticeHub's DOM */
.poly-card-name {
  font-family: var(--font-display);
  font-weight: 700;
  font-size: 1.1rem;
  color: white;         /* ← explains "color: rgb(255,255,255)" in bug report */
  line-height: 1.1;
}
.poly-card-sub { ... }
.poly-card-cta { ... }
```
