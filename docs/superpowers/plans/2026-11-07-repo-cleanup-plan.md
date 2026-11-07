# Repo Cleanup Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remove the unused design-system package, historical documentation, audit artifacts, prototypes, and three dead components from the La Coquille repository, leaving a coherent tree that still builds.

**Architecture:** This is a deletion-only change plus one documentation repair. Every deletion is justified by a zero-reference grep against live code; the Astro build and `astro check` are the safety net that proves nothing live was removed. No production behaviour changes.

**Tech Stack:** Astro 6, SolidJS, custom CSS, Git.

**Spec:** `docs/superpowers/specs/2026-11-07-repo-cleanup-design.md`

## Global Constraints

- `npm run build` (Astro build + `scripts/check-links.mjs`) must pass after every task.
- `npx astro check` must introduce **no new** errors. Baseline at `fecd9b0` is 2 pre-existing
  errors — `src/components/TypingRace.tsx:97` (`Set<unknown>` inference) and
  `src/pages/index.astro:6` (`hideNav` prop absent from `Base.astro`'s `Props`). Both are
  outside this change's scope and are documented in `README.md`.
- Nothing imported by live code may be deleted.
- No change to the site's appearance, behaviour, routes, content, or content collections.
- Do not touch `dist/`, `.astro/`, `.opencode/`, `.claude/`, `.vscode/`, `lacq/`, `public/`, or `scripts/`.
- Do not touch `AGENTS.md` — the coding harness reads it and it references no deleted path.
- Do not push to GitHub. Commit locally only; the human partner decides on pushing.

## Review Focus

- **A deleted file is still imported by a live page.** Expected behaviour: `astro build` or `astro check` fails with a module-not-found error. Pin: build + type-check step after every task.
- **A deleted path is still referenced by a source comment or a Markdown file.** Expected behaviour: the text survives as a dangling reference. Pin: the grep step in Task 5.
- **`git rm` leaves untracked files behind in a deleted directory.** Expected behaviour: the directory still exists on disk with untracked contents (for example `design/uploads/*.png`). Pin: the explicit `rm -rf` in Task 1 and the `git status --ignored` check in Task 5.
- **`dogfood-output/` remains listed in `.gitignore` after its tracked files are removed.** Expected behaviour: future audit runs are untracked by design. Pin: leave the `.gitignore` rule alone — confirmed in Task 5.
- **README still describes a stack the project does not use.** Expected behaviour: a contributor installs Pico.css or deploys to Cloudflare Pages. Pin: the grep for `Pico` / `Cloudflare` in Task 4.

---

### Task 1: Remove the design-system package and the HTML prototypes

**Files:**
- Delete: `design/` (30 tracked files + untracked `design/uploads/pasted-1776887844967-0.png`)
- Delete: `prototypes/` (13 tracked files)

**Interfaces:**
- Consumes: nothing.
- Produces: a repository with no `design/` or `prototypes/` directory; `public/favicon.svg` remains the only favicon.

- [ ] **Step 1: Confirm the packages are unreferenced before deleting**

Run:
```bash
grep -rn "design/\|prototypes/" src/ scripts/ public/ astro.config.mjs package.json
```
Expected: exactly one hit — a historical comment in `src/components/drills/data.ts` (repaired in Task 3). No import, no path, no config reference.

- [ ] **Step 2: Delete the tracked files and the untracked leftovers**

Run:
```bash
git rm -r --quiet design prototypes
rm -rf design
```
`git rm` removes tracked files only; the `rm -rf` clears `design/uploads/` and the now-empty directory.

- [ ] **Step 3: Verify the build still passes**

Run: `npm run build`
Expected: Astro build succeeds and prints `✓ All internal links are valid.`

- [ ] **Step 4: Commit**

`git rm` already staged the deletions, so commit them directly:
```bash
git commit -m "chore: remove unused design system package and prototypes"
```

---

### Task 2: Remove historical docs and audit artifacts

**Files:**
- Delete: `docs/SPEC-article-of-the-day.md`
- Delete: `docs/SPEC-article-of-the-day-v2.md`
- Delete: `docs/specs/2026-04-21-article-of-the-day-design.md`
- Delete: `docs/superpowers/plans/2026-04-23-article-of-the-day-plan.md`
- Delete: `docs/superpowers/specs/2026-04-23-article-of-the-day-design.md`
- Delete: `dogfood-output/` (29 tracked files)
- Delete: `outputs/` (18 tracked files)
- Delete: `papers/` (3 tracked files)
- Keep: `docs/superpowers/specs/2026-11-07-repo-cleanup-design.md` and `docs/superpowers/plans/2026-11-07-repo-cleanup-plan.md`

**Interfaces:**
- Consumes: nothing.
- Produces: a `docs/` tree holding only the active cleanup spec and plan.

- [ ] **Step 1: Delete the historical docs**

Run:
```bash
git rm --quiet \
  docs/SPEC-article-of-the-day.md \
  docs/SPEC-article-of-the-day-v2.md \
  docs/specs/2026-04-21-article-of-the-day-design.md \
  docs/superpowers/plans/2026-04-23-article-of-the-day-plan.md \
  docs/superpowers/specs/2026-04-23-article-of-the-day-design.md
```
Expected: `docs/specs/` disappears from the working tree; `docs/superpowers/` survives.

- [ ] **Step 2: Delete the audit and research artifact directories**

Run:
```bash
git rm -r --quiet dogfood-output outputs papers
```
Expected: all three directories are gone. Do **not** edit `.gitignore` — the `dogfood-output/` rule stays so future audit runs remain untracked.

- [ ] **Step 3: Verify the build still passes**

Run: `npm run build`
Expected: `✓ All internal links are valid.`

- [ ] **Step 4: Stage the new cleanup spec and plan, then commit**

`git rm` already staged the deletions. The cleanup spec and plan are new untracked files
under `docs/`, so stage them explicitly:
```bash
git add docs/superpowers/specs/2026-11-07-repo-cleanup-design.md docs/superpowers/plans/2026-11-07-repo-cleanup-plan.md
git commit -m "chore: remove historical docs and audit artifacts"
```

---

### Task 3: Remove dead components and loose root files

**Files:**
- Delete: `src/components/PolygonHero.tsx`
- Delete: `src/components/Term.tsx`
- Delete: `src/components/GrammarMap/AspectIcon.tsx`
- Modify: `src/components/drills/data.ts` (lines 6-8 — drop the reference to the deleted prototypes directory)
- Delete: `plan.md`, `progress.md`, `research.md`, `REVIEW_simplity_v2.md`, `practice-snapshot.md`, `false`
- Delete (untracked, disk only): `grammar-grid.html`, root-level `*.png` screenshots

**Interfaces:**
- Consumes: nothing.
- Produces: `src/components/GrammarMap/` contains only `types.ts`, which `src/components/VerbDiagram.tsx` imports.

- [ ] **Step 1: Re-confirm each component has zero references**

Run:
```bash
grep -rn --include="*.astro" --include="*.tsx" --include="*.ts" "PolygonHero\|\bTerm\b\|AspectIcon" src/
```
Expected: only the three files' own definition lines. No import of any of them.

- [ ] **Step 2: Delete the three dead components**

Run:
```bash
git rm --quiet \
  src/components/PolygonHero.tsx \
  src/components/Term.tsx \
  src/components/GrammarMap/AspectIcon.tsx
```

- [ ] **Step 3: Repair the dangling prototypes reference in the drill data comment**

Replace this comment header in `src/components/drills/data.ts`:
```ts
/**
 * Curated exercise data for the four drill types. Static for now — no
 * scheduler, no progress. Each array is hand-tuned to mix difficulty and
 * cover a useful spread of verbs and tenses.
 *
 * NOTE: the prototypes directory (`/prototypes/`) had the same data inline
 * in each HTML file. This file is the single source of truth for the
 * production drills.
 */
```
with:
```ts
/**
 * Curated exercise data for the four drill types. Static for now — no
 * scheduler, no progress. Each array is hand-tuned to mix difficulty and
 * cover a useful spread of verbs and tenses.
 *
 * This file is the single source of truth for the production drills.
 */
```

- [ ] **Step 4: Delete the loose root files and untracked leftovers**

Run:
```bash
git rm --quiet plan.md progress.md research.md REVIEW_simplity_v2.md practice-snapshot.md false
rm -f grammar-grid.html
find . -maxdepth 1 -name '*.png' -delete
```
The `find` targets only repository-root screenshots (all already covered by the `*.png` rule in `.gitignore`); `public/` has no PNG assets.

- [ ] **Step 5: Verify the type-check and build still pass**

Run: `npx astro check && npm run build`
Expected: no type errors, and `✓ All internal links are valid.`

- [ ] **Step 6: Commit**

```bash
git add src/components/drills/data.ts
git commit -m "chore: remove dead components and loose root files"
```

---

### Task 4: Update README.md to match the repository as it now stands

**Files:**
- Modify: `README.md`

**Interfaces:**
- Consumes: the repository state produced by Tasks 1-3.
- Produces: a README whose stack, layout, and links describe the actual project.

- [ ] **Step 1: Update the project layout block**

In `README.md`, remove these two lines from the layout tree:
```
design/
  grammar-map-kit.md    # Living spec for the shared diagram design language
plan.md                 # Project plan (mirrors ~/.claude/plans/…)
```
and replace the `styles/` entry with:
```
  styles/
    global.css          # Design tokens + base styles (the live palette)
    drills.css          # Practice drill styles
```

- [ ] **Step 2: Correct the stack claims**

- Intro line: replace `Built with Astro, SolidJS islands, and Pico.css.` with `Built with Astro, SolidJS islands, and a hand-rolled CSS design system.`
- Tech stack table: replace the `CSS` row's `Pico.css` entry with `Hand-authored CSS with custom-property tokens (`src/styles/global.css`)`; replace the `Framework` row's `Astro 5` with `Astro 6`; replace the `Deploy` row's `Cloudflare Pages (CI via GitHub Actions)` with `Fly.io (single Machine serving the Astro build and the Rust backend)`.

- [ ] **Step 3: Correct the `Base.astro` description in the layout tree**

Replace `Base.astro          # Shared layout: nav, breadcrumb, Pico.css import` with `Base.astro          # Shared layout: nav, breadcrumb, global.css import`.

- [ ] **Step 4: Verify no stale references remain**

Run:
```bash
grep -n "Pico\|Cloudflare\|Astro 5\|plan\.md\|grammar-map-kit" README.md
```
Expected: no output.

- [ ] **Step 5: Commit**

```bash
git add README.md
git commit -m "docs: update README to match the current stack and layout"
```

---

### Task 5: Final verification

**Files:**
- Verify only — no file changes expected.

**Interfaces:**
- Consumes: every task above.
- Produces: evidence that the cleanup broke nothing.

- [ ] **Step 1: Confirm no dangling references to deleted paths remain**

Run:
```bash
grep -rn "design/\|papers/\|outputs/\|dogfood-output/\|prototypes/\|plan\.md\|progress\.md\|research\.md\|REVIEW_simplity_v2\|practice-snapshot" \
  src/ scripts/ public/ astro.config.mjs package.json README.md AGENTS.md .github/
```
Expected: no output.

- [ ] **Step 2: Confirm the build and type-check pass from a clean state**

Run:
```bash
rm -rf dist .astro && npm run build && npx astro check
```
Expected: build succeeds with `✓ All internal links are valid.` and `astro check` reports zero errors.

Note: `lacq/dist` is a symlink to the absolute path `<repo>/dist`. Removing `dist/` leaves that
symlink dangling until `astro build` recreates the directory a moment later, at which point
the symlink resolves again. No action needed.

- [ ] **Step 3: Confirm the working tree contains only the intended changes**

Run:
```bash
git status --short
```
Expected: no output — the tree is clean, because every task committed its own work and
Task 2 staged the cleanup spec and plan. No deleted directory reappears.

- [ ] **Step 4: Confirm the ignored-directory rules were left intact**

Run:
```bash
git diff HEAD~4 -- .gitignore
```
Expected: no change to `.gitignore` — in particular the `dogfood-output/` and `*.png` rules remain.

- [ ] **Step 5: Report the summary to the human partner**

Report: number of files deleted by category, the three dead components removed, the README corrections, and the build / type-check output. State explicitly that nothing was pushed to GitHub.
