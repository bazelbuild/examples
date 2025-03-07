"Define linters as aspects"

load("@aspect_rules_lint//lint:eslint.bzl", "lint_eslint_aspect")
load("@aspect_rules_lint//lint:lint_test.bzl", "lint_test")

# NB: use of explicit Label constructor avoids our strings being interpreted
# in the context of aspect_rules_lint.
eslint = lint_eslint_aspect(
    binary = Label(":eslint"),
    configs = [
        Label("//:eslintrc"),
        Label("//react:package_json"),
        Label("//next.js:eslintrc"),
    ],
)

eslint_test = lint_test(aspect = eslint)
