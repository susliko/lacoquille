# Review: ISSUE-002 — Shadowing Practice "Coming Soon"

**Verdict: PASS**

## Branch
`fix/issue-002-shadowing-coming-soon` — commit `38b04db`

---

## Acceptance Criteria Checklist

| # | Criterion | Status | Evidence |
|---|-----------|--------|----------|
| 1 | `npx astro check` passes with no NEW warnings/errors | **PASS** | All 21 errors in `npx astro check` are pre-existing on `main` (verified by running the same check on `main` and comparing output — identical errors/warnings). The fix introduces zero new diagnostics. |
| 2 | `npm run build` completes without errors | **PASS** | Build succeeded: "82 page(s) built in 1.98s", link checker passed "✓ All internal links are valid." Pre-existing route-conflict warnings for grammar pages are unchanged. |
| 3 | `/practice/shadowing/` renders immediately with a "coming soon" card. No "Loading stories..." spinner ever appears. | **PASS** | Agent-browser evaluated `document.body.innerText.includes('Loading stories')` → `false`. Server log confirms only `[200]` responses for `/practice/shadowing/` after the Vite hot-reload picked up the fix; zero proxy errors to `/api/stories`. |
| 4 | There is a visible "← Back to practice hub" link that navigates to `/practice/` | **PASS** | Agent-browser evaluated `Array.from(document.querySelectorAll('a')).map(a => a.href)` → `["http://localhost:4322/practice/typing", "http://localhost:4322/practice/"]`. Clicking `.sc-back` navigates to `http://localhost:4322/practice/`. |
| 5 | The page does NOT make any `fetch('/api/...')` calls in the coming-soon state | **PASS** | The guard `if (!SHADOWING_AVAILABLE) return <ComingSoon />;` (line 204) is the **first statement** in `ShadowingPractice`. `createResource(fetchStories)` (line 64) is never reached. Vite server log shows no `/api/stories` proxy errors after the Vite watch detected the file change at 19:54:38. |

---

## Code Review

### Diff scope
- **Single file changed:** `src/components/ShadowingPractice.tsx`
- **Lines added:** 141 | **Lines deleted:** 0
- **Changes:** Feature flag constant, `ComingSoon` component definition, guard in exported function body

### Feature flag (line 6)
```typescript
const SHADOWING_AVAILABLE = false;
```
Placed after imports, before any interfaces or functions. Clear inline comment explains the purpose and how to re-activate the feature. ✓

### Guard (line 204)
```typescript
export default function ShadowingPractice() {
  if (!SHADOWING_AVAILABLE) return <ComingSoon />;
```
The guard is the **very first statement** in the function body, before `const [stories] = createResource(fetchStories);`. This ensures `createResource` is never called and no `fetch` is made. ✓

### `ComingSoon` component (lines 69–202)
- Defined before the exported function (alongside `splitSentences`, `getDistractors`, `charAccuracy`)
- Self-contained with inline `<style>` block using only existing CSS variables
- Eyebrow badge with "🔊 Coming Soon"
- H1: "Shadowing Practice"
- Body copy explains the feature and why it's not ready
- Hint box with "📖" pointing to `/practice/typing`
- Back link: `<a href="/practice/" class="sc-back">← Back to practice hub</a>`
- Responsive styles for mobile (`max-width: 480px`)
- ✓ All CSS variables (`--surface`, `--border-subtle`, `--radius`, `--text`, `--text-2`, `--text-muted`, `--coral`, `--emerald`, `--indigo`, `--surface-2`, `--border`, `--transition`) are existing project variables

### Dead code risk (acknowledged in plan)
The ~880-line body of `ShadowingPractice` is now dead code until `SHADOWING_AVAILABLE = true`. The comment on line 5 documents this clearly. The plan's risk mitigation is appropriate.

---

## Issues Found

**None.** No problems requiring action.

### Minor observations (non-blocking)
1. **Extra blank line at line 8:** The plan's template shows two blank lines after the feature flag; the implementation has two blank lines too (`const SHADOWING_AVAILABLE = false;\n\n\n`). The extra blank line is cosmetic and doesn't affect runtime behavior.
2. **Pre-existing errors in `npx astro check`:** 21 type errors (e.g., `hideNav`/`section`/`breadcrumb` props not on `Base` props, `Set<unknown>` in `TypingRace.tsx`) exist on `main` and are unrelated to this fix. The fix introduces no new errors.

---

## Verification Summary

```
npx astro check   → 21 pre-existing errors, 0 new
npm run build     → success, 82 pages, links valid
/practice/shadowing/ page:
  body text       → "🔊 COMING SOON / Shadowing Practice / ..."
  "Loading stories" in DOM → false
  links           → /practice/typing, /practice/ ✓
  back link click → navigates to /practice/ ✓
  fetch calls     → none (Vite proxy log: 0 /api/stories errors after fix)
```

---

## Merge Recommendation

**APPROVED for merge.**

The fix is minimal (single file, 141 lines), targeted, and correct. It solves the reported issue completely: the shadowing page now shows a friendly "coming soon" card immediately on load, with no network requests, and provides a clear path back to the practice hub and an alternative in Typing Race. The guard is placed correctly before any `createResource` call, ensuring the backend is never contacted in the disabled state. The feature flag is explicit and reversible.