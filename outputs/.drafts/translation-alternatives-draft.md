# Translation Alternatives for Article of the Day

## Executive Summary

The article-of-the-day feature needs word-level French→English translations with character spans for interactive highlighting. Our current LLM-based approach has reliability issues with free providers (GLM burns tokens on reasoning, returns `content: null`). This report investigates alternatives to determine if we're "solving from the wrong angle."

**Finding:** The LLM approach is appropriate for this problem. Word alignment and idiom handling genuinely benefit from LLM-level understanding. The issue is free provider reliability, not the fundamental approach.

**Recommendation:** Stick with LLM translation but address provider reliability through:
1. Make Groq the primary (already works, good quality)
2. Fix null-content handling in code for edge cases
3. Consider quantized OPUS-MT as a local fallback (needs further RAM feasibility testing)

---

## Requirements Analysis

### Current Implementation

The feature needs:
1. **Tokenization:** Split French text into words/phrases with character spans
2. **Translation:** Word-level FR→EN mapping
3. **Alignment:** Map each French token to corresponding English token(s) via `en_indices`

Current LLM prompt tries to solve all three simultaneously, returning:
```json
{
  "frTokens": [{"text": "...", "spans": [[start, end]], "en_indices": [0]}],
  "enTokens": [{"text": "word ", "index": 0}]
}
```

### Are These Requirements Achievable Without LLM?

| Requirement | Rule-Based? | Difficulty |
|-------------|-------------|------------|
| Tokenization | Yes | Low (regex already works) |
| Translation | Partial | Medium (idioms hard) |
| Word alignment | Yes | Medium (needs statistical methods) |

**Conclusion:** Tokenization can be fully rule-based. Translation and alignment could use statistical methods, but idioms and non-compositional expressions remain problematic without LLM understanding.

---

## Alternative Approaches

### 1. OPUS-MT (Helsinki-NLP/opus-mt-fr-en)

**What it is:** Pre-trained transformer model, 75.2M parameters, FR→EN translation.

**Benchmarks:**
| Test Set | BLEU | chr-F |
|----------|------|-------|
| News (various years) | 26-38 | 0.54-0.63 |
| Tatoeba (simple sentences) | 57.5 | 0.72 |

**Pros:**
- Runs locally, no API calls needed
- Good quality on standard sentences
- Pre-trained word alignments available
- HuggingFace transformers compatible

**Cons:**
- 75M params ≈ 300MB+ RAM (tight for 256MB Fly.io limit)
- Sentence-level only, no word alignment at inference
- Literary French quality may differ from training domain
- No quantized version with verified quality

**RAM Feasibility:** Borderline. Model weights ~300MB float32, plus tokenizer and runtime overhead may exceed 256MB.

### 2. fast_align (Word Alignment Tool)

**What it is:** Unsupervised IBM Model 2 implementation for word alignment.

**How it works:**
```bash
# Input format: source ||| target
Le chat mange la souris . ||| The cat eats the mouse .

# Output: alignments
0-0 1-1 2-2 3-3 4-4
```

**Pros:**
- Fast, production-ready
- No training needed (unsupervised)
- Produces exact word-level alignments

**Cons:**
- Needs parallel corpus as input
- Not real-time (batch processing)
- Cannot be embedded in Rust backend easily (C++ tool)

**Possible pipeline:**
1. Translate with OPUS-MT → get English sentence
2. Run fast_align on `french_sentence ||| english_sentence`
3. Map alignments back to tokens

### 3. Hybrid: OPUS-MT + fast_align + Rules

**Concept:**
1. Tokenize French with existing regex rules (`split_tokens_with_punct`)
2. Translate full sentences with OPUS-MT
3. Align words with fast_align
4. Map translations back to original tokens

**Pros:** Reduces or eliminates LLM API calls
**Cons:** Complex pipeline, alignment may be imperfect, RAM constraints

### 4. Dictionary + LLM Fallback

**Concept:**
- Use word-level dictionary for common vocabulary
- LLM only for idioms and unknown words
- Assemble final translation from pieces

**Pros:** Faster, more predictable costs
**Cons:** Literary French has rich vocabulary, dictionary coverage would be poor

---

## The Real Problem

Our testing revealed:
- **GLM (z-ai/glm-4.5-air:free):** Uses extended thinking that exhausts `max_tokens`, returning `content: null`
- **Liquid (liquid/lfm-2.5-1.2b-instruct:free):** Returns content but quality is poor (wrong spans, bad punctuation handling)
- **Groq (llama-3.1-8b-instant):** Works correctly, good quality

The bottleneck is **provider reliability**, not the LLM approach itself. Word alignment and idiom handling genuinely benefit from LLM understanding — this is not a problem that statistical methods solve cleanly.

---

## Recommendations

### Short-term: Fix Groq as Primary

1. **Make Groq the default** (already configured, works well)
2. **Fix null-content handling** in Rust code to handle edge cases from any provider
3. **Improve fallback logic** to log which provider/model succeeded

### Medium-term: Explore Local Fallback

1. **Test quantized OPUS-MT** in staging environment with Fly.io RAM limits
2. If feasible, add as last-resort fallback when all cloud providers fail
3. Cache translations to disk to reduce repeat API calls

### Don't Change: Fundamental Approach

The LLM approach for word alignment is appropriate. The complexity of mapping French tokens to English tokens with spans is genuinely a semantic problem that statistical methods handle poorly. 

What needs fixing:
- Provider reliability (Groq is good)
- Error handling (null content crash)
- Caching (reduce API calls)

---

## Open Questions

1. **RAM feasibility of quantized OPUS-MT:** Can a 75M model fit in 256MB with quantization? Need empirical testing.

2. **Quality on literary French:** OPUS-MT trained on OPUS corpus (open subtitles, documentation). Maupassant's style may differ. Need side-by-side comparison.

3. **Caching strategy:** Should translations be cached by book_id + text_hash? This would reduce API calls significantly for repeat visitors.

---

## Verdict: Are We Solving From the Wrong Angle?

**No.** The LLM approach is sound. The problem is:
1. Free LLM providers are unreliable (GLM burns tokens on thinking)
2. Small models lack quality (liquid 1.2B produces wrong spans)
3. Null-content handling crashes instead of gracefully failing

**Fix the provider strategy, not the approach.**

---

## Sources

1. Helsinki-NLP/opus-mt-fr-en model page: https://huggingface.co/Helsinki-NLP/opus-mt-fr-en
2. OPUS-MT project: https://github.com/Helsinki-NLP/OPUS-MT
3. fast_align word aligner: https://github.com/clab/fast_align