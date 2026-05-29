# Article of the Day — v2 Specification

## Status: Draft for Review

---

## Problems Identified in v1

| # | Problem | Evidence | Root Cause |
|---|---------|----------|------------|
| 1 | **Translation not robust** | No fallback between providers or models | Single provider used, fail-fast returns `None` |
| 2 | **Punctuation lost** | "Le petit Georges, à quatre pattes" → tokens "Le petit Georges" and "à quatre pattes" with no comma | LLM strips punctuation despite prompt instruction |
| 3 | **Tokens too large** | Avg 3.4 words/token (36 tokens for 122 words), max 30+ chars | Prompt allows "expressions/idioms" with no size limit |
| 4 | **Excerpt too short** | 3 paragraphs, ~122 words | `extract_excerpt(150)` target = ~350 words |

---

## Architecture Changes

### Provider Chain with Fallback

```
┌─────────────────────────────────────────────────────────────────────┐
│                     TranslationProviderChain                        │
├─────────────────────────────────────────────────────────────────────┤
│  Groq llama-3.1-8b-instant                                           │
│       │                                                             │
│       ├─ FAIL → Groq llama-3.3-70b-versatile                         │
│       │         │                                                    │
│       │         └─ FAIL → Gemini gemini-2.5-flash-preview            │
│       │                   │                                          │
│       │                   └─ FAIL → Gemini gemini-3.1-flash-lite     │
│       │                             │                                │
│       │                             └─ FAIL → Mock (warn + skip)    │
│       │                                                           │
│       └─ OK ✓                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

**Fallback rules:**
1. Try providers/models in priority order
2. On failure, log the error and try next in chain
3. Only fail completely if ALL providers fail
4. Track which provider succeeded for metrics

### New Token Structure

**v1 (broken):**
```json
{
  "text": "Le petit Georges,",
  "translation": "The little Georges",
  "spans": [[0, 17]]
}
```
Problem: LLM strips punctuation from text/translation.

**v2 (clean):**
```json
{
  "base_text": "Le petit Georges",
  "trailing_punct": ",",
  "leading_punct": "",
  "translation": "The little Georges,",
  "spans": [[0, 15]],
  "en_indices": [0]
}
```

**Design principles:**
1. **Punctuation is SEPARATE, not embedded** — no relying on LLM to preserve it
2. **`trailing_punct` is mandatory** — even empty string `""` (never null)
3. **`translation` includes punctuation** — copy from `trailing_punct` to end
4. **`base_text` never has punctuation** — punctuation is always in `leading_punct`/`trailing_punct`
5. **Spans reference `base_text` positions** — simpler offset validation

### Minimal Translatable Units

**v1 rules (too loose):**
```
- (a) a single word that makes sense on its own, OR 
- (b) an expression/idiom where parts don't translate independently
- For multi-word tokens, they must be contiguous in the French text
```

**v2 rules (strict):**
```
1. SINGLE WORDS: any word that translates independently
   - "chat" → "cat"
   - "rapidement" → "quickly"
   
2. CONTRACTED FORMS: kept as single tokens (contractions)
   - "d'une" → "of a"
   - "au" → "to the"
   - "du" → "of the" (article+preposition)
   - "l'homme" → "the man" (article+noun, elided)
   
3. PRONOMINAL VERBS: verb + attached pronoun
   - "se lève" → "gets up"
   - "me suis lavé" → "washed myself"
   
4. EXPRESSIONS (max 2 words, only when translation is non-compositional):
   - "peut-être" → "perhaps" (not "can be")
   - "ne...pas" → "not" (negation pair)
   - "tout à coup" → "suddenly"
   
5. NEVER include punctuation in base_text:
   - Split at: , . ! ? ; : — ( ) " ' ... 
   - These are SEPARATORS, not part of tokens
   
6. HARD LIMITS:
   - Max 3 words per token (expressions only)
   - Max 25 characters per base_text
   - If translation requires >3 words, break it up
