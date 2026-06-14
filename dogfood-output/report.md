# Dogfood Report: La Coquille (Production) — Article of the Day & Related Pages

| Field | Value |
|-------|-------|
| **Date** | 2026-06-14 |
| **App URL** | https://lacoquille.fly.dev/ |
| **Session** | lacoquille-prod |
| **Scope** | Article of the Day page and related practice pages, plus sanity checks on the verb diagram and home page, on both desktop (1280x720) and mobile (390x844) viewports. |

## Summary

| Severity | Count |
|----------|-------|
| Critical | 4 |
| High | 6 |
| Medium | 5 |
| Low | 5 |
| **Total** | **20** |

The Article of the Day is the showcase feature that promises a click-aligned French↔English reading experience. In its current state, the user-facing behaviour is severely degraded: the translation is incoherent, only one of five paragraphs is translated, the token alignment is broken, and the practice pages that depend on the same data are completely unusable. Several bugs also affect unrelated surfaces (the Practice hub, the verb diagram, the home-page navigation).

---

## Issues

### ISSUE-001: Practice hub page renders completely blank on production

| Field | Value |
|-------|-------|
| **Severity** | critical |
| **Category** | visual / functional |
| **URL** | https://lacoquille.fly.dev/practice/ |
| **Repro Video** | N/A (single screenshot) |

**Description**

`/practice/` is entirely empty on the page. The four expected cards (Typing Race, Shadowing, Article of Day, Vocabulary Mining) exist in the DOM but are completely invisible: every card has a transparent background and white text on a white page background, so the user sees a blank viewport.

**Root cause (observed in DevTools)**

- `<div class="practice-hub-grid">` has computed `display: block` and `grid-template-columns: none` instead of the intended 2×2 grid. The `display: grid; grid-template-columns: repeat(2, 1fr);` rule lives inside the page's `<style>` block, which Astro scopes with a `data-astro-cid-xxx` attribute. The PracticeHub component is mounted with `client:only="solid-js"`, so its DOM is generated client-side and does not carry that attribute — none of the scoped rules match.
- `.poly-card-name` (in `global.css`) has `color: white`, so even when the cards stack they show white text.
- Confirmed via `getComputedStyle`:
  - 4 `.practice-card` elements exist
  - All have `background: rgba(0, 0, 0, 0)`
  - All `.poly-card-name` children have `color: rgb(255, 255, 255)`
  - Grid has `display: block`

**Repro Steps**

1. Open https://lacoquille.fly.dev/practice/
2. Wait for any client-side rendering to finish
3. **Observe:** The viewport is empty. No background colors, no visible text. DevTools shows the cards and a flat `<div>` for the grid.
   ![blank practice hub](screenshots/P17-prod-practice.png)

---

### ISSUE-002: Shadowing practice page is stuck on "Loading stories..." forever

| Field | Value |
|-------|-------|
| **Severity** | critical |
| **Category** | functional |
| **URL** | https://lacoquille.fly.dev/practice/shadowing/ |
| **Repro Video** | N/A |

**Description**

The Shadowing page never finishes loading. It displays "Loading stories..." indefinitely and never transitions to an error state.

**Root cause**

`src/components/ShadowingPractice.tsx` calls `fetch('/api/stories')`, `fetch('/api/stories/:id')`, `fetch('/api/stories/:id/tts-cache', …)`, and `fetch('/api/tts')`. The Rust backend in `lacq/src/routes/mod.rs` only exposes `/health` and `/api/article-of-the-day` — none of those endpoints exist. `GET /api/stories` returns 404, and because the component only `try`-awaits without showing an error state on a non-OK response, it sits in its loading view forever.

**Repro Steps**

1. Open https://lacoquille.fly.dev/practice/shadowing/
2. Wait 30+ seconds
3. **Observe:** Page still says "Loading stories…"
   ![shadowing stuck](screenshots/P20b-prod-shadowing.png)
4. In DevTools network panel: `GET /api/stories` returns 404. No error is shown to the user.

---

### ISSUE-003: Article of the Day translation is incoherent and only covers the first paragraph

| Field | Value |
|-------|-------|
| **Severity** | critical |
| **Category** | content / ux |
| **URL** | https://lacoquille.fly.dev/stories/article-of-the-day/ |
| **Repro Video** | videos/P-issue-broken-translation.webm |

