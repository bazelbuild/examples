### Example showing how to use [Starlark configuration](https://bazel.build/extending/config) to write a
`cc_test` wrapper with a starlark transition

the `test_arg_cc_test` macro in `defs.bzl` defines a wrapper for basically a `cc_test` that has been transitioned.
This allows, e.g., the test itself to select attribute values based on the value of that transition. There is some
light magic in the `transition_rule` implementation that allows dependents of the `test_arg_cc_test` macros to
treat the targets it creates the exact same as a regular `cc_test`.

To run this example:

```
$ bazel test :all --test_output=all
```

```
==================== Test output for //:my-test:

################################################################################
MYTEST ARGV[0]: .../bazel-out/k8-fastbuild-ST-54535d7cadf4/bin/my-test.runfiles/__main__/my-test
MYTEST ARGV[1]: x
MYTEST ARGV[2]: y
MYTEST ARGV[3]: z
MYTEST ARGV[4]: new arg
################################################################################

================================================================================
```

Known limitation:

`bazel test :all --test_output=all --test_arg=a` does not work as expected (`--test_arg=a` is ignored).
This could be fixed with the approach shown under in the ../read_attr_in_transition example,
but it is omitted here to not make this example overly complex.
