# Article of the Day — Existing Implementation Spec

## Overview

The Article of the Day feature provides daily French Maupassant stories with word-level English translations. Each day rotates through a curated list of Gutenberg books, extracts an excerpt, and optionally tokenizes/translates it via LLM.

---

## Architecture

```
┌─────────────────┐     ┌──────────────────────────────────────────────────┐
│  Frontend (Astro │     │         Backend (Rust/Axum) — lacq/             │
│  + SolidJS)     │     │                                                  │
│                 │     │  ┌──────────────┐  ┌──────────────┐              │
│  /stories/      │────▶│  │ Gutenberg.rs │  │ Tokenizer.rs │              │
│  article-of-    │     │  │ (text fetch/ │  │ (LLM call/   │              │
│  the-day.astro  │◀────│  │  cleaning)   │  │  translation) │              │
│                 │     │  └──────────────┘  └──────────────┘              │
│  ArticleOfThe   │     │         │                  │                     │
│  Day.tsx        │     │         ▼                  ▼                     │
│                 │     │  ┌──────────────────────────────────────────┐    │
│                 │     │  │ AppState (lib.rs)                        │    │
└─────────────────┘     │  │ • text_cache (HashMap<u64, CachedText>)  │    │
                        │  │ • tokenization_cache (OnceCell pool)    │    │
                        │  │ • curated_books (Vec<BookMeta>)         │    │
                        │  └──────────────────────────────────────────┘    │
                        │                                                  │
                        │  GET /api/article-of-the-day ──▶ ArticleResponse  │
                        └──────────────────────────────────────────────────┘
```

---

## API Endpoint

### `GET /api/article-of-the-day`

**Response:** `ArticleResponse` JSON

```rust
struct ArticleResponse {
    title: String,           // e.g., "Le Horla"
    source: String,           // e.g., "Contes du jour et de la nuit"
    published_year: i32,     // e.g., 1887
    paragraphs: Vec<String>,  // ~350 words, split by double-newlines
    tokenized: Option<TokenizedPayload>, // None if API key missing or tokenization failed
}

struct TokenizedPayload {
    fr_tokens: Vec<FrToken>,
    en_tokens: Vec<EnToken>,
}

struct FrToken {
    text: String,           // French word
    translation: String,      // English word
    spans: Vec<[usize; 2]>, // Char offsets into combined_text
    en_indices: Vec<usize>, // Indices into en_tokens
}

struct EnToken {
    text: String,           // English word + trailing space
    index: usize,           // Position in English sentence
}
```

---

## Daily Book Rotation

**Source:** `BookMeta::curated()` in `lib.rs`

| Gutenberg ID | Title | Collection | Year |
|--------------|-------|------------|------|
| 10775 | Le Horla | Contes du jour et de la nuit | 1887 |
| 14790 | Contes du jour et de la nuit | Contes du jour et de la nuit | 1884 |
| 10746 | Boule de Suif | Les Rougon-Macquart | 1880 |
| 12011 | Monsieur Parent | Nouvelles | 1885 |
| 11131 | Pierre et Jean | Roman | 1888 |

**Rotation:** `days_since_epoch() % 5` — deterministic per day.

---

## Data Pipeline

### 1. Text Fetching (`AppState::get_french_text`)

```
Gutenberg URL: https://www.gutenberg.org/files/{id}/{id}-0.txt
         │
         ▼
  HTTP GET with 120s timeout
         │
         ▼
  clean_text() — strip Gutenberg header/footer
         │
         ▼
  Cache (24h TTL, in-memory HashMap)
```

### 2. Text Processing (`gutenberg.rs`)

```
cleaned_text
     │
     ├── first_story_content() — extract first story from collection
     │       (skips all-caps titles, chapter markers, author sign-offs)
     │
     ├── split_paragraphs() — split on "\n\n"
     │
     └── extract_excerpt(150) — ~350 words, respects paragraph boundaries
```

### 3. Tokenization (`tokenizer.rs`)

**Trigger:** Called only if `tokenized` field is `None` (no prior tokenization cached)

**Flow:**
```
combined_text (paragraphs joined with "\n\n")
     │
     ├── Check MOCK_TOKENIZED env var (for testing)
     │
     ├── Call LLM API (Gemini or Groq)
     │
     ├── Parse JSON response
     │
     ├── repair_spans() — fix LLM's broken character offsets
     │   - Span deserializer accepts both `[[30, 60]]` and `[["02", "30"]]` (string-encoded)
     │   - Algorithm: for each token, search original text for closest matching span
     │   - Tokens that can't be repaired (no match found) are dropped silently
     │
     └── Cache via OnceCell (key: "{book_id}-{text_hash}")
```

**LLM Prompt (`build_prompt`):**

Key rules given to LLM:
- Each token is either a single word or a contiguous expression/idiom
- Punctuation must be preserved identically between French and English
- Spans must be CHARACTER offsets (not byte offsets) into the original text
- enIndices must be consecutive and cover the entire English translation
```
Given the French text below, produce a word-level tokenization where:
- fr_tokens: each French word (keeping accents, punctuation attached)
- en_tokens: corresponding English words
- spans: [start, end] character offsets in the original French text
- en_indices: index of the English token in en_tokens array

Format: JSON with fr_tokens[] and en_tokens[] arrays.
Each fr_token: { text, translation, spans: [[start, end]], en_indices: [idx] }
Each en_token: { text, index }

French text:
{french_text}
```

**Provider Selection:**
- `GROQ_API_KEY` set → Groq (`llama-3.1-8b-instant` via `/v1/chat/completions`, `max_tokens: 2048`)
- `GEMINI_API_KEY` set → Gemini (`gemini-3.1-flash-lite` via `/v1beta/models/...:generateContent`)
- Neither → fails with error (tokenized = None)

