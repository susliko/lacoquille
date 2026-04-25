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
}

interface StorySummary {
  id: string;
  title: string;
  source: string;
  published_year: number;
  level: string;
}

async function fetchArticle(storyId: string): Promise<ArticleData> {
  const res = await fetch(`/api/stories/${storyId}`);
  if (!res.ok) throw new Error(`Failed to fetch article: ${res.status}`);
  return res.json();
}

async function fetchStories(): Promise<StorySummary[]> {
  const res = await fetch(`/api/stories`);
  if (!res.ok) throw new Error(`Failed to fetch stories: ${res.status}`);
  return res.json();
}

// Mock translation dictionary — ~20 common French words
const mockTranslations: Record<string, string> = {
  "Le": "the",
  "la": "the",
  "les": "the",
  "et": "and",
  "est": "is",
  "un": "a / one",
  "une": "a / one",
  "des": "some",
  "dans": "in",
  "pour": "for",
  "avec": "with",
  "sur": "on",
  "qui": "who",
  "que": "that / what",
  "il": "he",
  "elle": "she",
  "ne": "not",
  "pas": "not",
  "mais": "but",
  "par": "by / through",
  "se": "oneself",
  "en": "in / of / on",
  "dont": "whose",
  "tout": "all / everything",
  "sa": "his / her",
  "son": "his / her",
  "ses": "his / her (pl)",
};

