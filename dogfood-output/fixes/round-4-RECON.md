# Round 4 Recon: Typing Race Issues

## Files Retrieved

1. `src/components/TypingRace.tsx` (lines 1–225) — the full component
2. `src/pages/practice/typing.astro` (lines 1–5) — the page wrapper

---

## Key Code

### `src/components/TypingRace.tsx` — critical sections

**Resource + text memo (lines 8–18):**
```ts
const [article] = createResource(async () => {
  const res = await fetch("/api/article-of-the-day");
  if (!res.ok) throw new Error(`HTTP ${res.status}`);
  return res.json() as Promise<ArticleData>;
});

// ISSUE-017: hardcoded [0] — only the first paragraph is used
const text = createMemo(() => (article()?.paragraphs?.[0] ?? "").split(""));
```

**containerRef + auto-focus effect (lines 81–88):**
```ts
let containerRef: HTMLDivElement | undefined;

createEffect(() => {
  if (article() && containerRef) {
    containerRef.focus();
  }
});
```

**handleKeyDown (lines 44–72):**
```ts
const handleKeyDown = (e: KeyboardEvent) => {
  if (e.ctrlKey || e.metaKey || e.altKey) return;
  if (finished()) {
    if (e.key === "Enter") resetRace();
    return;
  }
  if (e.key === "Backspace") return;
  if (e.key.length !== 1) return;
  e.preventDefault();
  if (startTime() === null) setStartTime(Date.now());
  const idx = typed();
  const txt = text();
  if (idx >= txt.length) { setFinished(true); return; }
  if (e.key === txt[idx]) {
    setTyped(idx + 1);
  } else {
    setErrorSet((prev) => new Set([...prev, idx]));
    setErrors(errors() + 1);
    setTyped(idx + 1);
  }
  if (idx + 1 >= txt.length) setFinished(true);
};
```

**JSX — text area (lines 190–205):**
```tsx
<div class="tr-text-wrap">
  <div
    class="tr-text-area"
    ref={containerRef}
    tabIndex={0}
    onKeyDown={handleKeyDown}
  >
    {typedText().map((char, i) => (
      <span class={`tc-${errorSet().has(i) ? "error" : "correct"}`}>{char}</span>
    ))}
    <Show when={!finished()}>
      <span class="tr-cursor" />
    </Show>
    {remainingChars().map(({ char }) => (
      <span class="tc-idle">{char}</span>
    ))}
  </div>
</div>

<p class="tr-tap-hint">click the text · start typing</p>
```

**CSS — `.tr-text-area` (lines 119–132):**
```css
.tr-text-area {
  outline: none;           /* <-- no focus ring, user has no visual cue */
  white-space: pre-wrap;
  word-break: break-word;
  font-family: var(--font-body);
  font-size: clamp(1.1rem, 3vw, 1.4rem);
  line-height: 2.1;
  letter-spacing: 0.01em;
  padding: 1.25rem 1.5rem;
  background: var(--surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius);
  position: relative;
  user-select: none;
}
```

### `src/pages/practice/typing.astro`
```astro
---
import Base from "../../layouts/Base.astro";
import TypingRace from "../../components/TypingRace";
---

<Base title="Typing Race — La Coquille" section="practice">
  <TypingRace client:only="solid-js" />
</Base>
```

---

## Architecture

The page is `client:only="solid-js"` — fully client-side, no SSR. The article is fetched via `createResource` from `/api/article-of-the-day`. The response shape is:

```ts
interface ArticleData {
  title: string;
  source: string;
  published_year: number;
  paragraphs: string[];  // Vec<String> from Rust — may contain multiple paragraphs
}
```

The backend (`lacq/src/lib.rs` lines 260–283) already returns **multiple paragraphs** joined with `"\n\n"`, capped at ~1200 characters. The `paragraphs` array can have 2–5+ entries depending on story length and paragraph size. The frontend only uses `paragraphs[0]`, discarding everything else.

---

## ISSUE-009: Auto-Focus Not Working

### What's there

There **is** an auto-focus effect:
```ts
createEffect(() => {
  if (article() && containerRef) {
    containerRef.focus();
  }
});
```

The `containerRef` is set via SolidJS's `ref={containerRef}` directive on the inner `<div class="tr-text-area">`, which lives inside `<Show when={article()}>`.

### Why it fails

**Timing / SolidJS ref-order issue.** In SolidJS, `ref` callbacks run synchronously during the render phase when the element is mounted. The `createEffect` is also reactive and fires whenever `article()` changes. The sequence is:

1. `article()` transitions from `undefined` → truthy (fetch resolves)
2. `<Show when={article()}>` renders its children
3. The inner `<div ref={containerRef}>` mounts — `containerRef` is assigned
4. `createEffect` runs: `article()` is truthy, `containerRef` is set → `focus()` is called

This *should* work in theory. However, there are compounding factors:

1. **`outline: none`** — even if `focus()` succeeds, the element has no visible focus ring. The user has no indication that the element is focused. The only hint is `<p class="tr-tap-hint">click the text · start typing</p>` at the bottom, which actively tells the user to click.

2. **Browser autoplay/focus policy** — modern browsers will not honor programmatic `focus()` on an element unless the page has received a user gesture (click, keypress). If the user navigates directly to `/practice/typing/` (e.g., from a bookmark or fresh tab), the page loads, `createEffect` fires, but the browser may silently block `focus()`.

