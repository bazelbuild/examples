# Example 04: Select on constraint

This example demonstrates how to build a target (or set it's attribute)
differently under different configurations.

## Commands

```
bazel build //examples/04_select_on_constraint:a --platforms=//:linux_platform

>   yolo_library(
>     name = 'a_linux',
>     toolchain = {
>       'targetting_cpu': '-',
>       'targetting_os': 'linux',
>       'executing_on_cpu': '-',
>       'executing_on_os': 'linux',
>     },
>   )

bazel build //examples/04_select_on_constraint:a --platforms=//:windows_platform

>   yolo_library(
>     name = 'a_windows',
>     toolchain = {
>       'targetting_cpu': '-',
>       'targetting_os': 'windows',
>       'executing_on_cpu': '-',
>       'executing_on_os': 'linux',
>     },
>   )

bazel build //examples/04_select_on_constraint:a --platforms=//:android_platform

>   .../examples/04_select_on_constraint/BUILD:6:14:
>     Configurable attribute "actual" doesn't match this configuration (would a default condition help?).
>   Conditions checked:
>    //examples/04_select_on_constraint:is_windows
>    //examples/04_select_on_constraint:is_linux
```

# Description

In this example we wanted select a specialized target depending on the operating
system. We used
[`alias`](https://docs.bazel.build/versions/master/be/general.html#alias) to
create the entry-point target and made only that target visible to the outside.
We used
[`config_setting.constraint_values`](https://docs.bazel.build/versions/master/be/general.html#config_setting.constraint_values)
attribute to select on constraint values of the currently used platform. And we
used 
[`select`](https://docs.bazel.build/versions/master/be/functions.html#select) to
conditionally take the right target. Note we didn't include
`//conditions:default` in the select. As a result, the third Bazel invocation
fails on missing condition.
