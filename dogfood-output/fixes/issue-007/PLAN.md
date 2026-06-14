# Implementation Plan: ISSUE-007 — Grammar Topic Discoverability

## Goal

Make all 9 grammar sub-topics reachable from the UI in 2 clicks or fewer. Replace the redirect stub at `/reference/grammar/` with a real index page that lists all 9 topics, and add a "See all grammar →" link to the home page Knowledge tab so users can discover the full grammar index.

---

## Tasks

### 1. Rewrite `src/pages/reference/grammar/index.astro`

Replace the redirect stub with a real index page modeled after `src/pages/reference/verbs/index.astro`. The page uses the same pattern: eyebrow + h1 + intro paragraph + a grid of topic cards linking to each grammar sub-page.

**New file content** (`src/pages/reference/grammar/index.astro`):

```astro
---
import Base from "../../../layouts/Base.astro";

const title = "Grammar";
const description = "All French grammar topics: from articles and pronouns to negation, syntax, and asking questions.";

const topics = [
  { title: "Articles",         slug: "articles"        },
  { title: "Adjectives",       slug: "adjectifs"       },
  { title: "Adverbs",          slug: "adverbs"         },
  { title: "Prepositions",     slug: "prepositions"    },
  { title: "Pronouns",         slug: "pronouns"        },
  { title: "Negation",         slug: "negation"        },
  { title: "Asking Questions", slug: "interrogation"   },
  { title: "Reflexive Verbs",  slug: "reflexive-verbs" },
  { title: "Syntax & Order",   slug: "syntax"          },
];
---

<Base title={title} description={description}>
  <div style="margin-bottom: 1.5rem;">
    <p class="home-eyebrow">Grammaire</p>
    <h1>Grammar</h1>
    <p style="color: var(--text-2); margin-top: 0.4rem;">
      {description}
    </p>
  </div>

  <div class="grammar-index-grid">
    {topics.map(topic => (
      <a href={`/reference/grammar/${topic.slug}/`} class="grammar-index-card">
        <span class="grammar-index-card-title">{topic.title}</span>
        <span class="grammar-index-card-arrow">→</span>
      </a>
    ))}
  </div>
</Base>

<style>
  .grammar-index-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 0.75rem;
    margin-top: 1.5rem;
  }

  .grammar-index-card {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 1.25rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    text-decoration: none;
    color: var(--text);
    background: var(--bg-surface);
    transition: border-color 0.2s, background 0.2s;
  }

  .grammar-index-card:hover {
    border-color: var(--accent);
    background: var(--bg-elevated);
  }

  .grammar-index-card-title {
    font-weight: 500;
    font-size: 0.9375rem;
  }

  .grammar-index-card-arrow {
    font-size: 1rem;
    color: var(--text-muted);
    transition: color 0.2s, transform 0.2s;
  }

  .grammar-index-card:hover .grammar-index-card-arrow {
    color: var(--accent);
    transform: translateX(3px);
  }
</style>
```

**Acceptance**: Build passes, all 9 topics render as links on `/reference/grammar/`.

---

### 2. Add "See all grammar →" link to `src/components/HomepageTabs.tsx`

After the Knowledge grid (`<For each={REGIONS}>`), add a small text link that points to `/reference/grammar/`. The link should be visually subtle — below the grid, small text, no card styling — consistent with the site's in-page link style.

**Change location**: inside the Knowledge `<div role="tabpanel">`, after the closing `</div>` of `.poly-grid`, before the Practice tab panel.

**Exact diff**:

```tsx
      {/* Knowledge tab */}
      <div
        role="tabpanel"
        class="homepage-tab-panel"
        style={{ display: tab() === "knowledge" ? "block" : "none" }}
      >
        <div class="poly-grid">
          <For each={REGIONS}>
            {(region, i) => (
              <a
                href={PATHS[region.id]}
                class={`poly-card poly-card-${i()}`}
                style={{ "--card-color": region.color }}
              >
                <div class="poly-card-name">{region.name}</div>
                <div class="poly-card-sub">{region.sub}</div>
                <div class="poly-card-cta">explore →</div>
              </a>
            )}
          </For>
        </div>
        {/* NEW: See all grammar link */}
        <p style="margin-top: 1rem; text-align: center;">
          <a href="/reference/grammar" style="font-size: 0.875rem; color: var(--text-muted);">
            See all grammar →
          </a>
        </p>
      </div>
```

**Acceptance**: On the home page, the "See all grammar →" link appears below the Knowledge grid on the Knowledge tab.

---

## Files to Modify