```

### Excerpt Size

**v1:** `extract_excerpt(paragraphs, 150)` ≈ 350 words, 3 paragraphs

**v2:** `extract_excerpt(paragraphs, 200)` ≈ 400 words, 3-4 paragraphs

**Rationale:** Groq's free tier has 6000 TPM limit. 500-word excerpts (9000+ tokens with prompt) exceed this. 200-word target keeps total request under 6000 tokens.

**Also increase LLM max_tokens:**
- Groq: `max_tokens: 8192` (was 2048)
- Gemini: keep `temperature: 0.2`

---

## New Data Structures

### v2 Tokenization Response

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizedText {
    #[serde(rename = "frTokens")]
    pub fr_tokens: Vec<FrenchToken>,
    #[serde(rename = "enTokens")]
    pub en_tokens: Vec<EnglishToken>,
    #[serde(default)]
    pub book_id: u64,
    #[serde(default)]
    pub provider_used: Option<String>,  // e.g., "groq/llama-3.1-8b-instant"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrenchToken {
    #[serde(rename = "text")]
    pub text: String,           // FRENCH base text, NO punctuation (e.g., "Le petit Georges")
    #[serde(rename = "trailingPunct")]
    pub trailing_punct: String, // Punctuation AFTER token (e.g., "," or "" or ".")
    #[serde(rename = "leadingPunct")]
    pub leading_punct: String,  // Punctuation BEFORE token (rare, e.g., "(" or "\"")
    #[serde(rename = "translation")]
    pub translation: String,    // ENGLISH translation + trailing punctuation
    #[serde(rename = "spans")]
    pub spans: Vec<Span>,       // Char offsets into ORIGINAL French text
    #[serde(rename = "enIndices")]
    pub en_indices: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnglishToken {
    #[serde(rename = "text")]
    pub text: String,    // English word + trailing space
    #[serde(rename = "index")]
    pub index: usize,
}
```

### Frontend Changes

**Token rendering:**
```tsx
// v1 (broken):
<span class="fr-token">{t.text} </span>

// v2 (correct):
<span class="fr-token">
  {t.leading_punct}{t.text}{t.trailing_punct}
</span>
```

**English rendering:**
```tsx
// English tokens are just words, no punctuation handling needed
// because translation already includes punctuation
```

### API Response Changes

```rust
#[derive(Debug, Clone, Serialize)]
pub struct ArticleResponse {
    pub title: String,
    pub source: String,
    pub published_year: i32,
    pub paragraphs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokenized: Option<TokenizedPayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokenization_error: Option<String>,  // NEW: user-facing error message
}
```

---

## Provider Configuration

### Environment Variables

```env
# Translation provider chain (comma-separated, tried in order)
TRANSLATION_CHAIN=groq,gemini

# Groq models (in priority order, comma-separated)
GROQ_MODELS=llama-3.1-8b-instant,llama-3.3-70b-versatile

# Gemini models (in priority order, comma-separated)
GEMINI_MODELS=gemini-2.5-flash-preview,gemini-3.1-flash-lite

# Per-model settings
GROQ_MAX_TOKENS=8192
GEMINI_MAX_TOKENS=8192

# Fallback behavior
TRANSLATION_TIMEOUT_SECS=30
TRANSLATION_RETRIES=2
```

### Default Chain (if env vars not set)

```
groq/llama-3.1-8b-instant
    → groq/llama-3.3-70b-versatile
    → gemini/gemini-2.5-flash-preview
    → gemini/gemini-3.1-flash-lite
    → MOCK (with warning)
```

---

## New Tokenization Prompt

```
You are a French language learning assistant. Tokenize this French text for 
parallel bilingual reading. Every character in the French text MUST appear 
in exactly one token.

OUTPUT FORMAT (strict JSON, no markdown):
{
  "frTokens": [
    {
      "text": "word or short phrase WITHOUT punctuation",
      "trailingPunct": "," or "." or "" etc (the punctuation AFTER this word in original),
      "leadingPunct": "" or "(" etc (the punctuation BEFORE this word, rare),
      "translation": "English meaning INCLUDING trailing punctuation",
      "spans": [[start, end]],  // char offsets in ORIGINAL French text
      "enIndices": [N]
    }
  ],
  "enTokens": [
    { "text": "English word ", "index": N }  // note: trailing space
  ]
}

STRICT RULES:

1. SPLIT AT PUNCTUATION
   - Commas, periods, semicolons, colons, exclamation, question marks are SEPARATORS
   - "Le chat," → token "Le chat" + trailingPunct ","
   - "Il vient." → token "Il vient" + trailingPunct "."
   - "Comment allez-vous?" → three tokens: "Comment", "allez-vous", "?"

2. CONTRACTED FORMS ARE SINGLE TOKENS
   - "d'une" (de + une) → text "d'une"
   - "au" (à + le) → text "au"
   - "du" (de + le) → text "du"
   - "l'homme" (le + homme) → text "l'homme"

3. PRONOMINAL VERBS: verb + attached pronoun
   - "se lève" → text "se lève"
   - "me suis levé" → text "me suis levé"

4. SINGLE WORDS ARE PREFERRED
   - Prefer one word per token when possible
   - Only use 2-3 words together if:
     a) Contraction (d'une, au, l'homme)
     b) Pronominal verb (se lève)
     c) Expression that doesn't translate word-by-word (peut-être)
   
5. NEVER BREAK:
   - Contractions (l', d', qu', n', j', m', t', s')
   - "allez-vous" (formal you construction)

6. HARD LIMITS:
   - Max 3 words per token
   - Max 25 characters per text field (excluding punctuation)
   - If English needs >3 words, the translation handles it

7. SPANS:
   - Reference ORIGINAL French text (with punctuation)
   - Must be exact: `text + trailingPunct` must equal `original_text[start:end]`
   - Character positions, not byte positions

8. TRANSLATIONS:
   - Must preserve ALL punctuation from French
   - "Le chat," → translation "The cat," (with comma!)
   - "Comment?" → translation "What?" (with question mark!)
   - Translations should be natural English, not literal

French text:
{CONTENT}
```

