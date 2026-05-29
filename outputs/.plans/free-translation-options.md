# Plan: free-translation-options

## Key Questions
1. What free LLMs/APIs support JSON-mode structured output suitable for tokenization?
2. What are the rate limits, token quotas, and usage restrictions?
3. Which models can run locally (offline)?
4. What is the quality/reliability for French→English translation tasks?

## Evidence Needed
- Free tier offerings from major providers (OpenAI, Anthropic, Google, etc.)
- Local inference options (Ollama, LM Studio, etc.)
- Rate limits and JSON mode support
- Benchmark comparisons for translation quality

## Scale Decision
**Narrow topic** — "what is X" explainer. Direct search mode, no researcher subagents. 3-5 search queries to cover: free API tiers, local options, JSON mode support.

## Task Ledger
- [ ] Search: free LLM API tiers with JSON support (OpenAI, Groq alternatives, Cohere, etc.)
- [ ] Search: local inference options (Ollama, LM Studio, llama.cpp)
- [ ] Search: free translation APIs (Google, DeepL free tier, LibreTranslate)
- [ ] Synthesize findings into draft
- [ ] Cite and verify URLs
- [ ] Review and deliver

## Verification Log
- [ ] Check free tier rate limits
- [ ] Verify JSON mode / structured output support
- [ ] Confirm local inference viability
- [ ] Cross-reference quality benchmarks

## Decision Log
- 2026-05-21: Initial plan created. Groq 400 errors on tokenization → exploring free alternatives.