import { createResource, createSignal, For, Show } from "solid-js";

interface FrToken {
  text: string;
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
}

async function fetchArticle(): Promise<ArticleData> {
  const res = await fetch("/api/article-of-the-day");
  if (!res.ok) throw new Error(`Failed to fetch article: ${res.status}`);
  return res.json();
}

export default function ArticleOfTheDay() {
  const [article] = createResource(fetchArticle);
  const [activeIndices, setActiveIndices] = createSignal<Set<number>>(new Set());

  function setActiveIndicesFrom(enIndices: number[]) {
    // Toggle: if all clicked indices are already active, clear them
    const allActive = enIndices.every(i => activeIndices().has(i));
    if (allActive) {
      setActiveIndices(new Set());
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

  // Filter tokens that belong to a paragraph
  // Include tokens that START within the paragraph (even if they span into next paragraph)
  function getParaTokens(tokens: FrToken[], paras: string[], paraIndex: number): FrToken[] {
    const start = getParaOffset(paras, paraIndex);
    const end = start + paras[paraIndex].length;
    return tokens.filter(t => {
      const [s] = t.spans[0];
      return s >= start && s < end;
    });
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

          const combinedText = () => paras().join("\n\n");

          return (
            <>
              <header class="article-header">
                <h1>{data().title}</h1>
                <p class="article-meta">
                  {data().source} ({data().published_year})
                </p>
              </header>

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
                        const paraTokens = () => getParaTokens(tokens(), paras(), i());
                        return (
                          <p class="article-paragraphs">
                            <For each={paraTokens()}>
                              {(t) => (
                                <span
                                  class={`fr-token${isActive(t.en_indices[0]) ? " active" : ""}`}
                                  onClick={() => setActiveIndicesFrom(t.en_indices)}
                                  data-trans={t.translation}
                                >
                                  {combinedText().slice(...t.spans[0])}{" "}
                                </span>
                              )}
                            </For>
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
            </>
          );
        }}
      </Show>
    </div>
  );
}