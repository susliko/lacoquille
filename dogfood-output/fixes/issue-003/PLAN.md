# Implementation Plan

## Goal
Replace the "always render the bilingual view if any tokens are present" logic in `ArticleOfTheDay.tsx` with a quality check. When tokenization is broken (any of C1–C4 fails), show the French-only view with a "translation temporarily unavailable" notice. When tokenization is good, render the bilingual view as today. The user must never see the broken literal-English gloss with only the first paragraph translated.

---

## Tasks

### 1. Add `isTokenizationGood` memo to `ArticleOfTheDay.tsx`

**File:** `src/components/ArticleOfTheDay.tsx`

**Changes:** Inside the `Show when={article()}` callback (where `hasTokens` is currently defined at line ~252), add a new `createMemo` called `isTokenizationGood` that runs all four detection criteria.

**Placement:** Add it immediately after `const hasTokens = () => !!data().tokenized?.fr_tokens?.length;` (line ~252), before `const tokens = () => ...`.

**Code to insert:**

```typescript
const isTokenizationGood = createMemo(() => {
  const t = data().tokenized;
  if (!t?.fr_tokens?.length || !t?.en_tokens?.length) return false;
  const totalChars = paras().join("").length;
  const frTokens = t.fr_tokens;
  const enTokens = t.en_tokens;
  const groups = groupTokensByParagraph(frTokens, paras());

  // C1: token count vs text length
  if (frTokens.length < totalChars / 8) return false;

  // C3: English coverage (en/fr ratio)
  if (enTokens.length < frTokens.length * 0.3) return false;

  // C2: mis-grouped tokens — count tokens whose span start is NOT in the
  // paragraph they were grouped into
  let misplaced = 0;
  for (let pIdx = 0; pIdx < groups.length; pIdx++) {
    const paraStartOffset = paras().slice(0, pIdx).reduce((acc, p) => acc + p.length + 2, 0);
    const paraEndOffset = paraStartOffset + paras()[pIdx].length;
    for (const gi of groups[pIdx]) {
      const span = frTokens[gi]?.spans?.[0];
      if (!span) continue;
      if (span[0] < paraStartOffset || span[0] >= paraEndOffset) misplaced++;
    }
  }
  if (misplaced / frTokens.length > 0.3) return false;

  // C4: span/text mismatch — check that the actual text at the given span
  // equals token.text (with whitespace/punctuation normalization)
  const combined = paras().join("\n\n");
  let mismatched = 0;
  for (const tok of frTokens) {
    const span = tok.spans?.[0];
    if (!span) continue;
    const actual = combined.slice(span[0], span[1]);
    const norm = (s: string) => s.replace(/\s+/g, " ").trim();
    if (norm(actual) !== norm(tok.text)) mismatched++;
  }
  if (mismatched / frTokens.length > 0.25) return false;

  return true;
});
```

**Acceptance:** The memo exists in the file, compiles without TypeScript errors, and `npx astro check` reports no new errors.

---

### 2. Replace `hasTokens()` with `isTokenizationGood()` in the outer `<Show>` guard

**File:** `src/components/ArticleOfTheDay.tsx`

**Changes:** In the `<Show when={hasTokens()} fallback={...}>` block (around line ~258), change the `when` prop from `hasTokens()` to `isTokenizationGood()`.

**Before:**
```typescript
<Show
  when={hasTokens()}
  fallback={
    <div class="article-body">
      ...
    </div>
  }
>
```

**After:**
```typescript
<Show
  when={isTokenizationGood()}
  fallback={
    <div class="article-body">
      ...
    </div>
  }
>
```

**Acceptance:** The bilingual view only renders when `isTokenizationGood()` returns `true`.

---

### 3. Update the fallback body to include the `tokenization-error` notice

**File:** `src/components/ArticleOfTheDay.tsx`

**Changes:** In the fallback (French-only view), add a `tokenization-error` notice at the top of the article body. The notice must say: "The interactive translation for today's story is unavailable. Showing French only."

**Exact fallback JSX to use:**

```typescript
<Show
  when={isTokenizationGood()}
  fallback={
    <>
      <div class="tokenization-error">
        The interactive translation for today's story is unavailable. Showing French only.
      </div>
      <div class="article-body">
        <div class="language-col">
          <h2>Français</h2>
          <div class="article-paragraphs">
            <For each={paras()}>
              {(para) => <p>{para}</p>}
            </For>
          </div>
        </div>
        <div class="language-col">
          <h2>English</h2>
          <div class="article-paragraphs">
            <For each={paras()}>
              {() => <p>[No translations available]</p>}
            </For>
          </div>
        </div>
      </div>
    </>
  }
>
```

