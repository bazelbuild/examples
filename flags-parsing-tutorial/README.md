Bazel flags parsing examples
========================

This provides examples for the current behavior of flag parsing. This tutorial assumes that users have basic knowledge of creating a Bazel [WORKSPACE](https://docs.bazel.build/build-ref.html#workspace) and writing a [BUILD](https://docs.bazel.build/versions/main/build-ref.html#BUILD_files) file. Users should also be familiar with [built-in](https://docs.bazel.build/versions/main/configurable-attributes.html#built-in-flags) (non-Starlark) and [user-defined](https://docs.bazel.build/versions/main/configurable-attributes.html#custom-flags) (Starlark) flags. 

Note that a broader term, [options](https://docs.bazel.build/versions/main/command-line-reference.html#option-syntax) is often used interchangeably with flags. An important distinction is that only flags can be set on the command line. 

Terminologies
========================
`--config`: Throughout this tutorial, users will see regular usage of `--config`. Although, it's already defined [here](https://docs.bazel.build/guide.html#bazelrc), we will repeat important points for first-time Bazel users.
* `--config` can be used to represent a group of flags with a short name following the convention `<command>:<config_name>`. For example:
```
# bazelrc
build:foo --//:wibble=wibble
build:foo --//:wobble=wobble
build:foo --//:wubble=wubble
```
Here, we have defined a `--config` named `foo` expanding flags `--//:wibble, --//:wobble, --//:wubble`.

By default, flags defined in a `--config` will be ignored, unless `--config=<config_name>` is specified either on the command line or in the `bazelrc` file. Flags grouped by `--config` are expanded in place having the same priority with the `--config` expanding them. Please see example B.4.

Instructions
========================

```
cd examples/flags-parsing-tutorial
```
In this WORKSPACE, we have:
* <b>[bazelrc](https://docs.bazel.build/guide.html#bazelrc-the-bazel-configuration-file) file</b>: This is the user-defined bazelrc where flags can be defined.
* <b>build_defs.bzl</b>: This contains the Starlark rules' implementations.
* <b>BUILD</b>: This contains rules Bazel uses to build a package.

For each requirement below, run the following example commands and examine the outputs. 

❗❗❗<i>Please note that there is an outstanding [bug](https://github.com/bazelbuild/bazel/issues/13603) concerning `--config` and Starlark flags which will be pointed out later in this README. Once a fix is released, this doc will be updated.</i>

### A. Flags on the command line take precedence over those in bazelrc. ### 
Without `--config`, this is true for both Starlark and non-Starlark options
```
bazel --bazelrc=./bazelrc build --//:flag=cmd :flag
```
For user-defined (Starlark) flags, the evaluated value can be observed by adding a `DEBUG` statement as in line 6 of `build_defs.bzl`. Users should see the following `DEBUG` statement indicating that the final value for `--//:flag` is `cmd`
```
DEBUG: /my/root/examples/flags-parsing-tutorial/build_defs.bzl:6:10: evaluated value for flag: cmd
```
### B. The last option on the command line takes precedence. ###
⭐ <b>Tips</b>: Use [--announce_rc](https://docs.bazel.build/user-manual.html#flag--announce_rc) to debug flag parsing.

<u><b>Example B.1</b></u>
```
bazel --bazelrc=./bazelrc build --config=foo -c opt --announce_rc :wibble
```
`-c (--compilation_mode)` is first expanded by `--config=foo` with value `dbg`. The flag then got overridden by the explicit `-c opt` flag, which is the last on the command line as seen above. 

Since Bazel uses a different output directories for each compilation mode, users can observe the output by running `ls -l`. In the output path, `opt` should be present.

<u><b>Example B.2</b></u>
```
bazel --bazelrc=./bazelrc build -c opt --config=foo --announce_rc :wibble
```
Since `--config=foo` is last on the command line and flag `-c dbg` is expanded by `--config=foo`, flag `-c dbg` also takes the same precedence. In contrast of <b>Example B.1</b>, `dbg` should be present in the output path.

<u><b>Example B.3</b></u>
```
bazel --bazelrc=./bazelrc build --//:flag=cmd --//:flag=cmd_last :flag
```
Users should see the following `DEBUG` statement indicating that the final value for `--//:flag` is `cmd_last`.
```
DEBUG: /my/root/examples/flags-parsing-tutorial/build_defs.bzl:6:10: evaluated value for flag: cmd_last
```
<u><b>Example B.4</b></u>
```
bazel --bazelrc=./bazelrc build --config=foo --config=bar :wibble :wobble :wubble
```
Requirement <b>B</b> is also applicable for cascading `--config(s)`. `DEBUG` should show `flob` as the evaluated value for `--//:wibble`. Since `config=bar` does not expand `--//:wobble` and `--//:wubble`, their values should stay the same as defined in `--config=foo`.
```
DEBUG: /my/root/examples/flags-parsing-tutorial/build_defs.bzl:6:10: evaluated value for wibble: flob
DEBUG: /my/root/examples/flags-parsing-tutorial/build_defs.bzl:6:10: evaluated value for wubble: wubble
DEBUG: /my/root/examples/flags-parsing-tutorial/build_defs.bzl:6:10: evaluated value for wobble: wobble
```
⭐ <b>Tips</b>: Since `--config` is a group of options and it can override explicit flags specified on the command line (<b>Example B.2</b>), try to have your explicit options at the end to avoid unintentional overriding. In this case, Bazel will show a <b>WARNING</b>.

<u><b>Example B.5</b></u>

❗❗❗ As mentioned above, there is an outstanding [bug](https://github.com/bazelbuild/bazel/issues/13603). Currently, users cannot override Starlark flags expanded by a `--config` via an explicit flag on the command line
```
bazel --bazelrc=./bazelrc build --config=foo --//:wibble=flob :wibble
```
DEBUG should show `flob` for `--//:wibble` per requirement <b>A</b>, but instead `wibble` is shown.
```
DEBUG: /Users/aranguyen/examples/flags-parsing-tutorial/build_defs.bzl:6:10: evaluated value for wibble: wibble
```
### C. Within bazelrc file, precedence depends on specificity which is defined by inheritance ###
Commands such as test and release inherit flags from build. The inheriting command is said to be more specific and thus takes precedence.
```
# -c is resolved to opt, and --//:wibble resolves to flob.
bazel --bazelrc=./bazelrc test --config=baz :wibble --announce_rc
```
⭐ <b>Tips</b>: For readability, have your most common options at the top of bazelrc.
### D. Using `--enable_platform_specific_config` flag
If the value of `--enable_platform_specific_config` is `True`, Bazel enables host-OS-specific flags in the `bazelrc`. For example, considering the `bazelrc` in this WORKSPACE:
```
build --enable_platform_specific_config

build:linux --cpu=arm
build:macos --cpu=k8
build:windows --cpu=x64_windows
build:freebsd --cpu=ppc
build:openbsd --cpu=haswell
```
If the host platform (where Bazel is running) is `macos` and the `build` command is run, Bazel picks up `build:macos` lines in the `bazelrc`. In this example, `build:macos --cpu=k8` will be enabled. Try the following command and observe the output, `k8` should appear in the output path (.../bazel-out/k8-fastbuild/...).
```
bazel --bazelrc=./bazelrc build :wibble
```
Note that Bazel will only enable flags based on the host platform, instead of execution platform or target platform. The definitions for these platforms can be found [here](https://docs.bazel.build/platforms.html). 

