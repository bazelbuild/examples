# Frontends with Bazel

This folder contains various examples for writing JavaScript applications with Bazel.

Bazel's [rules_js] uses the pnpm package manager. This folder is the root of a pnpm workspace.
This allows npm packages within this monorepo to depend on each other.

## Linting

We demonstrate the usage of [rules_lint]. There are a few ways to wire this up, we show two:
- in the `next.js/` folder, `npm run lint` does a `bazel build` with a config setting that makes the build fail when lint violations are found.
- in the `react` folder, an `eslint_test` target results in test failures when lint violations are found.

[rules_js]: https://docs.aspect.build/rules/aspect_rules_js
[rules_lint]: https://github.com/aspect-build/rules_lint