---

## Punctuation Splitting Algorithm

Instead of relying on LLM, implement a preprocessing step:

```rust
/// Split text into tokens with separated punctuation
/// Input: "Le petit Georges, à quatre pattes."
/// Output: [
///   ("Le petit Georges", ",", ""),
///   ("à quatre pattes", ".", ""),
/// ]
fn split_tokens_with_punct(text: &str) -> Vec<(&str, &str, &str)> {
    let mut results = Vec::new();
    let re = Regex::new(r"^([«\"'(]*)([^\s,.:!?;:\-—]+)([,.:!?;:\-—»\"]*)").unwrap();
    
    for word in text.split_whitespace() {
        // Find the boundary between word content and punctuation
        let word_chars: Vec<char> = word.chars().collect();
        let mut leading = String::new();
        let mut trailing = String::new();
        let mut content_start = 0;
        let mut content_end = word_chars.len();
        
        // Extract leading punctuation
        for (i, c) in word_chars.iter().enumerate() {
            if is_opening_punct(*c) {
                leading.push(*c);
                content_start = i + 1;
            } else {
                break;
            }
        }
        
        // Extract trailing punctuation (reverse scan)
        for (i, c) in word_chars.iter().rev().enumerate() {
            if is_closing_punct(*c) {
                trailing.push(*c);
                content_end -= 1;
            } else {
                break;
            }
        }
        
        let content: String = word_chars[content_start..content_end].iter().collect();
        results.push((content, trailing, leading));
    }
    
    results
}

fn is_opening_punct(c: char) -> bool {
    matches!(c, '«' | '"' | '(' | '[' | '\'')
}

fn is_closing_punct(c: char) -> bool {
    matches!(c, '»' | '"' | ')' | ']' | '.' | ',' | '!' | '?' | ';' | ':' | '—' | '-' | '\'')
}
```

**Then:** Feed the PRE-SPLIT tokens to the LLM with explicit punctuation info, or use the split as a constraint.

**Alternative approach:** Pre-split the text, send each "content + punct" pair to LLM for translation only (not segmentation), then reassemble.

---

## Excerpt Extraction Changes

```rust
// Change from 150 to 500 words target
let excerpt_paras = crate::gutenberg::extract_excerpt(&paragraphs_raw, 500);
```

**Expected output:** 5+ paragraphs, ~1000 words

---

## Error Handling

### v1 (broken):
```rust
Err(e) => {
    tracing::error!("Tokenization failed: {}", e);
    None  // Silent failure, user sees "[No translations available]"
}
```

### v2 (robust):
```rust
Err(e) => {
    tracing::error!("Tokenization failed after all providers: {}", e);
    Some(TokenizationError {
        message: "Translation temporarily unavailable. Showing French text only.",
        providers_tried: vec!["groq/llama-3.1-8b-instant", ...],
        last_error: e,
    })
}
```

### Retry Logic

```rust
async fn tokenize_with_fallback(
    http: &reqwest::Client,
    french_text: &str,
    config: &Config,
) -> Result<TokenizedText, String> {
    let chain = config.translation_chain();
    let mut last_error = String::new();
    
    for provider_model in chain {
        for attempt in 0..config.retries() {
            match tokenize(http, french_text, provider_model).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_error = e;
                    tracing::warn!("Attempt {} failed for {}: {}", attempt, provider_model, e);
                    if attempt < config.retries() - 1 {
                        tokio::time::sleep(Duration::from_secs(2_u64.pow(attempt))).await;
                    }
                }
            }
        }
    }
    
    Err(format!("All providers failed. Last error: {}", last_error))
}
```

