# Progress — ISSUE-003 + ISSUE-004

## Status: Completed

## Tasks
- [x] Add `isTokenizationGood` memo to `ArticleOfTheDay.tsx`
- [x] Replace `hasTokens()` with `isTokenizationGood()` in `<Show>` condition
- [x] Update fallback body to include `tokenization-error` notice
- [x] Remove dead `hasTokens` constant
- [x] Run `npm run build` — PASS
- [x] Run `npx astro check` — 21 pre-existing errors, no new errors
- [x] Browser verification — bilingual view renders correctly (tokenization is currently good)
- [x] Take screenshot to `EVIDENCE.png`
- [x] Push branch to origin
- [x] Update `dogfood-output/fixes/PROGRESS.md`

## Files Changed
- `src/components/ArticleOfTheDay.tsx` — added `isTokenizationGood` memo, replaced `<Show when={hasTokens()}>` with `<Show when={isTokenizationGood()}>`, updated fallback to include `tokenization-error` notice, removed `hasTokens` constant

## Branch & Commit
- Branch: `fix/issue-003-article-tokenization-quality`
- Commit: `667db76affeefe96cf51de5e7f69cda83bc8a4bf`
- Pushed: YES (upstream set)

## Verification Results

### Build
```
npm run build: PASS
npx astro check: 21 pre-existing errors, 0 new errors
```

### Tokenization Quality (current article "Monsieur Parent")
```
French tokens: 46
English tokens: 65
en/fr ratio: 1.41
C1 (token density): PASS (46 >= 43.5)
C3 (en coverage): PASS (65 >= 13.8)
C2 (mis-grouped): PASS (0% misplaced)
C4 (span/text): PASS (0% mismatched)
```

### Browser State
- Bilingual view renders correctly (clickable tokens in both French and English columns)
- French paragraphs show proper Maupassant text
- No `tokenization-error` notice shown (because tokenization is good)

## Notes

**Important caveat:** The current article-of-the-day ("Monsieur Parent") has GOOD tokenization — the backend has apparently been fixed or the data refreshed since the issue was filed. The bilingual view renders correctly. The fix provides future protection: if/when the backend produces broken tokenization again (e.g., the 36-token, 28%-misgrouped state described in the plan), the `isTokenizationGood` memo will detect it and the fallback will activate automatically.

The `tokenization-error` notice and French-only fallback are designed to trigger when:
- Token count is too low (C1 fails)
- English coverage is too poor (C3 fails)  
- >30% of tokens are mis-grouped into wrong paragraphs (C2 fails)
- >25% of token spans don't match the actual text (C4 fails)

## Screenshot
- Path: `dogfood-output/fixes/issue-003/EVIDENCE.png`
- Shows: Clean bilingual view with "Monsieur Parent" article, French + English columns with clickable tokens
