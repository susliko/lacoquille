# ISSUE-002 Recon: Shadowing Practice Page — "Loading stories..." Forever

## Files Retrieved

1. `src/components/ShadowingPractice.tsx` (lines 1–880) — the SolidJS island that powers the shadowing page; contains all fetch calls, loading/error state, TTS logic, and UI
2. `src/pages/practice/shadowing.astro` (lines 1–7) — the Astro page wrapper that mounts the island
3. `lacq/src/routes/mod.rs` (lines 1–27) — the sole backend route file; only two routes exist
4. `lacq/src/lib.rs` (lines 1–200) — `AppState`, `BookMeta`, `ArticleResponse`, background worker, Gutenberg text fetching
5. `lacq/src/gutenberg.rs` (lines 1–260) — text cleaning, paragraph splitting, story extraction from Gutenberg plain-text books
6. `lacq/src/main.rs` (lines 1–30) — server bootstrap wiring
7. `src/components/PracticeHub.tsx` (lines 1–30) — practice hub tile list including the shadowing link
8. `src/components/HomepageTabs.tsx` (lines 1–120) — homepage practice tab including the shadowing link
9. Git history: `git log --oneline --all -- lacq/` and `git show de0471b -- lacq/src/routes/mod.rs`

---

## Key Code

### Frontend: `ShadowingPractice.tsx` — fetch calls

```typescript
// Line 20–24: initial story list fetch
async function fetchStories(): Promise<Story[]> {
  const host = `${window.location.protocol}//${window.location.host}`;
  const res = await fetch(`${host}/api/stories`);
  if (!res.ok) throw new Error(`Failed to fetch stories: ${res.status}`);
  return res.json();
}

// Line 27–31: per-story detail fetch
async function fetchStory(id: string): Promise<StoryDetail> {
  const host = `${window.location.protocol}//${window.location.host}`;
  const res = await fetch(`${host}/api/stories/${id}`);
  if (!res.ok) throw new Error(`Failed to fetch story: ${res.status}`);
  return res.json();
}

// Line 106: TTS cache warm-up (fire-and-forget POST)
fetch(`/api/stories/${storyId}/tts-cache`, { method: "POST" }).finally(() => { ... });

// Line 141–146: preload next sentence TTS (fire-and-forget)
fetch("/api/tts", { method: "POST", body: JSON.stringify({ text }), ... }).catch(() => {});

// Line 182–186: play audio TTS
const res = await fetch(`${host}/api/tts`, { method: "POST", body: JSON.stringify({ text }), ... });
if (!res.ok) throw new Error(`TTS failed: ${res.status}`);
const blob = await res.blob();
```

### Frontend: loading/error state (ShadowingPractice.tsx)

```tsx
// Line 64: resource definition
const [stories] = createResource(fetchStories);

// Lines 690–696: the only error handling that exists
<Show when={stories.loading}>
  <div class="loading-state">Loading stories...</div>
</Show>

<Show when={stories.error}>
  <div class="error-state">
    Unable to load stories. Make sure the lacq server is running on port 8080.
  </div>
</Show>
```

**The root cause of the stuck loading state:** `createResource` wraps an async function with no `Suspense` boundary. In SolidJS, `createResource` stores errors in the `.error` property rather than throwing them to a `Suspense` boundary. The `<Show when={stories.error}>` check *should* work — SolidJS sets `stories.error` to the thrown value when the promise rejects. However, the page is rendered with `client:only="solid-js"` (no SSR), and there is no `ErrorBoundary` wrapping the component, so the error state exists but may not render reactively if SolidJS's internal tracking misses the update.

More likely: the `stories.error` check fires correctly, but because no `Suspense` boundary wraps the resource, SolidJS may not propagate the error signal to child `<Show>` nodes reliably in all versions. The net effect is the same: the user sees "Loading stories..." indefinitely.

### Frontend: page wrapper (`shadowing.astro`)

```astro
<Base title="Shadowing Practice — La Coquille" section="practice">
  <ShadowingPractice client:only="solid-js" />
</Base>
```

`client:only="solid-js"` means the component is **never SSR'd** — it runs only in the browser. This is correct for a component that reads `window.location` and uses `localStorage`, but it means there is no server-side fallback that could surface an error.

### Backend: `lacq/src/routes/mod.rs` — the entire route file

```rust
use axum::{extract::State, routing::get, Router, http::StatusCode};
use std::sync::Arc;
use crate::{AppState, ArticleResponse};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health))
        .route("/api/article-of-the-day", get(article_of_the_day))
}

