# Architecture Review: La Coquille vs Best Practices

Based on research into Lawless French, Fluentu, Cambridge Grammar, and MDN Web Docs.

---

## What's Already Good ✓

| Practice | Status in La Coquille |
|---|---|
| Parts-of-speech taxonomy as top-level nav | ✓ HomepageTabs uses this (Adjectifs, Pronoms, Verbes, etc.) |
| Mood → Tense verb organization | ✓ VerbDiagram uses `lane` (mood) + `timePosition` |
| Literary tenses toggleable | ✓ `showLiterary()` signal, `literary: true` flag in diagram.json |
| Periphrases computed dynamically | ✓ `PERIPHRASES` map in VerbDiagram (futur proche, passé récent, présent progressif) |
| Multiple canonical verbs | ✓ VERBS has parler, être, avoir, aller, faire, finir, vendre, venir, partir, rester |
| Cross-linked navigation | ✓ Each grammar page links to related topics via `<a href>` tables |
| "Présent = start here" indicator | ✓ Gold ring + "start here" label in VerbDiagram |

---

## Gaps to Fix

### 1. No grammar landing page (`/reference/grammar` → redirects to `/`)

Lawless and Cambridge both have a grammar index page showing all parts-of-speech with descriptions. You redirect there, which hides the taxonomy from direct navigation.

**Fix:** Make `/reference/grammar/index.astro` show the full parts-of-speech grid (same tiles as HomepageTabs but without the Practice tab). Link from `/reference/grammar` to each sub-section.

---

### 2. No search / lookup bar

Reference sites live and die by search. MDN has persistent search; Cambridge has it in the nav. Users who know what they want come to look up, not browse.

**Fix:** Add a persistent search bar to the nav (or top of every reference page). Even a simple text input that filters known terms would help. Search is currently absent from Base.astro.

---

### 3. Noun pages undersized compared to VerbDiagram

VerbDiagram is excellent — interactive SVG, real conjugations, toggleable literary. The noun/adjective/pronoun pages are static prose + a simple diagram. Lawless treats all parts-of-speech with the same depth.

**Fix:** Give each grammar sub-section its own interactive diagram component (like ArticleDiagram for articles, PronounDiagram for pronouns) that can toggle sub-topics and show examples inline.

---

### 4. No CEFR level tags anywhere

Cambridge marks pages as B1-B2. Lawless implicitly signals difficulty through structure. No such markers exist in La Coquille.

**Fix:** Add A1–C2 badges to grammar topic pages. Mark the VerbDiagram moods: Indicative = A1-B1, Subjunctive = B2, Literary = C1. This helps learners know where they are.

---

### 5. No conjugation table as a standalone feature

The #1 reason learners return to French grammar sites is **verb conjugation lookup** — not the diagram, but the actual paradigm tables. VerbDiagram shows the selected person's form but not the full table.

**Fix:** Add a `/reference/verbs/conjugations/[verb]` page with a complete conjugation grid (all 6 persons × all tenses). Link to it from the VerbDiagram (click a verb card → opens the full table). This is what drives return visits.

---

### 6. Homepage hides grammar from nav

Currently `hideNav={true}` on the homepage. The only way to access grammar is through the polygon cards. Lawless has persistent top navigation.

**Fix:** Keep the homepage clean, but add a minimal top nav or a "Grammaire" + "Verbes" button row above the tabs. Users should be able to navigate to grammar from any page.

---

### 7. Verb groups and tense-choice guides are underused

`verbGroups` and `choicePages` are shown below the VerbDiagram but as plain `<ul>` lists. They deserve better visual treatment.

**Fix:** Turn them into card grid sections matching the HomepageTabs polygon style. Or integrate them into the VerbDiagram as secondary panels.

---

## Summary: Scorecard

| Area | Score | Notes |
|------|-------|-------|
| Top-level organization | 7/10 | Good taxonomy, but grammar index page missing |
| Verb coverage | 9/10 | VerbDiagram is excellent; need standalone tables |
| Literary tenses handling | 9/10 | Toggle works well |
| Periphrases | 9/10 | Computed dynamically, correct approach |
| Search/lookup UX | 3/10 | No search bar anywhere |
| Cross-linking | 6/10 | Internal links exist but no "related" panels |
| Difficulty indicators | 2/10 | No CEFR tags, no "start here" path for beginners |
| Grammar depth parity | 5/10 | VerbDiagram is rich; noun/adjective pages are sparse |

---

## Recommended Next Steps (Priority Order)

1. **Add `/reference/grammar/index.astro`** — grammar landing page showing parts-of-speech grid
2. **Add conjugation table pages** — `/reference/verbs/conjugations/[verb]` for each verb
3. **Add search bar to nav** — persistent lookup input
4. **Add CEFR badges** to grammar topic pages and VerbDiagram moods
5. **Add interactive diagrams to non-verb grammar sections**
6. **Show verb groups and tense-choice guides as cards** instead of `<ul>` lists