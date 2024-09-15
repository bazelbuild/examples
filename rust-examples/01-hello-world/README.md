
# Hello World 

In this first example, the goal is to build a minimal Hello World Rust binary with Bazel.

## Requirements

On your computer, you need:

* Cargo & Rust
* C compiler (gcc, or clang)

## Configuration 

A minimal MODULE.bazel file for Rust contains three parts:

1) A module declaration with a name and version
2) Loading the Rust rules
3) Configuration of the Rust toolchain

Add to your MODULE.bazel file:

```Starlark
module(
    name = "hello_world",
    version = "0.0.0"
)
 
# https://github.com/bazelbuild/rules_rust/releases
bazel_dep(name = "rules_rust", version = "0.50.1")

# Rust toolchain
RUST_EDITION = "2021"
RUST_VERSION = "1.79.0"

rust = use_extension("@rules_rust//rust:extensions.bzl", "rust")
rust.toolchain(
    edition = RUST_EDITION,
    versions = [RUST_VERSION],
)
use_repo(rust, "rust_toolchains")
register_toolchains("@rust_toolchains//:all")
```

## Rust targets

Next, you declare your Rust binary target in a BUILD file.

```Starlark
load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "bin",
    srcs = ["src/main.rs"],
    deps = [],
    visibility = ["//visibility:public"],
)
```

This is it. Now run your build:

`bazel build //...`

And run your target:

`bazel run //hello_world:bin`

