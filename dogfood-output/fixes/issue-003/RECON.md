# Recon: ISSUE-003 + ISSUE-004 — Broken Translation & Scrambled Token Spans

**Date:** 2026-06-14
**Scope:** `src/components/ArticleOfTheDay.tsx`, `lacq/src/lib.rs`, `lacq/src/tokenizer.rs`
**Status:** Recon complete — no code changes made

---

## 1. Interface Definitions & Props (from `ArticleOfTheDay.tsx`)

### `FrToken` (lines 3–10)
```typescript
interface FrToken {
  text: string;
  trailingPunct?: string | null;
  leadingPunct?: string | null;
  translation: string;
  spans: [number, number][];
  en_indices: number[];
}
```

### `EnToken` (lines 12–15)
```typescript
interface EnToken {
  text: string;
  index: number;
}
```

### `Tokenized` (lines 17–20)
```typescript
interface Tokenized {
  fr_tokens: FrToken[];
  en_tokens: EnToken[];
}
```

### `ArticleData` (lines 22–29)
```typescript
interface ArticleData {
  title: string;
  source: string;
  published_year: number;
  paragraphs: string[];
  tokenized?: Tokenized;
  tokenization_error?: string;
}
```

### Key helper functions

**`paraStart`** (lines 41–46) — character offset of paragraph `i` within `paragraphs.join("\n\n")`:
```typescript
function paraStart(paras: string[], i: number): number {
  let offset = 0;
  for (let k = 0; k < i; k++) offset += paras[k].length + 2; // +2 for "\n\n"
  return offset;
}
```

**`buildFrParts`** (lines 51–68) — renders a paragraph as original text with token spans overlaid as clickable regions. Skips spans that fall outside the paragraph or overlap an earlier token:
```typescript
function buildFrParts(
  paraText: string,
  paraOffset: number,
  tokenIdxs: number[],
  tokens: FrToken[],
): FrPart[] {
  const parts: FrPart[] = [];
  let cursor = 0;
  for (const gi of tokenIdxs) {
    const span = tokens[gi]?.spans?.[0];
    if (!span) continue;
    const s = span[0] - paraOffset;
    const e = Math.min(span[1] - paraOffset, paraText.length);
    // Skip spans that fall outside this paragraph or overlap an earlier token.
    if (s < cursor || s >= paraText.length || e <= s) continue;
    if (s > cursor) parts.push({ text: paraText.slice(cursor, s) });
    parts.push({ text: paraText.slice(s, e), gi });
    cursor = e;
  }
  if (cursor < paraText.length) parts.push({ text: paraText.slice(cursor) });
  return parts;
}
```

**`groupTokensByParagraph`** (lines 73–93) — assigns each token (by its index in `fr_tokens`) to a paragraph using the character span computed against `paragraphs.join("\n\n")`. If a span can't be matched, the token is carried into the current paragraph:
```typescript
function groupTokensByParagraph(tokens: FrToken[], paras: string[]): number[][] {
  const groups: number[][] = paras.map(() => []);
  if (paras.length === 0) return groups;

  const ranges: [number, number][] = [];
  let offset = 0;
  for (const p of paras) {
    const start = offset;
    const end = offset + p.length;
    ranges.push([start, end]);
    offset = end + 2; // "\n\n" separator
  }

  let current = 0;
  tokens.forEach((tok, gi) => {
    const pos = tok.spans?.[0]?.[0];
    if (typeof pos === "number") {
      const found = ranges.findIndex(([s, e]) => pos >= s && pos < e);
      if (found !== -1) current = found;
    }
    groups[current].push(gi);
  });
  return groups;
}
```

---

## 2. Page Wrapper (`article-of-the-day.astro`)

```astro
---
import Base from "../../layouts/Base.astro";
import ArticleOfTheDayComponent from "../../components/ArticleOfTheDay";
---

<Base title="Article of the Day — La Coquille">
  <main>
    <ArticleOfTheDayComponent client:only="solid-js" />
  </main>
</Base>
```

The page is a thin wrapper. All logic lives in the SolidJS island.

---

## 3. API Data Shape

The API response is shaped as `ArticleResponse` (`lacq/src/lib.rs` lines 217–225):

```rust
pub struct ArticleResponse {
    pub title: String,
    pub source: String,
    pub published_year: i32,
    pub paragraphs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokenized: Option<TokenizedPayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokenization_error: Option<String>,
}

pub struct TokenizedPayload {
    pub fr_tokens: Vec<FrToken>,
    pub en_tokens: Vec<EnToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

pub struct FrToken {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_punct: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leading_punct: Option<String>,
    pub translation: String,
    pub spans: Vec<[usize; 2]>,
    pub en_indices: Vec<usize>,
}

pub struct EnToken {
    pub text: String,
    pub index: usize,
}
```

