# vue-project

Created by running `npm init vue@latest` following https://vuejs.org/guide/quick-start.html#local

Then adding Bazel configuration files.

Install local tooling: `npx pnpm i`

Devmode: `npm run dev`

Typecheck and build: `npm run build`

Note: this project simply wraps the Vite build system with Bazel.
This doesn't provide any incrementality benefits of Bazel, because it just runs a single action
when any file changes, which calls through to Vite.
Furthermore we didn't teach Vite the ["ibazel_notify_changes protocol"](https://github.com/bazelbuild/bazel-watcher#running-a-target) so every time the code changes, the devserver restarts from scratch.

However, Vite is a lot faster than Webpack, so at a small scale like this, this developer roundtrip is actually fine.

Also, we show how to extract vue components as pre-built npm packages, using our pnpm workspaces support to link these into an app.
See the `libraries/` folder and the `pnpm-workspace.yaml` file.
This makes the build more incremental since those packages ("component libraries") are not be re-built every time.

To scale up the example further, and to be more Bazel-idiomatic, the Vite composition of tools like `esbuild` and  plugins like `@vitejs/plugin-vue` could be decomposed into an analogous Bazel pipeline.