export default function VocabMining() {
  const [stories] = createResource(fetchStories);
  const [article] = createResource(
    () => stories()?.[0]?.id,
    (storyId) => fetchArticle(storyId)
  );
  const [selectedStoryId, setSelectedStoryId] = createSignal<string | null>(null);

  // All highlighted words are "mined" — all included by default
  const [includedIndices, setIncludedIndices] = createSignal<Set<number>>(new Set());

  const initializeIncluded = (highlights: VocabHighlight[]) => {
    setIncludedIndices(new Set(highlights.map((_, i) => i)));
  };

  // Re-initialize included when article changes
  const highlights = () => article()?.french.vocab_highlights ?? [];
  const paragraphs = () => article()?.french.paragraphs ?? [];

  const toggleWord = (index: number) => {
    setIncludedIndices(prev => {
      const next = new Set(prev);
      if (next.has(index)) {
        next.delete(index);
      } else {
        next.add(index);
      }
      return next;
    });
  };

  const exportCSV = () => {
    const data = article();
    if (!data) return;

    const rows: string[][] = [["deck", "front", "back"]];
    highlights().forEach((h, i) => {
      if (!includedIndices().has(i)) return;
      const translation = mockTranslations[h.word] ?? "";
      rows.push([
        "Lacoquille Vocab",
        h.word,
        translation || "see sentence",
      ]);
    });

    const csv = rows.map(r => r.map(c => `"${c.replace(/"/g, '""')}"`).join(",")).join("\n");
    const blob = new Blob([csv], { type: "text/csv" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `lacoquille-vocab-${new Date().toISOString().split('T')[0]}.csv`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const getSentence = (paragraphIndex: number): string => {
    const para = paragraphs()[paragraphIndex];
    return para ?? "";
  };

  const highlightText = (text: string, paragraphIndex: number) => {
    const data = article();
    if (!data) return text;
    const hs = data.french.vocab_highlights.filter(h => h.paragraph_index === paragraphIndex);
    if (hs.length === 0) return text;
    // Sort by start_offset descending so we can slice safely
    const sorted = [...hs].sort((a, b) => b.start_offset - a.start_offset);
    let result = text;
    for (const h of sorted) {
      result =
        result.slice(0, h.start_offset) +
        `<span class="vm-highlight">${result.slice(h.start_offset, h.end_offset)}</span>` +
        result.slice(h.end_offset);
    }
    return result;
  };

  return (
    <div class="vocab-mining">
      <style>{`
        .vocab-mining {
          max-width: 1200px;
          margin: 0 auto;
          padding: 1.5rem 2rem;
        }

        .vm-header {
          margin-bottom: 2rem;
          border-bottom: 1px solid var(--border);
          padding-bottom: 1.5rem;
        }
        .vm-header h1 {
          font-size: 2rem;
          margin: 0 0 0.5rem 0;
        }
        .vm-header p {
          color: var(--text-2);
          font-size: 1rem;
          margin: 0;
        }

        .vm-story-picker {
          margin-bottom: 1.5rem;
        }
        .vm-story-picker label {
          display: block;
          font-size: 0.75rem;
          font-weight: 500;
          letter-spacing: 0.08em;
          text-transform: uppercase;
          color: var(--text-muted);
          margin-bottom: 0.5rem;
        }
        .vm-story-select {
          background: var(--surface-2);
          border: 1px solid var(--border);
          border-radius: var(--radius-sm);
          color: var(--text);
          font-family: var(--font-body);
          font-size: 0.875rem;
          padding: 0.4rem 0.6rem;
          cursor: pointer;
          outline: none;
          transition: border-color var(--transition);
        }
        .vm-story-select:hover,
        .vm-story-select:focus {
          border-color: var(--border-bright);
        }

        .vm-layout {
          display: grid;
          grid-template-columns: 1fr 320px;
          gap: 2rem;
          align-items: start;
        }

        @media (max-width: 768px) {
          .vm-layout {
            grid-template-columns: 1fr;
          }
          .vm-sidebar {
            order: -1;
          }
        }

        /* Article panel */
        .vm-article {
          background: var(--surface);
          border: 1px solid var(--border-subtle);
          border-radius: var(--radius);
          padding: 1.5rem 2rem;
        }
        .vm-article h2 {
          font-size: 0.75rem;
          font-weight: 500;
          letter-spacing: 0.1em;
          text-transform: uppercase;
          color: var(--text-muted);
          margin: 0 0 1rem 0;
        }
        .vm-article p {
          line-height: 2;
          margin-bottom: 1.25rem;
          font-size: 1.1rem;
        }
        .vm-highlight {
          background: var(--amber-soft);
          border-radius: 2px;
          padding: 0 2px;
          cursor: default;
        }

        /* Sidebar */
        .vm-sidebar {
          position: sticky;
          top: 72px;
        }
        .vm-sidebar-header {
          display: flex;
          align-items: center;
          justify-content: space-between;
          margin-bottom: 1rem;
        }
        .vm-sidebar h2 {
          font-size: 0.75rem;
          font-weight: 500;
          letter-spacing: 0.1em;
          text-transform: uppercase;
          color: var(--text-muted);
          margin: 0;
        }
        .vm-export-btn {
          padding: 0.45rem 1rem;
          background: var(--coral);
          color: #fff;
          border: none;
          border-radius: var(--radius-pill);
          cursor: pointer;
          font-size: 0.8rem;
          font-weight: 600;
          transition: background var(--transition), transform 0.1s;
        }
        .vm-export-btn:hover {
          background: #e63946;
          transform: translateY(-1px);
        }
        .vm-export-btn:active {
          transform: translateY(0);
        }

        .vm-cards {
          display: flex;
          flex-direction: column;
          gap: 0.75rem;
          max-height: calc(100vh - 200px);
          overflow-y: auto;
          padding-right: 0.5rem;
        }
        .vm-cards::-webkit-scrollbar {
          width: 4px;
        }
        .vm-cards::-webkit-scrollbar-thumb {
          background: var(--border);
          border-radius: 2px;
        }

        .vm-card {
          background: var(--surface);
          border: 1px solid var(--border);
          border-radius: var(--radius);
          padding: 0.85rem 1rem;
          display: flex;
          flex-direction: column;
          gap: 0.4rem;
          transition: border-color var(--transition);
        }
        .vm-card:hover {
          border-color: var(--border-bright);
        }
        .vm-card.excluded {
          opacity: 0.5;
        }
        .vm-card-top {
          display: flex;
          align-items: center;
          justify-content: space-between;
          gap: 0.5rem;
        }
        .vm-word {
          font-family: var(--font-display);
          font-size: 1.15rem;
          font-weight: 600;
          color: var(--text);
        }
        .vm-translation {
          font-size: 0.8rem;
          color: var(--text-2);
          font-style: italic;
        }
        .vm-translation.missing {
          color: var(--text-muted);
        }
        .vm-sentence {
          font-size: 0.78rem;
          color: var(--text-muted);
          line-height: 1.5;
          border-top: 1px solid var(--border-subtle);
          padding-top: 0.4rem;
          margin-top: 0.1rem;
          overflow: hidden;
          text-overflow: ellipsis;
          display: -webkit-box;
          -webkit-line-clamp: 2;
          -webkit-box-orient: vertical;
        }
        .vm-card-toggle {
          appearance: none;
          -webkit-appearance: none;
          width: 20px;
          height: 20px;
          border-radius: 4px;
          background: var(--surface-3);
          border: 1px solid var(--border);
          cursor: pointer;
          flex-shrink: 0;
          transition: background var(--transition), border-color var(--transition);
          position: relative;
        }
        .vm-card-toggle:checked {
          background: var(--coral);
          border-color: var(--coral);
        }
        .vm-card-toggle:checked::after {
          content: '✓';
          position: absolute;
          inset: 0;
          display: flex;
          align-items: center;
          justify-content: center;
          color: #fff;
          font-size: 0.7rem;
          font-weight: 700;
        }

        .vm-loading, .vm-error {
          text-align: center;
          padding: 3rem;
          color: var(--text-2);
        }
        .vm-error {
          color: var(--error);
        }

        .vm-count-bar {
          font-size: 0.78rem;
          color: var(--text-muted);
          margin-bottom: 0.75rem;
        }
      `}</style>

      <Show when={article.loading}>
        <div class="vm-loading">Loading today's story...</div>
      </Show>

      <Show when={article.error}>
        <div class="vm-error">
          Unable to load story. Make sure the lacq server is running on port 8080.
        </div>
      </Show>

      <Show when={article()}>
        {(function() {
          const data = article()!;
          // Initialize included set on first load
          if (includedIndices().size === 0 && highlights().length > 0) {
            initializeIncluded(highlights());
          }
          return (
            <>
              <div class="vm-header">
                <h1>Vocabulary Mining</h1>
                <p>Highlight words you don't know and export them to Anki.</p>
              </div>

              <Show when={stories()}>
                <div class="vm-story-picker">
                  <label for="story-select">Story</label>
                  <select
                    id="story-select"
                    class="vm-story-select"
                    value={selectedStoryId() ?? data.story.id}
                    onChange={(e) => setSelectedStoryId(e.currentTarget.value || null)}
                  >
                    <option value={data.story.id}>{data.story.title} (today)</option>
                    <For each={stories()!.filter(s => s.id !== data.story.id)}>
                      {(s) => <option value={s.id}>{s.title} — {s.published_year}</option>}
                    </For>
                  </select>
                </div>
              </Show>

              <div class="vm-layout">
                <div class="vm-article">
                  <h2>{data.story.title}</h2>
                  <For each={data.french.paragraphs}>
                    {(para, i) => (
                      <p innerHTML={highlightText(para, i())} />
                    )}
                  </For>
                </div>

                <aside class="vm-sidebar">
                  <div class="vm-sidebar-header">
                    <h2>Words</h2>
                    <button class="vm-export-btn" onClick={exportCSV}>
                      Export CSV
                    </button>
                  </div>
                  <div class="vm-count-bar">
                    {includedIndices().size} of {highlights().length} words selected
                  </div>
                  <div class="vm-cards">
                    <For each={highlights()}>
                      {(h, i) => {
                        const idx = i();
                        const isIncluded = () => includedIndices().has(idx);
                        return (
                          <div class={`vm-card ${isIncluded() ? "" : "excluded"}`}>
                            <div class="vm-card-top">
                              <span class="vm-word">{h.word}</span>
                              <input
                                type="checkbox"
                                class="vm-card-toggle"
                                checked={isIncluded()}
                                onChange={() => toggleWord(idx)}
                                aria-label={`Include ${h.word} in export`}
                              />
                            </div>
                            <span class={`vm-translation ${mockTranslations[h.word] ? "" : "missing"}`}>
                              {mockTranslations[h.word] ?? "—"}
                            </span>
                            <p class="vm-sentence">{getSentence(h.paragraph_index)}</p>
                          </div>
                        );
                      }}
                    </For>
                  </div>
                </aside>
              </div>
            </>
          );
        })()}
      </Show>
    </div>
  );
}