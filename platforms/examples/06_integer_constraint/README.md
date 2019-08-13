# Example 06: Matching multiple constraints

Example that demonstrates how to match multiple constraints of the same
`constraint_setting`.

## Commands

```
bazel build //examples/06_integer_constraint:a --platforms=//:linux_platform 

>   .../examples/06_integer_constraint/BUILD:7:14:
>     Configurable attribute "actual" doesn't match this configuration (would a default condition help?).
>   Conditions checked:
>    //examples/06_integer_constraint:yolo_lang_3

bazel build //examples/06_integer_constraint:a --platforms=//:linux_yolo_3_platform --host_platform=//:linux_yolo_3_platform

>   yolo_library(
>     name = 'a_needs_greater_or_equal_than_2',
>     toolchain = {
>       'targetting_cpu': '-',
>       'targetting_os': 'linux_with_yolo_lang_3',
>       'executing_on_cpu': '-',
>       'executing_on_os': 'linux_with_yolo_lang_3',
>     },
>   )
```

## Description

[config_setting](https://docs.bazel.build/versions/master/be/general.html#config_setting)
handles AND conditions natively. In this example we show how to express OR
condition using a little help of macros from
[Skylib](https://github.com/bazelbuild/bazel-skylib).

In this example we show how to emulate greater-than and less-than using selects.
Similar mechanism can be used to express hierarchical constraints. We can have a
`config_setting` that encodes that 'haswell' cpu implies 'SSE':

```
selects.config_setting_group(
    name = "has_sse",
    match_any = ["//cpu_extensions:sse3", "//cpu:haswell", ...],
)
```

Or a `config_setting` that encodes that Ubuntu is Linux:

```
selects.config_setting_group(
    name = "is_linux",
    match_any = ["//distributions:ubuntu", "@platforms//os:linux", ...],
)
```