---

## File Changes Summary

### `lacq/src/tokenizer.rs`

**Changes:**
1. Add `TranslationChain` struct with ordered provider/model list
2. Implement `tokenize_with_fallback()` with retry logic
3. Add `split_tokens_with_punct()` for preprocessing
4. Update `FrenchToken` struct with `trailing_punct`, `leading_punct`
5. Update prompt to v2 rules
6. Increase `max_tokens` for Groq to 8192
7. Add mock response format with punctuation

### `lacq/src/gutenberg.rs`

**Changes:**
1. Change `extract_excerpt(target_words)` default from 150 to 500

### `lacq/src/lib.rs`

**Changes:**
1. Update `ArticleResponse` with optional `tokenization_error`
2. Add `tokenization_error` field to `TokenizedPayload`

### `lacq/src/config.rs`

**Changes:**
1. Parse `TRANSLATION_CHAIN` env var
2. Parse `GROQ_MODELS`, `GEMINI_MODELS` env vars
3. Add `translation_chain()` method returning ordered list

### `lacq/src/routes/mod.rs`

**Changes:**
1. Update error handling to capture all provider errors
2. Return `tokenization_error` in response when tokenization fails

### `src/components/ArticleOfTheDay.tsx`

**Changes:**
1. Update `FrToken` interface with `trailingPunct`, `leadingPunct`
2. Render punctuation with tokens
3. Show `tokenizationError` message to user
4. Keep fallback message for truly unavailable translations

---


## Testing Requirements

## Coverage Audit: v1 Gaps

### Backend (Rust) — Current State: 25 tests, 6 dead warnings

**What IS tested:**
- `gutenberg.rs`: 12 tests — clean_text, split_paragraphs, extract_excerpt, first_story_content
- `tokenizer.rs`: 2 tests — Span JSON deserializer, mock_tokenize_async basic structure
- `article_pipeline.rs`: 10 tests — BookMeta rotation, cache init, gutenberg utils (DUPLICATES!)

**What is NOT tested (CRITICAL GAPS):**

| Gap | Impact | Severity |
|-----|--------|----------|
| `repair_spans()` function | Can't verify if span repair works or breaks tokens | HIGH |
| `get_tokenized()` / `get_french_text()` with mocked HTTP | No verification of cache behavior, error handling | HIGH |
| `tokenize_groq()` with mocked HTTP | Can't verify prompt format, error parsing | HIGH |
| `tokenize_gemini()` with mocked HTTP | Same as above | HIGH |
| Config parsing (env vars) | No verification of provider selection logic | MEDIUM |
| Combined text span alignment | Paragraph offsets in frontend could be wrong | HIGH |
| Error response transformation | No tests for ArticleResponse with errors | MEDIUM |
| `first_story_content()` all branches | Only 2 of ~10 branches covered | MEDIUM |
| `mock_tokenize_async` character positions | Only basic structure, not char vs byte correctness | MEDIUM |

**Duplicate tests:** `article_pipeline.rs` re-tests `clean_text`, `split_paragraphs`, `extract_excerpt` — waste of 3 tests.

### Frontend (TypeScript/SolidJS) — Current State: ZERO tests

**What must be tested:**

| Component | Test | Method |
|-----------|------|--------|
| `ArticleOfTheDay.tsx` | Fetches and renders article data | Vitest + MSW |
| `ArticleOfTheDay.tsx` | Handles loading/error states | Vitest |
| `ArticleOfTheDay.tsx` | Token highlighting toggle works | Vitest |
| `ArticleOfTheDay.tsx` | `getParaTokens()` filters correctly | Vitest |
| `ArticleOfTheDay.tsx` | `getParaOffset()` computes paragraph positions | Vitest |
| `ArticleOfTheDay.tsx` | Mobile tooltip shows translation | Vitest + jsdom |
| `ArticleOfTheDay.tsx` | Fallback renders when tokenized is None | Vitest |
| `getParaEnIndices()` | Returns correct en_indices per paragraph | Vitest |
| Token rendering | `trailingPunct` renders correctly | Vitest |
| `isActive()` | Toggle logic works correctly | Vitest |

---

## v2 Test Plan

### Backend: New Tests Required

#### 1. Punctuation Splitting (`split_tokens_with_punct`)