The combined text passed to the tokenizer is `paragraphs.join("\n\n")`, capped at 1200 characters total (`lib.rs` lines 272–290). Token character spans index into this same string.

---

## 4. Where the Component Decides "Is Tokenization Good Enough to Render?"

**Nowhere.** The component only checks whether `tokenized` is present at all:

```typescript
// ArticleOfTheDay.tsx line 252
const hasTokens = () => !!data().tokenized?.fr_tokens?.length;
```

It does **not** validate:
- Whether token spans are in-bounds for the paragraph they're assigned to
- Whether `fr_tokens` count is reasonable for the paragraph character count
- Whether `en_tokens` count is reasonable relative to `fr_tokens`
- Whether the English translation is coherent prose vs. a word-for-word gloss

The `buildFrParts` function silently skips out-of-bounds spans (line 64: `if (s < cursor || s >= paraText.length || e <= s) continue;`), so broken spans produce invisible tokens rather than a graceful fallback. This is the mechanism by which 16 of 36 tokens silently disappear from the UI.

---

## 5. Where the Component Decides "Is the Article Fully Translated?"

**Nowhere.** The component renders the English column in full whenever `hasTokens()` is true, regardless of how many paragraphs are covered by the token set. Only the fallback (`<Show when={hasTokens()} fallback={...}>`) handles the complete absence of tokenization.

---

## 6. Where the `tokenization_error` Field Is or Could Be Surfaced

The field is already wired end-to-end:

| Location | Role |
|---|---|
| `lacq/src/lib.rs:224` | `ArticleResponse.tokenization_error: Option<String>` — populated when all providers fail (line 308) |
| `lacq/src/lib.rs:291–330` | `compute_article_of_the_day` sets `tokenization_error` on `Err` from `get_tokenized` |
| `ArticleOfTheDay.tsx:270–274` | Renders a `tokenization-error` div when the field is present |

**Existing render:**
```typescript
<Show when={data().tokenization_error}>
  <div class="tokenization-error">
    {data().tokenization_error}
  </div>
</Show>
```

The existing `tokenization_error` path only fires when the LLM call **fails entirely** (network error, API error, all providers exhausted). It does **not** fire when the LLM returns a structurally valid but semantically broken response (wrong word order, missing paragraphs, bad spans). This is the gap to fill.

---

## 7. Detection Criteria for Broken Tokenization

### C1: `fr_tokens` count vs. paragraph character count
A paragraph of ~200 characters should have roughly 30–60 tokens. If the sum of `fr_tokens` across all paragraphs is far below `paragraphs.join("").length / 3`, the model dropped content. Conversely, if many tokens have empty or single-character spans, something is wrong.

**Implementation:** In `ArticleOfTheDay.tsx`, compute `totalChars = paras().join("").length` and `totalTokens = tokens().length`. Flag as broken if `totalTokens < totalChars / 8` (generous threshold).

### C2: P1 tokens whose span is outside P1's character range
This is the concrete bug from ISSUE-004. The `groupTokensByParagraph` function assigns tokens by scanning spans in order; when a P1 word like "tout" has span `[445, 449]` (pointing into P5), it gets mis-grouped. Ten P1 tokens (tout, à, le, Roland, les, et, par, au, de, la) are in the wrong paragraph.

**Implementation:** In `groupTokensByParagraph` or a pre-pass, check each token's span start against the paragraph ranges. Count how many tokens fall into the wrong paragraph. If >X% of tokens are misplaced, degrade.

### C3: `en_tokens` count vs. `fr_tokens` count
A real translation should have roughly the same number of English tokens as French tokens (within ~2×). A word-for-word gloss may have more tokens (each French word → separate English word, articles, etc.). But the broken output has **only 36 English tokens** for ~5 paragraphs of French, meaning ~80% of the content is untranslated.

**Implementation:** In the component, check `en_tokens.length / fr_tokens.length < 0.3` as a red flag.

### C4: `fr_tokens` text matching the actual paragraph text at the given span
After `repair_spans` in the backend, most tokens should match. But `repair_spans` does a fuzzy best-match, so a token might be placed at a wrong-but-close position. We can verify in the frontend by checking `paraText.slice(s, e) === tokens[gi].text` (with normalization).

**Implementation:** In `buildFrParts`, log or count mismatches. If >10% of tokens don't match the text at their span, degrade.

### C5: `en_tokens` being a 1:1 word-for-word gloss
This is the ISSUE-003 symptom. Detecting it programmatically is hard without a reference translation, but a few cheap heuristics exist:
- English tokens are almost all single words (no multi-word phrases)
- English articles ("the", "a") are present but in wrong positions relative to noun-adjective order
- The English side covers far fewer paragraphs than the French side

**Implementation:** Check if `en_tokens.length < fr_tokens.length * 0.5` across all paragraphs (i.e., fewer English tokens than French tokens suggests heavy omission).

