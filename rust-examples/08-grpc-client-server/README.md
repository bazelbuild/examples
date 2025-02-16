# gRPC Client & Server

This example shows how to build a gRPC server and client in Rust with Bazel.
There is a Cargo Workspace configuration and a Bazelmod configuration. Furthermore,
all binary targets apply optimization from the [compiler optimization example](../03-comp-opt). 


To run the example with Cargo, open one terminal and start the server with:

`
cargo run --bin grpc_server
`

And, in a second terminal, to run the client:

`
cargo run --bin grpc_client
`

The equivalent Bazel targets are:

Server:

`bazel run //grpc_server:bin`

Client:

`bazel run //grpc_client:bin`

Build with optimization:

`bazel build -c opt //grpc_server:bin`

And run the optimized binary:

`bazel run -c opt //grpc_server:bin`

See the compiler [optimization example](../03-comp-opt) for configuration details. 

## Updating dependencies

* Add or update dependencies in `thirdparty/BUILD.bazel`.
* Then run the vendoring target: `bazel run //thirdparty:crates_vendor`
* Rebuild the project: `bazel build //...`
* Run all tests: `bazel test //...`

See the [vendoring example](../07-deps-vendor) for details.

## Setup

The Prost and Tonic rules do not specify a default toolchain in order to avoid mismatched dependency issues. 
While the Tonic toolchain works out of the box when its dependencies are matched, however,
Prost requires a custom toolchain that you have to define.

The setup requires three steps to complete: 
1. Configure rules and dependencies vendoring 
2. Configure a custom Prost toolchain
3. Register custom Prost toolchain.

To keep the build hermetic, we use the LLVM Clang compiler to compile all C/C++ dependencies. 

### 1) Configure rules 

### Rules

In your MODULE.bazel, you add the following:

```starlark
# rules for proto
###############################################################################
bazel_dep(name = "bazel_skylib", version = "1.7.1")
bazel_dep(name = "rules_rust", version = "0.57.1")
bazel_dep(name = "rules_rust_prost", version = "0.57.1")
bazel_dep(name = "rules_proto", version = "7.1.0")

# Toolchains
bazel_dep(name = "toolchains_protoc", version = "0.3.7", dev_dependency = True)
bazel_dep(name = "toolchains_llvm", version = "1.2.0", dev_dependency = True)
```

### Configure clang/LLVM
```
llvm = use_extension("@toolchains_llvm//toolchain/extensions:llvm.bzl", "llvm")
llvm.toolchain(
    llvm_version = "18.1.8",
    sha256 = {
        # Generate checksums with shasum -a 256 filename.tar.zst
        "darwin-aarch64": "41d8dea52d18c4e8b90c4fcd31965f9f297df9f40a38a33d60748dbe7f8330b8",
        "darwin-x86_64": "",
        "linux-aarch64": "",
        "linux-x86_64": "",
    },
    stdlib = {
        "linux-x86_64": "stdc++",
        "linux-aarch64": "stdc++",
    },
    urls = {
        "darwin-aarch64": ["https://github.com/MaterializeInc/toolchains/releases/download/clang-18.1.8-4/darwin_aarch64.tar.zst"],
        "darwin-x86_64": ["https://github.com/MaterializeInc/toolchains/releases/download/clang-18.1.8-4/darwin_x86_64.tar.zst"],
        "linux-aarch64": ["https://github.com/MaterializeInc/toolchains/releases/download/clang-18.1.8-4/linux_aarch64.tar.zst"],
        "linux-x86_64": ["https://github.com/MaterializeInc/toolchains/releases/download/clang-18.1.8-4/linux_x86_64.tar.zst"],
    },
)
```

### Configure Rust & Prost
```
RUST_EDITION = "2021"  # NOTE: 2024 edition will be released with Rust 1.85.0

RUST_VERSION = "1.84.0"

rust = use_extension("@rules_rust//rust:extensions.bzl", "rust")
rust.toolchain(
    edition = RUST_EDITION,
    versions = [RUST_VERSION],
)
use_repo(rust, "rust_toolchains")

register_toolchains("@rust_toolchains//:all")

# Custom Rust Prost toolchain
register_toolchains("@//build/prost_toolchain")

# Rust dependencies. See thirdparty/BUILD.bazel 
```

Pay attention to the path, `build/prost_toolchain` because if your custom prost toolchain
is in a different folder, you have to update this path to make the build work.

### Dependencies

Dependencies are vendored to keep the build as hermetic as possible. 
See the [vendoring example](../07-deps-vendor) for details. 

This example uses the from_spec vendoring of direct dependencies, which supports two modes:
1)  mode = "local",
2)  mode = "remote"

Local means Bazel downloads each crate and stores the entire crate in the repo under the thirdparty folder. This comes handy when you need to debug or patch a dependency. Also, this is recommended for very large projects to enable things like licence scanning over dependencies and helps with remote builds especially when the entire repo is too large
to fit in a single Bazel cache.

Remote means Bazel only stores the BUILD.bazel files thirdparty folder and puts the downloaded crates in the cache. This works well when you don't need to debug or patch a crate and your project is small enough to fit in the Bazel cache.

