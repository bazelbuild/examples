# React Webpack Bazel example

This example shows how to transpile and bundle a React JS application using SWC for transpilation
and Webpack for bundling.

The Bazel build is build from two rules:

- transpilation, handled by rules_swc
- bundling, handled by rules_webpack

The transpilation is converting the JSX files to plain JS files, so that Webpack can use them
directly.
Then Webpack bundles the simple JS files in a single JS file.
Also, Webpack has an HTML template plugin added, so the result can be seen in a browser.
The plugin is added as a part of the Webpack rule.

There are two configurations for Webpack added:

- webpack.bazel.config.js - the configuration that Bazel uses
- webpack.config.js - a sample configuration for showcasing what is the difference when using Bazel
  with Webpack

## Install

Only needed if you want to run Webpack not through Bazel, in frontend folder run:

```shell
pnpm i
```

## Build

To run the Bazel build execute the following:

```shell
bazel build //react-webpack/...
```

To run the Non-Bazel Webpack build in this folder run:

```shell
pnpm build
```

## Dev server

To run the Bazel dev server execute the following:

```shell
ibazel run //react-webpack:dev_server
```

To run the Non-Bazel Webpack dev server in this folder run:

```shell
pnpm start
```