**Description**

The English column is supposed to be a fluent, sentence-level translation of the French. In practice it is a literal word-for-word gloss that drops articles, rearranges words into ungrammatical order, and is missing entirely for four of the five paragraphs of the story.

What the page renders:

- French (5 paragraphs): opening narration, the boat scene with Mme Roland and Mme Rosémilly, dialogue, the husband's reply, dialogue, the husband's complaint about not catching fish.
- English: only the first sentence is "translated" — and it reads:
  > "Zut! cried out sudden father who since a quarter of an hour remained immobile the eyes fixed on the water and lifting by moments a very movement very light his line descended to the bottom of the sea"
  followed by two orphan fragments ("at Roland and by to the the" / "all the the of") that obviously come from the alignment going wrong.

Calling the existing text a translation is generous. It contains no verbs of saying with their subjects in the right place ("cried out the father suddenly"), no "the" articles, contractions are left in their French form, the noun–adjective order is French ("movement very light"), and prepositions are translated one-to-one without idiom ("lifting by moments" instead of "lifting the line every now and then"). The four paragraphs of dialogue and the rest of the narration have no English at all, so a learner who only reads the right column misses ~80% of the story.

**Repro Steps**

1. Open https://lacoquille.fly.dev/stories/article-of-the-day/
2. Wait for the article to load
3. **Observe:** The English column ends with "all the the of" after only a couple of lines, while the French side has 5 paragraphs of content.
   ![broken translation](screenshots/P16-step1-loaded.png)
4. (Optional) `curl https://lacoquille.fly.dev/api/article-of-the-day | jq -r '.tokenized.en_tokens[].text'` to see the raw translation data: only 36 English words for 5 paragraphs of French.

---

### ISSUE-004: Token alignment is scrambled — many French words are not clickable, and the ones that are jump to the wrong English token

| Field | Value |
|-------|-------|
| **Severity** | critical |
| **Category** | functional / content |
| **URL** | https://lacoquille.fly.dev/stories/article-of-the-day/ |
| **Repro Video** | videos/P-issue-broken-translation.webm |

**Description**

The interactive selling point of this page is "click a French word, see its English translation highlighted (and vice versa)". Two separate bugs break this:

**Bug A — Tokens that should belong to paragraph 1 are assigned spans that point to other paragraphs.** When the user clicks, the React/Solid component groups tokens by paragraph using their character span. Because the span of "tout" in P1 is recorded as `[445, 449]` (a position inside paragraph 5), it gets grouped into P5 instead of P1. Same for: `à`, `le`, `Roland`, `les`, `et`, `par`, `au`, `de`, `la` — ten common words that disappear from the first paragraph.

Result: paragraph 1 has only **15 of its 26** tokens clickable. P2 has 3, P3 and P4 have 0, P5 has 2. Of the 36 tokens in the API payload, only 20 are rendered as clickable spans.

**Bug B — Even the clickable tokens point to the wrong English translation.** When the user clicks `père` (father) on the French side, the page highlights `father` in English. When the user clicks `coup` (the word for "blow/knock" in the phrase *tout à coup*, "suddenly"), the page highlights the English word `sudden`, which is roughly the right meaning but the wrong word pairing. The general pattern: a single French word is paired with whatever English word happens to occupy that token index, regardless of the original French sense.

**Repro Steps**

1. Open https://lacoquille.fly.dev/stories/article-of-the-day/
2. In the French column, scan the first paragraph and notice that "tout à le", "Roland", "depuis un quart d'heure", "immobile, les", "sur l'eau, et", "sa ligne descendue au fond de la mer" are mostly NOT clickable
3. Click on "coup" in the French column
4. **Observe:** the English word `sudden` is highlighted, even though "coup" by itself is not the right French word — the user can't click on "tout" or "à" to see the rest of the phrase
   ![click coup highlights sudden](screenshots/P16-step3-click2.png)
5. `curl -s https://lacoquille.fly.dev/api/article-of-the-day | jq '.tokenized.fr_tokens[] | {text, span: .spans[0]}'` to confirm spans are misaligned with the text.

---

### ISSUE-005: Verb diagram — the "durée" edge label is illegibly small

