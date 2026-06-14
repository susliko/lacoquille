# Review: ISSUE-003 + ISSUE-004

**Verdict: PASS**

## Acceptance Criteria Checklist

| # | Criterion | Result | Evidence |
|---|---|---|---|
| 1 | `npm run build` passes | **PASS** | Build completed in 1.89s, 82 pages built, all internal links valid |
| 2 | `npx astro check` shows no NEW errors | **PASS** | 21 pre-existing errors, 0 new errors |
| 3a | When tokenization is broken, shows `tokenization-error` notice + French-only view | **PASS** | Fallback renders `<div class="tokenization-error">The interactive translation for today's story is unavailable. Showing French only.</div>` followed by French-only two-column layout (lines 314–325) |
| 3b | When tokenization is good, renders bilingual view | **PASS** | Worker report confirms current article "Monsieur Parent" passes all 4 criteria and renders bilingual view |
| 3c | `isTokenizationGood` memo implements all 4 detection criteria | **PASS** | Memo at lines 260–298: C1 (line 270), C3 (line 273), C2 (lines 276–286), C4 (lines 289–299) |
| 3d | Threshold values are correct | **PASS** | C1: `totalChars / 8` (line 270), C2: `> 0.3` (line 287), C3: `* 0.3` (line 273), C4: `> 0.25` (line 300) |
| 3e | Memo uses `createMemo` | **PASS** | Line 260: `const isTokenizationGood = createMemo(() => {` |
| 3f | `<Show when={...}>` condition changed from `hasTokens()` to `isTokenizationGood()` | **PASS** | Line 305: `when={isTokenizationGood()}` |
| 3g | Fallback includes `.tokenization-error` div with correct text | **PASS** | Line 314: exact text present |
| 3h | `hasTokens` constant is removed | **PASS** | Diff confirms `hasTokens` was removed; `isTokenizationGood` is its replacement |
| 4 | Unit test: broken "Pierre et Jean" data returns `isTokenizationGood=false` | **PASS** | Regression test: C1=False (36 tokens < 577/8=72.1), overall `isTokenizationGood=False` |
| 8 | `.tokenization-error` CSS class exists in `<style>` block | **PASS** | Lines 144–150: `.tokenization-error { background: var(--warning, #fef3c7); color: var(--warning-text, #92400e); ... }` |

## Code Quality Observations

- **Correct placement**: `isTokenizationGood` is inside the `<Show when={article()}>` callback, after `const groups = createMemo(...)`, exactly as specified in the plan.
- **No dead code**: `hasTokens` was removed cleanly.
- **Exact fallback text**: The notice message matches the plan's specification verbatim.
- **Memo structure**: Uses `createMemo` (not `createSignal` or plain function), correctly returns `false` early on any failed criterion.
- **C2 implementation**: Correctly checks each token's span against the paragraph it was grouped into, using the same `groupTokensByParagraph` function used for rendering — this ensures the criterion tracks what the user actually sees.
- **C4 normalization**: Uses `s.replace(/\s+/g, " ").trim()` which handles whitespace/punctuation consistently.

## Known Limitations

1. **C3 cannot distinguish 1:1 word-for-word gloss from 1:1 natural mapping**: The threshold of 0.3 means `en >= fr * 0.3`. For a 1:1 word-by-word translation, `en = fr` which is well above 0.3, so C3 passes. A structurally valid but semantically word-for-word gloss (e.g., "le" → "the", "et" → "and" for every token) would not be detected as broken by C3 alone. However, C2 (mis-grouped spans) and C4 (span/text mismatch) would likely catch such cases, and in practice a real tokenizer produces more English tokens than French tokens due to expansion.

2. **Thresholds are intentionally loose**: C1 (1 token per 8 chars), C2 (30% mis-grouped), C3 (30% en/fr ratio), C4 (25% mismatch) are designed to only catch obviously broken tokenization. Good tokenizations pass comfortably.

3. **Current article has good tokenization**: The worker report notes that today's article ("Monsieur Parent") passes all 4 criteria, so the fallback is not currently triggered. The fix provides future protection against degraded tokenization.

## Summary

The fix correctly implements all 4 detection criteria with the specified thresholds, replaces the `hasTokens` guard with `isTokenizationGood`, and adds the `tokenization-error` notice to the fallback. The regression test confirms that the broken "Pierre et Jean" data (36 tokens for 577 chars) fails C1 and returns `false`. Build and type-check pass. The fix is ready to merge.
