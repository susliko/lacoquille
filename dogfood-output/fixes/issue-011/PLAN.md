# Implementation Plan

## Goal

Add the red "Grammaire" eyebrow (`<p class="home-eyebrow">Grammaire</p>`) to all drill-down pages under `/reference/verbs/` so users navigating from the verbs index to a specific tense or choice guide still see the section breadcrumb.

## Tasks

1. **Add eyebrow to `src/pages/reference/verbs/tenses/[slug].astro`**
   - File: `src/pages/reference/verbs/tenses/[slug].astro`
   - Changes: Insert a wrapper `<div>` with the eyebrow above the existing `<div class="hgroup">`
   - Acceptance: Build passes, eyebrow visible above H1 on any tense page

2. **Add eyebrow to `src/pages/reference/verbs/choice/[slug].astro`**
   - File: `src/pages/reference/verbs/choice/[slug].astro`
   - Changes: Insert a wrapper `<div>` with the eyebrow above the existing `<div class="hgroup">`
   - Acceptance: Build passes, eyebrow visible above H1 on any choice page

## Files to Modify

### `src/pages/reference/verbs/tenses/[slug].astro`

**Current state** (lines 16–20):

```astro
<Base
  title={title}
  description={oneLineRule}
  breadcrumb={[
    { label: "Verbes", href: "/reference/verbs" },
    { label: title },
  ]}
>
  <article>
    <div class="hgroup">
      <h1>{title}</h1>
      <p class="one-line-rule">{oneLineRule}</p>
    </div>
```

**Modified state** (insert wrapper div before `<div class="hgroup">`):

```astro
<Base
  title={title}
  description={oneLineRule}
  breadcrumb={[
    { label: "Verbes", href: "/reference/verbs" },
    { label: title },
  ]}
>
  <article>
    <div style="margin-bottom: 1.5rem;">
      <a href="/reference/verbs"><p class="home-eyebrow">Grammaire</p></a>
      <div class="hgroup">
        <h1>{title}</h1>
        <p class="one-line-rule">{oneLineRule}</p>
      </div>
    </div>
```

### `src/pages/reference/verbs/choice/[slug].astro`

**Current state** (lines 17–23):

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

**Modified state** (insert wrapper div before `<div class="hgroup">`):

```astro
<Base
  title={title}
  breadcrumb={[
    { label: "Verbes", href: "/reference/verbs" },
    { label: title },
  ]}
>
  <article>
    <div style="margin-bottom: 1.5rem;">
      <a href="/reference/verbs"><p class="home-eyebrow">Grammaire</p></a>
      <div class="hgroup">
        <h1>{title}</h1>
      </div>
    </div>
```

## New Files

None.

## Dependencies

- Task 1 and Task 2 are independent — they modify different files.
- Both depend on the existing `.home-eyebrow` CSS class in `src/styles/global.css` (line ~368: coral red, uppercase, small).

## Risks

| Risk | Mitigation |
|------|------------|
| The eyebrow style might shift the H1 layout | Keep the eyebrow outside the `hgroup` (in a wrapper `<div>` with `margin-bottom`), matching the pattern from the verbs index page |
| The `<a>` wrapping the `<p>` is semantically unusual | This is acceptable for a breadcrumb-style link; the visual effect is a clickable label. Alternatively, put the `<p>` inside the `<a>` as `<a class="home-eyebrow" href="...">Grammaire</a>` and let CSS handle the styling |

## Verification

```sh
# 1. Build
cd /home/susliko/programming/lacoquille && npm run build

# 2. Type-check
npx astro check
```

Then in agent-browser:
- Open `/reference/verbs/` — note the red "Grammaire" eyebrow above "Verbes"
- Click any tense node (e.g., "Présent") — verify the eyebrow is visible above the H1
- Navigate back to `/reference/verbs/`
- Click any "Tense-choice guide" (e.g., "Imparfait vs Passé composé") — verify the eyebrow is visible above the H1
- Click the eyebrow — should navigate to `/reference/verbs/`

## Out of Scope

- Changes to `src/pages/reference/verbs/index.astro` (already has the eyebrow)
- Changes to `src/layouts/Base.astro` (the `breadcrumb` prop is silently ignored; this is a separate issue)
- Changes to non-verbs pages (this fix is verbs-section-specific)

## Branch & PR

- Branch: `fix/issue-011-verbs-breadcrumb`
- PR title: `fix(verbs): add Grammaire breadcrumb to tense and choice pages (ISSUE-011)`