### C6: `tokenization_error` field present
The existing path. Not applicable here (the LLM succeeds but produces bad output), but useful as a catch-all for future provider failures.

---

## 8. Available Fix Approaches and Tradeoffs

### Approach A: Backend Quality Gate (in `lacq`)

Before returning the article, run a sanity check on `fr_tokens`:
- All spans must be within `[0, combined_text.len())`
- All spans must match the text at that position (after `repair_spans`)
- Token count must be reasonable (e.g., `fr_tokens.len() >= total_chars / 5`)
- No more than 10% of tokens may have spans that don't match their paragraph

If the check fails, set `tokenization_error` and clear `tokenized` — the frontend then renders the fallback view (French-only with "Translation temporarily unavailable").

**Pros:**
- Frontends get clean data; bad data never leaks
- The existing `tokenization_error` path already handles this gracefully
- One place to maintain the quality logic

**Cons:**
- Requires Rust code changes (new function in `lib.rs` or `tokenizer.rs`)
- "Good enough" threshold is inherently fuzzy — might reject a marginally-OK translation
- Re-running the LLM on failure is expensive; simply degrading is better UX
- `repair_spans` already does some of this work, but it drops bad tokens silently rather than surfacing an error

**Where it would go:** In `lib.rs` `compute_article_of_the_day` (around line 291), after `get_tokenized` succeeds but before constructing `TokenizedPayload`. If the sanity check fails, set `tokenization_error` and leave `tokenized = None`.

### Approach B: Frontend Degradation (in `ArticleOfTheDay.tsx`)

In the component, detect broken tokenization using criteria C1–C5, and if broken, fall back to the French-only view (the existing fallback that already shows for `!hasTokens()`).

**Pros:**
- Contained to one component, no backend change
- Immediate — no Rust compilation needed
- Can be tuned iteratively based on real data
- Matches the existing fallback pattern exactly

**Cons:**
- The user still doesn't get a translation — just French text and a notice
- Quality detection is heuristic and may have false positives/negatives
- The `tokenization_error` message is a generic string; we could improve it with a specific reason ("translation incomplete")

**Implementation pattern:** The existing fallback is:
```typescript
<Show
  when={hasTokens()}
  fallback={
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
  }
>
```

The fix would replace `hasTokens()` with a smarter check:
```typescript
const isTokenizationGood = createMemo(() => {
  const t = data().tokenized;
  if (!t?.fr_tokens?.length) return false;
  // detection criteria...
  return true;
});
```

### Approach C: Hide the English Column Entirely

Simply don't render the `en-col` when tokenization is broken. The French column is still interactive (click-to-highlight the word itself), and the English column is replaced with a notice.

**Pros:** Simplest implementation — just add `display: none` or omit the column
**Cons:** User gets nothing for the English side; the root problem (bad tokenization) is hidden, not fixed

### Approach D: Use a Known-Good Offline Translation

Pre-translate a curated set of articles and serve them from a JSON file.

**Pros:** User always gets a real translation for curated articles
**Cons:** Only works for the curated set; the "daily" part breaks; significant authoring overhead

---

## 9. Tokenizer Call Details (from `lacq/src/tokenizer.rs`)

### Prompt construction (`build_prompt`, lines 188–224)
The prompt instructs the LLM to:
- Tokenize French → English for bilingual reading
- Natural phrase boundaries (multi-word units together)
- Contractions stay together ("d'une", "au", "l'homme")
- Spans are **character offsets** in the original French text
- Translation **must include trailing punctuation**
- Respond with a single JSON object only

Example input: `"La fortune."` → one token with span `[[0, 11]]`, translation `"The fortune."`

### Response parsing
Each provider (`tokenize_openrouter`, `tokenize_groq`, `tokenize_gemini`):
1. Calls the API with `temperature: 0.2`, `max_tokens: 4096`
2. Strips markdown code fences defensively
3. Deserializes to `TokenizedText`
4. Calls `repair_spans(french_text, &mut parsed.fr_tokens)` — the key post-processing step

### `repair_spans` (lines 260–298)
For each token, searches the original text for the best-matching position (fuzzy, character-based). If found, replaces the span. If not found, **drops the token silently**. This is why bad spans produce invisible tokens rather than errors — the repair step rescues most cases, but silently drops the rest.

### Where a quality gate would go in the backend
After `repair_spans` and before returning from `tokenize_*` functions, add a validation pass:
```rust
fn validate_tokenization(text: &str, tokens: &[FrenchToken]) -> Result<(), String> {
    let total_chars = text.chars().count();
    let token_count = tokens.len();
    
    // Must have at least one token per ~8 chars (generous)
    if token_count < total_chars / 8 {
        return Err("Too few tokens for text length".to_string());
    }
    
    // Check each token's repaired span matches the text
    for token in tokens {
        if let Some(span) = token.spans.first() {
            let s = span.0[0];
            let e = span.0[1];
            if e > total_chars {
                return Err(format!("Token '{}' span out of bounds", token.text));
            }
            let extracted: String = text.chars().skip(s).take(e - s).collect();
            if extracted != token.text {
                return Err(format!("Token '{}' span mismatch", token.text));
            }
        }
    }
    Ok(())
}
```