```rust
#[cfg(test)]
mod punctuation_split_tests {
    use super::*;

    #[test]
    fn test_split_simple_trailing_comma() {
        let result = split_tokens_with_punct("Le chat,");
        assert_eq!(result, vec![("Le chat", ",", "")]);
    }

    #[test]
    fn test_split_sentence_with_punctuation() {
        let input = "Le petit Georges, à quatre pattes.";
        let result = split_tokens_with_punct(input);
        // Should split into: [("Le petit Georges", ",", ""), ("à quatre pattes", ".", "")]
    }

    #[test]
    fn test_split_parentheses() {
        let result = split_tokens_with_punct("(texte)");
        assert_eq!(result, vec![("texte", ")", "(")]);
    }

    #[test]
    fn test_split_contractions_preserved() {
        // Apostrophe is NOT a punctuation separator for contractions
        let result = split_tokens_with_punct("d'une");
        assert_eq!(result, vec![("d'une", "", "")]);
    }

    #[test]
    fn test_split_elided_article() {
        let result = split_tokens_with_punct("l'homme");
        assert_eq!(result, vec![("l'homme", "", "")]);
    }

    #[test]
    fn test_split_mixed_punctuation() {
        let input = r#"Il dit : «Oui !»"#;
        let result = split_tokens_with_punct(input);
        // Should handle: « » guillemets and !
    }

    #[test]
    fn test_split_empty_input() {
        let result = split_tokens_with_punct("");
        assert!(result.is_empty());
    }
}
```

#### 2. Token Reconstruction

```rust
#[test]
fn test_token_reconstruction_from_spans() {
    let original = "Le chat, dans la maison.";
    let tokens = vec![
        FrenchToken {
            text: "Le chat".into(),
            trailing_punct: ",".into(),
            leading_punct: "".into(),
            translation: "The cat,".into(),
            spans: vec![Span([0, 8])],
            en_indices: vec![0],
        },
        FrenchToken {
            text: "dans la maison".into(),
            trailing_punct: ".".into(),
            leading_punct: "".into(),
            translation: "in the house.".into(),
            spans: vec![Span([10, 25])],
            en_indices: vec![1],
        },
    ];

    // Verify: text + trailing_punct matches original span
    for token in &tokens {
        let [start, end] = token.spans[0];
        let reconstructed = format!("{}{}", token.text, token.trailing_punct);
        assert_eq!(&original[start..end], reconstructed);
    }
}

#[test]
fn test_all_characters_covered_no_overlap() {
    let original = "Le chat, dans la maison.";
    // Every character in original should appear in exactly one token
    // No character should be in two tokens
}
```

#### 3. Provider Chain Fallback (with wiremock — already in dev-dependencies)

```rust
#[tokio::test]
async fn test_fallback_to_second_model() {
    // Mock server that fails on first model, succeeds on second
}

#[tokio::test]
async fn test_retry_logic() {
    // First attempt fails, second succeeds
}

#[tokio::test]
async fn test_all_providers_fail_returns_error() {
    // Mock all providers failing
    // Verify error contains all provider names that were tried
}
```

#### 4. repair_spans() Function

```rust
#[test]
fn test_repair_spans_fixes_valid_tokens() {
    // Token with correct span should pass through
}

#[test]
fn test_repair_spans_drops_unrecoverable() {
    // Token whose text doesn't exist in source should be dropped
}

#[test]
fn test_repair_spans_finds_closest_match() {
    // LLM returns wrong position, repair finds closest valid
}
```

#### 5. Combined Text Span Alignment

```rust
#[test]
fn test_paragraph_offsets() {
    let paras = vec![
        "Premier.".to_string(),
        "Deuxième.".to_string(),
    ];
    let combined = paras.join("\n\n");
    // Para 0: [0, 7]
    // Para 1: [9, 17] (7 + 2 for "\n\n")
}

#[test]
fn test_get_para_tokens_filters_correctly() {
    // Same test but with actual tokens from tokenization
}
```

#### 6. Config Parsing

```rust
#[test]
fn test_config_defaults() {
    // Without env vars, Groq should be first choice
}

#[test]
fn test_config_groq_takes_precedence() {
    // With both API keys, Groq should be used
}
```

---

### Frontend: New Tests Required

**Install Vitest:**
```bash
npm install -D vitest @testing-library/solid jsdom @testing-library/jest-dom
```

#### 1. Token Filtering Tests