| Field | Value |
|-------|-------|
| **Severity** | high |
| **Category** | visual |
| **URL** | https://lacoquille.fly.dev/reference/verbs/ |
| **Repro Video** | N/A |

**Description**

Between the Présent (indicatif) and Subj. présent nodes, the dashed aspect-pair edge is labeled "durée" in a tiny italic font that sits directly on the line. The label is so small and the contrast so low that it's nearly invisible at 1280×720 — the user has no way to know what kind of relationship that edge represents without squinting.

**Repro Steps**

1. Open https://lacoquille.fly.dev/reference/verbs/
2. Look at the dashed vertical line between the Présent card and the Subj. présent card
3. **Observe:** there is a label there, but it reads as a faint smudge; the word "durée" is only barely legible
   ![diagram with tiny durée label](screenshots/P22-prod-verbs.png)

---

### ISSUE-006: Practice page on the home tab is missing the "Vocabulary Mining" card and lists only 3 of 4 items

| Field | Value |
|-------|-------|
| **Severity** | high |
| **Category** | ux / content |
| **URL** | https://lacoquille.fly.dev/ (Practice tab) |
| **Repro Video** | N/A |

**Description**

The home page's Practice tab shows 3 cards: Typing Race, Shadowing, Article of Day. The practice hub page (which itself is invisible — see ISSUE-001) advertises 4 cards, the fourth being "Vocabulary Mining". There is no link to Vocabulary Mining from anywhere reachable in the UI, and the home page Practice tab is missing it.

**Repro Steps**

1. Open https://lacoquille.fly.dev/
2. Click the "Practice" tab
3. **Observe:** 3 cards (Typing Race, Shadowing, Article of Day). "Vocabulary Mining" is missing.
4. Try to find Vocabulary Mining anywhere on the site
5. **Observe:** No links to /practice/vocabulary or similar exist on the home page or any other tested page.
   ![home practice tab](screenshots/P14-prod-home.png) (showing Knowledge tab; Practice tab is the same 3-card layout)

---

### ISSUE-007: Grammar topics "Interrogation" and "Reflexive verbs" are unreachable from the UI

| Field | Value |
|-------|-------|
| **Severity** | high |
| **Category** | ux / functional |
| **URL** | https://lacoquille.fly.dev/reference/grammar/ |
| **Repro Video** | N/A |

**Description**

The home page topic grid exposes 7 grammar sub-topics. The grammar subdirectory contains 9 topics including `interrogation/` and `reflexive-verbs/`, and both pages return 200, but neither is linked from the home page and the grammar index (`/reference/grammar/`) is a stub that 302-redirects to `/`. Result: those two pages are reachable only by typing the URL or following an external link.

**Repro Steps**

1. Open https://lacoquille.fly.dev/
2. Look at the Knowledge grid: Adjectifs, Adverbes, Articles, Négation, Pronoms, Prép., Syntaxe, Verbes — 8 topics total
3. **Observe:** No card for "Interrogation" or "Reflexive verbs"
4. Try to navigate to /reference/grammar/
5. **Observe:** 302 redirect to `/`
6. (Works only via direct URL) https://lacoquille.fly.dev/reference/grammar/interrogation and /reflexive-verbs both return 200 but there is no entry point in the UI.
   ![grammar redirect](screenshots/P14-prod-home.png)

---

### ISSUE-008: The active-token highlight is barely visible on the French side

| Field | Value |
|-------|-------|
| **Severity** | high |
| **Category** | visual / accessibility |
| **URL** | https://lacoquille.fly.dev/stories/article-of-the-day/ |
| **Repro Video** | videos/P-issue-broken-translation.webm |

**Description**

When the user clicks a French word, the active class applies `background-color: rgba(59, 130, 246, 0.2)` and a 2px blue outline. Against the default white page background, the 20%-opacity blue is very subtle — easy to miss, especially in bright environments or for users with low-contrast vision. The English-side highlight is the same low-contrast blue with no outline, making it even harder to spot which token is "active".

**Repro Steps**

1. Open https://lacoquille.fly.dev/stories/article-of-the-day/
2. Click the English word "cried out"
3. **Observe:** The French word "s'écria" is highlighted with a pale-blue background and faint outline that is hard to pick out at a glance, especially compared to the bold blue of a focused link.
   ![subtle highlight](screenshots/P07-click-french.png)

