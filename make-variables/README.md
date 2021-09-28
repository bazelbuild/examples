# Examples demonstrating "Make" variables

These examples demonstrate Bazel's ["Make"
variable](https://docs.bazel.build/versions/master/be/make-variables.html)
support.

## Predefined variables
`//testapp:show_predefined_variables` demonstrates predefined variables
available to all rules. These are parsed in any attribute marked "Subject to
'Make Variable' substitution".
```
$ bazel build //testapp:show_predefined_variables && cat bazel-bin/testapp/show_predefined_variables.out
COMPILATION_MODE: fastbuild
BINDIR: bazel-out/x86-fastbuild/bin
GENDIR: bazel-out/x86-fastbuild/bin
TARGET_CPU: x86
```

## Predefined genrule variables

### All genrules
`//testapp:show_genrule_variables` demonstrates predefined variables exclusively
available to `genrule`.
```
$ bazel build //testapp:show_genrule_variables && cat bazel-bin/testapp/subdir/show_genrule_variables1.out
SRCS: testapp/show_genrule_variables1.src testapp/show_genrule_variables2.src
OUTS: bazel-out/x86-fastbuild/bin/testapp/subdir/show_genrule_variables1.out bazel-out/x86-fastbuild/bin/testapp/subdir/show_genrule_variables2.out
RULEDIR: bazel-out/x86-fastbuild/bin/testapp
@D (prefer RULEDIR to this): bazel-out/x86-fastbuild/bin/testapp
 * Because this genrule has multiple outputs, @D is the same as RULEDIR.
```

### Genrules with one input or output
`//testapp:single_file_genrule` demonstrates predefined variables exclusively
available to `genrule`s that consume a single source file or produce a single
output file.
```
$ bazel build //testapp:single_file_genrule && cat bazel-bin/testapp/subdir/single_file_genrule.out
<: testapp/show_genrule_variables1.src
@: bazel-out/x86-fastbuild/bin/testapp/subdir/single_file_genrule.out
RULEDIR: bazel-out/x86-fastbuild/bin/testapp
@D: bazel-out/x86-fastbuild/bin/testapp/subdir
 * Because this genrule has one input, < is a valid variable.
 * Because this genrule has one output, @ is a valid variable.
 * Because this genrule has one output, @D is different than RULEDIR.
```

## Predefined path variables
`//testapp:show_app_output` demonstrates predefined variables related to source
and output paths.
```
$ bazel build //testapp:show_app_output && cat bazel-bin/testapp/app_output
:app output paths
 execpath: bazel-out/host/bin/testapp/app
 runfiles: testapp/app
 location: bazel-out/host/bin/testapp/app

source file paths
 execpath: testapp/empty.source
 runfiles: testapp/empty.source
 location: testapp/empty.source
```

## Custom Starlark-defined variables
`//testapp:show_custom_var` demonstrates custom variable defined in Starlrak.
```
$ bazel build //testapp:show_custom_var && cat bazel-bin/testapp/custom_var
Target //testapp:show_custom_var up-to-date:
  bazel-bin/testapp/custom_var
INFO: Build completed successfully, 2 total actions
FOO is equal to bar!
```

