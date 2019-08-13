# Example 01: Hello World

Example demonstrating a workspace with toolchains and platforms setup.

## Commands

```
bazel build //examples/01_hello_world:a

>   yolo_library(
>     name = 'a',
>     toolchain = {
>       'targetting_cpu': 'host',
>       'targetting_os': 'host',
>       'executing_on_cpu': 'host',
>       'executing_on_os': 'host',
>     },
>   )
```

## Description

There are a few relevant pieces that have to click in for this example to work.
Be sure to look into top-level `WORKSPACE`, `BUILD`, `yolo/BUILD`, and
`.bazelrc` files.

We don't tell Bazel which `--platforms` to use, so Bazel takes its defaults, which is
the autodetected host platform at `@local_config_platform//:host`. Bazel selects
the first toolchain that matches that platform. Our toolchain
`//yolo:host_toolchain` has exactly the same constraints, and it indeed matches
and Bazel selected it.