---

### ISSUE-009: Typing Race does not auto-focus the text area, so the first key press is lost

| Field | Value |
|-------|-------|
| **Severity** | high |
| **Category** | ux / functional |
| **URL** | https://lacoquille.fly.dev/practice/typing/ |
| **Repro Video** | N/A |

**Description**

On the Typing Race page, the user is invited to "click the text - start typing", but the page also tries to focus the text area automatically when the article loads. In practice, on first load the auto-focus often misses (e.g., the agent-browser test, and most first-load scenarios in real browsers) — the user starts typing, the keys go nowhere, and the only feedback is the WPM staying at 0 and accuracy at 100%. The user has to click into the text area to begin.

Even after clicking, the very first character that should be typed is the em-dash pair "--" used as French dialogue opening. There is no on-screen hint that the user must type these literal characters, and the bottom hint just says "click the text - start typing".

**Repro Steps**

1. Open https://lacoquille.fly.dev/practice/typing/
2. Without clicking on the text area, type a key
3. **Observe:** Nothing happens. WPM stays at 0, accuracy 100%, no character is marked.
4. Click the text area once, then type
5. **Observe:** Typing works now. The first two characters you must type are literal `--` (em-dashes for French dialogue), which is unusual for a typing-practice onboarding.
   ![typing page initial state](screenshots/P18-prod-typing.png)

---

### ISSUE-010: Verb dropdown overflows below the page bottom and obscures the diagram

| Field | Value |
|-------|-------|
| **Severity** | medium |
| **Category** | visual / ux |
| **URL** | https://lacoquille.fly.dev/reference/verbs/ |
| **Repro Video** | N/A |

**Description**

The verb selector dropdown is positioned to open below the button, but its 10+ entries (the three movement-verb sub-items are nested under "faire" and visually grouped) extend past the bottom of the diagram card. As a result, the "anterior" time-axis label is hidden behind the open dropdown, and the last few items in the list are partially obscured by the page edge / next section. There is no scroll affordance on the dropdown.

**Repro Steps**

1. Open https://lacoquille.fly.dev/reference/verbs/
2. Click the VERB selector (currently showing "-er (parler)")
3. **Observe:** The dropdown overlays the top-left of the diagram, hiding the "anterior" column label and partially clipping the bottom items.
   ![verb dropdown overflow](screenshots/17-verb-dropdown.png)

---

### ISSUE-011: Tense and choice-guide pages are missing the "GRAMMAIRE" breadcrumb present on the verbs index

| Field | Value |
|-------|-------|
| **Severity** | medium |
| **Category** | ux / navigation |
| **URL** | https://lacoquille.fly.dev/reference/verbs/tenses/present-indicatif etc. |
| **Repro Video** | N/A |

**Description**

`/reference/verbs/` shows a red "GRAMMAIRE" label above the "Verbes" H1 (a breadcrumb-style link back to the grammar hub). Drill-down pages — every `/reference/verbs/tenses/<slug>` and `/reference/verbs/choice/<slug>` page — do NOT have that breadcrumb. Users have no in-page way to navigate back to the verbs overview (or further up to the home) without using the browser back button.

**Repro Steps**

1. Open https://lacoquille.fly.dev/reference/verbs/ — note the red "GRAMMAIRE" label above the H1
2. Click any tense node, e.g. Présent
3. **Observe:** The tense page has no "GRAMMAIRE" breadcrumb, no "Verbes" link, no nav at all above the H1. The user must use the browser back button.
   ![verbs page with breadcrumb](screenshots/16-verbs-desktop-zoom.png) vs. tense page with no breadcrumb (in ISSUE-004 screenshots).

---

### ISSUE-012: Article of the Day — metadata is "Roman (1888)" with no context for an English-speaking audience

| Field | Value |
|-------|-------|
| **Severity** | medium |
| **Category** | content |
| **URL** | https://lacoquille.fly.dev/stories/article-of-the-day/ |
| **Repro Video** | N/A |

**Description**

The page is presented in English (headings "FRANÇAIS" / "ENGLISH", English instructions), but the work's type is given as "Roman" — a French word meaning "novel" that, to an English reader, suggests ancient Rome. The Typing Race variant spells it "Roman · 1888" (with a middle dot, inconsistent with the article page's parentheses). A learner encountering "Roman" has no way to tell what genre it is. The metadata also omits the author (Maupassant) entirely, even though the Practice tab advertises the page as a "daily Maupassant story".

