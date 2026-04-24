import { createSignal, createResource, For, Show, createEffect } from "solid-js";

interface VocabHighlight {
  word: string;
  paragraph_index: number;
  start_offset: number;
  end_offset: number;
}

interface StoryListItem {
  id: string;
  title: string;
  source: string;
  published_year: number;
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

async function fetchStoryList(): Promise<StoryListItem[]> {
  const res = await fetch(`${window.location.protocol}//${window.location.host}/api/stories`);
  if (!res.ok) throw new Error(`Failed to fetch stories: ${res.status}`);
  return res.json();
}

async function fetchArticle(storyId?: string): Promise<ArticleData> {
  const base = `${window.location.protocol}//${window.location.host}`;
  const url = storyId ? `${base}/api/stories/${storyId}` : `${base}/api/article-of-the-day`;
  const res = await fetch(url);
  if (!res.ok) throw new Error(`Failed to fetch article: ${res.status}`);
  return res.json();
}

type ViewMode = 'fr' | 'en' | 'side' | 'type';

type CharState = 'untyped' | 'correct' | 'error';

interface TypingState {
  typedChars: number;
  errorCount: number;
  totalTyped: number;
  startTime: number | null;
  charStates: CharState[];
  currentIndex: number;
  finished: boolean;
}

export default function ArticleOfTheDay() {
  const [mode, setMode] = createSignal<ViewMode>('side');
  const [storyIndex, setStoryIndex] = createSignal(0);
  const [stories] = createResource(fetchStoryList);
  const [article, { refetch: refetchArticle }] = createResource(
    () => stories()?.[storyIndex()]?.id,
    (storyId) => fetchArticle(storyId)
  );

  // Typing mode state
  const [typingState, setTypingState] = createSignal<TypingState>({
    typedChars: 0,
    errorCount: 0,
    totalTyped: 0,
    startTime: null,
    charStates: [],
    currentIndex: 0,
    finished: false,
  });

  const getFullText = () => {
    const data = article();
    if (!data) return '';
    return data.french.paragraphs.join('\n\n');
  };

  const initTyping = () => {
    const text = getFullText();
    setTypingState({
      typedChars: 0,
      errorCount: 0,
      totalTyped: 0,
      startTime: null,
      charStates: Array(text.length).fill('untyped'),
      currentIndex: 0,
      finished: false,
    });
  };

  // Navigation
  const navigatePrev = () => {
    const list = stories();
    if (!list || list.length === 0) return;
    setStoryIndex((list.length + storyIndex() - 1) % list.length);
  };

  const navigateNext = () => {
    const list = stories();
    if (!list || list.length === 0) return;
    setStoryIndex((storyIndex() + 1) % list.length);
  };

  // Reset typing state when story changes
  createEffect(() => {
    // Track article data changes
    article();
    if (mode() === 'type') {
      setMode('side');
    }
  });

  const startTyping = () => {
    initTyping();
    setMode('type');
  };

  // Auto-focus the typing mode when entering
  createEffect(() => {
    if (mode() === 'type' && typingInputRef) {
      setTimeout(() => typingInputRef?.focus(), 50);
    }
  });

  const calculateWPM = () => {
    const s = typingState();
    if (!s.startTime || s.typedChars === 0) return 0;
    const elapsedMinutes = (Date.now() - s.startTime) / 60000;
    if (elapsedMinutes < 0.01) return 0;
    return Math.round((s.typedChars / 5) / elapsedMinutes);
  };

  const calculateAccuracy = () => {
    const s = typingState();
    if (s.totalTyped === 0) return 100;
    return Math.round((s.typedChars / s.totalTyped) * 100);
  };

  const handleTypingKeyDown = (e: KeyboardEvent) => {
    if (mode() !== 'type') return;
    const s = typingState();
    if (s.finished) return;

    if (e.key === 'Escape') {
      setMode('side');
      return;
    }

    const text = getFullText();
    const currentChar = text[s.currentIndex];

    // Start timer on first character
    if (s.startTime === null && e.key.length === 1) {
      setTypingState(prev => ({ ...prev, startTime: Date.now() }));
    }

    if (e.key === 'Backspace') {
      // We don't allow backspace in this simple implementation
      return;
    }

    if (e.key.length !== 1) return;

    e.preventDefault();

    const newCharStates = [...s.charStates];
    let newTypedChars = s.typedChars;
    let newErrorCount = s.errorCount;
    let newTotalTyped = s.totalTyped + 1;

    if (e.key === currentChar) {
      newCharStates[s.currentIndex] = 'correct';
      newTypedChars++;
    } else {
      newCharStates[s.currentIndex] = 'error';
      newErrorCount++;
    }

    const newIndex = s.currentIndex + 1;
    const finished = newIndex >= text.length;

    setTypingState({
      typedChars: newTypedChars,
      errorCount: newErrorCount,
      totalTyped: newTotalTyped,
      startTime: s.startTime,
      charStates: newCharStates,
      currentIndex: newIndex,
      finished,
    });
  };

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

  let typingInputRef: HTMLInputElement | undefined;

  const TypingView = () => {
    const s = typingState();
    const text = getFullText();
    const paragraphs = article()?.french.paragraphs ?? [];
    const progress = text.length > 0 ? (s.currentIndex / text.length) * 100 : 0;

    // Group char states by paragraph
    let charIndex = 0;
    const paragraphChars = paragraphs.map(para => {
      const chars: { char: string; state: CharState }[] = [];
      for (const c of para) {
        chars.push({ char: c, state: s.charStates[charIndex] ?? 'untyped' });
        charIndex++;
      }
      charIndex++; // skip the newline between paragraphs
      return chars;
    });

    return (
      <div class="typing-mode" onKeyDown={handleTypingKeyDown} tabIndex={0} ref={typingInputRef}>
        <input type="text" class="typing-input" />

        <div class="typing-hud">
          <div class="typing-wpm">
            <span class="typing-wpm-value">{calculateWPM()}</span>
            <span class="typing-wpm-label">WPM</span>
          </div>
          <div class="typing-progress-bar">
            <div class="typing-progress-fill" style={{ width: `${progress}%` }} />
          </div>
          <div class="typing-accuracy">{calculateAccuracy()}%</div>
        </div>

        <div class="typing-text-container">
          {paragraphChars.map((chars, pIdx) => (
            <div class="typing-paragraph">
              {chars.map(({ char, state }) => (
                <span class={`typing-char typing-char-${state}`}>{char}</span>
              ))}
            </div>
          ))}
        </div>

        <Show when={s.finished}>
          <div class="typing-results-overlay">
            <div class="typing-results">
              <h2>Finished!</h2>
              <div class="typing-results-stats">
                <div class="typing-result-stat">
                  <span class="typing-result-value">{calculateWPM()}</span>
                  <span class="typing-result-label">WPM</span>
                </div>
                <div class="typing-result-stat">
                  <span class="typing-result-value">{calculateAccuracy()}%</span>
                  <span class="typing-result-label">Accuracy</span>
                </div>
              </div>
              <div class="typing-results-actions">
                <button class="typing-result-btn primary" onClick={() => { initTyping(); }}>
                  Try Again
                </button>
                <button class="typing-result-btn" onClick={() => setMode('side')}>
                  Back to Reading
                </button>
              </div>
            </div>
          </div>
        </Show>
      </div>
    );
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
        .article-nav {
          display: flex;
          align-items: center;
          gap: 0.75rem;
          margin-bottom: 0.75rem;
        }
        .nav-btn {
          background: var(--surface-2);
          border: 1px solid var(--border);
          color: var(--text-2);
          width: 36px;
          height: 36px;
          border-radius: 6px;
          cursor: pointer;
          font-size: 1.2rem;
          display: flex;
          align-items: center;
          justify-content: center;
          transition: all var(--transition);
        }
        .nav-btn:hover {
          background: var(--surface-3);
          color: var(--text);
        }
        .story-position {
          font-size: 0.9rem;
          color: var(--text-2);
          font-family: var(--font-mono);
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
        /* Race button */
        .race-btn {
          padding: 0.6rem 1.5rem;
          background: var(--coral);
          color: #fff;
          border: none;
          border-radius: var(--radius-pill);
          cursor: pointer;
          font-size: 0.9rem;
          font-weight: 600;
          margin-bottom: 1.5rem;
          transition: background var(--transition), transform 0.1s;
        }
        .race-btn:hover {
          background: #e63946;
          transform: translateY(-1px);
        }
        .race-btn:active {
          transform: translateY(0);
        }
        /* Typing mode */
        .typing-mode {
          position: fixed;
          inset: 0;
          background: var(--bg);
          z-index: 200;
          display: flex;
          flex-direction: column;
          align-items: center;
          padding: 2rem;
          outline: none;
          overflow: hidden;
        }
        .typing-input {
          position: absolute;
          opacity: 0;
          width: 0;
          height: 0;
        }
        .typing-hud {
          display: flex;
          align-items: center;
          gap: 1.5rem;
          margin-bottom: 2rem;
          width: 100%;
          max-width: 800px;
        }
        .typing-wpm {
          display: flex;
          flex-direction: column;
          align-items: center;
          min-width: 60px;
        }
        .typing-wpm-value {
          font-family: var(--font-display);
          font-size: 2rem;
          font-weight: 700;
          color: var(--coral);
          line-height: 1;
        }
        .typing-wpm-label {
          font-size: 0.7rem;
          text-transform: uppercase;
          letter-spacing: 0.1em;
          color: var(--text-muted);
        }
        .typing-progress-bar {
          flex: 1;
          height: 8px;
          background: var(--surface-2);
          border-radius: 4px;
          overflow: hidden;
        }
        .typing-progress-fill {
          height: 100%;
          background: var(--emerald);
          border-radius: 4px;
          transition: width 0.1s ease;
        }
        .typing-accuracy {
          font-family: var(--font-mono);
          font-size: 0.9rem;
          color: var(--text-2);
          min-width: 50px;
          text-align: right;
        }
        .typing-text-container {
          max-width: 800px;
          width: 100%;
          max-height: calc(100vh - 200px);
          overflow-y: auto;
          padding-right: 1rem;
        }
        .typing-paragraph {
          font-family: var(--font-body);
          font-size: 1.35rem;
          line-height: 2;
          margin-bottom: 1.5rem;
          padding-left: 1rem;
          border-left: 3px solid var(--surface-3);
          transition: border-color 0.2s;
        }
        .typing-char {
          transition: color 0.05s;
        }
        .typing-char-untyped {
          color: var(--text-muted);
        }
        .typing-char-correct {
          color: var(--text);
        }
        .typing-char-error {
          color: var(--coral);
          text-decoration: underline;
          text-decoration-style: wavy;
        }
        /* Results overlay */
        .typing-results-overlay {
          position: fixed;
          inset: 0;
          background: rgba(0, 0, 0, 0.6);
          display: flex;
          align-items: center;
          justify-content: center;
          z-index: 300;
        }
        .typing-results {
          background: var(--bg);
          border-radius: var(--radius);
          padding: 2.5rem 3rem;
          text-align: center;
          animation: panel-in 0.2s ease;
        }
        .typing-results h2 {
          font-family: var(--font-display);
          font-size: 2rem;
          margin: 0 0 1.5rem 0;
          color: var(--text);
        }
        .typing-results-stats {
          display: flex;
          gap: 2.5rem;
          justify-content: center;
          margin-bottom: 2rem;
        }
        .typing-result-stat {
          display: flex;
          flex-direction: column;
          align-items: center;
        }
        .typing-result-value {
          font-family: var(--font-display);
          font-size: 2.5rem;
          font-weight: 700;
          color: var(--coral);
          line-height: 1;
        }
        .typing-result-label {
          font-size: 0.75rem;
          text-transform: uppercase;
          letter-spacing: 0.1em;
          color: var(--text-muted);
          margin-top: 0.25rem;
        }
        .typing-results-actions {
          display: flex;
          gap: 1rem;
          justify-content: center;
        }
        .typing-result-btn {
          padding: 0.6rem 1.5rem;
          border: 1px solid var(--border);
          background: var(--surface-2);
          color: var(--text-2);
          border-radius: 6px;
          cursor: pointer;
          font-size: 0.9rem;
          transition: all var(--transition);
        }
        .typing-result-btn:hover {
          background: var(--surface-3);
          color: var(--text);
        }
        .typing-result-btn.primary {
          background: var(--coral);
          color: #fff;
          border-color: var(--coral);
        }
        .typing-result-btn.primary:hover {
          background: #e63946;
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
                <div class="article-nav">
                  <button class="nav-btn" onClick={navigatePrev} aria-label="Previous story">←</button>
                  <span class="story-position">
                    {stories() ? `${storyIndex() + 1} / ${stories()!.length}` : '...'}
                  </span>
                  <button class="nav-btn" onClick={navigateNext} aria-label="Next story">→</button>
                </div>
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
                <button class="race-btn" onClick={startTyping}>
                  🏃 Race
                </button>
              </div>

              <Show when={mode() === 'type'}>
                <TypingView />
              </Show>

              <Show when={mode() !== 'type'}>
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
              </Show>
            </>
          );
        }}
      </Show>
    </div>
  );
}
