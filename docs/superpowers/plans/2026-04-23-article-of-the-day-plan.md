# Article-of-the-Day Real Data: Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the hardcoded mock in `article.rs` with a real pipeline that fetches daily Maupassant stories from Gutenberg, with English from Gutenberg or Minimax LLM fallback.

**Architecture:** AppState shared via Axum's State extractor holds curated book list, in-memory LRU cache for French text (1hr TTL), shared reqwest client, optional Minimax LLM provider. Daily rotation via days_since_epoch % book_count. Translation pipeline: disk cache → Gutenberg English → Minimax LLM → French only.

**Tech Stack:** Rust (Axum, reqwest, tokio, LRU cache via std::collections::HashMap+Mutex), Gutendex API, Gutenberg plain text, Minimax Chat API.

---

## File Overview

| File | Role |
|---|---|
| `lacq/src/lib.rs` | AppState struct, BookMeta, CachedText, exports |
| `lacq/src/gutenberg.rs` | `fetch_book_text()`, `find_english_edition()`, `clean_text()` fix |
| `lacq/src/llm.rs` | Real `MinimaxProvider` using reqwest + MINIMAX_API_KEY |
| `lacq/src/routes/article.rs` | Rewrite handler with State<AppState>, daily rotation, full pipeline |
| `lacq/src/routes/mod.rs` | Router with all routes consolidated, health included |
| `lacq/src/main.rs` | Build AppState, pass to router, use routes() from mod.rs |

---

## Task 1: Add AppState to lib.rs

**Files:**
- Modify: `lacq/src/lib.rs` — add AppState, BookMeta, CachedText types and exports

- [ ] **Step 1: Add new types to lib.rs**

Add after existing types (before `impl VocabHighlight`):

```rust
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct BookMeta {
    pub gutenberg_id: u64,
    pub title: String,
    pub collection: String,
    pub published_year: i32,
    pub english_gutenberg_id: Option<u64>,
}

pub struct CachedText {
    pub text: String,
    pub fetched_at: std::time::Instant,
}

impl BookMeta {
    pub fn curated() -> Vec<Self> {
        vec![
            BookMeta {
                gutenberg_id: 13609, // La Maison Tellier
                title: "La Maison Tellier".to_string(),
                collection: "Contes du jour et de la nuit".to_string(),
                published_year: 1885,
                english_gutenberg_id: Some(46026), // English: The Maison Tellier
            },
            BookMeta {
                gutenberg_id: 16168, // Boule de Suif
                title: "Boule de Suif".to_string(),
                collection: "Les Rougon-Macquart".to_string(),
                published_year: 1880,
                english_gutenberg_id: Some(7832),
            },
            BookMeta {
                gutenberg_id: 514, // Une Partie de Campagne
                title: "Une Partie de Campagne".to_string(),
                collection: "Contes du jour et de la nuit".to_string(),
                published_year: 1881,
                english_gutenberg_id: None,
            },
            BookMeta {
                gutenberg_id: 42430, // Le Petit Chose
                title: "Le Petit Chose".to_string(),
                collection: "Divers".to_string(),
                published_year: 1868,
                english_gutenberg_id: Some(15654),
            },
            BookMeta {
                gutenberg_id: 11438, // La Petite Roque
                title: "La Petite Roque".to_string(),
                collection: "Nouvelles".to_string(),
                published_year: 1885,
                english_gutenberg_id: None,
            },
            BookMeta {
                gutenberg_id: 54755, // Mademoiselle Fifi
                title: "Mademoiselle Fifi".to_string(),
                collection: "Contes de la guerre".to_string(),
                published_year: 1882,
                english_gutenberg_id: None,
            },
            BookMeta {
                gutenberg_id: 11391, // Le Coloquinte
                title: "Le Coloquinte".to_string(),
                collection: "La Maison Tellier".to_string(),
                published_year: 1880,
                english_gutenberg_id: None,
            },
            BookMeta {
                gutenberg_id: 54764, // Claire de Lune
                title: "Claire de Lune".to_string(),
                collection: "La Maison Tellier".to_string(),
                published_year: 1883,
                english_gutenberg_id: None,
            },
            BookMeta {
                gutenberg_id: 54765, // Nuit de Noël
                title: "Nuit de Noël".to_string(),
                collection: "Contes du jour et de la nuit".to_string(),
                published_year: 1884,
                english_gutenberg_id: None,
            },
            BookMeta {
                gutenberg_id: 54286, // Levee en Masse
                title: "Levée en Masse".to_string(),
                collection: "Souvenirs de guerre".to_string(),
                published_year: 1888,
                english_gutenberg_id: None,
            },
        ]
    }

    pub fn days_since_epoch() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() / 86400
    }

    pub fn for_today() -> Self {
        let books = Self::curated();
        let idx = (Self::days_since_epoch() % books.len() as u64) as usize;
        books[idx].clone()
    }
}

#[derive(Clone)]
pub struct AppState {
    pub curated_books: Vec<BookMeta>,
    pub french_cache: std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<u64, CachedText>>>,
    pub http: reqwest::Client,
    pub llm: Option<std::sync::Arc<dyn LlmProvider>>,
    pub translation_cache: TranslationCache,
}
```