- `src/pages/reference/grammar/index.astro` — replace redirect stub with real index page (full rewrite, ~50 lines)
- `src/components/HomepageTabs.tsx` — add "See all grammar →" link below the Knowledge grid (3 lines of JSX + wrapping `<p>`)

---

## Exact Topic Map

These are the 9 grammar sub-topics, their display titles, and their URLs. Derived from the `<Base title="...">` frontmatter of each topic page.

| Slug | Display title | URL |
|------|--------------|-----|
| `articles` | Articles | `/reference/grammar/articles/` |
| `adjectifs` | Adjectives | `/reference/grammar/adjectifs/` |
| `adverbs` | Adverbs | `/reference/grammar/adverbs/` |
| `prepositions` | Prepositions | `/reference/grammar/prepositions/` |
| `pronouns` | Pronouns | `/reference/grammar/pronouns/` |
| `negation` | Negation | `/reference/grammar/negation/` |
| `interrogation` | Asking Questions | `/reference/grammar/interrogation/` |
| `reflexive-verbs` | Reflexive Verbs | `/reference/grammar/reflexive-verbs/` |
| `syntax` | Syntax & Order | `/reference/grammar/syntax/` |

> Note: The display titles in the index are English-friendly (e.g., "Adjectives" not "Adjectifs") to match the site's practice elsewhere (e.g., the REGIONS array uses "Adjectifs" internally but the cards show "Adjectifs" — the index page can use English titles for accessibility). Alternatively, match the frontmatter titles exactly (`Adjectives`, `Adverbs`, `French Pronouns`, `Reflexive Verbs`, `Syntax & Word Order`). Use whichever feels right; both work.

---

## Verification Commands

```sh
cd /home/susliko/programming/lacoquille
npm run build
npx astro check
```

Then in agent-browser (main session only, not subagent):

1. Open `http://localhost:4321/`
2. Click "See all grammar →" below the Knowledge grid
3. Verify the URL is `/reference/grammar/` (not a redirect loop back to `/`)
4. Verify 9 topic cards are visible, each with a working link
5. Click "Interrogation" and "Reflexive Verbs" to confirm they navigate to their pages
6. Open `/reference/verbs/` to confirm the verbs index still works

---

## Dependencies

- **Task 1** (grammar index) has no dependencies — it can be implemented first or second.
- **Task 2** (home page link) depends on Task 1 — the link should point to a working page.

---

## Risks & Mitigations

1. **Risk**: The grammar index CSS class names (`.grammar-index-grid`, etc.) might conflict with existing global styles or be unintentionally overridden.
   **Mitigation**: Use scoped `<style>` in the Astro file (Astro scopes styles by default). If conflicts arise, rename to more specific classes.

2. **Risk**: The "See all grammar →" link placement (centered, below the grid) might look visually disconnected on mobile.
   **Mitigation**: The `<p>` uses `margin-top: 1rem` and `text-align: center`. This matches the subtle, understated style used for secondary navigation links elsewhere in the site.

3. **Risk**: The `grammar-index-grid` uses `auto-fill` with `minmax(200px, 1fr)` which could produce a 1-column layout on very narrow screens (< 400px), making the grid look odd.
   **Mitigation**: Add a `@media (max-width: 480px)` rule that sets `grid-template-columns: 1fr 1fr` for compact screens, matching the mobile pattern used elsewhere.

4. **Risk**: The `home-eyebrow` class (coral red) is used in the verbs index but may be missing from the grammar index if the class name is wrong.
   **Mitigation**: The class name is confirmed from `src/pages/reference/verbs/index.astro` line 37: `<p class="home-eyebrow">Grammaire</p>`. Copy exactly.

5. **Risk**: If `npm run build` fails due to TypeScript errors (e.g., unused imports), fix them before committing.
   **Mitigation**: Run `npx astro check` after the rewrite and address any errors.

---

## Out of Scope

- Changes to any of the 9 grammar topic pages themselves
- Changes to the `REGIONS` array (keeping 8 cards, adding "see all" link instead of a 9th card)
- Changes to the verbs index page or verb diagram

---

## Branch & PR

- **Branch**: `fix/issue-007-grammar-discoverability`
- **PR title**: `fix(grammar): make all grammar topics reachable (ISSUE-007)`
- **PR description**: Replace the redirect stub at `/reference/grammar/` with a real index page listing all 9 grammar topics. Add a "See all grammar →" link below the Knowledge grid on the home page. Now `interrogation` and `reflexive-verbs` are reachable in 2 clicks from the home page.