**Repro Steps**

1. Open https://lacoquille.fly.dev/stories/article-of-the-day/
2. Read the metadata line under "Pierre et Jean"
3. **Observe:** "Roman (1888)" — the work type is in French and the author (Maupassant) is not shown.
4. Open https://lacoquille.fly.dev/practice/typing/
5. **Observe:** Same metadata, different formatting: "Roman · 1888"
   ![roman metadata](screenshots/P01-prod-article.png)

---

### ISSUE-013: Article of the Day — no link, no context, no author bio, no way to read past articles

| Field | Value |
|-------|-------|
| **Severity** | medium |
| **Category** | ux / content |
| **URL** | https://lacoquille.fly.dev/stories/article-of-the-day/ |
| **Repro Video** | N/A |

**Description**

The page is the showcase for daily reading practice, but the user gets no help to learn more about the work they're reading:

- No link to the source on Gutenberg, Wikisource, or a literary reference
- No author info (Maupassant) anywhere on the page, even though the home page Practice tab explicitly calls it "a daily Maupassant story"
- No plot summary, no era/genre information
- No "previous story" / "next story" navigation; once the day changes, the article is gone

A learner finishing the page has no obvious next step.

**Repro Steps**

1. Open https://lacoquille.fly.dev/stories/article-of-the-day/
2. Try to find any link, button, or further reading
3. **Observe:** Page contains only the title, the metadata, and the French/English columns. There are no links in the document.
4. (Confirm in DevTools) `document.querySelectorAll('a').length` returns 0.
   ![no nav](screenshots/P01-prod-article.png)

---

### ISSUE-014: The "durée" label in the verb diagram has no entry in the diagram's Key section

| Field | Value |
|-------|-------|
| **Severity** | medium |
| **Category** | content / consistency |
| **URL** | https://lacoquille.fly.dev/reference/verbs/ |
| **Repro Video** | N/A |

**Description**

The Key section (under the diagram) explains the difficulty legend and the five connection line styles (auxiliary compound, aspect pair, stem share, mood swap, anteriority). The `durée` label on the dashed aspect-pair line is not explained anywhere — it appears to be a French-only annotation that English readers will not understand. The Key should at minimum translate it, or the label should be in English.

**Repro Steps**

1. Open https://lacoquille.fly.dev/reference/verbs/
2. Click "+Key" at the bottom of the diagram
3. **Observe:** The Key lists "auxiliary compound, aspect pair, stem share, mood swap, anteriority" — but it never mentions "durée" or explains the small italic label between Présent and Subj. présent.
   ![diagram with key expanded](screenshots/05b-verbs-key-full.png)

---

### ISSUE-015: Mobile tooltip overlaps the previous line of text

| Field | Value |
|-------|-------|
| **Severity** | medium |
| **Category** | visual / ux |
| **URL** | https://lacoquille.fly.dev/stories/article-of-the-day/ (mobile) |
| **Repro Video** | N/A |

**Description**

On mobile, clicking a French word surfaces a `::after` tooltip that sits `calc(100% + 4px)` above the word. For a word that is the first token of a wrapped line, the tooltip floats over the previous line of text and even over the "FRANÇAIS" heading. The tooltip is not repositioned when it would go off the top of the viewport or overlap the section heading.

**Repro Steps**

1. Open https://lacoquille.fly.dev/stories/article-of-the-day/ on a 390px-wide viewport
2. Click "père" (the first word of the second wrapped line of paragraph 1)
3. **Observe:** The "father" tooltip floats high enough to overlap the "FRANÇAIS" heading, which is now sitting on top of the tooltip and partially through the tooltip.
   ![mobile tooltip overlap](screenshots/P10-mobile-click.png)

---

### ISSUE-016: Verb dropdown uses `expanded=false` on the button but the panel is actually a popover — minor a11y smell

| Field | Value |
|-------|-------|
| **Severity** | low |
| **Category** | accessibility |
| **URL** | https://lacoquille.fly.dev/reference/verbs/ |
| **Repro Video** | N/A |

**Description**