**Note:** The existing `tokenization_error` field from the API is already rendered above this `<Show>` block (lines ~270–274 in the original), so that path is preserved. The new notice in the fallback handles the case where the API call succeeded but the tokenization is semantically broken.

**Acceptance:** The fallback renders a `tokenization-error` div with the exact message above, followed by the French-only two-column layout.

---

### 4. Remove or keep `hasTokens` (optional cleanup)

**File:** `src/components/ArticleOfTheDay.tsx`

**Changes:** The `hasTokens` memo is no longer used after step 2. It can be removed to avoid dead code, or kept for documentation purposes. Recommend removing it.

**Acceptance:** No TypeScript errors from removing it; the component still compiles.

---

### 5. Verify build and type-check

**Commands:**
```sh
cd /home/susliko/programming/lacoquille && npm run build
cd /home/susliko/programming/lacoquille && npx astro check
```

**Acceptance:** Both commands pass with no new errors.

---

### 6. Run the regression verification

**Command:**
```sh
curl -s http://localhost:8080/api/article-of-the-day | python3 -c "import json,sys; d=json.load(sys.stdin); print(len(d['tokenized']['fr_tokens']))"
```

**Expected:** ~36 tokens. The new `isTokenizationGood` memo should detect this as broken (it fails C2 because ~28% of tokens are mis-grouped into wrong paragraphs, and C3 because only P1 content is covered).

**Acceptance:** The token count prints (e.g., `36`). The detection criteria will be verified by the browser test below.

---

### 7. Browser verification

**Commands:**
```sh
# Start dev servers (in background)
npm run dev:all

# Open in agent-browser, wait for hydration, check DOM state
open http://localhost:4321/stories/article-of-the-day/
# Wait 5s for SolidJS hydration
eval "({ hasBilingual: !!document.querySelector('.en-col'), hasError: !!document.querySelector('.tokenization-error'), hasFrench: document.querySelectorAll('.language-col').length })"
```

**Expected result:**
```javascript
{ hasBilingual: false, hasError: true, hasFrench: 1 }
```

`hasBilingual: false` — the English column (`.en-col`) is not rendered because `isTokenizationGood()` returned `false`.

`hasError: true` — the `.tokenization-error` notice is present.

`hasFrench: 1` — only one `language-col` is rendered (the French column; the English placeholder column is also a `language-col` so this will be `2`, which is fine — the key is `hasBilingual: false`).

**Acceptance:** The screenshot shows the French article with a yellow warning notice at the top, no English column visible, and no clickable tokens.

---

## Files to Modify

- `src/components/ArticleOfTheDay.tsx` — add `isTokenizationGood` memo, replace `hasTokens()` with `isTokenizationGood()` in the outer `<Show>`, update fallback to include the `tokenization-error` notice, optionally remove dead `hasTokens` memo

## New Files

None.

## Dependencies

Task 1 (add memo) must be complete before Task 2 (replace condition). Task 2 must be complete before Task 3 (update fallback). Tasks 4–7 are independent verification steps.

## Risks & Mitigations

| Risk | Likelihood | Mitigation |
|---|---|---|
| Thresholds are too aggressive and degrade a genuinely good tokenization | Low | C1 is very loose (1 token per 8 chars). C2 allows 30% mis-grouped. C3 allows 2:1 fr/en ratio. C4 allows 25% mismatch. A good tokenization will pass all four comfortably. |
| The broken tokenization (36 tokens, ~28% mis-grouped) slips through because thresholds are too loose | Low | The current state fails C2 (mis-grouped spans point to wrong paragraphs entirely), C3 (only P1 content covered), and C4 (spans point to wrong text). At least 2 of 4 criteria catch this case. |
| A future good tokenization has a few spans off by one character due to round-trip normalization and fails C4 | Low | 25% threshold gives 4× margin. A few off-by-one errors won't trigger it. |
| User gets less functionality (no clickable translations) when this degrades | Expected | This is strictly better than showing broken English. The French-only view + Typing Race still work. |
| `hasTokens` removal breaks something downstream | Very low | `hasTokens` is only used in the `<Show when={hasTokens()}>` condition replaced in Task 2. No other callers. |

## Out of Scope

The following are explicitly NOT addressed by this PR:
- Backend `repair_spans` changes (`lacq/src/tokenizer.rs`)
- Backend quality gate (`lacq/src/lib.rs`)
- Tokenization prompt changes
- Article author info (ISSUE-013)
- Active-token highlight color (ISSUE-008)
- Mobile tooltip overlap (ISSUE-015)

---

## Branch & PR

- **Branch:** `fix/issue-003-article-tokenization-quality`
- **PR title:** `fix(article): degrade gracefully when tokenization is broken (ISSUE-003, ISSUE-004)`
- **Reviewers:** Verify all acceptance criteria in Task 5 (build), Task 6 (API token count), and Task 7 (browser state).
