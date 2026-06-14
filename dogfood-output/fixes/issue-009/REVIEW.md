# Review: ISSUE-009 + ISSUE-017 — Typing Race UX Fix

**Branch:** `fix/issue-009-typing-race-ux` (`18e2d2d`)
**Reviewer:** review-subagent
**Date:** 2026-06-14

---

## Verdict: ✅ PASS

All four targeted changes are present and correct. The fix is ready to merge.

---

## Code-Level Acceptance Criteria

| # | Criterion | Status | Line refs |
|---|-----------|--------|-----------|
| 1 | `npm run build` passes | **PASS** | Build output: 82 pages, all internal links valid |
| 2 | `npx astro check` shows no NEW errors | **PASS** | 21 pre-existing errors (confirmed identical on `main`); 0 new errors introduced by this branch |
| 3a | `text` memo uses `paragraphs?.join("\n\n")` (all paragraphs) | **PASS** | `src/components/TypingRace.tsx:23` |
| 3b | `createEffect` has `text().length > 0` guard | **PASS** | `src/components/TypingRace.tsx:108` |
| 3c | `createEffect` has `setTimeout(() => containerRef?.focus(), 100)` fallback | **PASS** | `src/components/TypingRace.tsx:110` |
| 3d | CSS: `.tr-text-area:focus-visible { outline: 2px solid var(--coral, #ff4757); outline-offset: 2px; }` | **PASS** | `src/components/TypingRace.tsx:201–204` |
| 3e | Hint text is `"start typing"` (no "click the text ·") | **PASS** | `src/components/TypingRace.tsx:377` |

---

## Issues Found

None.

---

## Notes & Observations

1. **Pre-existing errors are unrelated.** The 21 `astro check` errors are all in `Base.astro` prop mismatches and `ShadowingPractice.tsx` unused variables — none touch `TypingRace.tsx` and all exist on `main`.

2. **The `setTimeout` fires on every reactive update**, not just once. Since `containerRef` is a constant ref and `article()` only resolves once, the effect fires at most twice in practice (initial + article load). This is acceptable. If strict one-time-only behavior is desired, a `once` flag could be added, but it is not required by the spec.

3. **Browser `:focus-visible` caveat noted in plan.** Some browsers may not show the ring for programmatic `focus()` calls. This is acceptable per the plan — the cursor is still placed and keys register.

4. **WPM impact for multi-paragraph stories is expected.** Per plan risk #1: adding paragraphs increases character count, which lowers WPM readout for users who historically completed races quickly. This is correct behavior and should be documented in the PR description.

---

## Conclusion

All four changes are present, correct, and minimal. Build and type-check pass. No new issues introduced. **Merge is approved.**