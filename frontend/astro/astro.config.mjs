import { defineConfig } from "astro/config";

// https://astro.build/config
export default defineConfig({
  vite: {
    server: {
      fs: {
        // allows loading astro dev toolbar even if outside of vite serving allow list
        strict: import.meta.env.PROD,
      },
    },
  },
});
