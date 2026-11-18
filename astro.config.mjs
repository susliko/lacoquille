// @ts-check
import { defineConfig } from 'astro/config';
import solidJs from '@astrojs/solid-js';
import { readdirSync, readFileSync } from 'node:fs';

/* Knowledge redesign: /reference/* folded into /knowledge/* — static builds
   need one redirect entry per real URL (no :param patterns in `dist/`).
   Verb-tense pages keep their URLs (they became the conjugation-table
   pages); everything else maps to its sheet in redirects.json. */
const knowledgeRedirects = JSON.parse(
  readFileSync('./src/data/knowledge/redirects.json', 'utf8')
);
delete knowledgeRedirects['_comment'];
const verbRedirects = /** @type {Record<string, string>} */ ({
  '/reference/verbs': '/knowledge/verbs',
});
for (const [sub] of [
  ['tenses'],
]) {
  verbRedirects[`/reference/verbs/${sub}`] = '/knowledge/verbs';
  try {
    for (const f of readdirSync(`./src/content/verbs/${sub}`)) {
      const slug = f.replace(/\.md$/, '');
      if (slug === 'index') continue;
      verbRedirects[`/reference/verbs/${sub}/${slug}`] = `/knowledge/verbs/${sub}/${slug}`;
    }
  } catch { /* dir absent — nothing to redirect */ }
}

export default defineConfig({
  integrations: [solidJs()],
  redirects: { ...knowledgeRedirects, ...verbRedirects },
  // Prefetch on tap (mousedown/touchstart) so the ~100ms before the
  // click lands are spent fetching the next page — pairs with
  // ClientRouter to make navigation feel instantaneous.
  prefetch: {
    prefetchAll: false,
    defaultStrategy: "tap",
  },
  vite: {
    server: {
      port: 4321,
      proxy: {
        '/api': {
          target: 'http://localhost:8080',
          changeOrigin: true,
        },
      },
    },
  },
});