### Example showing how to use [Starlark configuration](https://docs.bazel.build/versions/master/skylark/config.html) to write a
`cc_test` wrapper with a starlark transition

the `test_arg_cc_test` macro in `defs.bzl` defines a wrapper for basically a cc_test that has been transitioned. 
This allows, e.g., the test itself to select attribute values based on the value of that transition. There is some
light magic in the `transition_rule` implementation that allows dependents of the `test_arg_cc_test` macros to
treat the targets it creates the exact same as a regular cc test.