# AGENTS.md

## Design system (Penpot)

Self-hosted Penpot at `http://localhost:9001` (credentials in `~/programming/penpot/.env`).
Project **lacoquille** → file **00 — lacoquille Design System**:

- Pages: `Page 1` = foundations (cover, color, type, spacing), `Components` (spec boards + 36
  library components), `Screens — Mobile` (15 mobile frames + 3 desktop frames).
- Token catalog: sets `color` (25), `dimension` (16), `typography` (13) — visible in the TOKENS tab.
- **`src/styles/tokens.css` is the code-side source of truth** — "Paper & encre de travail":
  warm paper `#F6F4EF`, ink `#1F1D18`, bleu-de-travail indigo progress scale
  (`p0 #E3E1DA → p1 #C9D3EC → p2 #4A5AA8 → p3 #232D5C`), vermillon `#C7531E` for
  review **and** miss (a miss causes a review — one meaning, one colour), Manrope only,
  DM Serif Display only inside Article of the Day.
- Design rationale + prototypes: `french-app-handoff.zip` (docs/ design/ data/) — read
  `docs/01-concept.md` first. Reference snapshots in `design/exports/`.

### Penpot MCP notes (2.18.6)

- Design tools (`execute_code`, `export_shape`) need the Penpot tab open with the MCP plugin
  connected. The bundled plugin (2.17) needs a manual nudge on this instance:
  1. Reload plugin iframe with token in the **hash**:
     `http://localhost:9001/plugins/mcp/#/?theme=dark&userToken=<PAT>`
     (PAT: `~/.config/opencode/.secrets/penpot-token`)
  2. postMessage into that iframe: `{type: "start-server", url: "/mcp/ws", token: <PAT>}`
- `execute_code` quirks: shapes created via `create*()` keep **page-absolute** coordinates after
  `appendChild`; boards clip children outside their bounds (always offset by the board origin).
  Long scripts (>~6KB) can wedge the plugin tab — split work into small calls. Writes only work
  on the **active** page (`penpot.openPage` is async — switch in one call, write in the next).
- `export_shape` is broken on this version (exporter bridge) — use agent-browser screenshots instead.

### Penpot safety + text-centering rules (learned the hard way)

- **Text bounds measured in the same call that created the text are stale** (width ≈ 0), so
  "centered" labels end up with their LEFT edge at the container center. Correct pattern:
  at creation store `t.setPluginData("ctr", x+","+y+","+w)`, then in a LATER call run a recenter
  pass reading settled `bounds`. Never bulk-recenter by geometry guessing alone.
- **Plugin edits persist instantly, are NOT on the undo stack, and there were no saved versions**
  — a bad bulk-delete wiped the whole Screens page once. Before risky batch operations, save a
  named version via the UI (History panel → Save version).
- **Never delete shapes by parent heuristics**: the page root frame's `name` is null, so
  "parent has no name" matches every top-level frame. Identify shapes by name + coordinates only.
