# Implementation Plan

## Goal
Make active-token highlights clearly visible on desktop and reposition the mobile tooltip below the word instead of above it, eliminating overlap with the heading or previous text.

## Tasks

1. **Bump active-token highlight alpha and outline width**
   - File: `src/components/ArticleOfTheDay.tsx`
   - Changes: In the inline `<style>` block (lines ~155–170), update three CSS rules:
     - `.token.active`: change `rgba(59, 130, 246, 0.2)` → `rgba(59, 130, 246, 0.4)` and `outline: 2px` → `outline: 3px`
     - `.en-token.active`: change `rgba(59, 130, 246, 0.2)` → `rgba(59, 130, 246, 0.4)`
     - `.fr-token.active`: change `rgba(59, 130, 246, 0.2)` → `rgba(59, 130, 246, 0.4)`
   - Acceptance: `npm run build` passes; on desktop, clicking a French or English word produces a clearly visible blue highlight (not subtle)

2. **Flip mobile tooltip to appear below the word**
   - File: `src/components/ArticleOfTheDay.tsx`
   - Changes: In the `@media (max-width: 768px)` block (lines ~169–186), update `.fr-token.active::after`:
     - Replace `bottom: calc(100% + 4px)` with `top: 100%`
     - Add `margin-top: 4px` (to maintain the 4px gap between word and tooltip)
   - Acceptance: On mobile (390px viewport), clicking a word on the first line shows the tooltip BELOW the word, not above it, and it no longer overlaps the FRANÇAIS heading

## Files to Modify

- `src/components/ArticleOfTheDay.tsx` — two CSS changes in the inline `<style>` block

## New Files
None.

## Dependencies
Task 2 does not depend on Task 1. Both are independent CSS changes in the same file. They can be applied in either order.

## Exact Code Changes

### Change 1 — Active-token highlight contrast

**Current CSS (lines ~155–170):**
```css
.token.active {
  background-color: rgba(59, 130, 246, 0.2);
  outline: 2px solid var(--accent, #3b82f6);
  outline-offset: 1px;
}
.en-token.active {
  background-color: rgba(59, 130, 246, 0.2);
}
.fr-token.active {
  background-color: rgba(59, 130, 246, 0.2);
}
```

**Modified CSS:**
```css
.token.active {
  background-color: rgba(59, 130, 246, 0.4);
  outline: 3px solid var(--accent, #3b82f6);
  outline-offset: 1px;
}
.en-token.active {
  background-color: rgba(59, 130, 246, 0.4);
}
.fr-token.active {
  background-color: rgba(59, 130, 246, 0.4);
}
```

### Change 2 — Mobile tooltip position

**Current CSS (`.fr-token.active::after` inside `@media (max-width: 768px)`):**
```css
.fr-token.active::after {
  content: attr(data-trans);
  display: block;
  position: absolute;
  bottom: calc(100% + 4px);
  left: 50%;
  transform: translateX(-50%);
  background: var(--accent, #3b82f6);
  color: white;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 0.8rem;
  z-index: 100;
  pointer-events: none;
  max-width: 200px;
  white-space: normal;
  text-align: center;
  box-shadow: 0 2px 8px rgba(0,0,0,0.2);
}
```

**Modified CSS:**
```css
.fr-token.active::after {
  content: attr(data-trans);
  display: block;
  position: absolute;
  top: 100%;
  left: 50%;
  transform: translateX(-50%);
  margin-top: 4px;
  background: var(--accent, #3b82f6);
  color: white;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 0.8rem;
  z-index: 100;
  pointer-events: none;
  max-width: 200px;
  white-space: normal;
  text-align: center;
  box-shadow: 0 2px 8px rgba(0,0,0,0.2);
}
```

## Verification Commands

```sh
# 1. Build check
cd /home/susliko/programming/lacoquille && npm run build

# 2. Desktop browser test
# - Open http://localhost:4320/stories/article-of-the-day/
# - Click a French word in the Français column
# - Verify: blue highlight is clearly visible (not subtle)
# - Click an English word in the English column
# - Verify: blue highlight is clearly visible

# 3. Mobile browser test (390px)
# - Open http://localhost:4320/stories/article-of-the-day/
# - Click a word on the first line of the Français column
# - Verify: tooltip appears BELOW the word, not above
# - Verify: tooltip does not overlap the FRANÇAIS heading
```

## Risks

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| `rgba(59, 130, 246, 0.4)` blue is too strong on white backgrounds | Low | 0.4 alpha is a standard highlight value; well below 1.0 opacity. Common in code editors and reading apps. |
| Tooltip below the word overflows the bottom of the viewport | Low | Tooltip is dismissible by clicking the word again or another token. `z-index: 100` already present. |
| Tooltip still overlaps the next line of text | Low | The tooltip is only 1 line max (`white-space: normal` + `max-width: 200px`); the 4px gap and small size minimize this. |

## Out of Scope
- Changes to article content or structure
- Changes to non-active token styling
- Changes to desktop tooltip behavior
- Changes to other CSS rules in the component
- Changes to the `VerbDiagram.tsx` or any other file

## Branch and PR
- Branch: `fix/issue-008-article-highlight-and-tooltip`
- PR title: `fix(article): stronger highlight and below-word tooltip (ISSUE-008, ISSUE-015)`