Also add `use crate::llm::LlmProvider;` at the top and `use std::sync::Arc;`.

- [ ] **Step 2: Add impl block for AppState**

```rust
impl AppState {
    pub fn new(http: reqwest::Client, llm: Option<Arc<dyn LlmProvider>>, data_dir: String) -> Self {
        Self {
            curated_books: BookMeta::curated(),
            french_cache: std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
            http,
            llm,
            translation_cache: TranslationCache { data_dir },
        }
    }

    pub async fn get_french_text(&self, book_id: u64) -> Result<String, String> {
        {
            let cache = self.french_cache.lock().await;
            if let Some(cached) = cache.get(&book_id) {
                let age = cached.fetched_at.elapsed().as_secs();
                if age < 3600 {
                    return Ok(cached.text.clone());
                }
            }
        }

        let url = format!("https://www.gutenberg.org/files/{}/{}-0.txt", book_id, book_id);
        let text = self.http.get(&url).send().await
            .map_err(|e| format!("failed to fetch {}: {}", url, e))?
            .text().await
            .map_err(|e| format!("failed to read response: {}", e))?;

        let cleaned = crate::gutenberg::clean_text(&text);
        let cleaned = cleaned.trim().to_string();

        {
            let mut cache = self.french_cache.lock().await;
            cache.insert(book_id, CachedText {
                text: cleaned.clone(),
                fetched_at: std::time::Instant::now(),
            });
        }

        Ok(cleaned)
    }

    pub async fn get_english_text(&self, book_id: u64) -> Option<String> {
        let url = format!("https://www.gutenberg.org/files/{}/{}-0.txt", book_id, book_id);
        let text = self.http.get(&url).send().await.ok()?.text().await.ok()?;
        Some(crate::gutenberg::clean_text(&text).trim().to_string())
    }
}
```

- [ ] **Step 3: Run cargo build to verify it compiles**

Run: `cd lacq && cargo build 2>&1`
Expected: Compiles successfully (may have warnings about unused fields — those are fine)

---

## Task 2: Fix gutenberg.rs — add HTTP fetch and clean properly

**Files:**
- Modify: `lacq/src/gutenberg.rs`

- [ ] **Step 1: Update clean_text to strip Gutenberg headers/footers**

Replace the no-op `clean_text`:

```rust
pub fn clean_text(raw: &str) -> String {
    let start = raw.find("*** START").map(|i| raw[i..].find('\n').map(|j| i+j+1).unwrap_or(i)).unwrap_or(0);
    let end = raw.find("*** END").map(|i| i).unwrap_or(raw.len());
    let mut text = raw[start..end].to_string();
    text = text.replace("\r\n", "\n").replace('\r', '\n');
    text = text.replace("\n---", "\n\n").replace("---\n", "\n\n");
    while text.contains("\n\n\n") {
        text = text.replace("\n\n\n", "\n\n");
    }
    text.trim().to_string()
}
```

