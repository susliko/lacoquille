import { createMemo, createResource, createSignal, For, Show } from "solid-js";

interface FrToken {
  text: string;
  trailingPunct?: string | null;
  leadingPunct?: string | null;
  translation: string;
  spans: [number, number][];
  en_indices: number[];
}

interface EnToken {
  text: string;
  index: number;
}

interface Tokenized {
  fr_tokens: FrToken[];
  en_tokens: EnToken[];
}

interface ArticleData {
  title: string;
  source: string;
  published_year: number;
  paragraphs: string[];
  tokenized?: Tokenized;
  tokenization_error?: string;
}

// The article is computed by a background job on the server. On a cold start
// the endpoint returns 503 until the first computation finishes, so we poll.
async function fetchArticle(): Promise<ArticleData> {
  const maxAttempts = 30;
  for (let attempt = 0; attempt < maxAttempts; attempt++) {
    const res = await fetch("/api/article-of-the-day");
    if (res.ok) return res.json();
    if (res.status === 503) {
      await new Promise((r) => setTimeout(r, 2000));
      continue;
    }
    throw new Error(`Failed to fetch article: ${res.status}`);
  }
  throw new Error("Today's story is still being prepared. Please refresh in a moment.");
}

// Character offset of paragraph `i` within `paragraphs.join("\n\n")` — the same
// text the backend tokenized, so token char-spans index into it.
function paraStart(paras: string[], i: number): number {
  let offset = 0;
  for (let k = 0; k < i; k++) offset += paras[k].length + 2; // +2 for "\n\n"
  return offset;
}

type FrPart = { text: string; gi?: number };

// Render a paragraph as its original text with token spans overlaid as
// clickable regions. Text in the gaps between tokens (punctuation, whitespace)
// is emitted verbatim, so nothing — commas, periods, dashes — is ever dropped,
// regardless of how the model populated trailingPunct/leadingPunct.
function buildFrParts(
  paraText: string,
  paraOffset: number,
  tokenIdxs: number[],
  tokens: FrToken[],
): FrPart[] {
  const parts: FrPart[] = [];
  let cursor = 0;
  for (const gi of tokenIdxs) {
    const span = tokens[gi]?.spans?.[0];
    if (!span) continue;
    const s = span[0] - paraOffset;
    const e = Math.min(span[1] - paraOffset, paraText.length);
    // Skip spans that fall outside this paragraph or overlap an earlier token.
    if (s < cursor || s >= paraText.length || e <= s) continue;
    if (s > cursor) parts.push({ text: paraText.slice(cursor, s) });
    parts.push({ text: paraText.slice(s, e), gi });
    cursor = e;
  }
  if (cursor < paraText.length) parts.push({ text: paraText.slice(cursor) });
  return parts;
}

// Assign each token (by its index in fr_tokens) to a paragraph, using the
// character span the backend computed against `paragraphs.join("\n\n")`.
// Tokens stay in their original order; if a span can't be matched we carry the
// token into the current paragraph so nothing is ever dropped.
function groupTokensByParagraph(tokens: FrToken[], paras: string[]): number[][] {
  const groups: number[][] = paras.map(() => []);
  if (paras.length === 0) return groups;

  const ranges: [number, number][] = [];
  let offset = 0;
  for (const p of paras) {
    const start = offset;
    const end = offset + p.length;
    ranges.push([start, end]);
    offset = end + 2; // "\n\n" separator
  }

  let current = 0;
  tokens.forEach((tok, gi) => {
    const pos = tok.spans?.[0]?.[0];
    if (typeof pos === "number") {
      const found = ranges.findIndex(([s, e]) => pos >= s && pos < e);
      if (found !== -1) current = found;
    }
    groups[current].push(gi);
  });
  return groups;
}

