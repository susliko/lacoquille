# Round 3 Recon Report

## ISSUE-006: Home page Practice tab missing "Vocabulary Mining" card

### Files involved

**`src/components/HomepageTabs.tsx`** (lines 26–31) — PRACTICE array in the home page SolidJS island:

```tsx
const PRACTICE = [
  { id: "typing",    name: "Typing Race",   sub: "speed conjugate drills", color: "#ff6b35", path: "/practice/typing" },
  { id: "shadowing", name: "Shadowing",     sub: "listen & repeat",       color: "#ff9f43", path: "/practice/shadowing" },
  { id: "article",   name: "Article of Day", sub: "daily Maupassant story", color: "#ff4757", path: "/stories/article-of-the-day" },
];
```

**`src/components/PracticeHub.tsx`** (lines 3–9) — PRACTICE array in the practice hub page:

```tsx
const PRACTICE = [
  { id: "article-of-day", name: "Article of Day",  sub: "daily Maupassant story",  color: "#ff4757", path: "/stories/article-of-the-day" },
  { id: "typing",         name: "Typing Race",    sub: "speed conjugate drills",   color: "#ff6b35", path: "/practice/typing" },
  { id: "shadowing",      name: "Shadowing",      sub: "listen & repeat",          color: "#ff9f43", path: "/practice/shadowing" },
  { id: "vocabulary",     name: "Vocabulary Mining", sub: "mine texts for words", color: "#f7b731", path: "/practice/vocabulary" },
];
```

### What's different

| Field | HomepageTabs | PracticeHub |
|-------|-------------|-------------|
| count | 3 | 4 |
| order | typing, shadowing, article | article, typing, shadowing, vocabulary |
| `vocabulary` entry | **missing** | present (`id: "vocabulary"`, name: "Vocabulary Mining", sub: "mine texts for words", color: `#f7b731`, path: `/practice/vocabulary`) |

### Does `/practice/vocabulary/` exist?

No. Listing `src/pages/practice/`:
```
index.astro
shadowing.astro
typing.astro
```

There is no `vocabulary.astro` or `vocabulary/` subdirectory. The path `/practice/vocabulary` does not correspond to any page.

### Recommendation

**Add a placeholder page at `src/pages/practice/vocabulary.astro`.** This is the smallest change that resolves the UI inconsistency. The page can be a minimal stub with a title and a "coming soon" message — it at least makes the home page link functional and prevents a 404. Do NOT remove "Vocabulary Mining" from PracticeHub; the hub is correct and complete.

---

## ISSUE-007: "Interrogation" and "Reflexive verbs" grammar pages are unreachable

### Grammar subdirectories in `src/pages/reference/grammar/`

```
[...slug].astro        ← catch-all for content collection entries
adjectifs/
adverbs/
articles/
index.astro            ← stub: return Astro.redirect("/")
interrogation/
negation/
prepositions/
pronouns/
reflexive-verbs/
syntax/
```

### Subdirectory frontmatter titles and home page link status

| Subdirectory | Frontmatter `title` | Linked from `REGIONS` in HomepageTabs? |
|-------------|---------------------|----------------------------------------|
| `adjectifs/` | "Adjectifs" | ✅ yes (`id: "adjectifs"`) |
| `adverbs/` | "Adverbes" | ✅ yes (`id: "adverbes"`) |
| `articles/` | "Articles" | ✅ yes (`id: "articles"`) |
| `negation/` | "Négation" | ✅ yes (`id: "negation"`) |
| `pronouns/` | "Pronoms" | ✅ yes (`id: "pronoms"`) |
| `prepositions/` | "Prép." | ✅ yes (`id: "preps"`) |
| `syntax/` | "Syntaxe" | ✅ yes (`id: "syntaxe"`) |
| `interrogation/` | "Interrogation" / "Asking Questions" | ❌ **missing** |
| `reflexive-verbs/` | "Reflexive Verbs" | ❌ **missing** |

The `REGIONS` array in `HomepageTabs.tsx` has 8 entries. Both `interrogation` and `reflexive-verbs` are absent.

### Grammar index stub

**`src/pages/reference/grammar/index.astro`** (entire file):

```astro
---
return Astro.redirect("/");
---
```

The grammar index does nothing — it just bounces to `/`. There is no page that lists all grammar topics.

### Recommendation

**Option A (simplest):** Add a "See all grammar topics →" text link at the bottom of the Knowledge grid (or as a 9th card). This requires no grid layout changes. The link would go to a new `/reference/grammar/` page that lists all 9 topics.

**Option B (most complete):** Create a real grammar index page at `src/pages/reference/grammar/index.astro` (replacing the redirect stub) that lists all grammar topics with links. Then add the "See all →" link to the home page.

**Option C (avoids new page):** Add `interrogation` and `reflexive-verbs` to the REGIONS array. This makes 10 cards in a 3-column grid (3+3+3+1), worsening the ISSUE-019 asymmetry.

