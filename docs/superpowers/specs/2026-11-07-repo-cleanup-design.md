# Repo Cleanup — Unused Design System, Docs, and Dead Code

Date: 2026-11-07

## Intent

La Coquille has accumulated three layers of artifacts that nothing in the running site
depends on: a standalone `design/` design-system package, historical docs from finished
work, and three components in `src/` that no page imports. This change removes them so
the repository contains only what builds, ships, and is read.

## Why

- `design/` documents a **dark** theme on Cormorant Garamond / DM Sans. The live
  `src/styles/global.css` is a **light** theme on Bricolage Grotesque / Plus Jakarta Sans.
  The package is stale by two design generations, and nothing imports it.
- `design/SKILL.md` is a Claude skill definition that is registered in no skill
  directory, so it never loads. Its `README.md` also indexes `design/grammar-map-kit.md`
  and `preview/type-scale.html`, neither of which exists.
- `docs/`, `papers/`, `outputs/`, and `dogfood-output/` describe work that is already
  shipped or abandoned. `dogfood-output/` is tracked in git despite already being listed
  in `.gitignore`.
- `README.md` describes Pico.css, Cloudflare Pages, and Astro 5; the site is Astro 6,
  custom CSS, deployed on Fly.io, and `design/grammar-map-kit.md` does not exist.

## Goals

- Remove the unused standalone design-system package.
- Remove finished/abandoned documentation and audit artifacts.
- Remove dead components from `src/`.
- Leave the repository internally coherent: no dangling references, `README.md` accurate.

## Non-goals

- No change to the site's appearance, behaviour, routes, content, or content collections.
- No refactoring of live components, CSS, or the Rust backend.
- No touching build output (`dist/`, `.astro/`) or harness directories (`.opencode/`,
  `.claude/`, `.vscode/`).
- No rewriting of `AGENTS.md` — the harness reads it. It contains no reference to any
  path this change deletes.

## Constraints

- `npm run build` (Astro build + `scripts/check-links.mjs`) must pass afterwards.
- `npx astro check` must pass afterwards.
- Nothing imported by live code may be deleted.

## Approaches considered

1. **Delete wholesale, by directory plus an explicit loose-file list, then verify.**
   Chosen. Every deletion is justified by a grep showing zero references from live code,
   and the build is the safety net.
2. **Keep a trimmed `design/` (`README.md` + `colors_and_type.css`) as a living design
   reference.** Rejected: its tokens are already live in `src/styles/global.css`, and a
   second copy of the palette is exactly the drift that made the package stale. The site
   needs one source of truth for design tokens, and it already has one.
3. **Archive rather than delete** (`archive/` directory, files kept on disk). Rejected:
   the request is to remove unused material, and an archive directory keeps the repository
   weight and the stale references alive.

## Decisions

- `README.md` is **kept and updated** — it is the repository's front door, not unused
  documentation.
- `AGENTS.md` is **kept as-is** — it is read by the coding agent, and it references no
  deleted path.
- `docs/superpowers/specs/` is **kept** as the home for this spec and future active
  specs. The historical specs and plans beneath `docs/` are deleted.
- The `.gitignore` rule for `dogfood-output/` is **kept** so future audit runs stay
  untracked; only the existing tracked files are removed.
- `src/styles/global.css`, `src/styles/drills.css`, and
  `src/components/GrammarMap/types.ts` are **kept** — all imported by live pages.
- `src/components/GrammarMap/AspectIcon.tsx` is **deleted**. The aspect glyphs it
  defined are now inlined in the diagram components, so the file has zero references.
  `types.ts` in the same directory stays.

## Delete list

### Directories (all tracked, removed wholesale)

| Path | Tracked files | Why unused |
|---|---|---|
| `design/` | 30 | No import from `src/`; skill registered nowhere; stale palette |
| `docs/specs/`, `docs/SPEC-*.md`, `docs/superpowers/plans/` | 4 | Article-of-the-day work is shipped |
| `dogfood-output/` | 29 | Audit evidence for a finished QA round; already gitignored |
| `outputs/` | 19 | Research outputs, drafts, and provenance for finished topics |
| `papers/` | 3 | Superseded research papers |
| `prototypes/` | 13 | Standalone HTML mockups superseded by the shipped drill pages |

### Loose tracked files at repository root

`plan.md`, `progress.md`, `research.md`, `REVIEW_simplity_v2.md`,
`practice-snapshot.md`, `false` (a review file misnamed by a shell redirect).

### Dead code in `src/`

- `src/components/PolygonHero.tsx` — zero references
- `src/components/Term.tsx` — zero references
- `src/components/GrammarMap/AspectIcon.tsx` — zero references

### Untracked local files (disk only, not in git)

- `grammar-grid.html` — stray prototype at repository root
- Root-level `*.png` screenshots — stale QA captures, already covered by the `*.png`
  rule in `.gitignore`

## Update, not delete

`README.md`: remove the `design/grammar-map-kit.md` and `plan.md` layout entries;
replace the Pico.css / Astro 5 / Cloudflare Pages references with the actual stack
(Astro 6, SolidJS, custom CSS, Fly.io); refresh the layout section to match the pages
that exist.

## Verification

1. `npm run build` — Astro build plus internal-link check.
2. `npx astro check` — type-check across `.astro`, `.tsx`, `.ts`.
3. `git status` — only intended deletions and the `README.md` edit.
4. Grep for `design/`, `plan.md`, `papers/`, `outputs/`, `prototypes/`, `dogfood-output/`
   across the remaining tree — expect no hits outside `.gitignore` and this spec.