export default function ArticleOfTheDay() {
  const [article] = createResource(fetchArticle);
  // The single currently-highlighted token (index into fr_tokens), or null.
  const [activeIdx, setActiveIdx] = createSignal<number | null>(null);
  const [tooltipContent, setTooltipContent] = createSignal<string | null>(null);
  const [tooltipPos, setTooltipPos] = createSignal<{ x: number; y: number } | null>(null);

  function toggle(idx: number) {
    setActiveIdx((prev) => (prev === idx ? null : idx));
  }

  return (
    <div class="article-of-the-day">
      <style>{`
        .article-of-the-day {
          padding: 1.5rem 2rem;
          max-width: 1400px;
          margin: 0 auto;
        }
        .article-header {
          margin-bottom: 2rem;
          padding-bottom: 1.5rem;
          border-bottom: 1px solid var(--border);
        }
        .article-header h1 {
          font-size: 2rem;
          margin: 0 0 0.5rem 0;
        }
        .article-meta {
          color: var(--text-2);
          font-size: 1rem;
          margin: 0;
        }
        .tokenization-error {
          background: var(--warning, #fef3c7);
          color: var(--warning-text, #92400e);
          padding: 0.75rem 1rem;
          border-radius: 0.5rem;
          margin-bottom: 1.5rem;
          font-size: 0.9rem;
        }
        .article-body {
          display: grid;
          grid-template-columns: 1fr 1fr;
          gap: 2rem;
          align-items: start;
        }
        .language-col {
          min-width: 0;
        }
        .article-paragraphs p {
          line-height: 1.8;
          margin-bottom: 1.5rem;
          font-size: 1.1rem;
          white-space: pre-wrap;
        }
        .language-col h2 {
          font-size: 1.25rem;
          font-weight: 600;
          margin: 0 0 1rem 0;
          text-transform: uppercase;
          letter-spacing: 0.05em;
          color: var(--text-2);
        }
        .token {
          cursor: pointer;
          padding: 0 1px;
          border-radius: 2px;
        }
        .token.active {
          background-color: rgba(59, 130, 246, 0.2);
          outline: 2px solid var(--accent, #3b82f6);
          outline-offset: 1px;
        }
        .en-token {
          cursor: pointer;
          padding: 0 1px;
        }
        .en-token.active {
          background-color: rgba(59, 130, 246, 0.2);
        }
        .fr-token {
          cursor: pointer;
          padding: 0 1px;
          position: relative;
        }
        .fr-token.active {
          background-color: rgba(59, 130, 246, 0.2);
        }
        @media (max-width: 768px) {
          .article-body {
            grid-template-columns: 1fr;
          }
          .en-col {
            display: none;
          }
          /* Mobile popup tooltip */
          .fr-token.active::after {
            content: attr(data-trans);
            display: block;
            position: absolute;
            bottom: calc(100% + 4px);
            left: 50%;
            transform: translateX(-50%);
            background: var(--accent, #3b82f6);
            color: white;
            padding: 4px 8px;
            border-radius: 4px;
            font-size: 0.8rem;
            z-index: 100;
            pointer-events: none;
            max-width: 200px;
            white-space: normal;
            text-align: center;
            box-shadow: 0 2px 8px rgba(0,0,0,0.2);
          }
        }
        .loading-state,
        .error-state {
          text-align: center;
          padding: 3rem;
          color: var(--text-2);
        }
        .error-state {
          color: var(--error);
        }
        .tooltip {
          position: fixed;
          background: var(--accent, #3b82f6);
          color: white;
          padding: 6px 10px;
          border-radius: 6px;
          font-size: 0.9rem;
          z-index: 1000;
          pointer-events: none;
          max-width: 250px;
          white-space: normal;
          box-shadow: 0 4px 12px rgba(0,0,0,0.3);
          transform: translate(-50%, -100%);
          margin-top: -8px;
        }
        .tooltip::after {
          content: '';
          position: absolute;
          top: 100%;
          left: 50%;
          transform: translateX(-50%);
          border: 6px solid transparent;
          border-top-color: var(--accent, #3b82f6);
        }
      `}</style>

      <Show when={article.loading}>
        <div class="loading-state">Loading today's story...</div>
      </Show>

      <Show when={article.error}>
        <div class="error-state">
          Unable to load today's story. Make sure the lacq server is running on port 8080.
          <br />
          <small>{article.error?.message}</small>
        </div>
      </Show>

      <Show when={article()}>
        {(data) => {
          const hasTokens = () => !!data().tokenized?.fr_tokens?.length;
          const tokens = () => data().tokenized?.fr_tokens ?? [];
          const paras = () => data().paragraphs;
          // Token indices grouped per paragraph. Both language columns render
          // from this same grouping so French words and their English
          // translations stay aligned 1:1.
          const groups = createMemo(() => groupTokensByParagraph(tokens(), paras()));

          return (
            <>
              <header class="article-header">
                <h1>{data().title}</h1>
                <p class="article-meta">
                  {data().source} ({data().published_year})
                </p>
              </header>

              <Show when={data().tokenization_error}>
                <div class="tokenization-error">
                  {data().tokenization_error}
                </div>
              </Show>

              <Show
                when={hasTokens()}
                fallback={
                  <div class="article-body">
                    <div class="language-col">
                      <h2>Français</h2>
                      <div class="article-paragraphs">
                        <For each={paras()}>
                          {(para) => <p>{para}</p>}
                        </For>
                      </div>
                    </div>
                    <div class="language-col">
                      <h2>English</h2>
                      <div class="article-paragraphs">
                        <For each={paras()}>
                          {() => <p>[No translations available]</p>}
                        </For>
                      </div>
                    </div>
                  </div>
                }
              >
                <div class="article-body">
                  <div class="language-col">
                    <h2>Français</h2>
                    <For each={paras()}>
                      {(para, i) => {
                        const parts = () =>
                          buildFrParts(para, paraStart(paras(), i()), groups()[i()] ?? [], tokens());
                        return (
                          <p class="article-paragraphs">
                            <For each={parts()}>
                              {(part) =>
                                part.gi !== undefined ? (
                                  <span
                                    class={`fr-token${activeIdx() === part.gi ? " active" : ""}`}
                                    data-trans={tokens()[part.gi].translation}
                                    onClick={(e) => {
                                      toggle(part.gi!);
                                      setTooltipContent(tokens()[part.gi!].translation);
                                      setTooltipPos({ x: e.clientX, y: e.clientY });
                                    }}
                                  >
                                    {part.text}
                                  </span>
                                ) : (
                                  <>{part.text}</>
                                )
                              }
                            </For>
                          </p>
                        );
                      }}
                    </For>
                  </div>

                  <div class="language-col en-col">
                    <h2>English</h2>
                    <For each={paras()}>
                      {(_, i) => (
                        <p class="article-paragraphs">
                          <For each={groups()[i()] ?? []}>
                            {(gi) => {
                              const tok = tokens()[gi];
                              return (
                                <>
                                  <span
                                    class={`en-token${activeIdx() === gi ? " active" : ""}`}
                                    onClick={() => toggle(gi)}
                                    title={tok.text}
                                  >
                                    {tok.translation}
                                  </span>{" "}
                                </>
                              );
                            }}
                          </For>
                        </p>
                      )}
                    </For>
                  </div>
                </div>
              </Show>

              <Show when={tooltipContent()}>
                <div
                  class="tooltip"
                  style={{
                    left: `${tooltipPos()?.x ?? 0}px`,
                    top: `${tooltipPos()?.y ?? 0}px`
                  }}
                >
                  {tooltipContent()}
                </div>
              </Show>
            </>
          );
        }}
      </Show>
    </div>
  );
}