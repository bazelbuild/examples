# Example 05: Select on platform

*Don't do this. Read the reasoning below.*

## Commands

```
bazel build //examples/05_select_on_platform:a --platforms=//:linux_platform

>   yolo_library(
>     name = 'a_linux',
>     toolchain = {
>       'targetting_cpu': '-',
>       'targetting_os': 'linux',
>       'executing_on_cpu': '-',
>       'executing_on_os': 'linux',
>     },
>   )

bazel build //examples/05_select_on_platform:a --platforms=//:windows_platform

>   yolo_library(
>     name = 'a_windows',
>     toolchain = {
>       'targetting_cpu': '-',
>       'targetting_os': 'windows',
>       'executing_on_cpu': '-',
>       'executing_on_os': 'linux',
>     },
>   )
```

## Description

In this example we demonstrate that one can also select on bare `--platforms`
option. That's not recommended though:

1) In this example we select on the target platform. When the target that we
   select in is used during the build as a tool to build something else (= our
   target will be built in the host configuration), the select will be
   incorrect.
2) While `constraint_setting`s and `constraint_value`s tend to be universal,
   like operating system or cpu architecture (reasoning behind having one
   unified repository for the whole Bazel ecosystem and existence of
   [https://github.com/bazelbuild/platforms](https://github.com/bazelbuild/platforms)
   shows that),
   [`platform`](https://docs.bazel.build/versions/master/be/platform.html#platform)
   targets are very project/company specific (e.g. 'description of company
   servers', or 'specific mobile device', or 'development board for IoT
   project'). If there is any possibility that somebody else will use your
   project, select on constraints, not on platforms. By selecting on
   `--platforms` you'll force them to use your idea of platforms, or to fork
   your project.
