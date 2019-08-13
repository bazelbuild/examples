# Example 08: Using build_setting

This example demonstrates how to use Starlark configuration options.

## Commands

```
bazel build //examples/08_using_build_setting:a

>   yolo_library(
>     name = 'a',
>     toolchain = {
>       'targetting_cpu': 'host',
>       'targetting_os': 'host',
>       'executing_on_cpu': 'host',
>       'executing_on_os': 'host',
>     },
>   )

bazel build //examples/08_using_build_setting:a --//examples/08_using_build_setting:foo_enabled

>   yolo_library(
>     name = 'a',
>     toolchain = {
>       'targetting_cpu': 'host',
>       'targetting_os': 'host',
>       'executing_on_cpu': 'host',
>       'executing_on_os': 'host',
>     },
>   )
>   yolo_library(
>     name = 'only_with_foo',
>     toolchain = {
>       'targetting_cpu': 'host',
>       'targetting_os': 'host',
>       'executing_on_cpu': 'host',
>       'executing_on_os': 'host',
>     },
>   )

bazel build //examples/08_using_build_setting:a --//examples/08_using_build_setting:foo_enabled=0

>   yolo_library(
>     name = 'a',
>     toolchain = {
>       'targetting_cpu': 'host',
>       'targetting_os': 'host',
>       'executing_on_cpu': 'host',
>       'executing_on_os': 'host',
>     },
>   )

bazel build //examples/08_using_build_setting:a --//examples/08_using_build_setting:foo_enabled=1

>   yolo_library(
>     name = 'a',
>     toolchain = {
>       'targetting_cpu': 'host',
>       'targetting_os': 'host',
>       'executing_on_cpu': 'host',
>       'executing_on_os': 'host',
>     },
>   )
>   yolo_library(
>     name = 'only_with_foo',
>     toolchain = {
>       'targetting_cpu': 'host',
>       'targetting_os': 'host',
>       'executing_on_cpu': 'host',
>       'executing_on_os': 'host',
>     },
>   )
```

## Description

Here we show how to use Bazel's [Starlark configuration
options](https://docs.bazel.build/versions/master/skylark/config.html) to
express exactly what exercise 07 - Using --define does. To simplify our example
we make use of [Skylib's
common_settings.bzl](https://github.com/bazelbuild/bazel-skylib/blob/master/rules/common_settings.bzl).