### 4. Caching (`lib.rs`)

**Text Cache:**
```rust
text_cache: HashMap<u64, CachedText>  // book_id → { text, fetched_at }
TTL: 86400 seconds (24 hours)
```

**Tokenization Cache:**
```rust
tokenization_cache: HashMap<String, Arc<OnceCell<TokenizedText>>>
Key: "{book_id}-{hash(french_text)}"
Uses OnceCell for deduplication (multiple concurrent requests share the same future)
```

---

## Frontend Integration

**Page:** `/stories/article-of-the-day.astro`

**Component:** `ArticleOfTheDay.tsx` (SolidJS island)

**Features:**
1. **Two-column layout:** French left, English right
2. **Interactive token highlighting:**
   - Click French word → highlights corresponding English word(s)
   - Click English word → highlights French word(s)
   - Toggle behavior: clicking active token clears all highlights
3. **Token filtering:** tokens are filtered to only show those starting within each paragraph
4. **Fallback:** when no `tokenized` data, shows plain paragraphs with "[No translations available]" message
5. **Responsive:** on mobile (<768px), English column hidden, French words show translation tooltip on click

**Token Matching Logic:**
```typescript
// Para offset calculation
getParaOffset(paras, paraIndex) = sum of (paras[i].length + 2) for i < paraIndex

// Filter tokens to paragraph
getParaTokens(tokens, paras, paraIndex) = tokens.filter(t => 
    t.spans[0][0] >= paraStart && t.spans[0][0] < paraEnd
)

// Get en_indices for paragraph
getParaEnIndices(tokens, paras, paraIndex) = union of t.en_indices for matching tokens
```

---

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `GROQ_API_KEY` | No | Preferred; llama-3.1-8b-instant for tokenization |
| `GEMINI_API_KEY` | No | Fallback; gemini-3.1-flash-lite |
| `LACQ_DATA_DIR` | No | Default: "data" (currently unused) |
| `RUST_LOG` | No | Default: "info" |
| `MOCK_TOKENIZED` | No | Set to "true" for testing without API calls |

---

## Configuration (`config.rs`)

```rust
pub struct Config {
    pub data_dir: String,
    pub gemini_api_key: Option<String>,
    pub groq_api_key: Option<String>,
}
```

Priority: Groq > Gemini (checked first)

---

## Known Issues / Limitations

1. **Tokenization failures are silent:** When LLM call fails, `tokenized` is `None` — users see plain French with no error message
2. **Span repair is lossy:** `repair_spans()` may remove tokens that can't be fixed, creating gaps in `en_indices`
3. **No retry logic:** Failed tokenization requests are not retried
4. **No disk caching:** `data/translations/` directory exists but is empty — translations not persisted across restarts
5. **Excerpt size hardcoded:** `extract_excerpt(paragraphs, 150)` target is ~350 words, not configurable
6. **Paragraph offset assumes "\n\n":** Combined text uses `"\n\n"` separator; if paragraph lengths change, token spans become invalid
7. **No rate limiting:** Concurrent requests all hit LLM APIs independently

---

## File Structure

```
lacq/
├── Cargo.toml              # Dependencies: axum, reqwest, tokio, regex, serde, tracing
├── src/
│   ├── main.rs            # Entry point, router setup, 120s HTTP timeout
│   ├── lib.rs             # AppState, BookMeta, ArticleResponse types, routes
│   ├── routes/mod.rs      # /health, /api/article-of-the-day handlers
│   ├── gutenberg.rs       # clean_text, split_paragraphs, extract_excerpt, first_story_content
│   ├── tokenizer.rs       # LLM calls, JSON parsing, span repair, mock tokenization
│   └── config.rs          # Config, translation provider detection
├── data/
│   └── translations/      # (empty — no disk caching implemented)
└── tests/
    └── article_pipeline.rs # Unit tests for gutenberg parsing, book rotation
```

---

## Tests

**gutenberg.rs tests:**
- `test_clean_text_strips_gutenberg_header_footer`
- `test_clean_text_normalizes_line_endings`
- `test_clean_text_collapses_multiple_blank_lines`
- `test_split_paragraphs_splits_on_double_newline`
- `test_extract_excerpt_stops_before_exceeding_target`

**tokenizer.rs tests:**
- `test_span_deserializer` — validates Span JSON parsing (numbers, string-encoded, errors)
- `test_mock_tokenize_async` — validates mock output format

**article_pipeline.rs tests:**
- `test_curated_books_has_real_gutenberg_ids`
- `test_book_rotation_is_deterministic`
- `test_book_meta_fields`
- `test_app_state_creates_successfully`
- `test_app_state_text_cache_starts_empty`

---

## Deployment

- **Platform:** Fly.io (single Machine)
- **Port:** 8080
- **Health check:** `GET /health` returns "OK"
- **Static files:** `dist/` served at `/`
- **API routes:** `/api/*` handled by Axum
- **Memory:** 256MB RAM configured
- **API keys:** Set via `fly secrets set GROQ_API_KEY=...` or `GEMINI_API_KEY=...`

---

## Future Considerations (for v2)

- [ ] Add disk caching for translations (avoid re-tokenization on restart)
- [ ] Add user-facing error message when translations unavailable
- [ ] Add configurable excerpt length
- [ ] Add retry logic with exponential backoff for LLM calls
- [ ] Add rate limiting for concurrent requests
- [ ] Add metrics (tokenization latency, cache hit rate)
- [ ] Consider chunking long texts to avoid LLM context limits
- [ ] Add support for multiple target languages (not just English)
