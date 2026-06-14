# Review: ISSUE-019 — Mobile (390px) home grid is still 3 columns, text wraps awkwardly

**Branch:** `fix/issue-019-mobile-grid` (commit 14166b8)
**Reviewer:** agent review subagent
**Date:** 2026-06-14

---

## Summary

The fix is minimal, correct, and well-scoped. Two CSS values in the `@media (max-width: 640px)` block were changed: `flex-basis` from `calc(100% / 3)` to `calc(100% / 2)`, and `height` from `26%` to `25%`. Both changes are exactly as specified in the plan.

---

## Acceptance Criteria Checklist

| # | Criterion | Status |
|---|-----------|--------|
| 1 | `npm run build` passes | ✅ Passes — 82 pages built, all internal links valid |
| 2 | At 390px viewport, the home page Knowledge grid renders 2 cards per row, not 3 | ✅ Verified via agent-browser eval — 8 cards at positions with 2 distinct x values (16, 188) and 4 distinct y values (95, 288, 481, 675) |
| 3 | The 8 cards lay out as 2+2+2+2 (no empty slot) | ✅ Exactly 8 cards with real dimensions; 3 ghost cards at (0,0) are template elements |
| 4 | Text in each card fits cleanly | ✅ Cards are 172px wide (vs ~117px at 3-col); 25% height gives 193px per card |
| 5 | Desktop layout (≥640px) is unchanged | ✅ Verified at 1280×720 — 3 distinct x positions (57, 441, 825) and 3 rows |
| 6 | Other pages are not affected | ✅ `/practice/typing` and `/reference/verbs` load correctly at mobile viewport |

---

## Review

### Correct
- The diff targets exactly the two values described in the plan, in the correct `@media (max-width: 640px)` block
- The `flex: 0 0 calc(100% / 2)` change is clean and maintainable
- The `height: 25%` change fixes the overflow: 4 rows × 25% = 100% vs. the previous 4 × 26% = 104%
- The build produces no errors, only pre-existing route-conflict warnings unrelated to this change

### Fixed
- Mobile viewport (390px) now shows 2 cards per row instead of 3, with each card 172px wide — ample room for text
- The 4-row × 2-column layout uses all 8 cards with no empty slot, exactly matching the plan

### Note
- The diff also includes the `.practice-hub-*` styles block (from commit 667db76) — these are unrelated to ISSUE-019 but are benign additions on the same branch
- The ghost cards at (0,0) are template elements used by the animation system and do not affect the rendered output

---

## Merge Recommendation

**Approve.** The fix is correct, minimal, and meets all acceptance criteria. Ready to merge into `main`.