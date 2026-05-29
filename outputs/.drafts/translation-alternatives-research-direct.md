# Translation Alternatives Research Notes

## Search Terms Used

1. "opus-mt-fr-en quality literary French benchmarks"
2. "fast_align word alignment tool usage"
3. "DeepL API free tier limits alternatives"
4. "local neural machine translation small models"

---

## Findings

### 1. OPUS-MT (Helsinki-NLP/opus-mt-fr-en)

**Model specs:**
- 75.2M parameters
- Transformer-based seq2seq
- Trained on OPUS corpus
- BLEU scores: 26-38 on news, 57.5 on Tatoeba (simple sentences)

**Strengths:**
- Runs locally, no API key needed
- Pre-trained word alignments available (guided alignment using eflomal)
- Sentence-level translation is solid
- HuggingFace transformers compatible

**Weaknesses:**
- 75M params = ~300MB+ in float32 (tight for 256MB Fly.io RAM)
- No built-in word alignment at inference time
- Literary French may be weaker than news domain
- Quantized versions exist but quality loss unclear

**Feasibility for 256MB RAM:** Borderline. Model alone ~300MB, plus tokenizer and runtime overhead.

### 2. Word Alignment Tools

**fast_align:**
- Unsupervised IBM Model 2 implementation
- Takes parallel corpus as input (source ||| target format)
- Outputs Pharaoh format alignments (i-j pairs)
- Symmetrization with atools
- Good for batch processing, not real-time

**simalign:**
- Modern alternative to fast_align
- Uses pre-trained multilingual embeddings
- Better accuracy than fast_align
- Python library

**Approach possibility:**
1. Translate French → English with OPUS-MT
2. Align with fast_align on parallel output
3. Map alignments back to original tokens

**Problem:** Requires training data or additional model for embeddings.

### 3. DeepL API

**Free tier:**
- Not available for API
- Free web version has usage limits
- Pro API starts at $25/month

**Not viable** for production without budget.

### 4. Other Options Considered

**LibreTranslate:**
- Self-hosted, open source
- Argos Translate backend
- Could run on separate server

**Google Translate API:**
- $20/million characters (paid)

**WordReference API:**
- Dictionary only, not sentence translation

---

## Key Insight: The Real Problem

Looking at our requirements again:

1. **Tokenization** → regex can handle this (already partially implemented with `split_tokens_with_punct`)
2. **Translation** → sentence-level MT works, word-level is harder
3. **Word alignment** → needed to map French tokens to English tokens

The LLM approach tries to solve all three at once. A pipeline approach could:

1. **Split French into tokens** (rule-based, no LLM)
2. **Translate each token** (LLM or dictionary for multi-word)
3. **Align translations back to original** (span tracking)

Or:

1. **Translate full sentence** (OPUS-MT or Groq)
2. **Split both source and translation into words** (regex)
3. **Align words** (fast_align or statistical alignment)

---

## Constraints

- Fly.io: 256MB RAM, no GPU
- Need character spans for interactive highlighting
- User-facing: can't show obvious MT artifacts
- Free tier: no paid API calls

---

## Possible Approaches

### Approach A: Local OPUS-MT + fast_align
- Run OPUS-MT (75M) on server
- Use fast_align for word alignment
- Pros: free, no API calls after model download
- Cons: RAM heavy, alignment complexity

### Approach B: Groq only (already working)
- Use Groq for translation
- Improve prompt engineering for tokenization
- Pros: already in place, quality good
- Cons: depends on Groq free tier stability

### Approach C: Hybrid
- Pre-translate with OPUS-MT (cached)
- Use Groq for word alignment / refinement
- Pros: reduces LLM calls
- Cons: complex pipeline

### Approach D: Dictionary + Rules
- Use word-by-word dictionary for common words
- LLM only for idioms/expressions
- Pros: fast, predictable
- Cons: quality issues for literary French

---

## Recommendation

The LLM approach is NOT wrong — it's appropriate for:
- Word alignment (hard problem)
- Handling idioms and non-compositional expressions
- Generating `en_indices` correctly

**The real problem is the free LLM quality**, not the approach.

Options:
1. **Use Groq as primary** (already works, good quality)
2. **Add OpenRouter as backup** but only with models that don't use thinking (liquid/lfm-2.5-1.2b-instruct works but quality is poor)
3. **Investigate quantized OPUS-MT** for local fallback (needs more research on RAM feasibility)