**Recommended: Option B.** Replace the redirect stub with a real index page that lists all grammar topics, and add a small "See all grammar →" text link or card to the home page. This also fixes the root problem (no discoverable grammar index).

---

## ISSUE-011: Tense and choice-guide pages missing "GRAMMAIRE" breadcrumb

### Verbs index page — what it has

**`src/pages/reference/verbs/index.astro`** (lines 37–41):

```astro
<div style="margin-bottom: 1.5rem;">
  <p class="home-eyebrow">Grammaire</p>
  <h1>Verbes</h1>
  ...
</div>
```

The "GRAMMAIRE" element is a `<p class="home-eyebrow">` styled in coral red (`color: var(--coral)` per `global.css` line 368). It is **not** a `<nav class="breadcrumb">` element — it is a section eyebrow/label. The CSS:

```css
.home-eyebrow {
  font-size: 0.75rem;
  font-weight: 500;
  letter-spacing: 0.15em;
  text-transform: uppercase;
  color: var(--coral);
  margin-bottom: 0.75rem;
}
```

### Tense and choice pages — what they have

**`src/pages/reference/verbs/tenses/[slug].astro`** (lines 16–20):

```astro
<Base
  title={title}
  ...
  breadcrumb={[
    { label: "Verbes", href: "/reference/verbs" },
    { label: title },
  ]}
>
  <article>
    <div class="hgroup">
      <h1>{title}</h1>
      ...
    </div>
```

**`src/pages/reference/verbs/choice/[slug].astro`** (lines 17–23):

```astro
<Base
  title={title}
  breadcrumb={[
    { label: "Verbes", href: "/reference/verbs" },
    { label: title },
  ]}
>
  <article>
    <div class="hgroup">
      <h1>{title}</h1>
    </div>
```

Both pages:
- Have a `<div class="hgroup">` with `<h1>` but **no `<p class="home-eyebrow">` element**
- Pass a `breadcrumb` prop to `<Base>`, but `Base.astro` (see below) does not render it

### Base.astro — the breadcrumb prop is silently ignored

**`src/layouts/Base.astro`** (entire file):

```astro
---
interface Props {
  title: string;
  description?: string;
}
const { title, description = "..." } = Astro.props;
---
<!doctype html>
<html lang="en">
  <head>...</head>
  <body>
    <main>
      <slot />
    </main>
  </body>
</html>
```

`Base.astro` only accepts `title` and `description`. The `breadcrumb` and `section` props passed by grammar pages, the verbs index, and tense/choice pages are **silently dropped**. The `.breadcrumb` CSS class exists in `global.css` (lines 248–265) but is never rendered by any Astro layout in the codebase.

### Root cause

The "GRAMMAIRE" eyebrow is a page-level element, not a layout-level component. The tense and choice pages simply don't include it. Adding it to each drill-down page is the smallest fix.

### Recommendation

**Add `<p class="home-eyebrow">Grammaire</p>` above the `<h1>` in both drill-down page templates.** Specifically:

- In `src/pages/reference/verbs/tenses/[slug].astro`: add the eyebrow inside the `<div class="hgroup">` above `<h1>`.
- In `src/pages/reference/verbs/choice/[slug].astro`: same.

Do NOT try to use `Base.astro` for this — the layout has no breadcrumb rendering and adding it would require restructuring. The inline style approach used in the verbs index (`<div style="margin-bottom: 1.5rem;">`) can be reused or a shared CSS class can be added.

---

## ISSUE-013: Article of the Day — no author info, no source link, no context

### What ArticleOfTheDay.tsx currently renders

**`src/components/ArticleOfTheDay.tsx`** (lines 226–231):

```tsx
<header class="article-header">
  <h1>{data().title}</h1>
  <p class="article-meta">
    {data().source} ({data().published_year})
  </p>
</header>
```

The header shows only:
- Title: "Pierre et Jean"
- Meta: "Roman (1888)"

No author, no source link, no plot summary, no navigation to past articles.

### API response shape (from `lacq/src/lib.rs`)

**`ArticleResponse` struct** (lines 216–224):

```rust
pub struct ArticleResponse {
    pub title: String,           // e.g., "Pierre et Jean"
    pub source: String,          // e.g., "Roman" (from book.collection)
    pub published_year: i32,     // e.g., 1888
    pub paragraphs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokenized: Option<TokenizedPayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokenization_error: Option<String>,
}
```

The `source` field is `book.collection` from `BookMeta` (lines 15–55 in `lib.rs`). For "Pierre et Jean", `collection` is hardcoded as `"Roman"` — a genre label, not a source. There is **no `author` field** in `ArticleResponse`, and **no source URL field**.

### `BookMeta::curated()` entries (all 5 books)

| gutenberg_id | title | collection | year |
|-------------|-------|-----------|------|
| 10775 | Le Horla | Contes du jour et de la nuit | 1887 |
| 14790 | Contes du jour et de la nuit | Contes du jour et de la nuit | 1884 |
| 10746 | Boule de Suif | Les Rougon-Macquart | 1880 |
| 12011 | Monsieur Parent | Nouvelles | 1885 |
| 11131 | Pierre et Jean | Roman | 1888 |

