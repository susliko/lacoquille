import { createSignal, createResource, For, Show } from "solid-js";

interface VocabHighlight {
  word: string;
  paragraph_index: number;
  start_offset: number;
  end_offset: number;
}

interface ArticleData {
  story: {
    id: string;
    title: string;
    source: string;
    published_year: number;
  };
  french: {
    paragraphs: string[];
    vocab_highlights: VocabHighlight[];
  };
  english: {
    paragraphs: string[];
    source: string;
  } | null;
  date: string;
}

async function fetchArticle(): Promise<ArticleData> {
  const res = await fetch('http://localhost:8080/api/article-of-the-day');
  if (!res.ok) throw new Error(`Failed to fetch article: ${res.status}`);
  return res.json();
}

type ViewMode = 'fr' | 'en' | 'side';

export default function ArticleOfTheDay() {
  const [mode, setMode] = createSignal<ViewMode>('side');
  const [article] = createResource(fetchArticle);

  const highlightWords = (text: string, paragraphIndex: number) => {
    const data = article();
    if (!data) return text;
    const highlights = data.french.vocab_highlights.filter(
      h => h.paragraph_index === paragraphIndex
    );
    if (highlights.length === 0) return text;
    let result = text;
    const offsets = highlights.map(h => ({
      start: h.start_offset,
      end: h.end_offset,
      word: h.word
    }));
    offsets.sort((a, b) => b.start - a.start);
    for (const offset of offsets) {
      result = result.slice(0, offset.start) +
        `<span class="vocab-highlight">${result.slice(offset.start, offset.end)}</span>` +
        result.slice(offset.end);
    }
    return result;
  };

  return (
    <div class="article-of-the-day">
      <style>{`
        .article-of-the-day {
          max-width: 1100px;
          margin: 0 auto;
          padding: 1.5rem 2rem;
        }
        .article-header {
          margin-bottom: 2rem;
          border-bottom: 1px solid var(--border);
          padding-bottom: 1.5rem;
        }
        .article-header h1 {
          font-size: 2rem;
          margin: 0 0 0.5rem 0;
        }
        .article-meta {
          color: var(--text-2);
          font-size: 1rem;
        }
        .mode-toggle {
          display: flex;
          gap: 0.5rem;
          margin-bottom: 2rem;
          flex-wrap: wrap;
        }
        .mode-toggle button {
          padding: 0.5rem 1.25rem;
          border: 1px solid var(--border);
          background: var(--surface-2);
          color: var(--text-2);
          border-radius: 6px;
          cursor: pointer;
          font-size: 0.9rem;
        }
        .mode-toggle button.active {
          background: var(--primary);
          color: var(--primary-fg);
          border-color: var(--primary);
        }
        .article-body {
          display: grid;
          gap: 2rem;
        }
        .article-body.side-by-side {
          grid-template-columns: 1fr 1fr;
        }
        .article-column h2 {
          font-size: 0.875rem;
          color: var(--text-2);
          margin-bottom: 1rem;
          text-transform: uppercase;
          letter-spacing: 0.08em;
        }
        .article-column p {
          line-height: 2;
          margin-bottom: 1.25rem;
          font-size: 1.1rem;
        }
        .vocab-highlight {
          background: var(--highlight);
          border-radius: 2px;
          padding: 0 2px;
          cursor: default;
        }
        .loading-state, .error-state {
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
          Unable to load story. Make sure the lacq server is running on port 8080.
        </div>
      </Show>

      <Show when={article()}>
        {() => {
          const data = article()!;
          return (
            <>
              <div class="article-header">
                <h1>{data.story.title}</h1>
                <p class="article-meta">
                  {data.story.source} ({data.story.published_year}) · {data.date}
                </p>
              </div>

              <div class="mode-toggle">
                <button
                  class={mode() === 'fr' ? 'active' : ''}
                  onClick={() => setMode('fr')}
                >
                  Français
                </button>
                <button
                  class={mode() === 'en' ? 'active' : ''}
                  onClick={() => setMode('en')}
                >
                  English
                </button>
                <button
                  class={mode() === 'side' ? 'active' : ''}
                  onClick={() => setMode('side')}
                >
                  Side by side
                </button>
              </div>

              <div class={`article-body ${mode() === 'side' ? 'side-by-side' : ''}`}>
                <Show when={mode() === 'fr' || mode() === 'side'}>
                  <div class="article-column">
                    <h2>Français</h2>
                    <For each={data.french.paragraphs}>
                      {(para, i) => (
                        <p innerHTML={highlightWords(para, i())} />
                      )}
                    </For>
                  </div>
                </Show>

                <Show when={(mode() === 'en' || mode() === 'side') && data.english}>
                  <div class="article-column">
                    <h2>English</h2>
                    <For each={data.english!.paragraphs}>
                      {(para) => <p>{para}</p>}
                    </For>
                  </div>
                </Show>
              </div>
            </>
          );
        }}
      </Show>
    </div>
  );
}
