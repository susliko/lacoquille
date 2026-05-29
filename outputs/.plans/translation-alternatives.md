# Plan: LLM-free Translation Research

**Slug:** translation-alternatives

**Date:** 2026-05-29

---

## Context

The article-of-the-day feature needs word-level French→English translations with character spans. The current LLM-based approach has problems:
- Free models burn tokens on reasoning (GLM returns `content: null`)
- Small models (1.2B liquid) produce poor quality
- Punctuation handling is unreliable

**Core question:** Are we solving this from the wrong angle? Do we need LLM-level understanding, or can simpler methods work?

---

## Key Questions

1. **What are the actual requirements?**
   - Tokenization: rule-based possible? Need spans and punctuation separation
   - Translation: word-level alignment needed, or sentence-level + word alignment?
   
2. **What LLM-free alternatives exist?**
   - Pre-trained NMT models (opus-mt-fr-en, can run locally)
   - Dictionary APIs (WordReference, DeepL free tier)
   - Sentence MT + word alignment (fast_align, simalign)
   
3. **Tradeoffs:**
   - Cost vs quality vs complexity
   - Local vs cloud
   - Setup complexity vs reliability

4. **What's the bottleneck?**
   - Is tokenization the hard part? (No—regex can do this)
   - Is translation the hard part?
   - Is word-alignment the hard part?

---

## Evidence Needed

1. **opus-mt-fr-en quality check:** How good is this small model? Can it handle literary French?
2. **DeepL free tier limits:** What can we use without API key?
3. **Word alignment tools:** Are fast_align/simalign practical for this use case?
4. **Latency requirements:** Can we run a local model on Fly.io with 256MB RAM?

---

## Scale Decision

**Direct search** — narrow question, can answer with 5-10 tool calls. No researcher subagents needed.

---

## Task Ledger

1. **[owner] research-1:** Search for opus-mt-fr-en quality on literary French
2. **[owner] research-2:** Search for DeepL free API limits and alternatives  
3. **[owner] research-3:** Search for word alignment tools (fast_align, simalign)
4. **[owner] analyze:** Summarize findings and recommend approach

---

## Verification Log

- [ ] opus-mt-fr-en quality verified
- [ ] DeepL free limits documented
- [ ] Word alignment tools compared
- [ ] Recommendation ready

---

## Decision Log

| Date | Decision | Rationale |
|------|----------|-----------|
| - | - | - |

---

## Output Artifacts

- `outputs/.plans/translation-alternatives.md` (this file)
- `outputs/.drafts/translation-alternatives-research-direct.md` (notes)
- `outputs/translation-alternatives.md` (final report)
- `outputs/translation-alternatives.provenance.md`