import { createSignal, createResource, For, Show } from "solid-js";

interface Story {
  id: string;
  title: string;
  source: string;
}

interface StoryDetail {
  id: string;
  title: string;
  source: string;
  french: {
    paragraphs: string[];
  };
}

type PracticeMode = "dictation" | "multiple-choice";

async function fetchStories(): Promise<Story[]> {
  const host = `${window.location.protocol}//${window.location.host}`;
  const res = await fetch(`${host}/api/stories`);
  if (!res.ok) throw new Error(`Failed to fetch stories: ${res.status}`);
  return res.json();
}

async function fetchStory(id: string): Promise<StoryDetail> {
  const host = `${window.location.protocol}//${window.location.host}`;
  const res = await fetch(`${host}/api/stories/${id}`);
  if (!res.ok) throw new Error(`Failed to fetch story: ${res.status}`);
  return res.json();
}

function splitSentences(paragraphs: string[]): string[] {
  return paragraphs
    .join(" ")
    .split(/[.!?]+/)
    .map((s) => s.trim())
    .filter((s) => s.length > 0);
}

function getDistractors(
  currentSentence: string,
  allSentences: string[],
  count: number
): string[] {
  const others = allSentences.filter((s) => s !== currentSentence);
  // Shuffle and pick
  const shuffled = others.sort(() => Math.random() - 0.5);
  return shuffled.slice(0, count);
}

function charAccuracy(original: string, typed: string): number {
  if (typed.length === 0) return 0;
  let matches = 0;
  const minLen = Math.min(original.length, typed.length);
  for (let i = 0; i < minLen; i++) {
    if (original[i] === typed[i]) matches++;
  }
  return Math.round((matches / original.length) * 100);
}

