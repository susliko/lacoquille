import { Show } from "solid-js";

export default function VocabularyPractice() {
  return (
    <div class="vocab-practice">
      <style>{`
        .vocab-practice {
          max-width: 700px;
          margin: 0 auto;
          padding: 2rem;
        }

        .coming-soon-card {
          background: var(--surface);
          border: 1px solid var(--border-subtle);
          border-radius: var(--radius);
          padding: 3rem 2.5rem;
          text-align: center;
          animation: fade-in 0.3s ease;
        }

        @keyframes fade-in {
          from { opacity: 0; transform: translateY(8px); }
          to   { opacity: 1; transform: translateY(0); }
        }

        .coming-soon-badge {
          display: inline-block;
          background: var(--amber-soft);
          color: var(--amber);
          border: 1px solid var(--amber);
          border-radius: var(--radius-pill);
          padding: 0.25rem 0.75rem;
          font-size: 0.75rem;
          font-weight: 600;
          letter-spacing: 0.05em;
          text-transform: uppercase;
          margin-bottom: 1.5rem;
        }

        .vocab-heading {
          font-family: var(--font-display);
          font-size: 2rem;
          font-weight: 700;
          color: var(--text);
          margin: 0 0 0.75rem 0;
        }

        .vocab-sub {
          font-size: 1.1rem;
          color: var(--text-2);
          margin-bottom: 2rem;
          line-height: 1.6;
        }

        .vocab-description {
          background: var(--surface-2);
          border-radius: var(--radius-sm);
          padding: 1.25rem 1.5rem;
          margin-bottom: 2rem;
          text-align: left;
        }

        .vocab-description p {
          margin: 0 0 0.75rem 0;
          color: var(--text-2);
          font-size: 0.95rem;
          line-height: 1.6;
        }

        .vocab-description p:last-child {
          margin-bottom: 0;
        }

        .vocab-alternative {
          background: var(--indigo-soft);
          border: 1px solid var(--indigo);
          border-radius: var(--radius-sm);
          padding: 1rem 1.25rem;
          margin-bottom: 2rem;
          text-align: left;
        }

        .vocab-alternative p {
          margin: 0;
          color: var(--indigo);
          font-size: 0.9rem;
          line-height: 1.5;
        }

        .vocab-actions {
          display: flex;
          justify-content: center;
          gap: 1rem;
          flex-wrap: wrap;
        }

        .back-link {
          display: inline-flex;
          align-items: center;
          gap: 0.5rem;
          padding: 0.6rem 1.25rem;
          border: 1px solid var(--border);
          background: var(--surface-2);
          color: var(--text-2);
          border-radius: var(--radius-sm);
          text-decoration: none;
          font-size: 0.9rem;
          font-weight: 500;
          transition: all var(--transition);
        }

        .back-link:hover {
          border-color: var(--coral);
          color: var(--coral);
        }

        .alt-link {
          display: inline-flex;
          align-items: center;
          gap: 0.5rem;
          padding: 0.6rem 1.25rem;
          background: var(--coral);
          color: #fff;
          border-radius: var(--radius-sm);
          text-decoration: none;
          font-size: 0.9rem;
          font-weight: 500;
          transition: background var(--transition);
        }

        .alt-link:hover {
          background: #e63946;
        }

        @media (max-width: 600px) {
          .vocab-practice {
            padding: 1.5rem 1rem;
          }
          .vocab-heading {
            font-size: 1.5rem;
          }
        }
      `}</style>

      <div class="coming-soon-card">
        <div class="coming-soon-badge">Coming Soon</div>
        <h1 class="vocab-heading">Vocabulary Mining</h1>
        <p class="vocab-sub">Mine texts for words</p>

        <div class="vocab-description">
          <p>
            <strong>Vocabulary Mining</strong> will let you pull vocabulary from
            the Article of the Day text — automatically extracting key words,
            phrases, and their context so you can build your personal word list
            while reading.
          </p>
          <p>
            Select any passage, highlight words you want to learn, and save them
            to your vocabulary bank for later review. Mining has never been this
            easy.
          </p>
        </div>

        <div class="vocab-alternative">
          <p>
            💡 In the meantime, try the <strong>Article of the Day</strong> to read
            Maupassant stories in French — the best way to encounter vocabulary in
            context.
          </p>
        </div>

        <div class="vocab-actions">
          <a href="/practice" class="back-link">
            ← Back to practice hub
          </a>
          <a href="/stories/article-of-the-day" class="alt-link">
            Read Article of the Day →
          </a>
        </div>
      </div>
    </div>
  );
}
