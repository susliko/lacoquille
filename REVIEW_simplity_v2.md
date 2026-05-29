# Review: v2 Implementation for Simplicity & Maintainability

## Summary

The implementation is functionally sound but has **clear refactoring opportunities** in tokenization duplication and some dead code. The config chain parsing is appropriately simple. Error messages are decent but inconsistent between providers.

---

## Findings

### ✅ Correct: Config Chain Parsing (lacq/src/config.rs)

The `translation_chain()` approach is well-designed:
- Clear precedence (Groq first, then Gemini)
- Simple iteration through models
- Appropriately minimal for the use case

**Issue (minor):** Unused variable warnings in `first_provider()`:
```rust
// Line 72, 77: api_key is bound but never used
if let Some(ref api_key) = self.groq_api_key {  // should be _api_key
```

---

### ❌ Blocker: Code Duplication in tokenize_groq() and tokenize_gemini()

**Location:** `lacq/src/tokenizer.rs`

Both functions share ~70% identical structure. Here's the duplication:

| Shared Behavior | Details |
|-----------------|---------|
| `build_prompt()` call | Identical prompt construction |
| HTTP request setup | `client.post()`, headers, JSON body |
| Status check | `if !resp.status().is_success()` |
| JSON parsing | `resp.json()` → struct |
| `repair_spans()` call | Identical post-processing |
| Provider metadata set | `parsed.provider_used = Some(...)` |

**Before (current):** ~250 lines of near-duplicate code in two functions

**After (suggested refactor):**
```rust
// Extract common request/response handling
async fn tokenize_impl<F, R>(
    client: &reqwest::Client,
    api_key: &str,
    french_text: &str,
    provider: &TranslationProvider,
    make_request: F,
    parse_response: impl FnOnce(R) -> Result<String, String>,
) -> Result<TokenizedText, String>
where
    F: FnOnce(String, &str) -> (String, serde_json::Value),  // url, body
    R: DeserializeOwned,
{
    let (url, body) = make_request(build_prompt(french_text), api_key);
    
    let resp = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("{} request failed: {}", provider.provider_name(), e))?;

    // Common error handling...
    let raw_text = parse_response(resp).await?;
    parse_tokenized_json(&raw_text, french_text, provider)
}

async fn parse_tokenized_json(
    json_str: &str,
    french_text: &str,
    provider: &TranslationProvider,
) -> Result<TokenizedText, String> {
    let mut parsed: TokenizedText = serde_json::from_str(json_str)
        .map_err(|e| format!("Failed to parse JSON from {}: {}", provider.provider_name(), e))?;
    
    repair_spans(french_text, &mut parsed.fr_tokens);
    parsed.provider_used = Some(provider.display());
    Ok(parsed)
}
```

This would reduce ~150 lines to ~80 lines.

---

### ⚠️ Note: Dead Code Warnings in Response Structs

**Location:** `lacq/src/tokenizer.rs` lines 86-113

```rust
struct GroqResponse {
    id: String,              // never read
    ...
}
struct GroqChoice {
    finish_reason: Option<String>,  // never read
}
struct GroqError {
    code: Option<String>,    // never read
}
```

**Fix:** Prefix with underscore or remove:
```rust
#[allow(dead_code)]
struct GroqResponse {
    id: String,  // API returns this but we don't use it
    ...
}
```

---

### ⚠️ Note: Error Messages Are Inconsistent

| Provider | Error Detail |
|----------|--------------|
| Groq | ✅ Extracts `error_type` and `message` from JSON |
| Gemini | ❌ Raw status + body text only |

**Gemini error handling (lines 310-318):**
```rust
return Err(format!(
    "Gemini API error ({}): {}",
    status,
    &body_text[..body_text.len().min(200)]
));
```

**Groq error handling (lines 280-290):**
```rust
if let Some(err) = err_resp.error {
    return Err(format!(
        "Groq API error ({}): {} — {}",
        status, err.error_type, err.message
    ));
}
```

**Suggestion:** Add Gemini-specific error parsing if the API returns structured errors.

---

### ✅ Correct: lib.rs get_tokenized() Logic

The exponential backoff strategy is appropriate:
```rust
let delay = 2u64.pow(attempt);  // 1s, 2s, 4s, ...
```

The `OnceCell` pattern for deduplication is correct for concurrent requests.

**Minor readability issue:** The nested async block could be extracted:
```rust
// Current: deeply nested
cell.get_or_try_init(|| async { ... }).await.cloned()

// Alternative: named function
async fn tokenize_with_fallback(&self, text: &str) -> Result<TokenizedText, String>
```

---

### ⚠️ Note: Frontend Component Readability

**Location:** `src/components/ArticleOfTheDay.tsx`

**Strengths:**
- Clear interface definitions at top
- Logical structure with `For` loops
- Good use of `Show` for conditional rendering

**Issues:**

1. **Inline `<style>` block** — Hard to maintain and can't be extracted:
```tsx
// Lines 55-120: 65 lines of CSS in JSX
<style>{`
  .article-of-the-day { ... }
  ...
`}</style>
```

**Suggestion:** Move to `ArticleOfTheDay.css` file.

2. **Repeated calculation in render:**
```tsx
// getParaOffset() is called multiple times per paragraph
const start = getParaOffset(paras, paraIndex);
const end = start + paras[paraIndex].length;
```

**Before:** Inline in JSX, recalculated on every render

**After:** Pre-compute paragraph bounds:
```tsx
const paraBounds = () => paras().map((p, i) => {
  const start = getParaOffset(paras(), i);
  return { start, end: start + p.length };
});
```

3. **Redundant class composition:**
```tsx
<span class={`fr-token${isActive(t.en_indices[0]) ? " active" : ""}`}
```

This pattern repeats 3 times. Could use a helper:
```tsx
const tokenClass = (base: string, active: boolean) => 
  `${base}${active ? " active" : ""}`;
```

---

### ✅ Correct: Performance Architecture

The caching strategy is sound:
- **Text cache:** 24-hour TTL for Gutenberg fetches
- **Tokenization cache:** Uses `OnceCell` for request deduplication
- **Exponential backoff:** Reasonable retry strategy

No obvious performance issues in the hot paths.

---

## Summary: Simplification Opportunities

| Priority | Issue | Location | Effort |
|----------|-------|----------|--------|
| HIGH | Extract shared tokenization logic | `tokenizer.rs` | Medium |
| HIGH | Remove dead code warnings | `tokenizer.rs` | Low |
| MEDIUM | Move inline styles to CSS file | `ArticleOfTheDay.tsx` | Low |
| MEDIUM | Fix unused variable warnings | `config.rs` | Low |
| LOW | Add helper for class composition | `ArticleOfTheDay.tsx` | Low |
| LOW | Pre-compute paragraph bounds | `ArticleOfTheDay.tsx` | Low |

---

## Verdict

**Proceed?** Yes — no blockers. The duplication in tokenization is the main target for refactoring.

**Estimated refactoring time:** 1-2 hours for the tokenizer extraction + cleanup pass.
