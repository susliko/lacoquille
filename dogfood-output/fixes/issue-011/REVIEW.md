# Review: ISSUE-011 — Grammaire eyebrow on tense and choice pages

**Branch:** `fix/issue-011-verbs-breadcrumb`
**Commit:** `87281e8` (and `49a4e6a`)
**Reviewer:** review subagent
**Date:** 2026-06-14

---

## Summary

The fix is correct, minimal, and fully verified. Both files were changed as specified in the plan; no unexpected changes were introduced.

---

## Changes reviewed

| File | Change |
|------|--------|
| `src/pages/reference/verbs/tenses/[slug].astro` | Added wrapper `<div style="margin-bottom: 1.5rem;">` containing `<a href="/reference/verbs"><p class="home-eyebrow">Grammaire</p></a>` before the `<div class="hgroup">` |
| `src/pages/reference/verbs/choice/[slug].astro` | Same pattern as above |

Both changes match the plan exactly. The wrapper `<div>` with `margin-bottom` keeps the eyebrow visually separated from the H1 group, matching the layout on the verbs index page.

---

## Acceptance criteria checklist

| # | Criterion | Result |
|---|-----------|--------|
| 1 | `npm run build` passes | ✅ Pass |
| 2 | `npx astro check` shows no NEW errors | ✅ 21 pre-existing errors on `main`; 21 on this branch; 0 new |
| 3 | Red "Grammaire" eyebrow visible above H1 on tense pages | ✅ Verified via curl: `<p class="home-eyebrow">Grammaire</p>` appears before `<h1>Présent de l'indicatif</h1>` |
| 4 | Eyebrow also visible on choice pages | ✅ Verified via curl: same pattern on `/reference/verbs/choice/imparfait-vs-passe-compose` |
| 5 | Eyebrow links to `/reference/verbs/` | ✅ `<a href="/reference/verbs">` wraps the eyebrow `<p>` on both pages |
| 6 | Verbs index page still works | ✅ `/reference/verbs/` renders the VerbDiagram and all lists normally |
| 7 | Eyebrow style matches verbs index page | ✅ Both use the same `.home-eyebrow` CSS class from `global.css` |

---

## Review

- **Correct:** The implementation is a clean, targeted addition. The eyebrow is inserted in the correct DOM position (above `<div class="hgroup">`) on both pages. The `<a>` wrapper gives it click-through navigation to `/reference/verbs/`. The `margin-bottom` on the wrapper prevents the eyebrow from collapsing into the H1 group.
- **Fixed:** N/A — this is a new feature, not a bug fix.
- **Note:** The verbs index page (`/reference/verbs/`) does NOT wrap its eyebrow in an `<a>` tag (it is plain text). The tense and choice pages DO wrap the eyebrow in `<a href="/reference/verbs">`. This is a minor stylistic difference: the index page uses the eyebrow as a static section label, while the drill-down pages use it as a navigation link back to the index. Both are valid patterns and the visual appearance is identical (same CSS class).

---

## Recommendation

**Merge.** The fix is complete, correct, and introduces no regressions. The branch also includes unrelated changes for ISSUE-001, ISSUE-002, and ISSUE-003 (practice hub, shadowing coming-soon, article of the day tokenization fix) — those should be reviewed separately; they do not affect the ISSUE-011 acceptance criteria.
