import { createResource, For, Show } from "solid-js";

interface ArticleData {
  title: string;
  source: string;
  published_year: number;
  paragraphs: string[];
}

async function fetchArticle(): Promise<ArticleData> {
  const res = await fetch("/api/article-of-the-day");
  if (!res.ok) throw new Error(`Failed to fetch article: ${res.status}`);
  return res.json();
}

export default function ArticleOfTheDay() {
  const [article] = createResource(fetchArticle);

  return (
    <div class="article-of-the-day">
      <style>{`
        .article-of-the-day {
          max-width: 720px;
          margin: 0 auto;
          padding: 1.5rem 2rem;
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
        .article-body p {
          line-height: 2;
          margin-bottom: 1.5rem;
          font-size: 1.1rem;
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
        </div>
      </Show>

      <Show when={article()}>
        {(data) => (
          <>
            <header class="article-header">
              <h1>{data().title}</h1>
              <p class="article-meta">
                {data().source} ({data().published_year})
              </p>
            </header>
            <div class="article-body">
              <For each={data().paragraphs}>
                {(para) => <p>{para}</p>}
              </For>
            </div>
          </>
        )}
      </Show>
    </div>
  );
}
