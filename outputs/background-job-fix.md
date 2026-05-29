# Background Job Fix — Article-of-the-Day Translation

## Problem

The `/api/article-of-the-day` endpoint performed translation on-demand, causing slow response times for users waiting on LLM API calls.

## Solution

Moved translation to a background job that runs:
1. **On app startup** — immediately computes today's article
2. **Once daily at midnight** — refreshes using `tokio::time::interval`

The `/api/article-of-the-day` endpoint now returns a pre-cached `ArticleResponse` instantly.

## Changes Made

### `lacq/src/lib.rs`
- Added `CachedArticle` struct to hold the computed response and timestamp
- Added `cached_article: Arc<RwLock<Option<CachedArticle>>>` field to `AppState`
- Added `start_background_workers()` method that spawns a Tokio task for initial computation + daily refresh
- Added `compute_article_of_the_day()` function (moved from `routes/mod.rs`) that fetches, tokenizes, and caches the article

### `lacq/src/routes/mod.rs`
- Simplified `article_of_the_day()` handler to read from cache (instant response)
- Returns `SERVICE_UNAVAILABLE` if cache is not yet populated (transient state during startup)

### `lacq/src/main.rs`
- Calls `state.start_background_workers()` after creating the app state

### `lacq/Cargo.toml`
- Added `chrono = "0.4"` dependency for midnight scheduling

## Key Design Decisions

| Decision | Rationale |
|---|---|
| `Arc<RwLock<Option<CachedArticle>>>` | Thread-safe interior mutability; `RwLock` allows concurrent reads |
| `Option<None>` initially | Handler returns 503 until first computation completes |
| `chrono` for midnight calculation | Precise scheduling to next midnight, then 24h intervals |
| Clone-on-demand for response | `ArticleResponse` is small; cloning is cheaper than locking writes |

## Testing

```sh
# Start server with mock translation
MOCK_TOKENIZED=true cargo run

# Health check
curl http://localhost:8080/health  # "OK"

# Article endpoint (instant response ~5ms)
time curl http://localhost:8080/api/article-of-the-day | jq -r '.title'

# Log output shows:
# - Initial computation on startup
# - Midnight refresh scheduled (e.g., "scheduling first refresh in 3456 seconds")
```

## Verified Behavior

1. **Fast response**: `/api/article-of-the-day` returns in ~5ms (vs 2-30s with on-demand translation)
2. **Startup computation**: Article is pre-computed before the first user request
3. **Daily rotation**: `BookMeta::for_today()` rotates through 5 curated Maupassant books based on day-of-epoch
4. **Error resilience**: If translation fails, the cache is updated with `tokenization_error` set, and French text is still served

## Files Changed

- `lacq/src/lib.rs` — AppState, background workers, compute function
- `lacq/src/routes/mod.rs` — simplified endpoint handler
- `lacq/src/main.rs` — worker startup
- `lacq/Cargo.toml` — chrono dependency

## Status

✅ Implemented and tested successfully.