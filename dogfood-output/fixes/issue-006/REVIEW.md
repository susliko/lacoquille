# Review — ISSUE-006: Vocabulary Mining Card

## Verdict: FAIL

The fix is **not implemented** on branch `fix/issue-006-vocabulary-card`. The branch is identical to `main` with respect to the ISSUE-006 code changes. The worker report's claims are not reflected in the actual codebase.

---

## Acceptance Criteria Checklist

| # | Criterion | Status |
|---|-----------|--------|
| 1 | `npm run build` passes | ✅ PASS (builds cleanly, 82 pages) |
| 2 | `npx astro check` shows no NEW errors | ✅ PASS (21 pre-existing errors, 0 new) |
| 3 | Home page Practice tab shows 4 cards including "Vocabulary Mining" | ❌ FAIL — only 3 cards exist |
| 4 | Clicking the card navigates to `/practice/vocabulary/` (no 404) | ❌ FAIL — route does not exist |
| 5 | The page shows a "coming soon" message | ❌ FAIL — page does not exist |
| 6 | The page makes no `fetch` calls | ❌ FAIL — page does not exist |

---

## Evidence

### Branch state

```sh
$ git log main..HEAD --oneline
# (empty — branch is at same commit as main)

$ git diff main..HEAD -- src/components/HomepageTabs.tsx
# (no changes to PRACTICE array)

$ git diff main..HEAD -- src/components/VocabularyPractice.tsx
# (file does not exist)
```

### `PRACTICE` array (still 3 entries, no vocabulary):

```tsx
const PRACTICE = [
  { id: "typing",    name: "Typing Race",    sub: "speed conjugate drills", color: "#ff6b35", path: "/practice/typing" },
  { id: "shadowing", name: "Shadowing",      sub: "listen & repeat",       color: "#ff9f43", path: "/practice/shadowing" },
  { id: "article",   name: "Article of Day", sub: "daily Maupassant story", color: "#ff4757", path: "/stories/article-of-the-day" },
];
```

### Missing files

- `src/components/VocabularyPractice.tsx` — **Does not exist**
- `src/pages/practice/vocabulary.astro` — **Does not exist**

### Build & type check

```
$ npm run build
✓ All internal links are valid. (82 pages)

$ npx astro check
Result (58 files): 21 errors, 0 warnings, 6 hints
# All 21 errors are pre-existing (Base.astro prop mismatches, breadcrumb type issues)
# Zero errors introduced by this branch
```

### 404 on `/practice/vocabulary/`

```sh
$ curl http://localhost:4321/practice/vocabulary
<!doctype html>
<html lang="en">
  <title>404: Not Found</title>
```

---

## What Was Actually Changed on This Branch

The branch diff from `main` includes unrelated changes from other issues:

- `src/components/ArticleOfTheDay.tsx` — ISSUE-003: graceful degradation when tokenization is broken
- `src/pages/practice/index.astro` — ISSUE-001: make /practice/ visible (removed inline styles)
- `src/pages/reference/grammar/index.astro` — ISSUE-007: grammar discoverability
- `src/styles/global.css` — ISSUE-019: 2-column mobile grid
- `src/components/HomepageTabs.tsx` — ISSUE-019: added "See all grammar →" link

None of these implement the Vocabulary Mining card fix.

---

## Merge Recommendation: **DO NOT MERGE**

The branch does not address ISSUE-006. The worker report (`PROGRESS.md`) claims the fix was completed and committed (`cc55189`), but that commit does not exist on this branch:

```sh
$ git log --oneline -1
14166b8 fix(home): use 2 columns on mobile to fix cramped cards (ISSUE-019)
```

The branch needs the actual implementation:
1. Add `vocabulary` entry to the `PRACTICE` array in `HomepageTabs.tsx` (4th card)
2. Create `src/components/VocabularyPractice.tsx` placeholder component
3. Create `src/pages/practice/vocabulary.astro` page

---

## Note

The PROGRESS.md file in `dogfood-output/fixes/issue-006/` may need to be corrected. The evidence screenshot (`EVIDENCE.png`) was likely taken from a different working state (perhaps a local uncommitted branch) rather than from this pushed branch.