# Bazel Platforms Examples

*This repo requires Bazel 0.28 or later (or built from HEAD).*

This repo contains a collection of examples demonstrating how to use various
Bazel concepts related to
[platforms](https://docs.bazel.build/versions/master/platforms.html),
[toolchains](https://docs.bazel.build/versions/master/toolchains.html),
[configurations](https://docs.bazel.build/versions/master/skylark/config.html),
and [configurable
attributes](https://docs.bazel.build/versions/master/configurable-attributes.html).

It also tries to give guidance when each of these concepts is used and should
accompany documentation on [bazel.build](https://bazel.build). Be sure to use
[`--toolchain_resolution_debug`](https://docs.bazel.build/versions/master/command-line-reference.html#flag--toolchain_resolution_debug)
where the resolution isn't obvious.

## Structure

```
├── .bazelrc      # Setting defaults until https://github.com/bazelbuild/bazel/issues/7081 is fixed.
│
├── WORKSPACE     # Here we define @platforms repo with constraints. We use common
│                 # constraints repository from
│                 # https://github.com/bazelbuild/platforms. We also register
│                 # toolchains and we register execution platforms there.
│
├── BUILD         # Here we define all needed 'platform' targets that we then use
│                 # in examples.
│
├── examples      # Actual examples, one per subpackage.
│
└── yolo          # Yolo-lang rules definition.
    │
    ├── BUILD     # Here we define all 'toolchain' targets that we then use in
    │             # in examples.
    │
    └── defs.bzl
```

`Yolo-lang` here is obviously not a real programming language, it's just a
simple implementation of Bazel Starlark rules that is meant to demonstrate
examples without confusing us with technical details.