```typescript
// src/components/__tests__/ArticleOfTheDay.test.tsx
describe('getParaOffset', () => {
  it('calculates offset for first paragraph', () => {
    const paras = ['Hello', 'World'];
    expect(getParaOffset(paras, 0)).toBe(0);
  });

  it('calculates offset for second paragraph', () => {
    const paras = ['Hello', 'World'];
    expect(getParaOffset(paras, 1)).toBe(8); // 5 + 2 for "\n\n"
  });
});

describe('getParaTokens', () => {
  const tokens = [
    { text: 'Hello', spans: [[0, 5]], en_indices: [0] },
    { text: 'World', spans: [[7, 12]], en_indices: [1] },
  ];
  const paras = ['Hello', 'World'];

  it('filters tokens for first paragraph', () => {
    const result = getParaTokens(tokens, paras, 0);
    expect(result).toHaveLength(1);
    expect(result[0].text).toBe('Hello');
  });
});
```

#### 2. Token Highlighting Tests

```typescript
describe('setActiveIndicesFrom', () => {
  it('activates tokens when clicking inactive', () => {
    // Set active indices from clicked enIndices
  });

  it('clears all when clicking active token', () => {
    // Toggle behavior: clicking same token clears
  });

  it('toggles correctly with multiple active', () => {
    // If all clicked indices are active, clear; else add them
  });
});
```

#### 3. Error and Loading States

```typescript
describe('ArticleOfTheDay', () => {
  it('shows loading state initially', async () => {
    // Mock fetch that never resolves
  });

  it('shows error state when fetch fails', async () => {
    // Mock 500 response
  });

  it('shows fallback when tokenized is null', async () => {
    // Mock response without tokenized
  });
});
```

#### 4. Punctuation Rendering (v2)

```typescript
describe('Token rendering with punctuation', () => {
  it('renders trailingPunct after text', () => {
    const token = { text: 'Le chat', trailingPunct: ',', leadingPunct: '' };
    // Render: {leadingPunct}{text}{trailingPunct}
    // Should produce: "Le chat,"
  });

  it('renders leadingPunct before text', () => {
    const token = { text: 'texte', trailingPunct: ')', leadingPunct: '(' };
    // Should produce: "(texte)"
  });
});
```

---

### Integration Tests

#### Backend: Full Pipeline with WireMock

```rust
#[tokio::test]
async fn test_full_pipeline_gutenberg_to_tokenized() {
    // 1. Mock Gutenberg returning valid text
    // 2. Mock Groq returning valid tokenization
    // 3. Run full pipeline
    // 4. Verify paragraphs and tokenized are populated
}
```

---

### Test Summary

| Category | v1 | v2 Required | New Tests |
|----------|----|----|----|
| Backend Unit | 25 | 55 | +30 |
| Backend Integration | 0 | 5 | +5 |
| Frontend Unit | 0 | 25 | +25 |
| Frontend Integration | 0 | 5 | +5 |
| **Total** | 25 | 90 | **+65** |

---

### CI Pipeline

```yaml
# .github/workflows/tests.yml
name: Tests
on: [push, pull_request]

jobs:
  backend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cd lacq && cargo test
      - run: cd lacq && cargo tarpaulin --out TmL  # coverage

  frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '22'
      - run: npm ci
      - run: npm test -- --coverage
      - uses: actions/upload-artifact@v4
        with:
          name: coverage
          path: coverage/
```

---

### Migration Checklist (Updated)

- [ ] Update `FrenchToken` struct with `trailing_punct`, `leading_punct`
- [ ] Implement `split_tokens_with_punct()` preprocessing
- [ ] Update prompt with v2 rules
- [ ] Implement provider chain with fallback
- [ ] Increase `max_tokens` to 8192
- [ ] Change excerpt target from 150 to 500
- [ ] Update frontend to render punctuation
- [ ] Add `tokenization_error` to API response
- [ ] **Install Vitest and testing libraries**
- [ ] **Write 25+ frontend unit tests**
- [ ] **Write 35+ backend unit tests**
- [ ] **Write 5+ integration tests**
- [ ] **Set up GitHub Actions CI for tests**
- [ ] Set new environment variables on Fly.io

## Open Questions

1. **Pre-split vs LLM-split:** Should we pre-split text and send chunks to LLM, or trust LLM with strict rules?

2. **Token size limit:** Is 25 characters / 3 words strict enough? Should we make it configurable?

3. **Paragraph breaks:** Should paragraph boundaries become explicit tokens or just `\n\n` separators?

4. **Cache invalidation:** If we change the tokenization rules, should we invalidate existing caches?
