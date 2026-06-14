# Implementation Plan

## Goal

Make the "Vocabulary Mining" card on the home page Practice tab actually lead somewhere. Currently the card is missing from `HomepageTabs.tsx` (only 3 entries vs. 4 in `PracticeHub.tsx`), and the `/practice/vocabulary` path does not exist. After the fix, the home page Practice tab shows 4 cards including Vocabulary Mining, and clicking it navigates to a real (if minimal) page that explains the feature and that it's not yet available.

---

## Tasks

### Task 1: Add Vocabulary Mining to the `PRACTICE` array in `HomepageTabs.tsx`

- **File**: `src/components/HomepageTabs.tsx`
- **Changes**: Replace the 3-entry `PRACTICE` array with a 4-entry array. Add the vocabulary entry and reorder to match `PracticeHub.tsx` (Article, Typing, Shadowing, Vocabulary).
- **Acceptance**: `npx astro check` shows no errors. The home page Practice tab shows 4 cards.

### Task 2: Create `VocabularyPractice.tsx` placeholder component

- **File**: `src/components/VocabularyPractice.tsx` (new)
- **Changes**: Create a minimal SolidJS component that renders a "coming soon" card. Follow the same `client:only="solid-js"` island pattern as `ShadowingPractice.tsx`. The component should:
  - Display a heading "Vocabulary Mining"
  - Explain the feature (mine texts for words — pull vocabulary from the article-of-the-day text)
  - Show a "coming soon" badge
  - Provide a "← Back to practice hub" link
  - Mention Article of the Day as a working alternative
  - Use inline `<style>` for styling (matches the ShadowingPractice pattern)
  - No `fetch` calls (no backend dependency)
- **Acceptance**: Component renders correctly in isolation and in the page.

### Task 3: Create `vocabulary.astro` page

- **File**: `src/pages/practice/vocabulary.astro` (new)
- **Changes**: Create a thin wrapper page following the same pattern as `shadowing.astro` and `typing.astro`:
  - Import `Base` layout and `VocabularyPractice` component
  - Use `client:only="solid-js"` directive
  - Set title to "Vocabulary Mining — La Coquille"
- **Acceptance**: Page loads at `/practice/vocabulary/` without 404.

---

## Files to Modify

- `src/components/HomepageTabs.tsx` — Replace the 3-entry `PRACTICE` array with a 4-entry array

---

## New Files

- `src/components/VocabularyPractice.tsx` — Placeholder SolidJS component
- `src/pages/practice/vocabulary.astro` — Thin wrapper page

---

## Exact Code

### `src/components/HomepageTabs.tsx` — New `PRACTICE` array (4 entries)

```tsx
const PRACTICE = [
  { id: "article",   name: "Article of Day",   sub: "daily Maupassant story", color: "#ff4757", path: "/stories/article-of-the-day" },
  { id: "typing",    name: "Typing Race",       sub: "speed conjugate drills", color: "#ff6b35", path: "/practice/typing" },
  { id: "shadowing", name: "Shadowing",         sub: "listen & repeat",        color: "#ff9f43", path: "/practice/shadowing" },
  { id: "vocabulary", name: "Vocabulary Mining", sub: "mine texts for words",  color: "#f7b731", path: "/practice/vocabulary" },
];
```

### `src/components/VocabularyPractice.tsx` (new)

