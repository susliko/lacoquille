import { createSignal, createMemo, createResource, createEffect, onCleanup, Show } from "solid-js";

interface ArticleData {
  title: string;
  source: string;
  published_year: number;
  paragraphs: string[];
}

export default function TypingRace() {
  const [article] = createResource(async () => {
    const res = await fetch("/api/article-of-the-day");
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    return res.json() as Promise<ArticleData>;
  });

  const [typed, setTyped] = createSignal(0);
  const [errors, setErrors] = createSignal(0);
  const [startTime, setStartTime] = createSignal<number | null>(null);
  const [finished, setFinished] = createSignal(false);
  const [elapsedSecs, setElapsedSecs] = createSignal(0);

  const text = createMemo(() => (article()?.paragraphs?.[0] ?? "").split(""));

  // Tick elapsed time every 500ms once race starts
  createEffect(() => {
    if (startTime() === null) return;
    const id = setInterval(() => {
      if (!finished()) {
        setElapsedSecs(Math.floor((Date.now() - startTime()!) / 1000));
      }
    }, 500);
    onCleanup(() => clearInterval(id));
  });

  // Reset elapsed when race restarts
  createEffect(() => {
    if (startTime() === null) setElapsedSecs(0);
  });

  const wpm = createMemo(() => {
    const secs = elapsedSecs();
    const chars = typed();
    if (secs < 1 || chars === 0) return 0;
    return Math.round((chars / 5) / (secs / 60));
  });

  const accuracy = createMemo(() => {
    const t = typed();
    const e = errors();
    if (t === 0) return 100;
    return Math.round(((t - e) / t) * 100);
  });

  const [errorSet, setErrorSet] = createSignal<Set<number>>(new Set());

  const handleKeyDown = (e: KeyboardEvent) => {
    // Allow browser shortcuts with modifiers through
    if (e.ctrlKey || e.metaKey || e.altKey) return;

    if (finished()) {
      if (e.key === "Enter") resetRace();
      return;
    }
    if (e.key === "Backspace") return;
    if (e.key.length !== 1) return;

    e.preventDefault();

    if (startTime() === null) {
      setStartTime(Date.now());
    }

    const idx = typed();
    const txt = text();
    if (idx >= txt.length) {
      setFinished(true);
      return;
    }

    if (e.key === txt[idx]) {
      setTyped(idx + 1);
    } else {
      setErrorSet((prev) => new Set([...prev, idx]));
      setErrors(errors() + 1);
      setTyped(idx + 1);
    }

    if (idx + 1 >= txt.length) {
      setFinished(true);
    }
  };

  const resetRace = () => {
    setTyped(0);
    setErrors(0);
    setErrorSet(new Set());
    setStartTime(null);
    setElapsedSecs(0);
    setFinished(false);
    (document.activeElement as HTMLElement | null)?.blur();
    setTimeout(() => containerRef?.focus(), 50);
  };

  let containerRef: HTMLDivElement | undefined;

  createEffect(() => {
    if (article() && containerRef) {
      containerRef.focus();
    }
  });

  // Split text into typed and remaining for cursor placement
  const typedText = createMemo(() => text().slice(0, typed()));
  const remainingText = createMemo(() => text().slice(typed()));

  const getCharState = (i: number): string => {
    if (i >= typed()) return "idle";
    return errorSet().has(i) ? "error" : "correct";
  };

  // For remaining characters, derive state directly
  const remainingChars = createMemo(() =>
    remainingText().map((char, i) => ({ char, idx: typed() + i }))
  );

  return (
    <div class="tr-root">
      <style>{`
        .tr-root {
          min-height: 100vh;
          background: var(--bg);
          display: flex;
          flex-direction: column;
          align-items: center;
          justify-content: flex-start;
          padding: 2.5rem 1rem 4rem;
          box-sizing: border-box;
        }
        .tr-title {
          font-family: var(--font-display);
          font-size: 1.05rem;
          font-weight: 600;
          color: var(--text-2);
          margin-bottom: 0.2rem;
          letter-spacing: 0.03em;
        }
        .tr-meta {
          font-size: 0.78rem;
          color: var(--text-muted);
          margin-bottom: 2.5rem;
        }
        .tr-hud {
          display: flex;
          gap: 2.5rem;
          margin-bottom: 2.5rem;
          align-items: center;
        }
        .tr-stat {
          display: flex;
          flex-direction: column;
          align-items: center;
          min-width: 60px;
        }
        .tr-stat-value {
          font-family: var(--font-display);
          font-size: 2.5rem;
          font-weight: 700;
          color: var(--text);
          line-height: 1;
        }
        .tr-stat-label {
          font-size: 0.62rem;
          text-transform: uppercase;
          letter-spacing: 0.12em;
          color: var(--text-muted);
          margin-top: 0.25rem;
        }
        .tr-text-wrap {
          position: relative;
          max-width: 720px;
          width: 100%;
          cursor: text;
        }
        .tr-text-area {
          outline: none;
          white-space: pre-wrap;
          word-break: break-word;
          font-family: var(--font-body);
          font-size: clamp(1.1rem, 3vw, 1.4rem);
          line-height: 2.1;
          letter-spacing: 0.01em;
          padding: 1.25rem 1.5rem;
          background: var(--surface);
          border: 1px solid var(--border-subtle);
          border-radius: var(--radius);
          position: relative;
          user-select: none;
        }
        .tr-cursor {
          display: inline-block;
          width: 2px;
          height: 1.25em;
          background: var(--coral);
          vertical-align: text-bottom;
          animation: blink 0.85s step-end infinite;
        }
        @keyframes blink {
          0%, 100% { opacity: 1; }
          50%       { opacity: 0; }
        }
        .tc-idle    { color: var(--text-muted); }
        .tc-correct { color: #4ade80; }
        .tc-error   { color: #f87171; text-decoration: underline; text-decoration-style: wavy; }
        /* Results */
        .tr-overlay {
          position: fixed;
          inset: 0;
          background: rgba(0, 0, 0, 0.55);
          display: flex;
          align-items: center;
          justify-content: center;
          z-index: 100;
          animation: tr-fade-in 0.15s ease;
        }
        @keyframes tr-fade-in {
          from { opacity: 0; }
          to   { opacity: 1; }
        }
        .tr-results {
          background: var(--bg);
          border: 1px solid var(--border);
          border-radius: var(--radius-lg);
          padding: 2.5rem 3.5rem;
          text-align: center;
          animation: tr-slide-up 0.2s ease;
        }
        @keyframes tr-slide-up {
          from { transform: translateY(14px); opacity: 0; }
          to   { transform: translateY(0); opacity: 1; }
        }
        .tr-results h2 {
          font-family: var(--font-display);
          font-size: 2.25rem;
          margin: 0 0 2rem;
        }
        .tr-results-stats {
          display: flex;
          gap: 3rem;
          justify-content: center;
          margin-bottom: 2rem;
        }
        .tr-result-stat {
          display: flex;
          flex-direction: column;
          align-items: center;
        }
        .tr-result-value {
          font-family: var(--font-display);
          font-size: 3.25rem;
          font-weight: 700;
          color: var(--coral);
          line-height: 1;
        }
        .tr-result-label {
          font-size: 0.68rem;
          text-transform: uppercase;
          letter-spacing: 0.1em;
          color: var(--text-muted);
          margin-top: 0.3rem;
        }
        .tr-restart {
          display: inline-flex;
          align-items: center;
          gap: 0.5rem;
          padding: 0.7rem 2.25rem;
          background: var(--coral);
          color: #fff;
          border: none;
          border-radius: 9999px;
          cursor: pointer;
          font-size: 1rem;
          font-weight: 600;
          transition: background 0.15s, transform 0.1s;
        }
        .tr-restart:hover  { background: #e63946; transform: translateY(-1px); }
        .tr-restart:active { transform: translateY(0); }
        .tr-hint {
          margin-top: 1.25rem;
          font-size: 0.75rem;
          color: var(--text-muted);
        }
        .tr-hint kbd {
          font-family: var(--font-mono);
          background: var(--surface-2);
          padding: 0.1em 0.4em;
          border-radius: 3px;
          border: 1px solid var(--border);
        }
        .tr-loading,
        .tr-error {
          text-align: center;
          padding: 4rem;
          color: var(--text-2);
        }
        .tr-error { color: var(--error); }
        .tr-tap-hint {
          margin-top: 1.5rem;
          font-size: 0.8rem;
          color: var(--text-muted);
        }
        @media (max-width: 600px) {
          .tr-root { padding: 2rem 1rem 6rem; }
          .tr-text-area { padding: 1rem; font-size: 1.05rem; }
          .tr-results { padding: 2rem 1.5rem; }
          .tr-results-stats { gap: 2rem; }
          .tr-result-value { font-size: 2.5rem; }
        }
      `}</style>

      <Show when={article.loading}>
        <div class="tr-loading">Loading today's story...</div>
      </Show>

      <Show when={article.error}>
        <div class="tr-error">
          Unable to load story. Make sure the lacq server is running on port 8080.
        </div>
      </Show>

      <Show when={article()}>
        {(data) => (
          <>
            <div class="tr-title">{data().title}</div>
            <div class="tr-meta">{data().source} · {data().published_year}</div>

            <div class="tr-hud">
              <div class="tr-stat">
                <span class="tr-stat-value">{wpm()}</span>
                <span class="tr-stat-label">WPM</span>
              </div>
              <div class="tr-stat">
                <span class="tr-stat-value">{accuracy()}%</span>
                <span class="tr-stat-label">Accuracy</span>
              </div>
            </div>

            <div class="tr-text-wrap">
              <div
                class="tr-text-area"
                ref={containerRef}
                tabIndex={0}
                onKeyDown={handleKeyDown}
              >
                {/* Typed characters */}
                {typedText().map((char, i) => (
                  <span class={`tc-${errorSet().has(i) ? "error" : "correct"}`}>{char}</span>
                ))}
                {/* Cursor after typed, before remaining */}
                <Show when={!finished()}>
                  <span class="tr-cursor" />
                </Show>
                {/* Remaining characters */}
                {remainingChars().map(({ char }) => (
                  <span class="tc-idle">{char}</span>
                ))}
              </div>
            </div>

            <p class="tr-tap-hint">click the text · start typing</p>

            <Show when={finished()}>
              <div class="tr-overlay">
                <div class="tr-results">
                  <h2>Done!</h2>
                  <div class="tr-results-stats">
                    <div class="tr-result-stat">
                      <span class="tr-result-value">{wpm()}</span>
                      <span class="tr-result-label">WPM</span>
                    </div>
                    <div class="tr-result-stat">
                      <span class="tr-result-value">{accuracy()}%</span>
                      <span class="tr-result-label">Accuracy</span>
                    </div>
                  </div>
                  <button class="tr-restart" onClick={resetRace}>
                    ↺ Race Again
                  </button>
                  <p class="tr-hint">or press <kbd>Enter</kbd></p>
                </div>
              </div>
            </Show>
          </>
        )}
      </Show>
    </div>
  );
}
