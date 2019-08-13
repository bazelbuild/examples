# Example 02: Using different platforms

Example demonstrating how different toolchains are selected for different
platforms.

## Commands

```
bazel build //examples/02_using_different_platforms:a --platforms=//:linux_platform

>   yolo_library(
>     name = 'a',
>     toolchain = {
>       'targetting_cpu': '-',
>       'targetting_os': 'linux',
>       'executing_on_cpu': '-',
>       'executing_on_os': 'linux',
>     },
>   )

bazel build //examples/02_using_different_platforms:a --platforms=//:windows_platform

>   yolo_library(
>     name = 'a',
>     toolchain = {
>       'targetting_cpu': '-',
>       'targetting_os': 'windows',
>       'executing_on_cpu': '-',
>       'executing_on_os': 'linux',
>     },
>   )
```

## Description

Here we tell Bazel that we want to build
`//examples/02_using_different_platforms:a` first for Linux, then for Windows.
Both of these times we want to execute the build on Linux (note the call to
`register_execution_platforms` in the `WORKSPACE` file).

We see that toolchains used to build the library changed between Bazel
invocations.
