Bazel flags parsing examples
========================

This provides examples for the current behavior of flag/options parsing. This tutorial assumes that users have basic knowledge of creating a Bazel [WORKSPACE](https://docs.bazel.build/versions/main/build-ref.html#workspace) and writing a [BUILD](https://docs.bazel.build/versions/main/build-ref.html#BUILD_files) file. Users should also know what [options/flags](https://docs.bazel.build/versions/main/command-line-reference.html#option-syntax) including built-in (non-Starlark) and user-defined(Starlark) options are.

Instructions
========================

```
cd examples/flags-parsing-tutorial
```
In this WORKSPACE, we have:
* <b>bazelrc file</b>: This is the user-defined bazelrc where options/flags can be defined.
* <b>build_defs.bzl</b>: This contains the Starlark rules' implementations.
* <b>BUILD</b>: This contains rules Bazel uses to build a package.

For each requirement below, run the following example command and examine output. 

❗❗❗<i>Please note that there is an outstanding bug concerning `--config` and Starlark flags which will be pointed out later in this README. Once a fix is released, this doc will be updated.</i>

##A. Options on the command line takes precedence over those in bazelrc. 
  
Without `--config`, this is true for both Starlark and non-Starlark options
```
# DEBUG should show cmd for option flag
bazel --bazelrc=./bazelrc build --//:flag=cmd :flag
```
##B. The last flag on the command line takes precedence.
⭐ <b>Tips</b>: Use [--announce_rc](https://docs.bazel.build/versions/main/command-line-reference.html#options-common-to-all-commands) to debug options parsing
```
# Since -c is an output affecting option, the "winning" option 
# will be reflected in the output paths (i.e opt). To see this run `ls -l`
bazel --bazelrc=./bazelrc build --config=foo -c opt --announce_rc

# Here the winning option is that defined in the bazelrc file (i.e dbg).
# Note that -c=dbg is expanded by --config=foo, the last option on the command line
bazel --bazelrc=./bazelrc build -c opt --config=foo --announce_rc

# DEBUG should show cmd_last as the "winning" value.
bazel --bazelrc=./bazelrc build --//:flag=cmd --//:flag=cmd_last :flag

# This is also applicable for cascading --config(s)
# DEBUG should show flob as value for --//:wibble option
bazel --bazelrc=./bazelrc build --config=foo --config=bar :wibble :wobble :wubble
```
⭐ <b>Tips</b>: Since `--config` is a group of options and it can override explicit options (Example 2), try to have your explicit options at the end to avoid unintentional overriding. In this case, Bazel will show a <b>WARNING</b>.

❗❗❗ As mentioned above, there is an outstanding bug. Currently, users cannot override options associated with a `--config` via an explicit flag the command line
```
# DEBUG should show flob for --//:wibble per A, but instead wibble is shown
bazel --bazelrc=./bazelrc build --config=foo --//:wibble=flob :wibble
```
##C. Within bazelrc file, precedence depends on specificity which is defined by inheritance
Commands such as test and release inherit options from build. The inheriting command is said to be more specific and thus takes precedence.
```
# -c is resolved to opt, and --//:wibble resolves to flob.
bazel --bazelrc=./bazelrc test --config=baz :wibble --announce_rc
```
⭐ <b>Tips</b>: For readability, have your most common options at the top of bazelrc.


