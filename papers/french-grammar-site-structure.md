# Best Structure for a French Grammar Reference Site

## Executive Summary

Successful French grammar reference sites share a common organizational pattern: **parts-of-speech taxonomy as the top-level hierarchy**, with verbs subdivided by mood then tense. Key retention drivers are comprehensive conjugation tables with interactive lookup, clear entry points for beginners, and real-situation examples showing actual usage patterns. Sites that organize by difficulty level (A1→C2) and separate "literary" tenses from everyday ones outperform those with exhaustive academic taxonomies.

---

## 1. Top-Level Organization: Parts of Speech

The dominant pattern across successful sites is a **parts-of-speech top-level navigation**. This matches how learners encounter grammar problems: "I need to know about adjectives" not "I need to know about agreement rules."

| Site | Top-level categories |
|------|---------------------|
| Lawless French | Nouns, Adjectives, Verbs, Adverbs, Prepositions, Pronouns, Articles, Conjunctions, etc. |
| Fluentu | Adjective, Adverb, Article, Miscellaneous, Mood, Noun, Preposition, Pronoun, Tense, Verb |
| Cambridge Grammar | Adjectives & Adverbs, Confused words, Nouns/Pronouns/Determiners, Prepositions, Verbs, Using English, etc. |

**Recommendation for lacoquille:** Adopt a parts-of-speech top-level taxonomy. Group the 8 traditional *parties du discours* as the main navigation anchors.

---

## 2. Verb Organization: Mood → Tense

All major French grammar references organize verbs the same way:

**Indicative → Present / Past / Future**
**Subjunctive**
**Conditional**
**Imperative**

Lawless French (the most comprehensive) adds:

- **Participles** (present, past)
- **Infinitives** (as a mood category)
- **Verb forms** as a separate section from conjugations

Critical observation: **literary tenses** (passé simple, imparfait du subjonctif, etc.) are de-emphasized or separated from the main navigation. They exist as a secondary level, often behind a "Show literary tenses" toggle. This matches learner behavior: most users need present, passé composé, imparfait, future, and conditional 95% of the time.

**Recommendation:** Separate literary tenses behind a toggle in the verb diagram. Show only the 5 core tenses by default.

---

## 3. Entry Points and Progressive Difficulty

Cambridge Dictionary's approach: *"Don't know where to get started?"* — clear path for beginners, with curriculum levels (B1-B2) marked explicitly.

MDN Web Docs structure:
1. **Getting started** — complete beginner setup
2. **Core modules** — essential skills (learner→comfortable)
3. **Extension modules** — specializations for advanced

**Recommendation:** Add a "Start here" entry point for beginners on the main grammar page. Use CEFR level tags (A1, A2, B1, B2, C1, C2) on each tense/conjugation page.

---

## 4. Immediate Usefulness: Conjugation Tables as the Core Feature

The #1 reason learners return to French grammar sites is **verb conjugation lookup**. Lawless French excels here with:

- Complete paradigm tables for each tense per verb
- Common irregular verbs prominently featured (être, avoir, faire, aller, prendre, pouvoir, devoir, savoir, voir, venir)
- Tables showing the conjugation pattern, not just one example verb

Cambridge's approach for verbs: categorized subtopics (Tenses and time, Verb forms, Verb patterns, Phrasal verbs, Passive voice, Modal verbs, Conditionals, Using verbs) with a table of irregular verbs as a reference sheet.

**Recommendation:** Prioritize verb conjugation tables above all else. Use the 10 core verbs (parler, être, avoir, aller, faire, finir, vendre, prendre, pouvoir, devoir) as canonical examples for each tense.

---

## 5. Real-Context Examples Over Abstract Rules

Lawless French pages consistently show:
- Side-by-side French/English example sentences
- Tables with multiple example conjugations
- "Par exemple…" sections that are actual sentences, not grammar formulas

Cambridge uses corpus-based examples from real written and spoken English. For French, this would mean authentic sentence examples rather than constructed ones.

**Recommendation:** Every grammar rule page should have 3-5 real example sentences that show the rule in context, not just "je parle" as a standalone.

---

## 6. Cross-Reference Navigation

