
# Hello World with CLang

In the previous example, it was required to have a C/C++ toolchain on the host installed. 
However, this is not always wanted and this example shows how to setup the toolchain so that Bazel doesn't use the system's C++ toolchain. This improves hermeticity, making the build more portable and reproducible on other computers.

## Configuration 

First, you have to declare the toolchains_llvm dependency and then configure the toolchain to ensure Bazel downloads the LLVM toolchain matching the host.

Add to your WORKSPACE file:

```Starlark
module(
    name = "hello_clang",
    version = "0.0.0"
)
 
# https://github.com/bazelbuild/rules_rust/releases
bazel_dep(name = "rules_rust", version = "0.46.0")
# https://github.com/bazel-contrib/toolchains_llvm
bazel_dep(name = "toolchains_llvm", version = "1.0.0")

  
llvm = use_extension("@toolchains_llvm//toolchain/extensions:llvm.bzl", "llvm")
LLVM_VERSIONS = {
    "": "16.0.0",
}

# LLVM toolchain.
llvm.toolchain(
    name = "llvm_toolchain",
    llvm_versions = LLVM_VERSIONS,
)
use_repo(llvm, "llvm_toolchain", "llvm_toolchain_llvm")

# Register all LLVM toolchains
register_toolchains("@llvm_toolchain//:all")

 
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

 Now run your build:

`bazel build //...`

And run your target:

`bazel run //hello_clang:bin`

