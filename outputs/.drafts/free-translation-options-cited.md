# Free Translation Options for French→English Tokenization

## Executive Summary

The Groq API 400 error stems from `max_tokens: 2048` being insufficient for the tokenization task. This research covers viable free alternatives that support JSON structured output for the French tokenization use case.

**Top Recommendations:**
1. **Gemini API Free Tier** — Best free option with generous limits, JSON structured output support, excellent French comprehension
2. **Ollama (Local)** — Zero cost, privacy-preserving, supports structured outputs via `format` parameter with JSON schema
3. **OpenRouter Free Tier** — Gateway to multiple free models, 50 req/day on free plan

---

## Findings

### 1. Gemini API Free Tier (Recommended)

**Pricing:** Completely free for `gemini-3.5-flash` and `gemini-3.1-flash-lite` on Standard tier [[1]](https://ai.google.dev/gemini-api/docs/pricing).

| Model | Input | Output | Context Window | JSON Mode |
|-------|-------|--------|----------------|-----------|
| `gemini-3.5-flash` | Free | Free | 128K tokens | ✅ Supported |
| `gemini-3.1-flash-lite` | Free | Free | 128K tokens | ✅ Supported |

**Rust/Axum Implementation:**
```rust
// Use gemini-3.5-flash or gemini-3.1-flash-lite
let body = serde_json::json!({
    "contents": [{
        "parts": [{ "text": prompt }]
    }],
    "generationConfig": {
        "responseMimeType": "application/json",
    }
});
```

**Pros:**
- Generous free tier, no time expiration
- Excellent multilingual (French) capabilities
- Native structured output via `responseMimeType: "application/json"`
- 128K context window handles long French passages

**Cons:**
- Requires Google account and API key
- Subject to rate limits on free tier

---

### 2. Ollama (Local, Zero Cost)

**Pricing:** Free (runs on your local machine) [[2]](https://ollama.com/blog/structured-outputs)

Ollama runs LLMs locally with OpenAI-compatible API. Supports structured outputs via `format` parameter with JSON schema definitions.

**Example Request:**
```bash
curl -X POST http://localhost:11434/api/chat -d '{
  "model": "llama3.1:8b",
  "messages": [{"role": "user", "content": "Tokenize: ..."}],
  "stream": false,
  "format": {
    "type": "object",
    "properties": {
      "frTokens": { "type": "array" },
      "enTokens": { "type": "array" }
    },
    "required": ["frTokens", "enTokens"]
  }
}'
```

**Popular Models:**
- `llama3.1:8b` — Good balance of quality/speed
- `qwen3.5:8b` — Strong multilingual, 12M pulls
- `mistral-medium-3.5:128b` — Excellent quality
- `translategemma:12b` — **Purpose-built for translation** (55 languages)

**Pros:**
- Zero API costs (local compute only)
- No rate limits, no internet required
- Privacy-preserving (data stays local)
- OpenAI-compatible API (easy migration)

**Cons:**
- Requires local GPU (4-16GB VRAM for 8B models)
- Setup complexity for non-technical users
- Model quality depends on hardware

---

### 3. OpenRouter Free Tier

**Pricing:** 50 requests/day free, 20 RPM [[3]](https://openrouter.ai/pricing)

**Free Models Available:**
- Various Llama, Mistral, Qwen variants
- 25+ free models, 4 free providers
- JSON structured output supported (see `supported_parameters: structured_outputs`) [[4]](https://openrouter.ai/docs/guides/overview/models)

**Pros:**
- Unified API for 400+ models
- Easy model switching
- Structured outputs support

**Cons:**
- 50 req/day limit is restrictive for production
- Higher latency than direct provider APIs

---

## Current Groq Issue: Root Cause

The 400 error message indicates:
> "max completion tokens reached before generating a valid document"

**Diagnosis:** `max_tokens: 2048` is too low for the tokenization task. The model runs out of completion budget before producing valid JSON.

**Fix Options:**
1. Increase `max_tokens` to 8192 or higher
2. Simplify the prompt to produce shorter output
3. Chunk longer texts into smaller segments
4. Switch to a model with higher `max_completion_tokens` (e.g., `qwen/qwen3-32b` has 40,960 max)

---

## Recommendations

| Priority | Option | Best For |
|----------|--------|----------|
| 1st | **Gemini API** | Quick deployment, best free tier limits |
| 2nd | **Ollama Local** | Zero cost, privacy-sensitive, high volume |
| 3rd | **OpenRouter Free** | Experimentation, model variety |

---

## Open Questions

1. What is the actual French text length causing the 400? (Need to inspect the request payload)
2. Is GPU available for local Ollama deployment?
3. What's the acceptable latency threshold for tokenization?

---

## Sources

[1] Gemini API Pricing — https://ai.google.dev/gemini-api/docs/pricing  
[2] Ollama Structured Outputs Blog — https://ollama.com/blog/structured-outputs  
[3] OpenRouter Pricing — https://openrouter.ai/pricing  
[4] OpenRouter Models Documentation — https://openrouter.ai/docs/guides/overview/models  
[5] Groq Models Documentation — https://console.groq.com/docs/models