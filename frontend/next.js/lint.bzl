"Define linters as aspects"

load("@aspect_rules_lint//lint:eslint.bzl", "eslint_aspect")
load("@aspect_rules_lint//lint:lint_test.bzl", "make_lint_test")

eslint = eslint_aspect(
    binary = "@@//next.js:eslint",
    config = "@@//next.js:eslintrc",
)

eslint_test = make_lint_test(aspect = eslint)