- If the MCP tools start returning `null` (opencode's SSE stream to `/mcp/stream` died), drive the
  server directly: `tools/penpot-rpc.sh` pattern — POST JSON-RPC to
  `http://localhost:9001/mcp/stream?userToken=<PAT>` (init → `notifications/initialized` →
  `tools/call execute_code`), keeping the `mcp-session-id` header. Script kept at `/tmp/opencode/pp.sh`.

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

## Exercise generation (endless feed)

Two-tier, **stateless** system — all learner state stays client-side:

- `GET /api/exercises` — seeded curated generator (fixed sessions).
- `POST /api/exercises/feed` — endless feed. Body = learner context ONLY
  (`{level, topics[{slug,attempts,correct,lastAt}], recentTopics, recentItems}`,
  camelCase). Server attaches all pedagogy (topic catalog, blurbs, level
  taxonomy) and its **coach** (`lacq/src/coach.rs`) picks the next topic:
  external decision model when `TYPESAFE_API_KEY` is set (Jev via
  `POST api.typesafe.ai/v1/systemone` — naming stays domain-level:
  coach/DrillPlan/built-in, never vendor terms in the API), built-in
  weakest-topic heuristic otherwise or on timeout/error.
- Topic catalog: `lacq/src/exercises/topics.rs` (slugs + blurbs + levels +
  family). Generator families (`families.rs` dispatcher + `families/*.rs`,
  one file per engine cluster): tagged banks × slot combos → endless items
  (A1: present, periphrases, modals, articles, adjectives, pronouns_basic,
  questions, sentence_a1, mood_a1; A2: futur_simple, verbs_preps, compare,
  pronoun_en, a2_pronouns, a2_sentence; B1: b1_tenses, conditionnel,
  subjonctif_b1, passive, gerondif, b1_pronouns, b1_relatives, si_clauses,
  b1_sentence, nominalisation, b1_connectors; B2: anteriorite, subjonctif_b2,
  b2_tenses, participe_adjectif, b2_pronouns, b2_sentence; C1: c1_tenses,
  c1_moods, c1_nouns, c1_sentence; C2: c2_tenses, c2_moods, c2_style; plus
  the original starved-family engines pc-aux, pc-etre, pc-avoir,
  pronouns-cod-coi, pronoun-y, subjonctif, imparfait, imparfait-vs-pc,
  negation, cest-vs-il-est, accord-antepose — every catalog topic has a
  family; topics without one fall back to the curated corpus,
  `curated_for_topic`).
  Items carry `topicSlug` for client aggregation. Multi-topic engines
  dispatch on `items(slug)`; negation keeps a no-arg `items()` compat shim.
  Elision conventions: never end a cloze `answer` (or a choice sentence
  head) with an apostrophe — fold j'/qu'/s' into the answer chunk
  (j'aurai, S'il) or use elision-free frames.
  Coach + client levels span A1–C2: `LEVEL_TAXONOMY` in `coach.rs` and
  `LEVELS` in `feedStore.ts` must stay in sync with the catalog.
- Coverage gates, run before committing content changes: `cargo test
  coverage_gate -- --nocapture` (per-topic pool floors + matrix) and
  `python3 tools/coverage.py` (grid lighting report: lit vs dark entries,
  orphan slugs).
- Client: `/practice/feed` (`EndlessFeed.tsx`) — continuous, run-of-5,
  prefetches next run at 2 items left; misses requeue in-run. Learner state
  in `lacoquille.feed.v1` (`feedStore.ts`): per-topic aggregates, recent
  rings, rolling-window level estimate. The fixed session keeps its own
  `lacoquille.exercises.v1` store.
- Add a topic: append to `TOPICS` in topics.rs (+ `Family::` variant,
  engine file in `families/`, dispatch arm + `pool_floor` entry in
  families.rs, SLUG_MAP row in `taxonomy.ts`). Slugs are the wire
  contract — never rename them.


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

## Debugging and process management (lessons learned)

### Never wait blindly for servers

Backend servers (`lacq`, `cargo run`, etc.) run forever — they **won't finish**. Never chain `&` with curl in the same command like `cargo run & sleep 3 && curl ...`. The `sleep` will finish before the server is ready, causing timeouts and confusing failures.

**Instead: use the `process` tool to start background servers.**

```sh
# START once (do this first):
process action=start command="cargo run" name="lacq-server"

# Then TEST repeatedly (do this as many times as needed):
curl -s http://localhost:8080/api/article-of-the-day | jq .

```

See `pi-processes` skill for details on the process tool.

### Never use `sleep` with long arbitrary waits

1. Check process status with `process action=output` immediately after starting
2. Use quick `curl` tests without `sleep`
3. If a curl returns empty, check `curl ... | wc -c` and process logs before waiting
4. If tokenization times out, check that you're using `gemini-3.1-flash-lite` (not 3.5-flash which has different rate limits)
5. Test API changes directly with curl before assuming they work in the app

### Quick server verification
```sh
# After starting dev server, immediately check it's up
curl -s http://localhost:8080/health
# Don't do: sleep 10 && curl ... - just check right away
```

## Removing UI elements (lessons learned)

**To remove an element from every page:** edit `src/layouts/Base.astro` directly — delete the HTML, don't try to hide it. Conditionally rendering with `hideNav={true}` or `style={...}` is fragile: `style=""` ≠ `style="display: none"`, the element stays in the DOM, and Astro static builds can produce stale output if incremental builds are cached. Always delete the markup, not just hide it.

**After editing Base.astro:** run `npm run build` to verify. Check `dist/` directly (not just curl) to confirm changes are on disk. If a removed page is linked from other pages, create a redirect (`return Astro.redirect("/")`) or the build will fail with broken link errors.
