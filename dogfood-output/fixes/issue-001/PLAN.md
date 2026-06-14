# Implementation Plan

## Goal

Make `/practice/` render a visible 2×2 grid of four colored practice cards (Article of Day, Typing Race, Shadowing, Vocabulary Mining) — both in dev and in the production build.

---

## Root Cause Summary

`src/pages/practice/index.astro` is the only `.astro` file in the project that has a scoped `<style>` block adjacent to a `client:only="solid-js"` island. Astro scopes `<style>` blocks by appending a `data-astro-cid-xxx` attribute selector to every rule. Because `PracticeHub` is mounted with `client:only`, its entire DOM is created client-side by SolidJS at hydration time — those elements carry **no** scoped attribute. All 8 `.practice-*` CSS rules silently match zero elements, so `display: grid` never applies and the cards render with no visible layout or background.

---

## Tasks

### Task 1 — Move the 8 `.practice-*` rules from the scoped `<style>` block into `global.css`

- **File:** `src/styles/global.css`
- **Change:** Append the following 8 rules to the existing poly-card / practice-adjacent area of the file (after the existing `.poly-card-*` rules around line 477):
  ```css
  /* ── Practice hub ─────────────────────────────────────────────── */
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
  ```
- **Why this works:** `@keyframes fly0`–`fly6` are already defined globally in `global.css` (around line 457). Moving the card rules to the same file places them in the same scope. The class names are all prefixed `practice-*`, so they cannot accidentally style unrelated elements.
- **Acceptance:** After the change, `global.css` contains all `.practice-*` rules with no attribute selector.

### Task 2 — Remove the 8 rules from the scoped `<style>` block, fix the `main` height value

- **File:** `src/pages/practice/index.astro`
- **Change:** Replace the entire `<style>` block with only the `main` rule, with the height value corrected from `calc(100dvh - 64px)` to `calc(100dvh - 52px)`:
  ```astro
  <style>
    main {
      height: calc(100dvh - 52px);
      overflow: hidden;
    }
  </style>
  ```
- **Why keep `main` scoped:** The `<main>` element is SSR'd by `Base.astro` (not client-only), so it carries the scoped `data-astro-cid-xxx` attribute and the rule matches correctly. Scoping is appropriate here because `main` height only needs this treatment on the practice page.
- **Why `52px`:** The site nav is 52px tall (defined in `global.css` `.site-nav { height: 52px; }`). This page has no nav rendered by `Base.astro`, but the value is kept correct for future-proofing.
- **Acceptance:** The `<style>` block in `index.astro` contains exactly 2 lines (one rule group).

---

## Files to Modify

- `src/styles/global.css` — append 8 `.practice-*` rules (see Task 1 for exact CSS)
- `src/pages/practice/index.astro` — replace `<style>` block (see Task 2 for exact result)

---

## New Files

None.

---

## Dependencies

Task 2 depends on Task 1 (the `.practice-*` rules must be in `global.css` before they are removed from the scoped block, otherwise the stylesheet would be temporarily empty during the edit window). However, since both changes will be applied atomically by the executing agent, no actual runtime dependency exists.

---

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Global styles bleed into other pages | Class names are all prefixed `practice-*`; no other page uses these classes. |
| `main` height rule affects all pages | Rule stays scoped in `index.astro`; `<main>` is SSR'd so scoping works. |
| Keyframes missing | Confirmed: `fly0`–`fly6` are already in `global.css` (lines 457–464). |
| Regression in other `client:only` pages | Recon confirmed no other `.astro` file has scoped `<style>` + `client:only`. No similar bug exists elsewhere. |
| `PracticeHub.tsx` needs changes | No — the component renders correct class names; only the CSS needs fixing. |

---

## Out of Scope

The following are **NOT** being changed in this PR:
- Shadowing API (separate issue)
- Typing race page
- Any other page
- `PracticeHub.tsx` or any other component
- Backend / API changes
- `diagram.json` or verb content

---

## Acceptance Criteria

The reviewer will verify:

1. `npm run build` completes without errors.
2. `npm run dev` serves `/practice/` and four cards are visible with colored backgrounds and white text.
3. `getComputedStyle(document.querySelector('.practice-hub-grid')).display === "grid"`.
4. `getComputedStyle(document.querySelector('.practice-hub-grid')).gridTemplateColumns === "repeat(2, 1fr)"`.
5. On the production build (`npm run preview`), the same page renders correctly.
6. `npm run build` confirms no broken internal links.
7. `npx astro check` passes with no new warnings.
8. Home page (`/`) and verbs index (`/reference/verbs/`) still load correctly.

---

## Verification Commands

```sh
# 1. Build (must succeed)
cd /home/susliko/programming/lacoquille && npm run build

# 2. Start dev server (run in background process, then test)
# Using the process tool:
process action=start command="npm run dev" name="dev-server"
# Then wait for port 4321, open http://localhost:4321/practice/ and screenshot

# 3. Verify grid layout in DevTools console
getComputedStyle(document.querySelector('.practice-hub-grid')).display
# Expected: "grid"
getComputedStyle(document.querySelector('.practice-hub-grid')).gridTemplateColumns
# Expected: "repeat(2, 1fr)"

# 4. Type-check
npx astro check

# 5. Verify other pages still load
curl -s http://localhost:4321/ | grep -o '<title>[^<]*</title>'
curl -s http://localhost:4321/reference/verbs/ | grep -o '<title>[^<]*</title>'
```

---

## Branch and PR

- **Branch:** `fix/issue-001-practice-hub-blank`
- **PR title:** `fix(practice): make /practice/ visible (ISSUE-001)`
- **PR body should reference:** ISSUE-001, root cause (scoped CSS + `client:only`), fix (moved rules to `global.css`)
