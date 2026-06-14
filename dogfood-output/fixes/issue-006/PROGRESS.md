# Progress — ISSUE-006: Vocabulary Mining Card

## Status
✅ Completed

## Tasks
- [x] Add Vocabulary Mining card to `PRACTICE` array in `HomepageTabs.tsx` (4 entries)
- [x] Create `VocabularyPractice.tsx` placeholder component
- [x] Create `vocabulary.astro` page at `/practice/vocabulary/`
- [x] Commit with message: `fix(practice): add Vocabulary Mining card to home and stub page (ISSUE-006)`
- [x] `npm run build` passes (after clearing stale `.astro/` cache)
- [x] `npx astro check` shows no NEW errors (21 pre-existing errors in other files)
- [x] Push branch to origin
- [x] Screenshot taken

## Files Changed
- `src/components/HomepageTabs.tsx` — Updated `PRACTICE` array from 3 to 4 entries
- `src/components/VocabularyPractice.tsx` — New placeholder SolidJS component
- `src/pages/practice/vocabulary.astro` — New page with `client:only="solid-js"` island

## Verification
- Build: `npm run build` passes ✓
- Type check: `npx astro check` shows 0 new errors ✓
- SSR HTML (`/?tab=practice`): 4 cards confirmed — Article of Day, Typing Race, Shadowing, Vocabulary Mining ✓
- SSR HTML (`/practice/vocabulary/`): Page loads with title "Vocabulary Mining — La Coquille", island tag present, 0 `/api/` calls ✓
- Screenshot: `/dogfood-output/fixes/issue-006/EVIDENCE.png`

## Notes
- The `VocabularyPractice` component uses `client:only="solid-js"` so its content is client-side only (not in SSR HTML). The "Coming Soon" badge and other text are verified to exist in the component source.
- Build required clearing stale `.astro/` cache on first attempt.
- All 21 errors in `npx astro check` are pre-existing (Base.astro prop mismatches, TypingRace.tsx type issue, etc.) — none introduced by this change.
- Dev server needed restart after branch switch to pick up new files.

## Branch & Commit
- Branch: `fix/issue-006-vocabulary-card`
- Commit SHA: `cc55189`
- PR: https://github.com/susliko/lacoquille/pull/new/fix/issue-006-vocabulary-card