All five books are Maupassant works. The author is never stored in the API response but is derivable from the known dataset.

### What data is available vs. missing

| Data | Available in API? | Currently shown? |
|------|------------------|-----------------|
| Title | ✅ yes | ✅ yes |
| Genre/source | ✅ yes (but wrong — "Roman" means "novel" in French) | ✅ yes |
| Year | ✅ yes | ✅ yes |
| Author | ❌ no | ❌ no |
| Source URL | ❌ no | ❌ no |
| Past articles nav | ❌ no | ❌ no |

### Recommendation

**Frontend-only, no backend changes needed.** In `ArticleOfTheDay.tsx`, update the header rendering block:

1. **Add author name:** Hardcode "Maupassant" (or a mapping from known gutenberg_ids → author names). Since all 5 curated books are Maupassant, a simple `const AUTHOR = "Guy de Maupassant"` constant is sufficient.

2. **Add source link:** The `source` field ("Roman") is not a URL. Use the known gutenberg_id `11131` to construct a Wikisource URL: `https://www.gutenberg.org/ebooks/11131`. Alternatively, link to `https://en.wikisource.org/wiki/Pierre_et_Jean` for the specific work. Since the API doesn't expose the gutenberg_id, hardcode the URL in the component (or derive it from a known-mapping dict).

3. **Improve "Roman" label:** The word "Roman" is in French and confusing to English readers. Change the display to say "Novel" or "Maupassant · Novel" to clarify.

Example change to the header block in `ArticleOfTheDay.tsx`:

```tsx
const AUTHOR = "Guy de Maupassant";
const SOURCE_URL = "https://www.gutenberg.org/ebooks/11131"; // Pierre et Jean

// In the header:
<p class="article-meta">
  {AUTHOR} · <a href={SOURCE_URL} target="_blank" rel="noopener">Novel</a> ({data().published_year})
</p>
```

For past-article navigation: the API has no mechanism for this. A simple "← Previous story" / "Next story →" pair with no actual navigation (just disabled links) is acceptable as a placeholder, or this can be noted as a future enhancement.

---

## ISSUE-019: Mobile (390px) — Knowledge grid still 3 columns, text wraps awkwardly

### Current mobile CSS

**`src/styles/global.css`** (lines 1391–1400):

```css
@media (max-width: 640px) {
  .homepage-tab-panel .poly-grid {
    display: flex;
    flex-wrap: wrap;
    height: 100%;
    align-content: flex-start;
  }

  .homepage-tab-panel .poly-card {
    flex: 0 0 calc(100% / 3);   /* ← 3 columns = ~114px at 390px */
    height: 26%;
  }
}
```

### Problem

- `calc(100% / 3)` = 33.33% per card → 3 columns at ~114px each on a 390px viewport
- 8 REGIONS cards in a 3-column grid = 3+3+2 layout (third row has only 2 cards, leaving an empty slot)
- Text in each card wraps to 2–3 lines: "personal · relative", "tenses & conjugation", "ne…pas · jamais"
- Cards are too narrow for the content

### Recommendation

**Switch to 2 columns on mobile.** This matches the PracticeHub grid (which uses `repeat(2, 1fr)` in its CSS) and eliminates the empty-slot problem (8 cards → 2+2+2+2 layout). The cards become ~195px wide at 390px, which is comfortable for text.

Change the mobile rule to:

```css
@media (max-width: 640px) {
  .homepage-tab-panel .poly-grid {
    display: flex;
    flex-wrap: wrap;
    height: 100%;
    align-content: flex-start;
  }

  .homepage-tab-panel .poly-card {
    flex: 0 0 calc(100% / 2);   /* 2 columns = ~195px at 390px */
    height: 26%;
  }
}
```

Note: `height: 26%` may still be too tall for 2-column cards. Consider reducing to `height: 20%` or removing the fixed height and letting content determine size (e.g., `min-height: 100px`).

---

## Summary of Recommended Fixes

| Issue | File(s) to change | Change |
|-------|-------------------|--------|
| ISSUE-006 | `src/pages/practice/vocabulary.astro` (new) | Add a placeholder "coming soon" page so the home page link works |
| ISSUE-007 | `src/pages/reference/grammar/index.astro` | Replace redirect stub with a real index listing all 9 topics; add a "See all grammar →" link to HomepageTabs |
| ISSUE-011 | `src/pages/reference/verbs/tenses/[slug].astro`, `src/pages/reference/verbs/choice/[slug].astro` | Add `<p class="home-eyebrow">Grammaire</p>` above `<h1>` in each |
| ISSUE-013 | `src/components/ArticleOfTheDay.tsx` | Add hardcoded author "Guy de Maupassant", link to Gutenberg/Wikisource, change "Roman" label to "Novel" |
| ISSUE-019 | `src/styles/global.css` | Change `calc(100% / 3)` → `calc(100% / 2)` in the `@media (max-width: 640px)` block |
