

**Status Report — ISSUE-009 + ISSUE-017**

| Field | Value |
|-------|-------|
| **Branch** | `fix/issue-009-typing-race-ux` |
| **Commit SHA** | `18e2d2d` |
| **Build** | ✅ `npm run build` passes (82 pages, all internal links valid) |
| **Type check** | ✅ No new errors — only pre-existing errors in `Base.astro` prop mismatches and unused variables in `ShadowingPractice.tsx` |
| **Browser verification** | ⚠️ Skipped (subagent cannot run `agent-browser`); code review confirms all 4 changes are correct |
| **Screenshot** | N/A (skipped per above) |
| **Deviation from plan** | None |

**Changes made to `src/components/TypingRace.tsx`:**
1. **`text` memo** — `paragraphs?.[0]` → `paragraphs?.join("\n\n")` — all paragraphs rendered with blank-line separation
2. **`createEffect`** — added `text().length > 0` guard + `setTimeout(() => containerRef?.focus(), 100)` fallback
3. **CSS** — added `.tr-text-area:focus-visible { outline: 2px solid var(--coral, #ff4757); outline-offset: 2px; }`
4. **Hint** — `"click the text · start typing"` → `"start typing"`

**Files changed:** `src/components/TypingRace.tsx` (+9 lines, -3 lines)