```tsx
import { Show } from "solid-js";

export default function VocabularyPractice() {
  return (
    <div class="vocab-practice">
      <style>{`
        .vocab-practice {
          max-width: 700px;
          margin: 0 auto;
          padding: 2rem;
        }

        .coming-soon-card {
          background: var(--surface);
          border: 1px solid var(--border-subtle);
          border-radius: var(--radius);
          padding: 3rem 2.5rem;
          text-align: center;
          animation: fade-in 0.3s ease;
        }

        @keyframes fade-in {
          from { opacity: 0; transform: translateY(8px); }
          to   { opacity: 1; transform: translateY(0); }
        }

        .coming-soon-badge {
          display: inline-block;
          background: var(--amber-soft);
          color: var(--amber);
          border: 1px solid var(--amber);
          border-radius: var(--radius-pill);
          padding: 0.25rem 0.75rem;
          font-size: 0.75rem;
          font-weight: 600;
          letter-spacing: 0.05em;
          text-transform: uppercase;
          margin-bottom: 1.5rem;
        }

        .vocab-heading {
          font-family: var(--font-display);
          font-size: 2rem;
          font-weight: 700;
          color: var(--text);
          margin: 0 0 0.75rem 0;
        }

        .vocab-sub {
          font-size: 1.1rem;
          color: var(--text-2);
          margin-bottom: 2rem;
          line-height: 1.6;
        }

        .vocab-description {
          background: var(--surface-2);
          border-radius: var(--radius-sm);
          padding: 1.25rem 1.5rem;
          margin-bottom: 2rem;
          text-align: left;
        }

        .vocab-description p {
          margin: 0 0 0.75rem 0;
          color: var(--text-2);
          font-size: 0.95rem;
          line-height: 1.6;
        }

        .vocab-description p:last-child {
          margin-bottom: 0;
        }

        .vocab-alternative {
          background: var(--indigo-soft);
          border: 1px solid var(--indigo);
          border-radius: var(--radius-sm);
          padding: 1rem 1.25rem;
          margin-bottom: 2rem;
          text-align: left;
        }

        .vocab-alternative p {
          margin: 0;
          color: var(--indigo);
          font-size: 0.9rem;
          line-height: 1.5;
        }

        .vocab-actions {
          display: flex;
          justify-content: center;
          gap: 1rem;
          flex-wrap: wrap;
        }

        .back-link {
          display: inline-flex;
          align-items: center;
          gap: 0.5rem;
          padding: 0.6rem 1.25rem;
          border: 1px solid var(--border);
          background: var(--surface-2);
          color: var(--text-2);
          border-radius: var(--radius-sm);
          text-decoration: none;
          font-size: 0.9rem;
          font-weight: 500;
          transition: all var(--transition);
        }

        .back-link:hover {
          border-color: var(--coral);
          color: var(--coral);
        }

        .alt-link {
          display: inline-flex;
          align-items: center;
          gap: 0.5rem;
          padding: 0.6rem 1.25rem;
          background: var(--coral);
          color: #fff;
          border-radius: var(--radius-sm);
          text-decoration: none;
          font-size: 0.9rem;
          font-weight: 500;
          transition: background var(--transition);
        }

        .alt-link:hover {
          background: #e63946;
        }

        @media (max-width: 600px) {
          .vocab-practice {
            padding: 1.5rem 1rem;
          }
          .vocab-heading {
            font-size: 1.5rem;
          }
        }
      `}</style>

      <div class="coming-soon-card">
        <div class="coming-soon-badge">Coming Soon</div>
        <h1 class="vocab-heading">Vocabulary Mining</h1>
        <p class="vocab-sub">Mine texts for words</p>

        <div class="vocab-description">
          <p>
            <strong>Vocabulary Mining</strong> will let you pull vocabulary from
            the Article of the Day text — automatically extracting key words,
            phrases, and their context so you can build your personal word list
            while reading.
          </p>
          <p>
            Select any passage, highlight words you want to learn, and save them
            to your vocabulary bank for later review. Mining has never been this
            easy.
          </p>
        </div>

        <div class="vocab-alternative">
          <p>
            💡 In the meantime, try the <strong>Article of the Day</strong> to read
            Maupassant stories in French — the best way to encounter vocabulary in
            context.
          </p>
        </div>

        <div class="vocab-actions">
          <a href="/practice" class="back-link">
            ← Back to practice hub
          </a>
          <a href="/stories/article-of-the-day" class="alt-link">
            Read Article of the Day →
          </a>
        </div>
      </div>
    </div>
  );
}
```

### `src/pages/practice/vocabulary.astro` (new)

```astro
---
import Base from "../../layouts/Base.astro";
import VocabularyPractice from "../../components/VocabularyPractice";
---

<Base title="Vocabulary Mining — La Coquille" section="practice">
  <VocabularyPractice client:only="solid-js" />
</Base>
```

---

## Dependencies

- Task 2 (VocabularyPractice component) must be completed before Task 3 (vocabulary.astro page), since the .astro file imports the component.
- Task 1 (HomepageTabs update) is independent and can be done in parallel.

---

## Risks & Mitigations

| Risk | Likelihood | Mitigation |
|------|-----------|------------|
| 4 cards in the 3-column grid layout breaks the visual layout on the home page | Low (pre-existing) | ISSUE-019 tracks the mobile grid asymmetry separately. Adding the 4th card is the right move for consistency; layout fix is out of scope here. |
| When the full Vocabulary Mining feature ships, the placeholder needs to be replaced | Medium | Future implementors should add a `VOCABULARY_AVAILABLE` feature flag (following the `SHADOWING_AVAILABLE` pattern if one exists) to make the transition explicit. The placeholder component name `VocabularyPractice` is intentional — the full feature can reuse it. |
| The order of cards in the Practice tab differs from the Practice Hub | Low | The user-specified order (Article, Typing, Shadowing, Vocabulary) is implemented to match the user's request. |
| No API endpoints exist for vocabulary mining (placeholder has no data) | Low | The placeholder makes no `fetch` calls. When the real feature ships, the component will be expanded with API integration. |

---

## Acceptance Criteria

1. `npm run build` passes with no errors
2. `npx astro check` shows no new errors
3. On the home page, the Practice tab shows 4 cards (Article of Day, Typing Race, Shadowing, Vocabulary Mining)
4. Clicking "Vocabulary Mining" navigates to `/practice/vocabulary/` (no 404)
5. The `/practice/vocabulary/` page shows a friendly "coming soon" message with:
   - Heading "Vocabulary Mining"
   - Explanation of the feature
   - "Coming Soon" badge
   - "← Back to practice hub" link
   - "Read Article of the Day →" link as an alternative
6. The page makes no `fetch` calls to non-existent endpoints
7. The "Vocabulary Mining" text appears in the DOM on the home page Practice tab

---

## Verification Commands

```sh
# Build verification
cd /home/susliko/programming/lacoquille && npm run build

# Type check
cd /home/susliko/programming/lacoquille && npx astro check

# Browser verification (after starting servers with npm run dev:all)
# Open http://localhost:4321/?tab=practice
# Verify 4 cards are visible: "Article of Day", "Typing Race", "Shadowing", "Vocabulary Mining"
# Click "Vocabulary Mining" card
# Verify it navigates to /practice/vocabulary/ and shows the coming soon message
```

---

## Branch & PR

- **Branch**: `fix/issue-006-vocabulary-card`
- **PR title**: `fix(practice): add Vocabulary Mining card to home and stub page (ISSUE-006)`

---

## Out of Scope

- Building the actual vocabulary mining feature
- Changes to other practice pages
- Fixing the 4-card grid layout (tracked separately as ISSUE-019)
- Adding vocabulary mining API endpoints