export default function ShadowingPractice() {
  const [stories] = createResource(fetchStories);
  const [selectedStoryId, setSelectedStoryId] = createSignal<string | null>(null);

  const [storyDetail, setStoryDetail] = createSignal<StoryDetail | null>(null);
  const [sentences, setSentences] = createSignal<string[]>([]);
  const [currentIndex, setCurrentIndex] = createSignal(0);
  const [mode, setMode] = createSignal<PracticeMode>("dictation");

  // TTS state
  const [isPlaying, setIsPlaying] = createSignal(false);
  const [isWarmingUp, setIsWarmingUp] = createSignal(false);
  const ttsCache = new Map<string, string>();

  // Dictation state
  const [typedText, setTypedText] = createSignal("");
  const [dictationSubmitted, setDictationSubmitted] = createSignal(false);
  const [accuracy, setAccuracy] = createSignal(0);

  // MC state
  const [mcOptions, setMcOptions] = createSignal<string[]>([]);
  const [selectedOption, setSelectedOption] = createSignal<string | null>(null);
  const [mcSubmitted, setMcSubmitted] = createSignal(false);

  let audioRef: HTMLAudioElement | undefined;

  const warmUpTts = (storyId: string) => {
    setIsWarmingUp(true);
    fetch(`/api/stories/${storyId}/tts-cache`, { method: "POST" }).finally(() => {
      setTimeout(() => setIsWarmingUp(false), 5000);
    });
  };

  const loadStorySentences = async (id: string) => {
    warmUpTts(id);
    const detail = await fetchStory(id);
    setStoryDetail(detail);
    const sents = splitSentences(detail.french.paragraphs);
    setSentences(sents);
    setCurrentIndex(0);
    resetPractice();
  };

  const resetPractice = () => {
    setTypedText("");
    setDictationSubmitted(false);
    setAccuracy(0);
    setSelectedOption(null);
    setMcSubmitted(false);
  };

  const currentSentence = () => sentences()[currentIndex()] ?? "";

  const handleStoryChange = (e: Event) => {
    const id = (e.target as HTMLSelectElement).value;
    setSelectedStoryId(id);
    if (id) loadStorySentences(id);
  };

  const prevSentence = () => {
    if (currentIndex() > 0) {
      setCurrentIndex((i) => i - 1);
      resetPractice();
    }
  };

  const nextSentence = () => {
    if (currentIndex() < sentences().length - 1) {
      setCurrentIndex((i) => i + 1);
      resetPractice();
    }
  };

  const playAudio = async () => {
    const text = currentSentence();
    if (!text) return;

    // Use cached URL if available
    if (ttsCache.has(text)) {
      if (audioRef) {
        audioRef.src = ttsCache.get(text)!;
        audioRef.play();
      }
      return;
    }

    setIsPlaying(true);
    try {
      const host = `${window.location.protocol}//${window.location.host}`;
      const res = await fetch(`${host}/api/tts`, {
        method: "POST",
        body: JSON.stringify({ text }),
        headers: { "Content-Type": "application/json" },
      });
      if (!res.ok) throw new Error(`TTS failed: ${res.status}`);
      const blob = await res.blob();
      const url = URL.createObjectURL(blob);
      ttsCache.set(text, url);
      if (audioRef) {
        audioRef.src = url;
        audioRef.play();
      }
    } catch (err) {
      console.error("TTS error:", err);
    } finally {
      setIsPlaying(false);
    }
  };

  const submitDictation = () => {
    const acc = charAccuracy(currentSentence(), typedText());
    setAccuracy(acc);
    setDictationSubmitted(true);
  };

  const showTextAndRetry = () => {
    setDictationSubmitted(false);
    setTypedText("");
  };

  const prepareMC = () => {
    const allSents = sentences();
    const current = currentSentence();
    const distractors = getDistractors(current, allSents, 3);
    const options = [...distractors, current].sort(() => Math.random() - 0.5);
    setMcOptions(options);
    setSelectedOption(null);
    setMcSubmitted(false);
  };

  const submitMC = () => {
    if (!selectedOption()) return;
    setMcSubmitted(true);
  };

  // When sentence changes in MC mode, prepare options
  const handleModeChange = (newMode: PracticeMode) => {
    setMode(newMode);
    if (newMode === "multiple-choice") {
      prepareMC();
    }
  };

  // Set default story when stories load
  const defaultStorySet = () => {
    const list = stories();
    if (list && list.length > 0 && !selectedStoryId()) {
      setSelectedStoryId(list[0].id);
      loadStorySentences(list[0].id);
    }
  };

  return (
    <div class="shadowing-practice">
      <style>{`
        .shadowing-practice {
          max-width: 900px;
          margin: 0 auto;
          padding: 1.5rem 2rem;
        }

        .sp-header {
          margin-bottom: 2rem;
        }

        .sp-header h1 {
          font-size: 2rem;
          margin: 0 0 0.5rem 0;
        }

        .sp-controls {
          display: flex;
          align-items: center;
          gap: 1rem;
          flex-wrap: wrap;
          margin-bottom: 1.5rem;
        }

        .story-select {
          background: var(--surface-2);
          border: 1px solid var(--border);
          border-radius: var(--radius-sm);
          color: var(--text);
          font-family: var(--font-body);
          font-size: 0.9rem;
          padding: 0.5rem 0.75rem;
          cursor: pointer;
          outline: none;
          min-width: 240px;
        }

        .story-select:focus {
          border-color: var(--coral);
        }

        .mode-tabs {
          display: flex;
          gap: 0.5rem;
        }

        .mode-tab {
          padding: 0.5rem 1rem;
          border: 1px solid var(--border);
          background: var(--surface-2);
          color: var(--text-2);
          border-radius: 6px;
          cursor: pointer;
          font-size: 0.875rem;
          transition: all var(--transition);
        }

        .mode-tab:hover {
          border-color: var(--coral);
          color: var(--text);
        }

        .mode-tab.active {
          background: var(--coral);
          color: #fff;
          border-color: var(--coral);
        }

        .sentence-nav {
          display: flex;
          align-items: center;
          justify-content: center;
          gap: 1rem;
          margin-bottom: 2rem;
        }

        .sentence-counter {
          font-family: var(--font-mono);
          font-size: 0.875rem;
          color: var(--text-2);
        }

        .nav-btn {
          padding: 0.4rem 0.75rem;
          border: 1px solid var(--border);
          background: var(--surface-2);
          color: var(--text-2);
          border-radius: 6px;
          cursor: pointer;
          font-size: 0.875rem;
          transition: all var(--transition);
        }

        .nav-btn:hover:not(:disabled) {
          border-color: var(--coral);
          color: var(--coral);
        }

        .nav-btn:disabled {
          opacity: 0.4;
          cursor: not-allowed;
        }

        .french-display {
          background: var(--surface);
          border: 1px solid var(--border-subtle);
          border-radius: var(--radius);
          padding: 2rem 2.5rem;
          margin-bottom: 2rem;
          text-align: center;
        }

        .french-sentence {
          font-family: var(--font-mono);
          font-size: 1.5rem;
          line-height: 1.8;
          color: var(--text);
        }

        .audio-controls {
          display: flex;
          justify-content: center;
          gap: 0.75rem;
          margin-bottom: 2rem;
        }

        .play-btn {
          padding: 0.6rem 1.5rem;
          background: var(--coral);
          color: #fff;
          border: none;
          border-radius: var(--radius-pill);
          cursor: pointer;
          font-size: 0.9rem;
          font-weight: 600;
          transition: background var(--transition), transform 0.1s;
        }

        .play-btn:hover:not(:disabled) {
          background: #e63946;
          transform: translateY(-1px);
        }

        .play-btn:active {
          transform: translateY(0);
        }

        .play-btn:disabled {
          opacity: 0.6;
          cursor: not-allowed;
        }

        .play-btn.loading {
          background: var(--surface-3);
          color: var(--text-2);
        }

        /* Dictation mode */
        .dictation-area {
          background: var(--surface);
          border: 1px solid var(--border-subtle);
          border-radius: var(--radius);
          padding: 1.5rem 2rem;
          margin-bottom: 1.5rem;
        }

        .dictation-label {
          font-size: 0.8rem;
          text-transform: uppercase;
          letter-spacing: 0.08em;
          color: var(--text-muted);
          margin-bottom: 0.75rem;
        }

        .dictation-input {
          width: 100%;
          padding: 0.75rem 1rem;
          border: 1px solid var(--border);
          border-radius: var(--radius-sm);
          font-family: var(--font-mono);
          font-size: 1.1rem;
          color: var(--text);
          background: var(--bg);
          outline: none;
          transition: border-color var(--transition);
        }

        .dictation-input:focus {
          border-color: var(--coral);
        }

        .dictation-input:disabled {
          background: var(--surface-2);
          cursor: not-allowed;
        }

        .dictation-actions {
          display: flex;
          justify-content: flex-end;
          gap: 0.75rem;
          margin-top: 1rem;
        }

        .submit-btn {
          padding: 0.5rem 1.25rem;
          background: var(--coral);
          color: #fff;
          border: none;
          border-radius: 6px;
          cursor: pointer;
          font-size: 0.875rem;
          font-weight: 500;
          transition: background var(--transition);
        }

        .submit-btn:hover {
          background: #e63946;
        }

        .submit-btn:disabled {
          opacity: 0.5;
          cursor: not-allowed;
        }

        .retry-btn {
          padding: 0.5rem 1.25rem;
          border: 1px solid var(--border);
          background: var(--surface-2);
          color: var(--text-2);
          border-radius: 6px;
          cursor: pointer;
          font-size: 0.875rem;
          transition: all var(--transition);
        }

        .retry-btn:hover {
          border-color: var(--coral);
          color: var(--coral);
        }

        .dictation-result {
          margin-top: 1rem;
          padding: 1rem 1.25rem;
          border-radius: var(--radius-sm);
          animation: panel-in 0.2s ease;
        }

        .dictation-result.correct {
          background: var(--emerald-soft);
          border: 1px solid var(--emerald);
        }

        .dictation-result.incorrect {
          background: var(--coral-soft);
          border: 1px solid var(--coral);
        }

        .accuracy-badge {
          font-family: var(--font-display);
          font-size: 1.5rem;
          font-weight: 700;
          margin-bottom: 0.5rem;
        }

        .dictation-result.correct .accuracy-badge {
          color: var(--emerald);
        }

        .dictation-result.incorrect .accuracy-badge {
          color: var(--coral);
        }

        .dictation-original {
          font-family: var(--font-mono);
          font-size: 0.9rem;
          color: var(--text-2);
          margin-top: 0.75rem;
        }

        /* Multiple choice mode */
        .mc-area {
          background: var(--surface);
          border: 1px solid var(--border-subtle);
          border-radius: var(--radius);
          padding: 1.5rem 2rem;
        }

        .mc-label {
          font-size: 0.8rem;
          text-transform: uppercase;
          letter-spacing: 0.08em;
          color: var(--text-muted);
          margin-bottom: 1rem;
        }

        .mc-options {
          display: flex;
          flex-direction: column;
          gap: 0.75rem;
        }

        .mc-option {
          padding: 0.75rem 1rem;
          border: 1px solid var(--border);
          border-radius: var(--radius-sm);
          background: var(--bg);
          cursor: pointer;
          font-size: 0.95rem;
          color: var(--text);
          text-align: left;
          transition: all var(--transition);
        }

        .mc-option:hover:not(:disabled) {
          border-color: var(--indigo);
          background: var(--indigo-soft);
        }

        .mc-option.selected {
          border-color: var(--indigo);
          background: var(--indigo-soft);
        }

        .mc-option:disabled {
          cursor: default;
        }

        .mc-option.correct {
          border-color: var(--emerald);
          background: var(--emerald-soft);
        }

        .mc-option.incorrect {
          border-color: var(--coral);
          background: var(--coral-soft);
        }

        .mc-actions {
          display: flex;
          justify-content: flex-end;
          margin-top: 1rem;
        }

        .mc-feedback {
          margin-top: 1rem;
          padding: 1rem 1.25rem;
          border-radius: var(--radius-sm);
          animation: panel-in 0.2s ease;
        }

        .mc-feedback.correct {
          background: var(--emerald-soft);
          border: 1px solid var(--emerald);
          color: var(--emerald);
        }

        .mc-feedback.incorrect {
          background: var(--coral-soft);
          border: 1px solid var(--coral);
          color: var(--coral);
        }

        .mc-feedback-text {
          font-weight: 500;
        }

        .mc-original {
          font-family: var(--font-mono);
          font-size: 0.85rem;
          color: var(--text-2);
          margin-top: 0.5rem;
        }

        .loading-state,
        .empty-state {
          text-align: center;
          padding: 3rem;
          color: var(--text-2);
        }

        .error-state {
          text-align: center;
          padding: 3rem;
          color: var(--error);
        }

        @keyframes panel-in {
          from {
            opacity: 0;
            transform: translateY(6px);
          }
          to {
            opacity: 1;
            transform: translateY(0);
          }
        }

        @media (max-width: 600px) {
          .shadowing-practice {
            padding: 1rem;
          }
          .french-sentence {
            font-size: 1.2rem;
          }
          .story-select {
            min-width: 100%;
          }
        }
      `}</style>

      <audio ref={audioRef} style={{ display: "none" }} />

      <Show when={stories.loading}>
        <div class="loading-state">Loading stories...</div>
      </Show>

      <Show when={stories.error}>
        <div class="error-state">
          Unable to load stories. Make sure the lacq server is running on port 8080.
        </div>
      </Show>

      <Show when={stories()}>
        {() => {
          defaultStorySet();
          return (
            <>
              <div class="sp-controls">
                <select
                  class="story-select"
                  value={selectedStoryId() ?? ""}
                  onChange={handleStoryChange}
                >
                  <option value="" disabled>
                    Select a story...
                  </option>
                  <For each={stories()}>
                    {(story) => (
                      <option value={story.id}>{story.title}</option>
                    )}
                  </For>
                </select>

                <div class="mode-tabs">
                  <button
                    class={`mode-tab ${mode() === "dictation" ? "active" : ""}`}
                    onClick={() => handleModeChange("dictation")}
                  >
                    Dictation
                  </button>
                  <button
                    class={`mode-tab ${mode() === "multiple-choice" ? "active" : ""}`}
                    onClick={() => handleModeChange("multiple-choice")}
                  >
                    Multiple Choice
                  </button>
                </div>
              </div>
            </>
          );
        }}
      </Show>

      <Show when={sentences().length > 0}>
        {() => (
          <>
            <div class="sentence-nav">
              <button
                class="nav-btn"
                onClick={prevSentence}
                disabled={currentIndex() === 0}
              >
                ← Prev
              </button>
              <span class="sentence-counter">
                {currentIndex() + 1} / {sentences().length}
              </span>
              <button
                class="nav-btn"
                onClick={nextSentence}
                disabled={currentIndex() === sentences().length - 1}
              >
                Next →
              </button>
            </div>

            <div class="french-display">
              <Show
                when={
                  mode() !== "dictation" ||
                  !dictationSubmitted() ||
                  typedText()
                }
              >
                <p class="french-sentence">
                  {mode() === "dictation" && dictationSubmitted()
                    ? currentSentence()
                    : currentSentence()}
                </p>
              </Show>
              <Show when={mode() === "dictation" && !dictationSubmitted()}>
                <p
                  class="french-sentence"
                  style={{ "font-style": "italic", color: "var(--text-muted)" }}
                >
                  [French text hidden — type what you hear]
                </p>
              </Show>
            </div>

            <div class="audio-controls">
              <button
                class={`play-btn ${isPlaying() || isWarmingUp() ? "loading" : ""}`}
                onClick={playAudio}
                disabled={isPlaying() || !currentSentence()}
              >
                {isWarmingUp() ? "🔊 Warming up..." : isPlaying() ? "Playing..." : "🔊 Play Audio"}
              </button>
            </div>

            {/* Dictation mode */}
            <Show when={mode() === "dictation"}>
              <div class="dictation-area">
                <div class="dictation-label">
                  {dictationSubmitted()
                    ? "Result"
                    : "Type the sentence you heard"}
                </div>
                <Show when={!dictationSubmitted()}>
                  <input
                    type="text"
                    class="dictation-input"
                    placeholder="Type in French..."
                    value={typedText()}
                    onInput={(e) => setTypedText(e.currentTarget.value)}
                    onKeyDown={(e) => {
                      if (e.key === "Enter" && typedText().trim()) {
                        submitDictation();
                      }
                    }}
                  />
                  <div class="dictation-actions">
                    <button
                      class="submit-btn"
                      onClick={submitDictation}
                      disabled={!typedText().trim()}
                    >
                      Check
                    </button>
                  </div>
                </Show>
                <Show when={dictationSubmitted()}>
                  <div
                    class={`dictation-result ${
                      accuracy() >= 80 ? "correct" : "incorrect"
                    }`}
                  >
                    <div class="accuracy-badge">{accuracy()}% accurate</div>
                    <div>
                      {accuracy() >= 80
                        ? "Great job!"
                        : "Keep practicing!"}
                    </div>
                    <div class="dictation-original">
                      Correct: {currentSentence()}
                    </div>
                  </div>
                  <div class="dictation-actions">
                    <button class="retry-btn" onClick={showTextAndRetry}>
                      Try again without showing text
                    </button>
                  </div>
                </Show>
              </div>
            </Show>

            {/* Multiple choice mode */}
            <Show when={mode() === "multiple-choice"}>
              <div class="mc-area">
                <div class="mc-label">
                  {mcSubmitted()
                    ? "Result"
                    : "Select the correct translation"}
                </div>
                <Show when={!mcSubmitted()}>
                  <div class="mc-options">
                    <For each={mcOptions()}>
                      {(option) => (
                        <button
                          class={`mc-option ${
                            selectedOption() === option ? "selected" : ""
                          }`}
                          onClick={() => setSelectedOption(option)}
                          disabled={mcSubmitted()}
                        >
                          {option}
                        </button>
                      )}
                    </For>
                  </div>
                  <div class="mc-actions">
                    <button
                      class="submit-btn"
                      onClick={submitMC}
                      disabled={!selectedOption()}
                    >
                      Submit
                    </button>
                  </div>
                </Show>
                <Show when={mcSubmitted()}>
                  <div
                    class={`mc-feedback ${
                      selectedOption() === currentSentence()
                        ? "correct"
                        : "incorrect"
                    }`}
                  >
                    <div class="mc-feedback-text">
                      {selectedOption() === currentSentence()
                        ? "✓ Correct!"
                        : "✗ Incorrect"}
                    </div>
                    <div class="mc-original">
                      Correct answer: {currentSentence()}
                    </div>
                  </div>
                </Show>
              </div>
            </Show>
          </>
        )}
      </Show>

      <Show when={!stories.loading && stories()?.length === 0}>
        <div class="empty-state">No stories available.</div>
      </Show>
    </div>
  );
}