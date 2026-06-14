# Implementation Plan

## Goal

Replace the broken "Loading stories..." spinner on `/practice/shadowing/` with a clean, self-contained "coming soon" card that renders immediately — no network requests, no spinner, no broken state — and links the user back to `/practice/`.

---

## Tasks

### 1. Add the `SHADOWING_AVAILABLE` feature flag to `ShadowingPractice.tsx`

- **File:** `src/components/ShadowingPractice.tsx`
- **Changes:** Add a single constant declaration near the top of the file, after the imports block (before any interface or function declarations):
  ```typescript
  // ── Feature flag ──────────────────────────────────────────────
  // Set to `true` once the /api/stories + /api/tts backend is restored.
  // When false, the component renders a friendly "coming soon" state instead.
  const SHADOWING_AVAILABLE = false;
  ```
- **Acceptance:** The constant is defined before any `createResource` or `fetch` call. Changing it to `true` in the future will activate the full component without any other edits.

### 2. Guard the exported component body with the feature flag

- **File:** `src/components/ShadowingPractice.tsx`
- **Changes:** In the `ShadowingPractice` function body, as the very first statement (before `const [stories] = createResource(fetchStories);` on line 64), add:
  ```typescript
  if (!SHADOWING_AVAILABLE) return <ComingSoon />;
  ```
- **Acceptance:** When `SHADOWING_AVAILABLE` is `false`, the `createResource` is never called and no `fetch` is made to any `/api/...` endpoint. Verified in DevTools network tab.

### 3. Define the `ComingSoon` component inline in `ShadowingPractice.tsx`

- **File:** `src/components/ShadowingPractice.tsx`
- **Changes:** Add the following component definition **before** the `export default function ShadowingPractice()` declaration (i.e., alongside the other module-level helpers like `splitSentences`, `getDistractors`, `charAccuracy`):

  ```tsx
  function ComingSoon() {
    return (
      <div class="shadowing-coming-soon">
        <style>{`
          .shadowing-coming-soon {
            max-width: 640px;
            margin: 0 auto;
            padding: 3rem 2rem;
            display: flex;
            flex-direction: column;
            align-items: center;
            min-height: 60vh;
            justify-content: center;
          }

          .sc-card {
            background: var(--surface);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius);
            padding: 2.5rem 3rem;
            text-align: center;
            max-width: 480px;
            width: 100%;
          }

          .sc-eyebrow {
            display: inline-flex;
            align-items: center;
            gap: 0.4rem;
            font-size: 0.7rem;
            font-weight: 500;
            letter-spacing: 0.12em;
            text-transform: uppercase;
            color: var(--coral);
            margin-bottom: 1rem;
          }

          .sc-title {
            font-family: var(--font-display);
            font-size: 1.75rem;
            font-weight: 700;
            color: var(--text);
            margin: 0 0 0.75rem 0;
            line-height: 1.2;
          }

          .sc-body {
            font-size: 0.95rem;
            color: var(--text-2);
            line-height: 1.65;
            margin: 0 0 1.5rem 0;
          }

          .sc-hint {
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 0.5rem;
            font-size: 0.8rem;
            color: var(--text-muted);
            background: var(--surface-2);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-sm);
            padding: 0.6rem 1rem;
            margin-bottom: 1.75rem;
          }

          .sc-hint-icon {
            color: var(--emerald);
            font-size: 1rem;
          }

          .sc-back {
            display: inline-flex;
            align-items: center;
            gap: 0.4rem;
            font-size: 0.875rem;
            color: var(--text-2);
            border: 1px solid var(--border);
            border-radius: var(--radius-sm);
            padding: 0.5rem 1rem;
            transition: border-color var(--transition), color var(--transition);
          }

          .sc-back:hover {
            border-color: var(--coral);
            color: var(--coral);
          }

          @media (max-width: 480px) {
            .sc-card {
              padding: 2rem 1.5rem;
            }
            .sc-title {
              font-size: 1.4rem;
            }
          }
        `}</style>

        <div class="sc-card">
          <div class="sc-eyebrow">
            <span>🔊</span>
            <span>Coming Soon</span>
          </div>

          <h1 class="sc-title">Shadowing Practice</h1>

          <p class="sc-body">
            Listen to a French sentence, then type it out or choose the right
            answer — a great way to train your ear and your accuracy at the
            same time. We're still building the story library and audio
            infrastructure; this page will be ready soon.
          </p>

          <div class="sc-hint">
            <span class="sc-hint-icon">📖</span>
            <span>
              In the meantime, the{" "}
              <a href="/practice/typing" style={{ color: "var(--indigo)", "font-weight": "500" }}>
                Typing Race
              </a>{" "}
              practice is available — same stories, different format.
            </span>
          </div>

          <a href="/practice/" class="sc-back">
            ← Back to practice hub
          </a>
        </div>
      </div>
    );
  }
  ```

- **Acceptance:** The card renders centered on the page, uses only existing CSS variables (`--surface`, `--border-subtle`, `--radius`, `--text`, `--text-2`, `--text-muted`, `--coral`, `--emerald`, `--indigo`, `--surface-2`, `--border`, `--transition`), has a visible "← Back to practice hub" link, and mentions Typing Race as an alternative.

### 4. Verify build and type checks

- **Commands:**
  ```sh
  cd /home/susliko/programming/lacoquille && npx astro check
  cd /home/susliko/programming/lacoquille && npm run build
  ```
- **Acceptance:** Both commands succeed with no new warnings or errors.

---

## Files to Modify

- `src/components/ShadowingPractice.tsx` — three changes: feature flag constant, guard in component body, and the `ComingSoon` component definition (added before the exported function)

---

## New Files

None.

---

## Dependencies

Task 3 (adding `ComingSoon`) must be completed before Task 2 (wiring the guard). Task 4 is independent and runs last.

---

## Risks & Mitigations

| Risk | Mitigation |
|---|---|
| The 880-line `ShadowingPractice.tsx` body is now dead code until the flag is flipped. | The `SHADOWING_AVAILABLE = false` comment makes the situation explicit. A future developer flipping the flag can use git blame to find the recon and the original code. |
| A user bookmarks `/practice/shadowing/` and finds a static page instead of the feature. | The "coming soon" card is a meaningful, helpful state — not a 404. It explains what the feature is and offers an alternative. |
| The `SHADOWING_AVAILABLE` flag is accidentally left as `true` and the page breaks again. | The acceptance criteria require verifying no `fetch` calls are made in the coming-soon state. A reviewer checking DevTools will catch this. |
| Astro's incremental build caches the old component output. | Running `npm run build` before deployment clears the cache. The `Base.astro` shell has no conditional logic so SSR caching is not a concern here (the component is `client:only`). |

---

## Out of Scope

- Any backend work (Rust routes, TTS, stories endpoints)
- Changes to `PracticeHub.tsx` or `HomepageTabs.tsx` (the shadowing tile stays linked)
- Changes to `shadowing.astro` (the page wrapper is fine as-is)
- Changes to other practice pages
- Adding the feature flag to environment variables or a config file — a simple `const` is sufficient for the current scope

---

## Branch and PR

- **Branch:** `fix/issue-002-shadowing-coming-soon`
- **PR title:** `fix(practice): show coming-soon state on /practice/shadowing/ (ISSUE-002)`
