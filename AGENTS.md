# AGENTS.md

## Commands

```sh
npm run dev      # Dev server at http://localhost:4321/
npm run build    # Build to ./dist/
npm run preview  # Preview build locally
npx astro check  # Type-check .astro, .tsx, .ts files (run before committing)
```

Node 22+ required.

## GitHub Operations

**Use GitHub MCP tools for all repo operations** (not git CLI):
- `github_push_files` — push files to repo
- `github_create_pull_request` — create PR
- `github_get_file_contents` — read files from repo
- `github_list_commits` — view commit history
- Repository: `susliko/lacoquille` on branch `main`

## Architecture

- **Astro 5** static-first framework with **SolidJS** islands for interactivity
- Single source of truth for the verb diagram: `src/content/verbs/diagram.json` — **both** the SVG nodes/edges and the tense content `.md` files must be kept in sync
- Conjugation data lives in `src/data/conjugations/` (parler, être, avoir, aller, faire, finir, vendre)
- `VerbDiagram.tsx` is a SolidJS island — **uses `solid-js` imports, NOT React** (tsconfig has `jsxImportSource: "solid-js"`)

## Adding tenses

1. Create `src/content/verbs/tenses/<slug>.md` with required frontmatter (see README.md)
2. Add the node to `src/content/verbs/diagram.json` — the diagram will not render it until it's in both places

## Adding verbs

1. Create `src/data/conjugations/<verb>.ts` following existing patterns
2. Export from `src/data/conjugations/index.ts` and add to the `VERBS` map

## Key conventions

- Literary tenses (passé simple, subjunctive imperfect, etc.) are toggled via a checkbox in the diagram — filtering is handled by `showLiterary()` signal in VerbDiagram
- Periphrases (futur proche, passé récent, present progressif) are computed dynamically in VerbDiagram, not stored in conjugation files
- Mobile layout is transposed: mood=col, time=row; desktop is time=col, mood=row
