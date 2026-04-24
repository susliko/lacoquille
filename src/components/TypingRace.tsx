import { createSignal, createResource, For, Show, createEffect, onMount } from "solid-js";

interface StoryListItem {
  id: string;
  title: string;
  source: string;
  published_year: number;
}

interface StoryData {
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
  date: string;
}

interface VocabHighlight {
  word: string;
  paragraph_index: number;
  start_offset: number;
  end_offset: number;
}

interface Props {
  storyId?: string;
  onBack?: () => void;
}

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

async function fetchStoryList(): Promise<StoryListItem[]> {
  const res = await fetch(`/api/stories`);
  if (!res.ok) throw new Error(`Failed to fetch stories: ${res.status}`);
  return res.json();
}

async function fetchStory(id: string): Promise<StoryData> {
  const res = await fetch(`/api/stories/${id}`);
  if (!res.ok) throw new Error(`Failed to fetch story: ${res.status}`);
  return res.json();
}

export default function TypingRace(props: Props) {
  const [selectedStoryId, setSelectedStoryId] = createSignal<string | undefined>(undefined);
  const [stories] = createResource(fetchStoryList);

  onMount(() => {
    const saved = localStorage.getItem('typing-race-story');
    if (saved && stories()?.some(s => s.id === saved)) {
      setSelectedStoryId(saved);
    }
  });

  const warmUpTts = (storyId: string) => {
    fetch(`/api/stories/${storyId}/tts-cache`, { method: "POST" });
  };

  const [storyIndex, setStoryIndex] = createSignal(0);
  const [story, { refetch: refetchStory }] = createResource(
    () => props.storyId ?? stories.data?.[storyIndex()]?.id,
    (id) => fetchStory(id)
  );

  const [typingState, setTypingState] = createSignal<TypingState>({
    typedChars: 0,
    errorCount: 0,
    totalTyped: 0,
    startTime: null,
    charStates: [],
    currentIndex: 0,
    finished: false,
  });

  const [showResults, setShowResults] = createSignal(false);
  const [showHistory, setShowHistory] = createSignal(false);

  const saveResult = (storyId: string, wpm: number, accuracy: number) => {
    const key = `typing-race-history-${storyId}`;
    const history = JSON.parse(localStorage.getItem(key) || '[]');
    history.unshift({ storyId, wpm, accuracy, timestamp: new Date().toISOString() });
    localStorage.setItem(key, JSON.stringify(history.slice(0, 10)));

    const bestKey = `typing-race-best-${storyId}`;
    const prevBest = parseInt(localStorage.getItem(bestKey) || '0');
    if (wpm > prevBest) localStorage.setItem(bestKey, wpm.toString());
  };

  const getHistory = (storyId: string) => {
    const key = `typing-race-history-${storyId}`;
    return JSON.parse(localStorage.getItem(key) || '[]');
  };

  const getPersonalBest = (storyId: string) => {
    const bestKey = `typing-race-best-${storyId}`;
    return parseInt(localStorage.getItem(bestKey) || '0');
  };

  const getFullText = () => {
    const data = story();
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
    setShowResults(false);
  };

  // Start typing immediately when story loads
  createEffect(() => {
    if (story() && !story().loading && !story().error) {
      setTimeout(initTyping, 50);
    }
  });

  const navigatePrev = () => {
    const list = stories();
    if (!list || list.length === 0) return;
    const newIndex = (list.length + storyIndex() - 1) % list.length;
    setStoryIndex(newIndex);
    const newId = list[newIndex].id;
    setSelectedStoryId(newId);
    localStorage.setItem('typing-race-story', newId);
    warmUpTts(newId);
    setShowResults(false);
  };

  const navigateNext = () => {
    const list = stories();
    if (!list || list.length === 0) return;
    const newIndex = (storyIndex() + 1) % list.length;
    setStoryIndex(newIndex);
    const newId = list[newIndex].id;
    setSelectedStoryId(newId);
    localStorage.setItem('typing-race-story', newId);
    warmUpTts(newId);
    setShowResults(false);
  };

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
    const s = typingState();
    if (s.finished && e.key !== 'Escape' && e.key !== 'Enter') return;

    if (e.key === 'Escape') {
      if (props.onBack) {
        props.onBack();
      }
      return;
    }

    // Tab = skip current word
    if (e.key === 'Tab') {
      e.preventDefault();
      if (s.finished) return;
      const text = getFullText();
      let i = s.currentIndex;
      const newCharStates = [...s.charStates];
      let newTypedChars = s.typedChars;
      // Mark current word chars as correct until space or newline
      while (i < text.length && text[i] !== ' ' && text[i] !== '\n') {
        newCharStates[i] = 'correct';
        newTypedChars++;
        i++;
      }
      // Also skip any trailing spaces/newlines
      while (i < text.length && (text[i] === ' ' || text[i] === '\n')) {
        newCharStates[i] = 'correct';
        newTypedChars++;
        i++;
      }
      setTypingState({
        ...s,
        charStates: newCharStates,
        typedChars: newTypedChars,
        currentIndex: i,
      });
      return;
    }

    // Enter on results overlay = Try Again
    if (e.key === 'Enter' && showResults()) {
      initTyping();
      setTimeout(() => typingInputRef?.focus(), 50);
      return;
    }

    const text = getFullText();
    const currentChar = text[s.currentIndex];

    // Start timer on first character
    if (s.startTime === null && e.key.length === 1) {
      setTypingState(prev => ({ ...prev, startTime: Date.now() }));
    }

    if (e.key === 'Backspace') return;

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

    if (finished) {
      const id = props.storyId ?? stories.data?.[storyIndex()]?.id;
      if (id) saveResult(id, Math.round((newTypedChars / 5) / ((Date.now() - s.startTime!) / 60000)), Math.round((newTypedChars / newTotalTyped) * 100));
      setTimeout(() => setShowResults(true), 300);
    }
  };

  let typingInputRef: HTMLInputElement | undefined;

  createEffect(() => {
    if (story() && typingInputRef) {
      setTimeout(() => typingInputRef?.focus(), 100);
    }
  });

  const TypingView = () => {
    const s = typingState();
    const text = getFullText();
    const paragraphs = story()?.french.paragraphs ?? [];
    const progress = text.length > 0 ? (s.currentIndex / text.length) * 100 : 0;

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
          {paragraphChars.map((chars) => (
            <div class="typing-paragraph">
              {chars.map(({ char, state }) => (
                <span class={`typing-char typing-char-${state}`}>{char}</span>
              ))}
            </div>
          ))}
        </div>

        <div class="typing-shortcut-hint">
          <kbd>Tab</kbd> = skip word · <kbd>Esc</kbd> = exit
        </div>

        <Show when={showResults()}>
          {() => {
            const id = props.storyId ?? stories.data?.[storyIndex()]?.id ?? '';
            const currentWPM = calculateWPM();
            const bestWPM = getPersonalBest(id);
            const isNewRecord = currentWPM > bestWPM && currentWPM > 0;
            const history = getHistory(id).slice(0, 5);
            return (
              <div class="typing-results-overlay">
                <div class="typing-results">
                  <h2>Finished!</h2>
                  <Show when={isNewRecord}>
                    <div class="new-record-badge">New record!</div>
                  </Show>
                  <div class="typing-results-stats">
                    <div class="typing-result-stat typing-result-wpm">
                      <span class="typing-result-value">{currentWPM}</span>
                      <span class="typing-result-label">WPM</span>
                      <Show when={bestWPM > 0 && !isNewRecord}>
                        <span class="personal-best">Best: {bestWPM}</span>
                      </Show>
                    </div>
                    <div class="typing-result-stat">
                      <span class="typing-result-value">{calculateAccuracy()}%</span>
                      <span class="typing-result-label">Accuracy</span>
                    </div>
                  </div>
                  <Show when={history.length > 0}>
                    <div class="typing-history-section">
                      <button class="history-toggle" onClick={() => setShowHistory(h => !h)}>
                        History {showHistory() ? '▲' : '▼'}
                      </button>
                      <Show when={showHistory()}>
                        <ul class="typing-history-list">
                          {history.map((r: any, i: number) => (
                            <li>
                              <span class="history-wpm">{r.wpm} WPM</span>
                              <span class="history-accuracy">{r.accuracy}%</span>
                              <span class="history-date">{new Date(r.timestamp).toLocaleDateString()}</span>
                            </li>
                          ))}
                        </ul>
                      </Show>
                    </div>
                  </Show>
                  <div class="typing-results-actions">
                    <button class="typing-result-btn primary" onClick={() => { initTyping(); setTimeout(() => typingInputRef?.focus(), 50); }}>
                      Try Again
                    </button>
                    <button class="typing-result-btn" onClick={() => props.onBack?.()}>
                      Back to Stories
                    </button>
                  </div>
                </div>
              </div>
            );
          }}</Show>
      </div>
    );
  };

  const StorySelector = () => (
    <div class="story-selector">
      <h2>Choose a Story</h2>
      <div class="story-nav">
        <button class="nav-btn" onClick={navigatePrev} aria-label="Previous story">←</button>
        <span class="story-position">
          {stories() ? `${storyIndex() + 1} / ${stories()!.length}` : '...'}
        </span>
        <button class="nav-btn" onClick={navigateNext} aria-label="Next story">→</button>
      </div>
      <Show when={stories()}>
        <div class="story-title">{stories()?.[storyIndex()]?.title}</div>
        <p class="story-meta">
          {stories()?.[storyIndex()]?.source} ({stories()?.[storyIndex()]?.published_year})
        </p>
      </Show>
    </div>
  );

  return (
    <div class="typing-race">
      <style>{`
        .typing-race {
          max-width: 1100px;
          margin: 0 auto;
          padding: 1.5rem 2rem;
        }
        .story-selector {
          text-align: center;
          padding: 3rem;
        }
        .story-selector h2 {
          font-family: var(--font-display);
          font-size: 2rem;
          margin: 0 0 1.5rem 0;
        }
        .story-nav {
          display: flex;
          align-items: center;
          justify-content: center;
          gap: 0.75rem;
          margin-bottom: 1rem;
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
        .story-title {
          font-family: var(--font-display);
          font-size: 1.5rem;
          margin-bottom: 0.5rem;
        }
        .story-meta {
          color: var(--text-2);
        }
        .loading-state, .error-state {
          text-align: center;
          padding: 3rem;
          color: var(--text-2);
        }
        .error-state {
          color: var(--error);
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
        .typing-result-wpm {
          position: relative;
        }
        .typing-result-value {
          font-family: var(--font-display);
          font-size: 2.5rem;
          font-weight: 700;
          color: var(--coral);
          line-height: 1;
        }
        .personal-best {
          font-size: 0.7rem;
          color: var(--text-muted);
          margin-top: 0.25rem;
        }
        .new-record-badge {
          background: var(--emerald);
          color: #fff;
          padding: 0.25rem 0.75rem;
          border-radius: 4px;
          font-size: 0.85rem;
          font-weight: 600;
          margin-bottom: 1rem;
          display: inline-block;
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
        .typing-history-section {
          margin: 1.5rem 0;
          text-align: center;
        }
        .history-toggle {
          background: none;
          border: none;
          color: var(--text-2);
          cursor: pointer;
          font-size: 0.85rem;
          padding: 0.25rem 0.5rem;
        }
        .history-toggle:hover {
          color: var(--text);
        }
        .typing-history-list {
          list-style: none;
          padding: 0;
          margin: 0.75rem 0 0 0;
          font-size: 0.8rem;
          color: var(--text-2);
        }
        .typing-history-list li {
          display: flex;
          gap: 1rem;
          justify-content: center;
          padding: 0.25rem 0;
        }
        .history-wpm {
          font-family: var(--font-mono);
          color: var(--text);
        }
        .history-accuracy {
          color: var(--text-muted);
        }
        .history-date {
          color: var(--text-muted);
          font-size: 0.75rem;
        }
        .typing-shortcut-hint {
          position: fixed;
          bottom: 1rem;
          left: 50%;
          transform: translateX(-50%);
          font-size: 0.75rem;
          color: var(--text-muted);
          opacity: 0.6;
        }
        .typing-shortcut-hint kbd {
          font-family: var(--font-mono);
          background: var(--surface-2);
          padding: 0.1em 0.4em;
          border-radius: 3px;
          border: 1px solid var(--border);
        }
      `}</style>

      <Show when={stories.loading || story.loading}>
        <div class="loading-state">Loading story...</div>
      </Show>

      <Show when={stories.error || story.error}>
        <div class="error-state">
          Unable to load story. Make sure the lacq server is running on port 8080.
        </div>
      </Show>

      <Show when={story() && !story()!.loading}>
        {() => (
          <TypingView />
        )}
      </Show>
    </div>
  );
}
