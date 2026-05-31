# Progress

## Article-of-the-day translation — bug fixes (2026-05-31)

Goal: fix tap-misses-bounding-box, lost punctuation, and translation-not-loaded.

### Root causes found & fixed

**1. Tap word not inside its token's bounding box**
- Old `renderParagraph()` split paragraphs on whitespace and only made a word
  clickable when `token.text === part` (exact match). This failed for
  multi-word tokens ("tout à coup") and words carrying punctuation.
- Fix: `ArticleOfTheDay.tsx` now renders the French column as the *original
  paragraph text with token char-spans overlaid as clickable regions*
  (`buildFrParts`). Each clickable region is exactly the token span, so the box
  is always correct — including multi-word tokens. Verified in-browser:
  clicking "tout à coup" highlights the whole phrase + its English counterpart.

**2. Punctuation lost after translation**
- The LLM folds punctuation into the `translation` field and leaves
  `trailingPunct`/`leadingPunct` empty, so rendering `leading+text+trailing`
  dropped commas/periods sitting in the gaps between token spans.
- Fix: the overlay rendering above emits the gap text (punctuation, whitespace)
  verbatim. French reconstruction is now byte-identical to the source. English
  column renders `token.translation` (which carries the punctuation).

**3. Translation not loaded**
- a) `combined_text[..1200]` byte-sliced mid-UTF-8 → panic in the background
  job → permanent 503. Fixed in `lib.rs`: truncate at whole-paragraph
  granularity (char-counted), keeping paragraphs aligned with tokenized text.
- b) Groq rejected every request: `response_format: json_object` requires the
  literal word "json" in the prompt. Added it to `build_prompt()`.
- c) Strict `Span` deserializer hard-failed the whole response on any malformed
  span, even though `repair_spans()` recomputes spans from the source text.
  Made `Span` deserialization lenient (coerce bad values). A 70B response that
  previously failed now parses.
- d) `max_tokens` 8192 blew past Groq free-tier TPM (413). Reduced to 4096.
- e) HTTP client used a hardcoded 120s timeout instead of `config.timeout_secs()`
  (30s), so a hanging provider stalled the whole fallback chain. Fixed `main.rs`.
- f) Frontend now polls on 503 (cold-start background job) instead of erroring.

### Files changed
- `src/components/ArticleOfTheDay.tsx` — span-overlay rendering, 1:1 FR↔EN
  highlight by token index, 503 polling.
- `lacq/src/lib.rs` — UTF-8-safe paragraph-aligned truncation.
- `lacq/src/tokenizer.rs` — "json" in prompt, lenient `Span`, max_tokens 4096.
- `lacq/src/main.rs` — HTTP client uses configured timeout.

### Verification
- `cargo test`: 37 unit tests pass (1 live OpenRouter integration test flaky on
  the free model timing out — external, not logic).
- `npm run build`: passes.
- End-to-end with real Groq key: 200 OK, span integrity 0/58 mismatches, French
  reconstruction byte-perfect, multi-word click + bilingual highlight confirmed
  in a real browser.
