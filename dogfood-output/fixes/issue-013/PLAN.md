# Implementation Plan

## Goal

Improve the Article of the Day header to include the author name (Guy de Maupassant), a clickable link to the source text, and an English genre label instead of French.

## Tasks

1. **Add constants for author name, genre translations, and URL builders**
   - File: `src/components/ArticleOfTheDay.tsx`
   - Changes: Add three constants after the imports and before the interface declarations:
     - `const AUTHOR = "Guy de Maupassant";`
     - `const SOURCE_LABELS: Record<string, string>` mapping French genres to English
     - `const getGutenbergSearchUrl(title: string)` and `const getWikisourceUrl(title: string)` helper functions
   - Acceptance: Constants are defined and can be used in the template

2. **Update the header rendering block**
   - File: `src/components/ArticleOfTheDay.tsx`
   - Changes: Replace lines 226–231 (the `<header class="article-header">` block) with an updated version that:
     - Shows `AUTHOR` as plain text
     - Shows the English genre label (falls back to French if not in mapping)
     - Includes a "Read on Gutenberg" link (search URL)
     - Includes a "Wikisource" link (title-based guess)
     - Shows the year in parentheses
   - Acceptance: The header renders author + genre + links + year

3. **Verify build and type checking**
   - Run `npm run build` to ensure no build errors
   - Run `npx astro check` to ensure no new TypeScript errors
   - Acceptance: Both commands pass without errors

4. **Browser verification (manual)**
   - Open `/stories/article-of-the-day/` in agent-browser
   - Verify the header shows "Guy de Maupassant", an English genre label (e.g., "Novel"), and two clickable links
   - Click the links to confirm they open Gutenberg/Wikisource in a new tab
   - Acceptance: Header displays correctly with working links

## Files to Modify

- `src/components/ArticleOfTheDay.tsx` — Add constants block and update header rendering

## New Files

None.

## Dependencies

Task 2 depends on Task 1 (constants must be defined before they can be used).

## Risks

| Risk | Mitigation |
|------|------------|
| Hardcoding "Maupassant" breaks if a non-Maupassant author is added | Add a comment noting this is temporary; future work can derive author from gutenberg_id |
| Wikisource URL is a guess and may 404 for some titles | Include a fallback Gutenberg search URL that always works |
| New genre labels not in the mapping fall through to French | Default to original French value; mapping is easily extensible |

## Code Changes

### New constants block (add after imports, before interfaces)

```typescript
// All curated books are Maupassant works. Hardcoded until the API exposes author.
const AUTHOR = "Guy de Maupassant";

// Translate French genre labels to English for clarity.
const SOURCE_LABELS: Record<string, string> = {
  "Roman": "Novel",
  "Nouvelles": "Short Stories",
  "Contes du jour et de la nuit": "Short Stories",
  "Les Rougon-Macquart": "Novel Cycle",
};

// Build a Gutenberg search URL from the article title.
function getGutenbergSearchUrl(title: string): string {
  return `https://www.gutenberg.org/ebooks/search/?query=maupassant+${encodeURIComponent(title)}`;
}

// Build a Wikisource URL from the article title (best-effort guess).
function getWikisourceUrl(title: string): string {
  return `https://en.wikisource.org/wiki/${encodeURIComponent(title)}`;
}
```

### Old header block (lines 226–231)

```tsx
<header class="article-header">
  <h1>{data().title}</h1>
  <p class="article-meta">
    {data().source} ({data().published_year})
  </p>
</header>
```

### New header block

```tsx
<header class="article-header">
  <h1>{data().title}</h1>
  <p class="article-meta">
    {AUTHOR} · <a href={getGutenbergSearchUrl(data().title)} target="_blank" rel="noopener">Read on Gutenberg</a> · <a href={getWikisourceUrl(data().title)} target="_blank" rel="noopener">Wikisource</a> ({data().published_year})
  </p>
</header>
```

## Out of Scope

- Backend changes to `lacq/src/lib.rs`
- Changes to article content or paragraphs
- Past/next article navigation (no API support)
- Changes to any other component or page

## Branch and PR

- Branch: `fix/issue-013-article-context`
- PR title: `fix(article): add author, source link, and clearer genre label (ISSUE-013)`