Add test:

```rust
#[test]
fn test_clean_text_strips_gutenberg_markers() {
    let input = "Some junk\n*** START OF THIS PROJECT GUTENBERG EBOOK ***\n\nReal content here.\n\n*** END OF THIS PROJECT GUTENBERG EBOOK ***\nMore junk";
    let result = clean_text(input);
    assert!(result.starts_with("Real content here."));
    assert!(!result.contains("GUTENBERG"));
}
```

- [ ] **Step 2: Run tests**

Run: `cd lacq && cargo test gutenberg 2>&1`
Expected: Tests pass

---

## Task 3: Add real MinimaxProvider to llm.rs

**Files:**
- Modify: `lacq/src/llm.rs`

- [ ] **Step 1: Add MinimaxProvider struct**

Add after `MockMinimaxProvider`:

```rust
pub struct MinimaxProvider {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl MinimaxProvider {
    pub fn new(api_key: String, base_url: String) -> Self {
        Self {
            api_key,
            base_url,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for MinimaxProvider {
    async fn translate(&self, text: &str) -> Result<String, LlmError> {
        #[derive(serde::Serialize)]
        struct Request {
            model: String,
            messages: Vec<serde_json::Value>,
        }
        #[derive(serde::Deserialize)]
        struct Response {
            choices: Vec<serde_json::Value>,
        }

        let body = Request {
            model: "deepseek-ai/DeepSeek-V2.5".to_string(),
            messages: vec![
                serde_json::json!({"role": "system", "content": "You are a translator. Translate the following French text to English. Return ONLY the translation, no explanations."}),
                serde_json::json!({"role": "user", "content": text}),
            ],
        };

        let resp = self.client
            .post(format!("{}/v1/text/translatechatcompletion", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError { message: format!("HTTP error: {}", e) })?;

        let resp_json: Response = resp.json().await
            .map_err(|e| LlmError { message: format!("JSON parse error: {}", e) })?;

        resp_json.choices.get(0)
            .and_then(|c| c.get("messages"))
            .and_then(|m| m.as_array())
            .and_then(|arr| arr.last())
            .and_then(|m| m.get("text"))
            .map(|t| t.to_string())
            .ok_or_else(|| LlmError { message: "No translation in response".to_string() })
    }
}
```

Also update `MockMinimaxProvider` to include `client: reqwest::Client` field (unused but avoids the need for async_trait on the struct).

- [ ] **Step 2: Run cargo build**

Run: `cd lacq && cargo build 2>&1`
Expected: Compiles with warnings about unused fields (fine)

---

## Task 4: Rewrite article.rs route handler

**Files:**
- Modify: `lacq/src/routes/article.rs`

- [ ] **Step 1: Write the full handler using AppState**

Replace the entire file:

```rust
use crate::{AppState, ArticleResponse, EnglishContent, FrenchContent, StoryMeta, VocabHighlight};
use axum::{extract::State, Json};
use std::sync::Arc;

pub async fn article_of_the_day(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ArticleResponse>, axum::http::StatusCode> {
    let book = crate::BookMeta::for_today();

    let french_text = state.get_french_text(book.gutenberg_id).await
        .map_err(|e| {
            tracing::error!("Failed to fetch French text for book {}: {}", book.gutenberg_id, e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let paragraphs_raw = crate::gutenberg::split_paragraphs(&french_text);
    let excerpt_paras = crate::gutenberg::extract_excerpt(&paragraphs_raw, 350);
    let excerpt_strings: Vec<String> = excerpt_paras.iter().map(|s| s.to_string()).collect();

    let vocab = crate::vocab::extract_vocab_words(&excerpt_strings);

    let english = if let Some(eng_id) = book.english_gutenberg_id {
        if let Ok(text) = state.get_english_text(eng_id).await {
            let eng_paragraphs = crate::gutenberg::split_paragraphs(&text);
            let eng_excerpt = crate::gutenberg::extract_excerpt(&eng_paragraphs, 350);
            let eng_strings: Vec<String> = eng_excerpt.iter().map(|s| s.to_string()).collect();
            Some(crate::EnglishContent {
                paragraphs: eng_strings,
                source: format!("gutenberg:{}", eng_id),
            })
        } else {
            None
        }
    } else {
        None
    };

    if english.is_none() {
        if let Some(llm) = &state.llm {
            let text = excerpt_strings.join("\n\n");
            let translated = llm.translate(&text).await
                .map_err(|e| {
                    tracing::error!("LLM translation failed: {}", e);
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR
                })?;
            let eng_content = crate::EnglishContent {
                paragraphs: translated.split("\n\n").map(|s| s.to_string()).collect(),
                source: "llm".to_string(),
            };
            return Ok(Json(ArticleResponse {
                story: StoryMeta {
                    id: book.gutenberg_id.to_string(),
                    title: book.title,
                    source: book.collection,
                    published_year: book.published_year,
                },
                french: FrenchContent {
                    paragraphs: excerpt_strings,
                    vocab_highlights: vocab,
                },
                english: Some(eng_content),
                date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            }));
        }
    }

    Ok(Json(ArticleResponse {
        story: StoryMeta {
            id: book.gutenberg_id.to_string(),
            title: book.title,
            source: book.collection,
            published_year: book.published_year,
        },
        french: FrenchContent {
            paragraphs: excerpt_strings,
            vocab_highlights: vocab,
        },
        english,
        date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
    }))
}
```

Note: `chrono` needs to be added as a dependency for the date string.

- [ ] **Step 2: Run cargo build**

Run: `cd lacq && cargo build 2>&1`
Expected: May fail — we need to add `chrono` to Cargo.toml and fix any type mismatches

---

## Task 5: Update main.rs to build AppState and use routes()

**Files:**
- Modify: `lacq/src/main.rs`
- Modify: `lacq/Cargo.toml` (add chrono)

- [ ] **Step 1: Add chrono to Cargo.toml dependencies**

Add to `[dependencies]`:
```toml
chrono = "0.4"
```

- [ ] **Step 2: Rewrite main.rs to build AppState**

Replace `main.rs`:

```rust
use axum::routing::get;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = lacq::config::Config::from_env();

    let http = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("reqwest client");

    let llm = if config.minimax_api_key != "mock-key" && !config.minimax_api_key.is_empty() {
        Some(Arc::new(lacq::llm::MinimaxProvider::new(
            config.minimax_api_key,
            config.minimax_base_url,
        )) as Arc<dyn lacq::llm::LlmProvider>)
    } else {
        None
    };

    let state = Arc::new(lacq::AppState::new(http, llm, config.data_dir));

    let app = lacq::routes::routes()
        .nest_service("/", ServeDir::new("dist"))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

Note: Need to update `routes/mod.rs` to expose a `routes()` function that accepts state.

- [ ] **Step 3: Update routes/mod.rs to accept AppState**

The `routes()` function in `routes/mod.rs` currently returns `Router` without state. Update it:

```rust
use axum::{routing::get, Router};
use std::sync::Arc;

pub mod article;
pub mod stories;

pub fn routes() -> Router<(), Arc<crate::AppState>> {
    Router::new()
        .route("/health", get(health))
        .route("/api/article-of-the-day", get(article::article_of_the_day))
        .route("/api/stories", get(stories::list_stories))
        .route("/api/stories/:id", get(stories::get_story))
}