3. **Race with hydration** — the page is `client:only="solid-js"`. There is a window between when the component mounts and when the fetch resolves. Even if the focus were reliable, the article may not be loaded yet.

### Recommendation

**The fix should be minimal and robust:**

1. Add a visible focus indicator to `.tr-text-area` — a subtle `box-shadow` or `border-color` change when the element is focused. Use `:focus-visible` or a JS-controlled class toggled by the same `createEffect`.

2. Ensure the focus effect also waits for the text to be non-empty (defensive):
   ```ts
   createEffect(() => {
     if (article() && containerRef && text().length > 0) {
       containerRef.focus();
     }
   });
   ```

3. Consider also calling `focus()` on a timer (fallback) after the article loads, as a belt-and-suspenders approach:
   ```ts
   createEffect(() => {
     if (article() && containerRef) {
       containerRef.focus();
       // Fallback: if browser blocked it, try again after a tick
       setTimeout(() => containerRef?.focus(), 100);
     }
   });
   ```

4. The "click the text · start typing" hint should either be removed or changed to "start typing" once focus is confirmed, to avoid actively misleading the user.

---

## ISSUE-017: Only First Paragraph Loaded

### What's there

```ts
const text = createMemo(() => (article()?.paragraphs?.[0] ?? "").split(""));
```

The backend (`lacq/src/lib.rs` lines 260–283) already returns a `paragraphs: Vec<String>` containing multiple paragraphs, capped at ~1200 characters total. The `text` memo discards all paragraphs after index 0.

### Options analysis

| Option | Description | Pros | Cons |
|--------|-------------|------|------|
| **(a) Paragraph selector** | Buttons for "Para 1", "Para 2", … | User choice | UI change, state management |
| **(b) Concatenate all paragraphs** | `paragraphs.join("\n\n")` | Zero UI change, simplest | Text gets longer; WPM/accuracy calculations are unaffected |
| **(c) Cycle paragraphs** | "Next paragraph" button or auto-advance | More engaging | Complex state, UI change |
| **(d) First 2–3 paragraphs** | Cap at N paragraphs | Simple | Still arbitrary cutoff |

### Recommendation: Option (b) — concatenate all paragraphs

The smallest change with maximum benefit:

```ts
const text = createMemo(() => (article()?.paragraphs?.join("\n\n") ?? "").split(""));
```

This uses the same `text()` memo — no other code needs to change. The typing experience becomes meaningfully longer for long stories. WPM and accuracy calculations are unaffected (they're character-based). The blank-line separator `"\n\n"` preserves paragraph structure visually.

**Important:** `resetRace()` calls `setTyped(0)`, `setErrors(0)`, etc. — all state is already reset-friendly. No state changes needed for the paragraph concatenation to work correctly.

---

## Related Concerns

### `--` em-dash problem (ISSUE-009 mention)
The first two characters to type are literal `--` (French dialogue opening, e.g., `—` or em-dash). These are invisible on-screen in the rendered text — the text display shows them as part of the story text (`.tr-text-area` uses `white-space: pre-wrap`), but the hint says "click the text · start typing" with no visual indicator of the `--`. This is a UX issue separate from but related to the focus problem: even if focus worked, the user sees no cue about what to type first.

**Fix:** Consider rendering the first 1–2 characters with a subtle highlight or a distinct cursor color to indicate the starting position, or add a small "first characters: —" hint above the text area.

### WPM / accuracy edge cases
- `wpm` memo: `(chars / 5) / (secs / 60)` — standard formula. Unaffected by paragraph concatenation.
- `accuracy` memo: `((t - e) / t) * 100` — if `text()` changes (e.g., paragraph cycling), `typed()` and `errors()` would be out of sync. With option (b) this doesn't apply (text is static per race). With option (c) or (a), `resetRace()` must also reset `typed`/`errors`/`errorSet` — which it already does.

### `user-select: none` on `.tr-text-area`
The text area has `user-select: none`, which prevents accidental text selection during typing. This is correct and should be preserved.

### `tabIndex={0}` on inner div
The `containerRef` div has `tabIndex={0}`. This is correct — it makes the div focusable via keyboard (Tab key). The `onKeyDown` handler fires on this div. No change needed here.

### `resetRace` blur-then-focus pattern
```ts
const resetRace = () => {
  (document.activeElement as HTMLElement | null)?.blur();
  setTimeout(() => containerRef?.focus(), 50);
};
```
This pattern is correct: blur any active element, then re-focus the container after a tick. Preserving this ensures the restart flow works.

---

## Start Here

Open `src/components/TypingRace.tsx` and make these two targeted changes:

1. **Line 14** — change the `text` memo to concatenate all paragraphs:
   ```ts
   const text = createMemo(() => (article()?.paragraphs?.join("\n\n") ?? "").split(""));
   ```

2. **Lines 83–87** — add a visible focus indicator class and a fallback focus call:
   ```ts
   createEffect(() => {
     if (article() && containerRef && text().length > 0) {
       containerRef.focus();
       setTimeout(() => containerRef?.focus(), 100);
     }
   });
   ```
   And add a `:focus-visible` style to `.tr-text-area` for the focus ring.

The page wrapper `src/pages/practice/typing.astro` needs no changes.
