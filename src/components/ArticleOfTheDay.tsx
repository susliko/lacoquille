import { createResource, createSignal, For, Show } from "solid-js";

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

async function fetchArticle(): Promise<ArticleData> {
  const res = await fetch("/api/article-of-the-day");
  if (!res.ok) throw new Error(`Failed to fetch article: ${res.status}`);
  return res.json();
}

export default function ArticleOfTheDay() {
  const [article] = createResource(fetchArticle);
  const [activeIndices, setActiveIndices] = createSignal<Set<number>>(new Set());
  const [tooltipContent, setTooltipContent] = createSignal<string | null>(null);
  const [tooltipPos, setTooltipPos] = createSignal<{ x: number; y: number } | null>(null);

  function setActiveIndicesFrom(enIndices: number[]) {
    const allActive = enIndices.every(i => activeIndices().has(i));
    if (allActive) {
      setActiveIndices(new Set<number>());
    } else {
      setActiveIndices(new Set(enIndices));
    }
  }

  function isActive(enIdx: number): boolean {
    return activeIndices().has(enIdx);
  }

  // Get combined text offset for a paragraph
  function getParaOffset(paras: string[], paraIndex: number): number {
    let offset = 0;
    for (let i = 0; i < paraIndex; i++) {
      offset += paras[i].length + 2; // +2 for "\n\n" separator
    }
    return offset;
  }

  // Find token that contains a character position (absolute in combined text)
  function findTokenAtPosition(tokens: FrToken[], position: number): FrToken | undefined {
    for (const token of tokens) {
      const [start, end] = token.spans[0];
      if (position >= start && position < end) {
        return token;
      }
    }
    return undefined;
  }

  // Get en_token indices for a paragraph
  function getParaEnIndices(tokens: FrToken[], paras: string[], paraIndex: number): number[] {
    const start = getParaOffset(paras, paraIndex);
    const end = start + paras[paraIndex].length;
    const indices = new Set<number>();
    for (const t of tokens) {
      const [s] = t.spans[0];
      if (s >= start && s < end) {
        t.en_indices.forEach(i => indices.add(i));
      }
    }
    return Array.from(indices).sort((a, b) => a - b);
  }

  // Find token for a word in the paragraph (matches by text content)
  function findTokenForWord(
    tokens: FrToken[],
    paraOffset: number,
    word: string,
    wordStartInPara: number
  ): FrToken | undefined {
    const absStart = paraOffset + wordStartInPara;
    const absEnd = absStart + word.length;

    for (const token of tokens) {
      const [tokenStart, tokenEnd] = token.spans[0];
      // Match if token spans overlap with the word position
      // and the token text matches the word (ignoring punctuation)
      const tokenText = token.text;
      if (tokenText === word &&
          tokenStart >= paraOffset &&
          tokenStart < paraOffset + tokens[0].spans[0][0] + 1000) { // rough para end
        return token;
      }
    }
    // Fallback: find by position
    return findTokenAtPosition(tokens, absStart);
  }

  // Render paragraph by splitting into words and whitespace, wrapping words with tokens
  function renderParagraph(
    para: string,
    paraOffset: number,
    tokens: FrToken[]
  ): any {
    // Split into words and whitespace (preserve whitespace)
    const parts = para.split(/(\s+)/);
    let posInPara = 0;

    return parts.map((part, partIdx) => {
      if (part.match(/^\s+$/)) {
        // Whitespace - just return it and advance position
        posInPara += part.length;
        return part;
      }

      // Word - find token for it
      const absPos = paraOffset + posInPara;
      const token = findTokenAtPosition(tokens, absPos);

      posInPara += part.length;

      if (token && token.text === part) {
        return (
          <span
            class={`fr-token${isActive(token.en_indices[0]) ? " active" : ""}`}
            onClick={(e) => {
              setActiveIndicesFrom(token.en_indices);
              setTooltipContent(token.translation);
              setTooltipPos({ x: e.clientX, y: e.clientY });
            }}
          >
            {part}
          </span>
        );
      }

      // No token found - render plain text
      return part;
    });
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
          const enTokens = () => data().tokenized?.en_tokens ?? [];
          const paras = () => data().paragraphs;

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
                        const paraOffset = () => getParaOffset(paras(), i());
                        return (
                          <p class="article-paragraphs">
                            {renderParagraph(para, paraOffset(), tokens())}
                          </p>
                        );
                      }}
                    </For>
                  </div>

                  <div class="language-col en-col">
                    <h2>English</h2>
                    <For each={paras()}>
                      {(_, i) => {
                        const enIndices = () => getParaEnIndices(tokens(), paras(), i());
                        return (
                          <p class="article-paragraphs">
                            <For each={enIndices()}>
                              {(idx) => {
                                const enTok = enTokens().find(e => e.index === idx);
                                return enTok ? (
                                  <span
                                    class={`en-token${isActive(idx) ? " active" : ""}`}
                                    onClick={() => setActiveIndicesFrom([idx])}
                                    title={enTok.text}
                                  >
                                    {enTok.text}{" "}
                                  </span>
                                ) : null;
                              }}
                            </For>
                          </p>
                        );
                      }}
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