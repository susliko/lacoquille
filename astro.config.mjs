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
  // Inline the CSS into every page's HTML. A <link> stylesheet is one
  // more round-trip a cold machine or a flaky mobile connection can
  // lose — and the router swaps DOM in without waiting for it, so a
  // pending stylesheet paints as unstyled raw HTML (the 600ms flash
  // on tab switches). Inlined styles make that state impossible.
  build: {
    inlineStylesheets: "always",
  },
  // Prefetch every link as it scrolls into view — paired with ClientRouter,
  // tapping a lesson card swaps in an already-downloaded page. Cards are
  // ~12KB; Astro throttles viewport prefetching.
  prefetch: {
    prefetchAll: true,
    defaultStrategy: "viewport",
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