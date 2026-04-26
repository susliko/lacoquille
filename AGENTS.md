# AGENTS.md

## Commands

```sh
npm run dev      # Frontend dev server at http://localhost:4321/
npm run dev:all  # Both frontend + backend (lacq/Rust) on port 4321 + 8080
npm run dev:prod # Production-like: build static, backend serves it on 8080 + live-reload preview
npm run build    # Build to ./dist/ + verify all internal links
npm run check:links  # Verify internal links only (requires prior build)
npm run preview  # Preview build locally
npx astro check  # Type-check .astro, .tsx, .ts files (run before committing)
```

Node 22+ required.

## Agent-browser testing

**Always use `npm run agent` for browser testing.** Never run servers manually.

> **Note:** Do not use `agent-browser snapshot`, `agent-browser click`, or `agent-browser screenshot` for SolidJS islands — these components hydrate client-side and elements may not exist in the DOM at evaluation time. Click/screenshot also won't work. For SolidJS pages, open the URL, use `--wait 5000` to let it hydrate, then verify via `agent-browser snapshot -i` if elements appear. If they don't, check SSR HTML with `curl` instead.

```sh
npm run agent
```

This is a single command that:
1. Builds the frontend to `dist/`
2. Symlinks `dist/` → `lacq/dist/` (so the Rust backend can serve it)
3. Starts the Rust backend on port 8080
4. Waits for port 8080 to be ready
5. Opens agent-browser and takes a snapshot

If you need to do further testing after the initial snapshot, use agent-browser commands directly — the Rust backend keeps running.

**Subagents cannot run browser tests** — they spawn without API key access and fail. Do all browser testing in the main session.

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

## Deployment

Single Fly.io Machine serves both the Astro static frontend and the Rust backend.

The `Dockerfile` is multi-stage: Node 22 builds the frontend, Rust compiles the binary, and Debian slim runs both.

```sh
fly launch   # First time: creates app + Machine
fly deploy  # Subsequent deploys
```

- `fly.toml` — app name, region, port 8080, `/health` health check; `vm` section sets 256MB RAM + 1 shared CPU
- Backend serves `dist/` at `/` (static files) and routes `/api/*` to Axum handlers
- App name in `fly.toml` is `lacoquille` — update if needed

## Article of the Day Backend

The `lacq/` directory is a Rust (Axum) backend server that:
- Serves `/api/article-of-the-day` with daily Maupassant story
- Fetches from Gutendex API and Gutenberg for book text
- Caches translations and story metadata in `lacq/data/`
- Run with: `npm run dev:all` (both frontend + backend)

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
