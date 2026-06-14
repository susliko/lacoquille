# Implementation Plan

## Goal
Three small a11y and correctness fixes: (a) add missing ARIA attributes to the verb dropdown, (b) stop the browser tab title from showing "La Coquille" twice, (c) add a skip-to-main-content link for keyboard users.

---

## Tasks

### Task 1 — Verb dropdown ARIA attributes
**File:** `src/components/VerbDiagram.tsx`

The dropdown panel already has `role="listbox"` and items have `role="option"` + `aria-selected`. The missing pieces are on the trigger button and the menu container.

**Changes:**

1. Add `id="verb-dropdown-menu"` to the menu `<div>` (so `aria-controls` has a valid target):
   ```tsx
   <div id="verb-dropdown-menu" class="verb-dropdown-menu" role="listbox">
   ```

2. Add `aria-haspopup="listbox"` and `aria-controls="verb-dropdown-menu"` to the trigger `<button>`:
   ```tsx
   <button
     class="verb-dropdown-trigger"
     type="button"
     aria-expanded={verbDropdownOpen()}
     aria-haspopup="listbox"
     aria-controls="verb-dropdown-menu"
     onClick={() => setVerbDropdownOpen(o => !o)}
   >
   ```

**Acceptance:** After opening the dropdown and running `eval "document.querySelector('[aria-haspopup]')?.getAttribute('aria-haspopup')"` in agent-browser, the result is `"listbox"`. The button also has `aria-controls="verb-dropdown-menu"` and the menu div has `id="verb-dropdown-menu"`.

**Note:** Keyboard navigation (↑↓ navigate, Enter select, Escape close) is **deferred to a separate PR**. This task only adds the static ARIA attributes.

---

### Task 2 — Strip duplicate site name from page title
**File:** `src/layouts/Base.astro`

The `title` prop already contains " — La Coquille" on some pages (e.g., `article-of-the-day.astro` passes `title="Article of the Day — La Coquille"`). Base.astro appends ` · La Coquille`, producing "Article of the Day — La Coquille · La Coquille".

**Changes:**

1. In the frontmatter, add a `cleanTitle` variable that strips the site name suffix before re-appending it:
   ```astro
   ---
   import "../styles/global.css";

   interface Props {
     title: string;
     description?: string;
   }

   const { title, description = "An interactive French grammar reference." } = Astro.props;
   const cleanTitle = title.replace(/ — La Coquille$/, "").replace(/ · La Coquille$/, "");
   ---
   ```

2. Use `cleanTitle` in the `<title>` tag:
   ```astro
   <title>{cleanTitle} · La Coquille</title>
   ```

**Acceptance:** `npm run build` passes. In agent-browser, open `/stories/article-of-the-day/` and run `eval "document.title"` — the result is `"Article of the Day · La Coquille"` (not duplicated). The home page (`/`) still shows `"La Coquille"`.

---

### Task 3 — Add skip-to-main-content link
**File:** `src/layouts/Base.astro` + `src/styles/global.css`

**Changes to `Base.astro`:**

1. Add the skip link as the very first child of `<body>`:
   ```astro
   <body>
     <a href="#main-content" class="skip-link">Skip to main content</a>
   ```

2. Add `id="main-content"` to the `<main>` element:
   ```astro
   <main id="main-content">
   ```

**Changes to `src/styles/global.css`:**

Add the skip-link styles at the end of the file (or in the accessibility section):
```css
/* ── Skip link ───────────────────────────────────────────────── */
.skip-link {
  position: absolute;
  top: -100px;
  left: 0;
  padding: 0.5rem 1rem;
  background: var(--coral);
  color: white;
  font-weight: 600;
  text-decoration: none;
  z-index: 1000;
  border-radius: 0 0 4px 0;
  transition: top 0.2s ease;
}
.skip-link:focus {
  top: 0;
}
```

**Acceptance:** On any page, pressing Tab once shows the skip link at the top-left corner. Pressing Enter or continuing to Tab moves focus to `<main>` (or its first focusable child). The link is invisible until focused.

---

## Files to Modify

- `src/components/VerbDiagram.tsx` — add `id` to menu div, add `aria-haspopup` + `aria-controls` to trigger button
- `src/layouts/Base.astro` — add `cleanTitle` helper, use it in `<title>`, add skip link + `id="main-content"` on `<main>`
- `src/styles/global.css` — add `.skip-link` CSS block

## New Files
None.

## Dependencies
- Task 2 and Task 3 are independent of each other and can be done in any order.
- Task 1 is independent of Tasks 2 and 3.

## Risks

| Risk | Mitigation |
|------|------------|
| `aria-controls` points to a non-existent ID if the menu `id` is missing | Always add `id="verb-dropdown-menu"` to the menu div in the same commit |
| The `cleanTitle` regex only handles the em-dash variant (` — La Coquille`); if a page uses a different separator, the fix misses it | The known titles use only the em-dash variant; `replace()` (first match only) is intentional to avoid double-stripping |
| The skip link `href="#main-content"` fails if `<main>` has no `id` | The `id` is added in the same commit in `Base.astro`; all pages using this layout are covered |
| `top: -100px` on the skip link may not be enough to hide it on very small viewports | `-100px` is sufficient for any viewport — the link is pushed well off-screen. Using `-100%` is also acceptable but `-100px` is more predictable |

## Verification Commands

```sh
# Build check
cd /home/susliko/programming/lacoquille && npm run build

# Title dedup (agent-browser or curl)
# Open /stories/article-of-the-day/ in agent-browser:
eval "document.title"
# Expected: "Article of the Day · La Coquille"

# Skip link (agent-browser):
# Press Tab — the skip link should appear. Press Enter — focus moves to main.
# Verify the link is the first element: eval "document.body.firstElementChild?.className"
# Expected: "skip-link"

# Verb dropdown ARIA (agent-browser):
# Open /reference/verbs/, click the verb dropdown trigger, then run:
eval "document.querySelector('[aria-haspopup]')?.getAttribute('aria-haspopup')"
# Expected: "listbox"
eval "document.querySelector('[aria-haspopup]')?.getAttribute('aria-controls')"
# Expected: "verb-dropdown-menu"
eval "document.getElementById('verb-dropdown-menu')?.getAttribute('role')"
# Expected: "listbox"
```

## Branch & PR
- **Branch:** `fix/issue-016-018-020-a11y-polish`
- **PR title:** `fix(a11y): dropdown ARIA, dedupe title, add skip link (ISSUE-016, ISSUE-018, ISSUE-020)`
