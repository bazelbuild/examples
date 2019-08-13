# Example 03: Execution of target not compatible with constraint

Example demonstrating how to express that a target can only be built on a
platform that has a specific constraint.

## Commands

```
bazel build //examples/03_target_not_compatible_with_constraint:a --platforms=//:linux_platform

>   yolo_library(
>     name = 'a',
>     toolchain = {
>       'targetting_cpu': '-',
>       'targetting_os': 'linux',
>       'executing_on_cpu': '-',
>       'executing_on_os': 'linux',
>     },
>   )

bazel build //examples/03_target_not_compatible_with_constraint:b --platforms=//:linux_platform

>   ERROR: While resolving toolchains for target //examples/03_target_not_compatible_with_constraint:b:
>       no matching toolchains found for types //yolo:toolchain_type
>   ERROR: Analysis of target '//examples/03_target_not_compatible_with_constraint:b' failed;
>       build aborted: no matching toolchains found for types //yolo:toolchain_type
```

## Description

In this example we show how to prevent people from building a target on an
unsupported platform (in this case `:b` can only be built on Windows).
