# Article of the Day — Design Spec

**Date**: 2026-04-21
**Status**: Approved

---

## 1. Concept & Vision

A daily Maupassant short story or excerpt delivered in-context for French learners. Each day a different story appears — French text, English translation (from Gutenberg where available, LLM-generated otherwise). The feature turns passive reading into active mining: highlighted words are vocabulary candidates outside common French.

The tone is literary and calm — not gamified. It's a daily appointment, like a newspaper, but one that actually helps you learn.

---

## 2. Data Sources

### Primary Source: Gutendex API

`GET https://gutendex.com/books?languages=fr&author=Maupassant`

Returns book metadata. Text fetched from:

```
https://www.gutenberg.org/files/<book_id>/<book_id>-0.txt
```

### Initial Book List (Preload)

| Title | Gutenberg ID | Notes | Rotation starts |
|---|---|---|---|
| Contes du jour et de la nuit | 14790 | 1885, ~40 stories | Day 1 |
| Original Short Stories Vol 1 | 13509 | 1884, ~20 stories | Day 41 |
| Original Short Stories Vol 4 | 13512 | ~20 stories | Day 61 |

Total: **~80 stories** (~3 months before rotation repeats)

### Translation Strategy

1. **Gutenberg English first** — check Gutendex for same author + similar title in English (`languages=en`, author=Maupassant)
2. **Minimax fallback** — if no English version, call Minimax LLM to translate. Cache result.

---

## 3. Story Parsing

### Text Cleaning

- Strip Gutenberg header/footer (lines before `*** START OF` and after `*** END OF`)
- Replace `\r\n` with `\n`, normalize whitespace
- Split on double newlines to get paragraphs

### Story Separation

Maupassant collections separate stories with blank lines. Blank-line splitting gives individual story units.

### Excerpt Sizing

Target **~350 words** per daily article. Stories shorter than 350 words use the full story.

### Metadata Per Story

```json
{
  "id": "maupassant-contes-001",
  "title": "La Maison Tellier",
  "originalTitle": "La Maison Tellier",
  "source": "Contes du jour et de la nuit",
  "bookId": 14790,
  "publishedYear": 1885,
  "genre": ["short story", "normandy", "comedy"],
  "level": "B1-B2",
  "paragraphs": ["...", "..."],
  "excerptStart": 0,
  "excerptEnd": 3,
  "hasEnglishTranslation": true,
  "englishBookId": 13509
}
```

---

## 4. Vocabulary Extraction (Highlighted Only)

- Compare against a common French stop-word list (top ~3000 most frequent words)
- Words **outside** this list are highlighted in the text
- No popovers, no save functionality in this version

---

## 5. Rust Backend (`lacq/`)

### Tech Stack

- **Axum** web framework
- **Reqwest** for HTTP
- **Tokio** async runtime
- **Serde** for JSON
- **File-based cache** — no database

### LLM Provider Trait

```rust
trait LlmProvider: Send + Sync {
    async fn translate(&self, text: &str, target_lang: &str) -> Result<String, LlmError>;
}
```

Implementations: `MiniMaxProvider` (mocked for now), `OpenAiProvider`, `AnthropicProvider`, `OllamaProvider`.

### Endpoints

| Method | Path | Description |
|---|---|---|
| GET | `/api/article-of-the-day` | Full daily article (French + translation + vocab highlights) |
| GET | `/api/stories` | List of all story metadata |
| GET | `/api/stories/{id}` | Full story text |

### Daily Rotation

```rust
fn get_daily_story_index() -> usize {
    let days_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() / 86400;
    (days_since_epoch as usize) % STORIES.len()
}
```

Deterministic, no state, same story for all users on a given UTC day.

### Cache Structure

```
data/
  stories.json               # All story metadata (refreshed weekly)
  translations/
    <book_id>/
      <paragraph_hash>.json  # Cached translation
  vocab/
    <word>.json             # Cached vocab definitions (future)
```

---

## 6. Frontend

### Route

`GET /stories/article-of-the-day` — Astro SSR page calling `lacq` API

### Toggle Modes (SolidJS signal)

- **Français** — French only
- **English** — English translation only
- **Side by side** — French left, English right

### Interaction

- Tap highlighted vocab word — no action in this version (just visual highlight)
- Toggle between modes — SolidJS signal, no page reload

### Homepage Badge

```astro
{
  id: "article-of-the-day",
  name: "Article du jour",
  desc: "A daily Maupassant short story with translation.",
  href: "/stories/article-of-the-day",
  badge: "NEW"
}
```

---

## 7. File Structure

```
frenchlation/                   # existing Astro project
  src/
    pages/
      stories/
        article-of-the-day.astro
    components/
      ArticleOfTheDay.tsx       # SolidJS island with toggle

lacq/                           # new Rust binary
  Cargo.toml
  src/
    main.rs
    lib.rs
    config.rs                  # LLM API keys, base URLs
    gutendex.rs                # Gutendex API client
    gutenberg.rs              # Text fetching + cleaning + parsing
    translation.rs            # Translation (Gutenberg first, Minimax fallback)
    cache.rs                  # File-based cache layer
    llm.rs                    # LLM provider trait + implementations
    routes/
      mod.rs
      article.rs
      stories.rs
  data/                        # populated at runtime (gitignored)
    translations/
  tests/
    parsing_tests.rs

docs/
  specs/
    2026-04-21-article-of-the-day-design.md
```

---

## 8. Implementation Order

1. Scaffold `lacq/` — `Cargo.toml`, minimal Axum server with health check
2. Gutendex client — fetch and cache book list for all 3 books, write `data/stories.json`
3. Gutenberg text fetch — fetch, clean, split into story paragraphs
4. Story metadata — map paragraph ranges to story titles; store in `stories.json`
5. Translation layer — check Gutenberg English first, Minimax fallback with cache
6. Vocabulary extraction — highlight only (no popovers)
7. `/api/article-of-the-day` endpoint — combine all above with daily rotation
8. `/api/stories/{id}` endpoint — full story text
9. Astro SSR page — fetch from `lacq` API
10. SolidJS toggle component — Français / English / Side by side
11. Homepage badge — link article-of-the-day on the topics grid

---

## 9. Deferred (future iterations)

- Vocab word popovers with definitions
- Save words to localStorage vocab list
- "Read more" link to full story page
- User accounts for synced vocab
- Periphrases exercises from LLM
- Speech-to-text features

---

## 10. Open Questions

- [x] Preload 3 Maupassant books (14790, 13509, 13512) — decided
- [x] Excerpt ~350 words — decided
- [x] Minimax LLM with mock implementation for now — decided
- [x] Vocab saving not implemented — deferred
- [ ] LLM provider config — tokens provided later, env var for now