The verb selector button reports `aria-expanded=false` when closed and presumably `true` when open, but the dropdown content is a sibling element with no `role="listbox"` / `role="menu"` / `aria-controls` association. Keyboard users can open the menu but cannot navigate the items with arrow keys — there is no focus management, and `Esc` does not close the dropdown.

**Repro Steps**

1. Open https://lacoquille.fly.dev/reference/verbs/
2. Tab to the VERB selector and press Enter
3. **Observe:** The dropdown opens. Tab moves focus to the next focusable element on the page (the PERSON row), not to the first item in the list. Arrow keys do nothing.

---

### ISSUE-017: Typing Race — only the first paragraph of the article is loaded

| Field | Value |
|-------|-------|
| **Severity** | low |
| **Category** | ux |
| **URL** | https://lacoquille.fly.dev/practice/typing/ |
| **Repro Video** | N/A |

**Description**

`TypingRace.tsx` does `(article()?.paragraphs?.[0] ?? "").split("")` — it always uses only the first paragraph of the article. For a story like Pierre et Jean that opens with a long descriptive paragraph, the typing practice is just that one paragraph; for an article that opens with a short dialogue, it's just a single line. There's no option to type the full article or to pick a specific paragraph.

**Repro Steps**

1. Open https://lacoquille.fly.dev/practice/typing/
2. **Observe:** The typing area contains only the first paragraph (and stops at the first period). The other 4 paragraphs of the article are not offered for typing practice.
   ![typing only first para](screenshots/P18-prod-typing.png)

---

### ISSUE-018: Page title contains the site name twice: "Article of the Day — La Coquille · La Coquille"

| Field | Value |
|-------|-------|
| **Severity** | low |
| **Category** | content |
| **URL** | https://lacoquille.fly.dev/stories/article-of-the-day/ (and likely all pages) |
| **Repro Video** | N/A |

**Description**

`Base.astro` templates the `<title>` as `${title} · La Coquille`. Several pages set `title="... La Coquille"` themselves, which produces a doubled name in the browser tab. "Article of the Day — La Coquille · La Coquille" is visible in the tab; same on the home page (`La Coquille · La Coquille`).

**Repro Steps**

1. Open https://lacoquille.fly.dev/stories/article-of-the-day/
2. Look at the browser tab title
3. **Observe:** "Article of the Day — La Coquille · La Coquille"

---

### ISSUE-019: Mobile (390px) — the Knowledge topic grid is still 3 columns wide, with text wrapping awkwardly in each tile

| Field | Value |
|-------|-------|
| **Severity** | low |
| **Category** | visual / ux |
| **URL** | https://lacoquille.fly.dev/ (mobile) |
| **Repro Video** | N/A |

**Description**

On a 390px viewport, the home page Knowledge grid is still 3 tiles per row (≈114px per tile). Card labels wrap awkwardly: "personal · relative" becomes 2 lines, "tenses & conjugation" becomes 2 lines, "ne…pas · jamais" becomes 3 lines, and only 2 cards appear in the third row (7 + 1 = 8 cards, in a 3-column grid). The 33%/33%/33% rule applies the same way at every viewport size, so the mobile experience is just a cramped version of the desktop.

**Repro Steps**

1. Open https://lacoquille.fly.dev/ at 390px width
2. **Observe:** 3 cards per row at 114px each; text wraps to 2–3 lines inside each card; the bottom row has 2 cards left-aligned and a large empty cell.
   ![mobile home grid](screenshots/15-home-mobile-fresh.png)

---

### ISSUE-020: No `<nav>` and no skip link on most pages

| Field | Value |
|-------|-------|
| **Severity** | low |
| **Category** | accessibility |
| **URL** | Multiple (verbs, article, tense, choice) |
| **Repro Video** | N/A |

**Description**

Most pages (every page except the home, which uses the tabs) have no `<header>` or `<nav>` element. There is no site-wide menu, no skip-to-content link, and the only way for a screen-reader user to navigate between sections is the browser's rotor. This is most visible on the Article of the Day page, which is a dead end with no breadcrumb, no menu, and no in-page links at all.

**Repro Steps**

1. Open https://lacoquille.fly.dev/stories/article-of-the-day/
2. Inspect the DOM
3. **Observe:** No `<nav>`, no `<header>`, no `<main>` landmark beyond the implicit main, no skip link, no in-page links.

---
