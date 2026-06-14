# Implementation Plan

## Goal
On a 390px-wide mobile viewport, the home page Knowledge topic grid switches from 3 columns (cramped) to 2 columns (comfortable). After the fix, 8 cards in a 2-column grid = 2+2+2+2 layout with no empty slot, and text in each card fits on 1–2 lines.

## Tasks

1. **Change `.homepage-tab-panel .poly-card` flex basis from 3 columns to 2 columns**
   - File: `src/styles/global.css`
   - Changes: In the `@media (max-width: 640px)` block, change `flex: 0 0 calc(100% / 3)` → `flex: 0 0 calc(100% / 2)`
   - Acceptance: `npm run build` passes; agent-browser at 390px viewport shows 2 cards per row

2. **Adjust card height for cleaner 4-row layout**
   - File: `src/styles/global.css`
   - Changes: Change `height: 26%` → `height: 25%` in the mobile `.poly-card` rule (4 rows × 25% = 100%, vs. current 4 rows × 26% = 104% which overflows slightly)
   - Acceptance: The grid fills the panel without overflow

## Files to Modify

- `src/styles/global.css` — Change two values in the `@media (max-width: 640px)` block:
  - `flex: 0 0 calc(100% / 3)` → `flex: 0 0 calc(100% / 2)`
  - `height: 26%` → `height: 25%`

## New Files

(None)

## Dependencies

Task 1 and Task 2 are independent and can be applied in any order. Both are contained in the same CSS block.

## Exact Code

**Current CSS (lines 1391–1400 of `src/styles/global.css`):**

```css
@media (max-width: 640px) {
  .homepage-tab-panel .poly-grid {
    display: flex;
    flex-wrap: wrap;
    height: 100%;
    align-content: flex-start;
  }

  .homepage-tab-panel .poly-card {
    flex: 0 0 calc(100% / 3);
    height: 26%;
  }
}
```

**Modified CSS:**

```css
@media (max-width: 640px) {
  .homepage-tab-panel .poly-grid {
    display: flex;
    flex-wrap: wrap;
    height: 100%;
    align-content: flex-start;
  }

  .homepage-tab-panel .poly-card {
    flex: 0 0 calc(100% / 2);
    height: 25%;
  }
}
```

## Risks

- **Risk**: The 2-column layout creates 4 rows of 2 cards (8 cards total, no empty slot). On very short viewports (e.g., landscape phone), 4 rows may not all be visible without scrolling.
  - **Mitigation**: The grid uses `height: 100%` on `.poly-grid` and `align-content: flex-start`, so the page scrolls naturally. This is acceptable mobile UX.
- **Risk**: 2 columns make cards wider (~195px at 390px), which could make them look too wide/tall with `height: 25%`.
  - **Mitigation**: The `height: 25%` keeps the aspect ratio reasonable. Cards will have more breathing room for text. If cards feel too tall, reduce `height` to `20%` (5 rows would require content-based sizing) — but try 25% first.
- **Risk**: Changing `height: 26%` → `height: 25%` is a minor visual change; some cards may appear slightly shorter.
  - **Mitigation**: This is a net positive — the current 26% causes 4 rows × 26% = 104% overflow. 25% = 100% with clean math.

## Verification

1. **Build check:**
   ```sh
   cd /home/susliko/programming/lacoquille && npm run build
   ```
   Must pass with no errors.

2. **Mobile viewport (390×844):**
   - Open `http://localhost:4321/`
   - Switch to the Knowledge tab
   - Verify: exactly 2 cards per row, 4 rows total (2+2+2+2 layout, no empty slot)
   - Verify: card text fits cleanly (e.g., "personal · relative" on 1 line, "tenses & conjugation" on 1 line)

3. **Desktop viewport (1280×720):**
   - Open the same page
   - Verify: 3 cards per row (desktop layout unchanged)

4. **Other pages:**
   - Check `/practice/typing`, `/reference/verbs`, and any other card grids — confirm they are unaffected

## Out of Scope

- Changes to other responsive breakpoints (e.g., tablet 641px–1023px)
- Changes to the Practice Hub layout (already 2×2)
- Changes to the home page tab switcher mechanism
- Font-size adjustments to `.poly-card-name` (defer unless reviewer reports issues)

## Branch and PR

- **Branch:** `fix/issue-019-mobile-grid`
- **PR title:** `fix(home): use 2 columns on mobile to fix cramped cards (ISSUE-019)`