Lawless French consistently links related concepts at the bottom of each page:
- "Related lessons" section with 5-10 cross-links
- Internal links to prerequisite knowledge
- "Parts of speech" breadcrumbs

Cambridge uses "→ Verbs → Tenses and time" breadcrumb trail.

**Recommendation:** Every page should have:
1. Breadcrumb showing: Grammar → [Part of Speech] → [Specific topic]
2. "Related" section with 5-10 linked topics
3. Prerequisites noted at the top of advanced topics (e.g., "Before learning the subjunctive, you should know...")

---

## 7. "Look-up" vs "Learn" Modes

The most successful grammar references support two distinct user modes:

1. **Look-up mode** (most common): User knows what they want, searches for it. Needs: fast search, clear nav, direct linkable URLs.
2. **Learn mode**: User wants to understand a topic from scratch. Needs: progressive structure, exercises, examples.

MDN addresses this with "Test your skills" quizzes throughout, and "Challenges" that require combining multiple skills.

**Recommendation:** Support look-up with a persistent search bar (always visible) and direct verb-specific URLs (e.g., `/grammar/verbs/present/parier`). Support learn mode with exercises and clear progression paths.

---

## 8. Features That Drive Return Visits

Based on cross-site analysis:

| Feature | Evidence |
|---------|----------|
| Complete conjugation tables | Lawless French main draw; every verb + every tense |
| Search by verb form | Learners remember only partial forms (e.g., "je finissais") |
| Interactive verb conjugator | Enables direct use without reading full explanations |
| CEFR level indicators | Helps learners know where they are |
| Mobile-friendly layout | Most grammar lookups happen on mobile |
| Quick examples | Users want to confirm a rule, not read a chapter |
| Bookmarking / saved verbs | Return visitors track their problem verbs |

---

## 9. Structural Recommendations for lacoquille

### Navigation Structure

```
Grammar (top nav)
├── Nouns
│   ├── Gender
│   ├── Plurals
│   ├── Articles
│   └── Noun endings (masculine/feminine patterns)
├── Adjectives
│   ├── Agreement
│   ├── Position
│   ├── Comparatives
│   └── Irregular forms
├── Verbs
│   ├── Indicative
│   │   ├── Present
│   │   ├── Passé composé
│   │   ├── Imparfait
│   │   └── Future
│   ├── Subjunctive
│   ├── Conditional
│   ├── Imperative
│   ├── Participles
│   └── [Literary tenses — toggleable]
├── Pronouns
├── Prepositions
├── Adverbs
├── Conjunctions
└── [Literary forms — toggleable]
```

### Key Design Decisions

1. **Verb as the hero feature** — 60%+ of page views on French grammar sites are verb conjugations
2. **Toggle literary tenses** — default off, user-enablable (checkbox in VerbDiagram)
3. **Periphrases as computed, not stored** — futur proche, passé récent, etc. are derived dynamically, not separate entries
4. **CEFR tags on every page** — A1 through C2, shown as small badges
5. **Verb grid on homepage** — visual grid of 10 core verbs with one-click conjugation
6. **Search always visible** — persistent search bar in nav or top of every page

---

## Open Questions

1. Should "participles" be a separate top-level category or under verbs?
2. How to handle *periphrases* (être en train de, aller + inf, venir de + inf) — as a dedicated section or cross-referenced?
3. What is the optimal number of example conjugations per tense page? (Lawless uses ~3 canonical verbs)
4. Should pronunciation guides be inline with conjugations or separate?

---

## Sources

- [Lawless French Grammar - Nouns](https://www.lawlessfrench.com/grammar/nouns/)
- [Lawless French Grammar - Adjectives](https://www.lawlessfrench.com/grammar/adjectives/)
- [Lawless French Grammar - Prepositions](https://www.lawlessfrench.com/grammar/prepositions/)
- [Lawless French - Present Tense](https://www.lawlessfrench.com/grammar/indicative-mood/present-tense/)
- [Lawless French - Subjunctive](https://www.lawlessfrench.com/grammar/subjunctive/)
- [Fluentu French Grammar](https://www.fluentu.com/french/grammar/)
- [Cambridge Dictionary Grammar](https://dictionary.cambridge.org/grammar/british-grammar/)
- [MDN Learning Web Development](https://developer.mozilla.org/en-US/docs/Learn_web_development)