This repo uses the local mode to because it is small enough to fit in the Bazel cache and
we have to annotate some crates, as shown below.

Three steps are required to configure vendoring:

1) Create a folder thirdparty with a build file (see [vendoring example](../07-deps-vendor) for details)
2) Vendor the dependencies by running the bazel target defined in the build file
3) Use the vendored dependencies in your build

For step 1), add the following to your BUILD file in thirdparty/BUILD.bazel:

```starlark
load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_vendor")

crates_vendor(
    name = "crates_vendor",
    annotations = {
        "protoc-gen-prost": [
            crate.annotation(
                gen_binaries = ["protoc-gen-prost"],
            ),
        ],
        "protoc-gen-tonic": [
            crate.annotation(
                gen_binaries = ["protoc-gen-tonic"],
            ),
        ],
    },
    mode = "remote",
    packages = {
        # protobufs/gRPC in Rust
        "prost": crate.spec(
            package = "prost",
            version = "0.13.0",
        ),
        "prost-types": crate.spec(
            default_features = False,
            package = "prost-types",
            version = "0.13.0",
        ),
        "tonic": crate.spec(
            features = ["transport"],
            package = "tonic",
            version = "0.12.0",
        ),
        "tonic-build": crate.spec(
            package = "tonic-build",
            version = "0.12.0",
        ),
        "tonic-health": crate.spec(
            default_features = False,
            features = ["transport"],
            package = "tonic-health",
            version = "0.12.0",
        ),
        "protoc-gen-prost": crate.spec(
            package = "protoc-gen-prost",
            version = "0.3.1",
        ),
        "protoc-gen-tonic": crate.spec(
            package = "protoc-gen-tonic",
            version = "0.4.0",
        ),

        # Other external crates
        "tokio": crate.spec(
            default_features = False,
            features = [
                "macros",
                "net",
                "rt-multi-thread",
                "signal",
            ],
            package = "tokio",
            version = "1.38",
        ),

    },
    repository_name = "grpc_example_vendored",
    tags = ["manual"],
)
```

For step 2), you just run the vendor target:

```shell
bazel run //thirdparty:crates_vendor
``` 

A few important details:
* The crate annotations are important because they tell the bazel that the name of the binary to use for generated prost gRPC bindings. 
* You can define multiple versions of the same dependency. In this case, you can add the version as a suffix to the package name. 
* The repository_name can be chosen arbitrarily. 


From there, you can use the vendored dependencies with the prefix `//thirdparty/crates` across your project. 

### 2) Configure a custom Prost toolchain

Configuring a custom Prost toolchain is straightforward, you create a new folder with an empty BUILD.bazl file, and add
the toolchain definition.
As your Bazel setup grows over time, it is a best practice to put all custom macros, rules, and toolchains in a
dedicated folder, for example: `build/`.

Suppose you have your BUILD.bazl file in `build/prost_toolchain/BUILD.bazel`, then add the following content:

```starlark
load("@rules_rust//rust:defs.bzl", "rust_library_group")
load("@rules_rust_prost//:defs.bzl", "rust_prost_toolchain")

rust_library_group(
    name = "prost_runtime",
    deps = [
        "//thirdparty/crates:prost",
    ],
)

rust_library_group(
    name = "tonic_runtime",
    deps = [
        ":prost_runtime",
        "//thirdparty/crates:tonic",
    ],
)

# https://github.com/bazelbuild/rules_rust/blob/main/examples/bzlmod/proto_with_toolchain/build/prost_toolchain/BUILD.bazel
rust_prost_toolchain(
    name = "prost_toolchain_impl",
    prost_plugin = "//thirdparty/crates:protoc-gen-prost__protoc-gen-prost",
    prost_runtime = ":prost_runtime",
    prost_types = "//thirdparty/crates:prost-types",
    tonic_plugin = "//thirdparty/crates:protoc-gen-tonic__protoc-gen-tonic",
    tonic_runtime = ":tonic_runtime",
)

toolchain(
    name = "prost_toolchain",
    toolchain = "prost_toolchain_impl",
    toolchain_type = "@rules_rust_prost//:toolchain_type",
)
```

The Prost and Tonic dependencies are pulled from the previously configured
crate dependencies in the MODULE file. With this custom toolchain in place, the last step is to register it.

## Usage

Once the setup has been completed, you use the proto & prost targets as you normally do. For example, to configure rust
bindings for a proto file, just add the target:

```starlark
load("@rules_proto//proto:defs.bzl", "proto_library")
load("@rules_rust_prost//:defs.bzl", "rust_prost_library")

# Build proto files
# https://bazelbuild.github.io/rules_rust/rust_proto.html#rust_proto_library
proto_library(
    name = "proto_bindings",
    srcs = [
        "proto/helloworld.proto",
    ],
)

# Generate Rust bindings from the generated proto files
# https://bazelbuild.github.io/rules_rust/rust_proto.html#rust_prost_library
rust_prost_library(
    name = "rust_proto",
    proto = ":proto_bindings",
    visibility = ["//visibility:public"],
)
```


From there you just build, test, and run the targets:

```shell
  bazel build //...
  bazel test //...
```

Run the server:

`bazel run //grpc_server:bin`

Run the client:

`bazel run //grpc_client:bin`