async fn health() -> &'static str { "OK" }

async fn article_of_the_day(
    State(state): State<Arc<AppState>>,
) -> Result<axum::Json<ArticleResponse>, StatusCode> {
    let cached = state.cached_article.read().await;
    match &*cached {
        Some(c) => Ok(axum::Json(c.response.clone())),
        None => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}
```

**Only two routes exist:** `/health` and `/api/article-of-the-day`. None of the four endpoints the frontend calls exist.

---

## Endpoint Coverage

| Endpoint | Frontend call site | Expected request/response | Backend implemented? |
|---|---|---|---|
| `GET /api/stories` | `ShadowingPractice.tsx:22` | Returns `Story[]` — array of `{id, title, source, published_year, level}` | **No** — 404 |
| `GET /api/stories/:id` | `ShadowingPractice.tsx:29` | Returns `StoryDetail` — `{id, title, source, french: {paragraphs: string[]}}` | **No** — 404 |
| `POST /api/stories/:id/tts-cache` | `ShadowingPractice.tsx:106` | Fire-and-forget warm-up; no response body needed | **No** — 404 |
| `POST /api/tts` | `ShadowingPractice.tsx:141`, `ShadowingPractice.tsx:182` | Request: `{text: string}`; Response: audio blob (MP3/WAV) | **No** — 404 |

---

## Audio/TTS Infrastructure in the Backend

**There is no TTS infrastructure in the current backend.** The `lacq/src/` directory contains:

- `lib.rs` — `AppState`, `BookMeta`, `ArticleResponse`, Gutenberg text fetch, tokenization, background worker
- `gutenberg.rs` — text cleaning, paragraph splitting, story extraction
- `routes/mod.rs` — two routes only
- `main.rs` — server bootstrap
- `config.rs` — environment config
- `tokenizer.rs` — LLM tokenization (French→English)

There is no `tts.rs`, no `stories.rs`, no TTS API key config, and no audio handling whatsoever. The backend has no stub or TODO for TTS or stories routes.

---

## Git History: Shadowing/Stories Were Previously Implemented, Then Deliberately Removed

The git log reveals a clear history:

| Commit | Message | What happened |
|---|---|---|
| `324d44e` | `feat(lacq): add TTS endpoint with Minimax API and caching` | Added `lacq/src/routes/tts.rs` |
| `6ddd214` | `feat(lacq): implement GET /api/stories and /api/stories/:id routes` | Added `lacq/src/routes/stories.rs` |
| `97a5fe2` | `feat(lacq): add POST /api/stories/:id/tts-cache endpoint for TTS pre-caching` | Extended `stories.rs` |
| `bd75469` | `feat: site revision — tab nav, typeracer, shadowing, vocab mining, TTS, typing page` | Added `ShadowingPractice.tsx` + `shadowing.astro` |
| `a987461` | `feat: add dev proxy for /api and update article components to use /api/stories/:id` | Added Vite dev proxy for local dev |
| `de0471b` | `clean up article-of-the-day and typing-race features` | **Deleted** `routes/stories.rs`, `routes/tts.rs`, `routes/article.rs`, `llm.rs`, `vocab.rs`, `gutendex.rs`, `translation.rs`; rewrote `routes/mod.rs` to only expose `/api/article-of-the-day` |

**Conclusion:** The shadowing backend was built and then deliberately removed in commit `de0471b` ("clean up article-of-the-day and typing-race features"). The frontend component `ShadowingPractice.tsx` was **not deleted** at that time — it was left in place without its backend. The `VocabMining.tsx` was deleted in the same commit, but `ShadowingPractice.tsx` was overlooked.

---

## `client:only` vs `client:load` on the Shadowing Page

`shadowing.astro` uses `client:only="solid-js"`. This is actually **correct** for this component because:

1. `ShadowingPractice.tsx` reads `window.location.protocol` and `window.location.host` directly in its fetch functions (lines 22, 29, 182)
2. It uses `localStorage` in `onMount` (line 81)
3. SSR would fail on both counts

However, `client:only` means the component has **no SSR fallback**. There is no server-side error page or graceful degradation. The only thing the server sends is the empty `<Base>` shell. All rendering happens in the browser.

This is why the page "gets stuck" — without SSR, the browser is the sole environment. When `fetch('/api/stories')` returns 404, the `createResource` error is set, but the `<Show when={stories.error}>` may not fire reactively without a `Suspense` boundary. The net effect is the user sees "Loading stories..." forever.

---

## Fix Approaches

### Approach A: Hide / unlink the shadowing page until a future release wires it up

**What to change:**
- Remove the shadowing entry from `HomepageTabs.tsx` (`PRACTICE` array, line 30)
- Remove the shadowing entry from `PracticeHub.tsx` (`PRACTICE` array, line 7)
- Optionally delete `src/pages/practice/shadowing.astro` and `src/components/ShadowingPractice.tsx`

**Tradeoffs:**
- Pros: Zero backend work; eliminates the broken UX immediately; one-line change in each of two files
- Cons: Feature is completely removed; must be re-implemented later to restore; `ShadowingPractice.tsx` is large (880 lines) and may be needed when the backend is built

**Invasiveness:** Very low. Two small edits to static data arrays.

---

### Approach B: Add a graceful "coming soon" / "not yet available" empty state to the page

**What to change:**
- Edit `ShadowingPractice.tsx` to show a static "coming soon" message instead of calling `createResource(fetchStories)`, OR
- Add a top-level `ErrorBoundary` from `solid-js` that catches the resource error and shows a friendly message

The most surgical version: replace the `createResource` with a static empty array and add a `<Show when={true}>` with the coming-soon message, or guard all the fetch calls with a feature flag.

**Tradeoffs:**
- Pros: Keeps the page URL alive; gives users a clear signal instead of a spinner; minimal code change; `ShadowingPractice.tsx` stays intact for future reactivation
- Cons: The "coming soon" message is static HTML/CSS work; the component's 880 lines still exist; if the page is linked from elsewhere, users still land on a static page

**Invasiveness:** Low to medium. Either a small guard around the resource, or a top-level `ErrorBoundary` added to the component.

---

### Approach C: Implement the missing endpoints (TTS + stories) — full backend build

**What to change:**
- Restore `lacq/src/routes/stories.rs` (was deleted in `de0471b`) — `GET /api/stories` and `GET /api/stories/:id`
- Restore `lacq/src/routes/tts.rs` (was deleted in `de0471b`) — `POST /api/tts` using Minimax or another TTS provider
- Wire both into `lacq/src/routes/mod.rs`
- Add TTS API key to `lacq/src/config.rs`
- Update `lacq/Cargo.toml` to restore removed dependencies (`sha2`, `hex`, `regex`, `url`, `anyhow`, `futures`)

**Tradeoffs:**
- Pros: Full feature functionality restored; shadowing page works end-to-end
- Cons: Very large change — multiple Rust files, API keys, dependencies, TTS provider integration; requires Minimax API key or migration to another TTS provider; more testing surface; the TTS endpoint especially is complex (audio streaming, caching, rate limits)
- Risk: The original TTS implementation used a specific Minimax API integration that may need updates; the `stories.rs` implementation had `english_gutenberg_id`, `llm.rs` translation, and `vocab.rs` highlight extraction — all of which were deleted in `de0471b` and may not be desired in the new version

**Invasiveness:** Very high. Multiple Rust files, environment configuration, and frontend integration testing.

---

## Summary of Root Cause

The `ShadowingPractice.tsx` SolidJS island was built alongside a Rust backend (`/api/stories`, `/api/stories/:id`, `/api/stories/:id/tts-cache`, `/api/tts`) in a feature branch, but the entire backend for TTS, stories, vocab, and translations was **deliberately removed** in commit `de0471b` ("clean up article-of-the-day and typing-race features") while the frontend component was **not deleted**. The `createResource(fetchStories)` now makes a `GET /api/stories` request that returns 404, the promise rejects, and `stories.error` is set — but without a `Suspense` boundary wrapping the resource, SolidJS may not propagate the error signal to the `<Show when={stories.error}>` node, leaving the user stuck on "Loading stories..." indefinitely.

The backend has no TTS or stories infrastructure whatsoever. The `/api/article-of-the-day` endpoint is unrelated and uses a different data model (`ArticleResponse` with `paragraphs`, not `StoryDetail` with `french.paragraphs`).
