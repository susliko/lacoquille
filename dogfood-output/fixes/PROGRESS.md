# Live Progress Log

> Append-only. Newest entries at the bottom.

## Round 1 — 2026-06-14 (DONE)

| Issue | Branch | Commit | Review |
|-------|--------|--------|--------|
| ISSUE-001 Practice hub blank | `fix/issue-001-practice-hub-blank` | `3283c84` | PASS |
| ISSUE-002 Shadowing stuck loading | `fix/issue-002-shadowing-coming-soon` | `38b04db` | PASS |

### ISSUE-001 outcome

Moved 8 `.practice-*` rules from the scoped `<style>` block in `src/pages/practice/index.astro` to `src/styles/global.css`. Kept the `main` height rule scoped (since `<main>` is SSR'd) and fixed its value from `100dvh - 64px` to `100dvh - 52px` (the site nav is 52px tall). Verified: build passes, `/practice/` renders a 2×2 grid with all four cards colored, `getComputedStyle(.practice-hub-grid).display === 'grid'`.

### ISSUE-002 outcome

Added a `SHADOWING_AVAILABLE = false` feature flag at the top of `src/components/ShadowingPractice.tsx`, a guard `if (!SHADOWING_AVAILABLE) return <ComingSoon />;` as the first statement of the exported function, and an inline `ComingSoon` component with a friendly card, a hint linking to `/practice/typing` as an alternative, and a "← Back to practice hub" link. The 880-line body is preserved for the day the backend comes back. Verified: page renders immediately, no `fetch` to `/api/stories`, all internal links valid.

### Open follow-ups from Round 1

- Reviewer flagged the dev server crashed during ISSUE-001 review; restarted. New process: `astro-dev-2`.
- Round 1 fixes are on their branches (pushed to remote). They have not been merged into `main` yet.

## Round 2 — 2026-06-14 (in progress)

Focus: ISSUE-003 (broken translation) + ISSUE-004 (scrambled token spans) on the Article of the Day.

Both bugs share a root cause: the LLM tokenization/translation step in the backend produces a word-for-word, literal English gloss with character spans that don't actually match the source text. Trying to make the LLM produce better output has been attempted multiple times (see git log: `20e7dc9`, `26dc962`, `820fd54`); the result is still bad. The pragmatic fix is a frontend-side degradation: detect when the backend's tokenization is too broken to be useful, and show a clean "translation not available" state with the French source readable, instead of confusing literal-English that doesn't parse.

Plan: dispatch scout, then planner.