This would be called in `lib.rs` after `get_tokenized` succeeds, with `Err` routing to the existing `tokenization_error` path.

---

## 10. Comparison: Approach A vs. B

| Dimension | A: Backend Quality Gate | B: Frontend Degradation |
|---|---|---|
| Scope of change | `lacq/src/lib.rs` (+ optionally `tokenizer.rs`) | `ArticleOfTheDay.tsx` only |
| Build required | Rust recompile | No |
| Testable in isolation | Requires API key + real LLM call | Unit-testable with mock data |
| Granularity of error msg | Can return specific reason ("N tokens missing") | Generic "translation unavailable" |
| False positive risk | May reject marginal-but-OK translations | May degrade translations that render OK |
| Existing infrastructure | `tokenization_error` path already exists | Existing fallback pattern already exists |
| Heuristic complexity | Must run in Rust without DOM context | Can access paragraph text + token data directly |

**Recommendation: Approach B (Frontend Degradation)** is more pragmatic for this PR because:
1. The existing `tokenization_error` path in the frontend already handles the degraded case — we just need to trigger it more aggressively
2. The detection criteria (C1–C5) are straightforward to implement in TypeScript against the already-loaded `tokenized` data
3. No Rust recompile needed; faster iteration
4. The detection is tunable without redeploying the backend
5. Approach A's `repair_spans` already handles most span-repair silently; adding a quality gate on top would be partially redundant

If Approach B proves insufficient (too many false positives, or the degraded state is unsatisfying for users), Approach A can be added in a follow-up PR with the benefit of real-world data from Approach B's telemetry.

---

## 11. Existing Error-Handling Code (for style matching)

### `tokenization_error` branch (`ArticleOfTheDay.tsx` lines 270–274)
```typescript
<Show when={data().tokenization_error}>
  <div class="tokenization-error">
    {data().tokenization_error}
  </div>
</Show>
```

### Loading state (`ArticleOfTheDay.tsx` lines 244–247)
```typescript
<Show when={article.loading}>
  <div class="loading-state">Loading today's story...</div>
</Show>
```

### Error state (`ArticleOfTheDay.tsx` lines 249–254)
```typescript
<Show when={article.error}>
  <div class="error-state">
    Unable to load today's story. Make sure the lacq server is running on port 8080.
    <br />
    <small>{article.error?.message}</small>
  </div>
</Show>
```

### Existing CSS for error states
```css
.tokenization-error {
  background: var(--warning, #fef3c7);
  color: var(--warning-text, #92400e);
  padding: 0.75rem 1rem;
  border-radius: 0.5rem;
  margin-bottom: 1.5rem;
  font-size: 0.9rem;
}
.loading-state,
.error-state {
  text-align: center;
  padding: 3rem;
  color: var(--text-2);
}
.error-state {
  color: var(--error);
}
```

---

## 12. Related Issues to Be Aware Of

| Issue | Summary | Same PR? |
|---|---|---|
| **ISSUE-008** | Active-token highlight is barely visible (`rgba(59, 130, 246, 0.2)` is too subtle) | **No** — visual/contrast fix, separate concern |
| **ISSUE-013** | No link, no author info, no context on the article page | **No** — content/UX fix, separate concern |
| **ISSUE-015** | Mobile tooltip overlaps the previous line of text | **No** — mobile CSS fix, separate concern |

All three are independent of the tokenization quality issue and should be left for Round 5 (or a separate visual polish PR). The tokenization fix (B) should be scoped to a single PR addressing only ISSUE-003 + ISSUE-004.

---

## Summary: What the Next Agent Needs to Know

**Start in `src/components/ArticleOfTheDay.tsx`.**

The fix is a new `createMemo` called `isTokenizationGood` that replaces `hasTokens()` in the `<Show when={...}>` condition. The memo checks:
1. `fr_tokens.length >= total_chars / 8` (C1)
2. Count of tokens assigned to the wrong paragraph < 20% of total (C2 — needs a pre-pass over `groupTokensByParagraph` output)
3. `en_tokens.length >= fr_tokens.length * 0.3` (C3 + C5)

If any check fails, render the existing fallback (French-only with "[No translations available]") and show a `tokenization-error` message: `"The interactive translation is incomplete for today's story. Showing French text only."`

The existing `tokenization_error` CSS class and `<Show>` branch already handle the rendering. No new styles needed.
