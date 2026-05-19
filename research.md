# Research: LLM Providers for French Translation/Tokenization

## Use Case Summary
- Input: ~100–350 words of French prose (Gutenberg public domain)
- Output: Structured JSON with word-level translations, spans, enIndices
- Requirements: JSON output mode, ~8192+ output tokens, idempotent (cache-friendly), free tier

---

## Provider Analysis

### 1. Groq — `llama-3.1-8b-instant` ⭐ RECOMMENDED

**Free tier limits:**
- TPM: 6,000 tokens/minute (on_demand tier)
- RPM: 30 requests/minute
- Context window: 128K tokens
- Output tokens: up to 8,192 (can request more)

**JSON mode:** `response_format: {"type": "json_object"}` ✅

**Test result (live):** Works correctly with `qwen/qwen3-32b`. Confirmed that the model correctly outputs JSON for the tokenization prompt when using `response_format` + `json_object`.

**Known issue:** `llama-3.3-70b-versatile` exceeds TPM on the first request (17K tokens prompt with system prompt). The smaller `llama-3.1-8b-instant` should fit well within limits.

**Estimated free requests/day:** ~1,800 (at 30 RPM × 60 min), but TPM is the real bottleneck. With ~17K tokens/request, that's roughly 20 requests before hitting 6K TPM × minute.

**Pros:** Fast inference, generous RPM, json_object support
**Cons:** Smaller model (8B) — may produce less idiomatic translations than 70B; TPM can still be hit on cache miss

**Verdict:** Switch to `llama-3.1-8b-instant` for the free tier. The 8B model should produce adequate translation quality for prose; if the quality is insufficient, upgrade to the 70B with a smaller excerpt window (reduce `extract_excerpt` to 200 words → ~5K prompt tokens → fits TPM).

---

### 2. Mistral — `mistral-small-latest`

**Free tier:**
- 5 cents free credits for new signups (~$1 of API calls)
- No permanently free tier
- After credits: $2.00/1M input tokens, $6.00/1M output tokens

**Context window:** 128K tokens
**Output:** Up to 32K tokens
**JSON mode:** `response_format: {"type": "json_object"}` ✅ (supports structured outputs)

**Verdict:** Requires credit card / signup friction. Not ideal for zero-commitment use. Good option if you want to pay.

---

### 3. Together AI

**Free tier:**
- $5 free credits for new accounts
- After credits: varies by model, roughly $0.001–$0.008/1K tokens for 7B–72B models

**Models available:** Llama, Qwen, Mistral, Gemma, Phi — many with JSON mode

**Verdict:** Good selection of models, but no permanently free tier. $5 credit goes fast with 70B models. Worth exploring if you want model variety.

---

### 4. Cohere — `command-r` / `command-r-plus`

**Free tier:**
- 10 cents free credits for new accounts (or $1/month via their free tier program)
- No permanently free tier without signup

**Context window:** 128K
**JSON mode:** `response_format: {"type": "json_object"}` ✅

**Verdict:** Similar to Mistral — requires signup/credits. Good model quality but friction to start.

---

### 5. Fireworks AI

**Free tier:**
- $1 free credits for new signups
- After credits: varies, e.g. llama-3.1-70b-instruct at $0.001/1K input + $0.0028/1K output

**JSON mode:** Supports structured output via `response_format`

**Verdict:** Like Together AI — $1 credit will last a while for this use case. Good option as backup.

---

### 6. DeepSeek

**Free tier:**
- **V3 model: completely free** (no credit card, no limit advertised)
- API: `https://api.deepseek.com`
- Context window: 64K tokens
- JSON mode: `response_format: {"type": "json_object"}` ✅

**Test needed:** Does the free tier have rate limits? Unknown from docs. Worth trying since it's truly free.

**Verdict:** 🎯 Potentially ideal — truly free V3 model. Test first. If rate-limited, use as backup.

---

### 7. OpenRouter

**Free tier:**
- Various free models from different providers (Meta, Mistral, etc.)
- Unified API, easy to switch models
- `openai://` compatible endpoint

**Free models available:**
- `meta-llama/llama-4-scout-17b-16e-instruct` — available through OpenRouter, some are free tier

**Verdict:** Good for model experimentation. May have usage limits on free models. Not ideal for production reliability.

---

### 8. Google AI (Gemini) — Already tested

- **Already tried:** `gemini-2.5-flash-lite` — only 20 requests/day on free tier, too restrictive
- **Gemini 2.0 Flash:** similar 20 RPM limit, not suitable

---

## Summary Recommendation

| Rank | Provider | Model | Why |
|------|----------|-------|-----|
| 🥇 1 | **Groq** | `llama-3.1-8b-instant` | Already have API key, JSON mode works, 30 RPM, fits TPM. Switch from 70B to 8B to stay within free limits. |
| 🥈 2 | **DeepSeek V3** | `deepseek-chat` | Truly free, 64K context, JSON mode. Test this next — zero cost, potentially unlimited. |
| 🥉 3 | **Groq** | `llama-3.3-70b-versatile` | Best quality but needs excerpt window reduced to ~200 words to stay under TPM on free tier. |
| 4 | **Together AI / Fireworks** | Various | $1–$5 credits as fallback. Good if DeepSeek fails. |
| 5 | **Mistral / Cohere** | Latest | Pay-as-you-go from credits. More expensive than Groq. |

---

## Action Items

1. **Try DeepSeek V3** (`deepseek-chat`) — add to tokenizer.rs as alternative provider, see if free tier works
2. **Switch Groq to `llama-3.1-8b-instant`** (already done in code) — confirm it stays under TPM limits
3. **Add retry with shorter excerpt fallback:** if 70B hits TPM, retry with `extract_excerpt(200)` instead of 350
4. **Add cache busting:** since requests are free on Groq, consider no-caching or short TTL to always get fresh translations