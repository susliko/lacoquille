// @ts-check
import { defineConfig } from 'astro/config';
import solidJs from '@astrojs/solid-js';

export default defineConfig({
  integrations: [solidJs()],
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