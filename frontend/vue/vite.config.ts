import { fileURLToPath, URL } from 'url';

import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import vueJsx from '@vitejs/plugin-vue-jsx';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(), vueJsx()],
  resolve: {
    alias: {
      // WORKAROUND: Use process.cwd() instead of import.meta.url to avoid sandbox escape
      // See https://github.com/bazelbuild/examples/issues/614
      // @ts-ignore - We know process exists at runtime
      '@': process.cwd() + '/src',
    },
    dedupe: ['vue'],
  },
});