async fn health() -> &'static str {
    "OK"
}
```

But this creates a conflict: main.rs builds `AppState` as `Arc<AppState>` but `Router::with_state` takes `T` not `Arc<T>`. Axum's `State` extractor works with `Arc<T>` wrapping. Update the routes to use `State<Arc<AppState>>` consistently.

Actually, since `Router::with_state(state)` takes ownership, we should pass `Arc<AppState>` directly:

```rust
let state: Arc<AppState> = Arc::new(AppState::new(http, llm, config.data_dir));
let app = lacq::routes::routes().with_state(state);
```

And update `routes/mod.rs` to use `State<Arc<AppState>>` in handlers. But `article::article_of_the_day` currently takes `State<Arc<AppState>>` — that should work.

Actually, `Router::new().with_state(state)` takes `T` where `T` is `Arc<AppState>`. This works because `Arc<T>` implements `Clone`. The `State` extractor inside handlers will see `Arc<AppState>`.

Wait, `with_state` takes `self` and consumes `self`. The issue is the generic type of the Router. If `routes()` returns `Router<(), Arc<AppState>>`, then `with_state` takes `Arc<AppState>`. That should work.

Actually let me just check: in `routes/mod.rs`, the Router needs type params: `Router<_, Arc<AppState>>`. Let's use `()` for the first param since we don't have nested routes.

```rust
pub fn routes() -> Router<(), Arc<AppState>> {
    Router::new()
        .route("/health", get(health))
        .route("/api/article-of-the-day", get(article::article_of_the_day))
        .route("/api/stories", get(stories::list_stories))
        .route("/api/stories/:id", get(stories::get_story))
}
```

And article.rs uses `State<Arc<AppState>>`. That should compile.

- [ ] **Step 4: Run cargo build**

Run: `cd lacq && cargo build 2>&1`
Expected: Compiles successfully. If errors, fix them iteratively.

---

## Task 6: Test end-to-end

- [ ] **Step 1: Start the backend**

Run: `cd lacq && cargo run`
Expected: Compiles and binds to 0.0.0.0:8080

- [ ] **Step 2: Test the endpoint**

Run: `curl http://localhost:8080/api/article-of-the-day | python3 -m json.tool`
Expected: Returns real ArticleResponse with French paragraphs, real title, date = today. English may be present if the curated book has an english_gutenberg_id, else it falls back to LLM or French-only.

- [ ] **Step 3: Verify vocab highlights are populated**

The response should have `vocab_highlights` array with word, paragraph_index, start_offset, end_offset entries.

- [ ] **Step 4: Test rotation** (manual)

Wait 1 second, kill server, restart, curl again. Should get same book (rotation is daily, not per-request).

---

## Task 7: Add cargo test for the full pipeline

- [ ] **Step 1: Add integration test in tests/ directory**

Run: `mkdir -p lacq/tests && cat > lacq/tests/article_pipeline.rs << 'EOF'
use lacq::{AppState, BookMeta};

#[tokio::test]
async fn test_book_rotation_is_deterministic() {
    let book1 = BookMeta::for_today();
    let book2 = BookMeta::for_today();
    assert_eq!(book1.gutenberg_id, book2.gutenberg_id, "same day should return same book");
}

#[tokio::test]
async fn test_app_state_creates_successfully() {
    let http = reqwest::Client::new();
    let state = AppState::new(http, None, "data".to_string());
    assert_eq!(state.curated_books.len(), 10);
}
EOF`

- [ ] **Step 2: Run the test**

Run: `cd lacq && cargo test --test article_pipeline 2>&1`
Expected: Tests pass

---

## Task 8: Commit

- [ ] **Step 1: Stage and commit**

```bash
cd /home/susliko/programming/lacoquille
git add lacq/src/
git commit -m "feat(lacq): wire article-of-the-day to real Gutenberg data

- Add AppState with curated Maupassant book list (10 books)
- Implement daily rotation via days_since_epoch % len
- French text fetched from Gutenberg with 1hr in-memory cache
- clean_text() now strips Gutenberg header/footer markers
- Add real MinimaxProvider for LLM translations
- Rewrite article_of_the_day handler using State<AppState>
- Translation pipeline: disk cache → Gutenberg English → LLM → French-only
- Add integration tests for rotation and AppState creation
- Add chrono dependency for date formatting"
```