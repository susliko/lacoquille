# Knowledge prototypes — Le Manuel

HTML prototypes for the Knowledge tab. Direction 01 (**Le Manuel**, `1-manuel.html`)
was chosen and iterated to final form; 02–04 and `index.html` are the rejected
exploration kept for reference.

## Chosen design

`1-manuel.html` — self-contained (tokens inlined, data injected inline by the build):

- Six level tabs A1–C2, groups 1:1 with the grammar-map columns
  (**Tense · Mood · Nouns & articles · Adjectives & adverbs · Pronouns · Sentence** —
  the Noun group expanded into its three subgroups, like the map's bottom sheet).
- Substring search across all levels, accent/case-insensitive, with highlighting.
- Topic sheets: numbered rules with examples, exception cards, see-also chips,
  Practice CTA (a not-drillable-yet note remains as fallback for any entry
  the catalog hasn't reached — none since the 121/121 C1/C2 wiring).
- No progress indicators, counts, codes or banners — reference only.

## Data pipeline

| File | Source | Role |
|------|--------|------|
| `extract.py` → `topics.json` | `lacq/src/exercises/topics.rs` | catalog (slugs + blurbs) |
| `extract.py` → `taxonomy.json` | `src/components/progress/taxonomy.ts` | **the 1:1 contract**: 121 entries, six levels, one level each |
| `rules-src/rules_{a,b,c}.py` | hand-authored | rule sheets keyed by taxonomy name (A1–A2 / B1–B2 / C1–C2) |
| `rules-src/build.py` | — | validates + resolves, writes `rules.json`, injects `RULES` + `ENTRIES` into `1-manuel.html` |
| `rules.json` | build | 121 topics → 203 rules, keyed by taxonomy entry id |

Re-extract after the catalog or taxonomy moves (new topics, SLUG_MAP rows):

```sh
cd design/prototypes/knowledge/rules-src && python3 extract.py
```

Rebuild after editing rules:

```sh
cd design/prototypes/knowledge/rules-src && python3 build.py
```

The build fails on: missing/unknown/duplicated topic names, bad rule shape,
unresolved see-references, or AI-tell patterns (humanizer word list).

## Porting to the real app

`taxonomy.json` + `rules.json` are the content source for the future
Astro pages (`/knowledge/` index + per-topic sheets). Slugs are the wire
contract; taxonomy ids (`t0…t120`) key the rule sheets.
