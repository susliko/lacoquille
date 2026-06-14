# Implementation Plan: ISSUE-009 + ISSUE-017 Typing Race UX

## Goal

Fix two UX issues in the Typing Race component: (a) auto-focus reliably so the first keypress is registered without a click, and (b) use all paragraphs of the article instead of just the first.

## Tasks

### 1. Change the `text` memo to use all paragraphs

- **File**: `src/components/TypingRace.tsx`
- **Line 14** — replace:
  ```ts
  const text = createMemo(() => (article()?.paragraphs?.[0] ?? "").split(""));
  ```
  with:
  ```ts
  const text = createMemo(() => (article()?.paragraphs?.join("\n\n") ?? "").split(""));
  ```
- **Acceptance**: The text area shows all paragraphs joined by `"\n\n"`. For a 2-paragraph story, the text area contains both paragraphs separated by a blank line. `white-space: pre-wrap` on `.tr-text-area` preserves the newlines visually.

### 2. Improve the auto-focus effect with a fallback and a text-ready guard

- **File**: `src/components/TypingRace.tsx`
- **Lines 84–88** — replace:
  ```ts
  createEffect(() => {
    if (article() && containerRef) {
      containerRef.focus();
    }
  });
  ```
  with:
  ```ts
  createEffect(() => {
    if (article() && containerRef && text().length > 0) {
      containerRef.focus();
      // Fallback: if browser blocked the focus, try again after a tick
      setTimeout(() => containerRef?.focus(), 100);
    }
  });
  ```
- **Acceptance**: On page load, after the article fetch resolves and `text()` is non-empty, `focus()` is called immediately and again 100ms later as a belt-and-suspenders fallback.

### 3. Add a `:focus-visible` focus ring to `.tr-text-area`

- **File**: `src/components/TypingRace.tsx`
- **Inline `<style>` block** — add inside the `.tr-text-area` CSS block (after `user-select: none;`):
  ```css
  .tr-text-area:focus-visible {
    outline: 2px solid var(--coral, #ff4757);
    outline-offset: 2px;
  }
  ```
- **Do NOT remove the existing `outline: none`** from `.tr-text-area`. The browser's default focus ring is suppressed by `outline: none`; the `:focus-visible` rule adds it back only for keyboard-navigated focus.
- **Acceptance**: When the text area is focused (either by user Tab or by the `createEffect`), a visible coral-colored ring appears around the text area. No ring appears if only mouse-clicked.

### 4. Update the hint text

- **File**: `src/components/TypingRace.tsx`
- **Line ~288** — replace:
  ```tsx
  <p class="tr-tap-hint">click the text · start typing</p>
  ```
  with:
  ```tsx
  <p class="tr-tap-hint">start typing</p>
  ```
- **Acceptance**: The hint no longer tells the user to click.

## Files to Modify

- `src/components/TypingRace.tsx` — four targeted changes (text memo, createEffect, CSS, hint text)

## New Files

None.

## Dependencies

Tasks 1–4 are independent — they touch different lines in the same file and can be applied in any order. All four should be committed together on branch `fix/issue-009-typing-race-ux`.

## Risks

1. **Longer text may lower WPM numbers** — Users type more characters. The WPM formula `(chars / 5) / (secs / 60)` normalizes this, but if the user historically completed races in ~30 seconds, adding a second paragraph may push completion to 60 seconds, halving their WPM readout. This is correct behavior, not a bug. Document this in the PR description.
2. **`:focus-visible` browser heuristics** — Some browsers may not show the ring for programmatic `focus()` calls. If the ring doesn't appear on load, the user still benefits from the `setTimeout` fallback focus (cursor is in the text area, keys are registered). Acceptable.
3. **`outline: none` + `:focus-visible` interaction** — In some browsers, `outline: none` suppresses `:focus-visible` styling. If the ring doesn't appear, switch the rule to use `box-shadow` instead: `box-shadow: 0 0 0 2px var(--coral, #ff4757);`.
4. **Build step** — No changes to `typing.astro` or any other file. `npm run build` must pass; run `npx astro check` to confirm no new type errors.

## Acceptance Criteria (Verification Commands)

```sh
# 1. Build passes
cd /home/susliko/programming/lacoquille && npm run build

# 2. Type-check passes
cd /home/susliko/programming/lacoquille && npx astro check

# 3. Browser verification (after starting dev servers with `npm run dev:all`):
#    - Open /practice/typing/, wait for article to load
agent-browser eval "({ hasTextArea: !!document.querySelector('.tr-text-area'), hasFocus: document.activeElement?.className === 'tr-text-area', charCount: document.querySelectorAll('.tr-text-area span').length, hintText: document.querySelector('.tr-tap-hint')?.textContent })"
#    Expected: hasTextArea=true, hasFocus=true, charCount>0, hintText="start typing"

# 4. Without clicking, type a character — should be registered immediately
agent-browser press z

# 5. Verify multiple paragraphs (for multi-paragraph stories):
agent-browser eval "document.querySelector('.tr-text-area').textContent.split('\n').filter(l => l.trim()).length"
#    Expected: > 1 for stories with 2+ paragraphs
```

## Branch and PR

- **Branch**: `fix/issue-009-typing-race-ux`
- **PR title**: `fix(typing): auto-focus + use all paragraphs (ISSUE-009, ISSUE-017)`