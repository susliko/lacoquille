# Article-of-the-Day: Real Data Pipeline

> **Goal:** Replace hardcoded mock in `article.rs` with a real pipeline that fetches daily Maupassant stories from Gutenberg.

## Architecture

```
GET /api/article-of-the-day
  → Daily book rotation: (days_since_epoch % curated_list.len())
  → Fetch French text from Gutenberg (in-memory cache, 1hr TTL)
  → Parse: clean → split_paragraphs → extract_excerpt (~350 words)
  → Extract vocab highlights via vocab.rs (stopword filtering)
  → English translation:
      1. Try Gutenberg English edition of same title
      2. Fallback: Minimax LLM API (if MINIMAX_API_KEY set)
      3. Fallback: return French-only (english = None)
  → Return ArticleResponse JSON
```

## Key Design Decisions

1. **AppState** — Axum shared state containing:
   - `curated_books: Vec<BookMeta>` — hardcoded ~15 Maupassant Gutenberg book IDs with metadata
   - `french_cache: Arc<Mutex<LruCache<u64, CachedText>>>` — in-memory LRU, 1hr TTL
   - `http: reqwest::Client` — shared HTTP client
   - `llm: Option<Arc<dyn LlmProvider>>` — real Minimax if key set, else None
   - `translation_cache: TranslationCache` — file-based (existing)

2. **Curated book list** — Hardcoded in `AppState::new()`. Each entry: `(gutenberg_id, title, collection, year, english_gutenberg_id?)`. ~15 Maupassant stories covering different periods and collections. Daily rotation = `days_since_epoch % curated_books.len()`.

3. **Gutenberg French text fetching** — `GET https://www.gutenberg.org/files/{id}/{id}-0.txt`. Cache entry: `(text, fetched_at)`. TTL = 1 hour. Key = book ID.

4. **Gutenberg English lookup** — For books with known English editions, store `english_gutenberg_id` in curated list. For others, call `find_english_edition(client, title)` to search Gutenberg by title keyword.

5. **`clean_text()`** — Strip Gutenberg boilerplate: start at "*** START" marker, end at "*** END" marker. Remove page break markers like `\r`.

6. **Translation pipeline** — `get_translation(paragraphs, book_id, has_english, data_dir)`:
   - Check disk cache first
   - If `has_english` and `english_gutenberg_id`, fetch English from Gutenberg
   - Else if LLM available, use it
   - Else return `None` (French only)

7. **Error handling** — If fetch fails, log and try cache. If all fail, return error JSON with status 500.

## File Changes

| File | Change |
|---|---|
| `lacq/src/lib.rs` | Add `AppState`, `BookMeta`, `CachedText` types; export `AppState` |
| `lacq/src/gutenberg.rs` | Add `fetch_book_text()`, `find_english_edition()`, fix `clean_text()` |
| `lacq/src/llm.rs` | Add `MinimaxProvider` struct using reqwest + MINIMAX_API_KEY |
| `lacq/src/routes/article.rs` | Rewrite to use `State<AppState>`, daily rotation, real pipeline |
| `lacq/src/main.rs` | Build AppState, pass to Router, use routes() from mod.rs |
| `lacq/src/routes/mod.rs` | Add `health` route, consolidate routing here (remove duplication) |

## No Disk Cache for French Text

Only translations are cached to disk (existing `TranslationCache`). French text is fetched on demand with in-memory TTL cache. This keeps the implementation simple — French texts are small (~50KB) and fetching is fast.

## Out of Scope

- `/api/stories` and `/api/stories/:id` remain stubs
- Vocabulary definition lookup (only highlight words, no definitions)
- User preference for specific stories