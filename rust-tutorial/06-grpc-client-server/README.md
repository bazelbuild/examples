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

## Setup

The Prost and Tonic rules do not specify a default toolchain in order to avoid mismatched dependency issues. 
While the Tonic toolchain works out of the box when its dependencies are matched, however,
Prost requires a custom toolchain that you have to define.

The setup requires three steps to complete: 
1. Configure rules and dependencies in MODULE.bazel
2. Configure a custom Prost toolchain
3. Register custom Prost toolchain.

To keep the build hermetic, we use the LLVM Clang compiler to compile all C/C++ dependencies. 

### 1) Configure rules and dependencies

In your MODULE.bazel, you add the following:

```starlark
# rules for proto
###############################################################################
# https://github.com/bazelbuild/rules_proto/releases
bazel_dep(name = "rules_proto", version = "6.0.2")
# https://github.com/aspect-build/toolchains_protoc/releases
bazel_dep(name = "toolchains_protoc", version = "0.3.1")
# https://registry.bazel.build/modules/protobuf
bazel_dep(name = "protobuf", version = "27.1")
# rules for LLVM 
# https://github.com/bazel-contrib/toolchains_llvm
bazel_dep(name = "toolchains_llvm", version = "1.0.0")

# 1 Register LLVM 
###############################################################################
# L L V M
# https://github.com/bazel-contrib/toolchains_llvm/blob/master/tests/MODULE.bazel
###############################################################################
llvm = use_extension("@toolchains_llvm//toolchain/extensions:llvm.bzl", "llvm")
LLVM_VERSIONS = { "": "16.0.0",}

# LLVM toolchain.
llvm.toolchain(
    name = "llvm_toolchain",
    llvm_versions = LLVM_VERSIONS,
)
use_repo(llvm, "llvm_toolchain", "llvm_toolchain_llvm")
register_toolchains("@llvm_toolchain//:all")

# 2 Register Proto toolchain 
###############################################################################
# Proto toolchain
register_toolchains("@rules_rust//proto/protobuf:default-proto-toolchain")

# Custom Prost toolchain will be added later. See next section

# 3 Register proto / prost / tonic crates 
###############################################################################
crate = use_extension("@rules_rust//crate_universe:extension.bzl", "crate")

# protobufs / gRPC
crate.spec(package = "prost", version = "0.12")
crate.spec(package = "prost-types", default_features = False, version = "0.12")
crate.spec(package = "tonic", features = ["transport"], version = "0.11")
crate.spec(package = "tonic-build", version = "0.11")
crate.spec(package = "protoc-gen-prost", version = "0.3.1")
crate.spec(package = "protoc-gen-tonic", version = "0.4.0")

crate.annotation(
    crate = "protoc-gen-prost",
    gen_binaries = ["protoc-gen-prost"],
)

crate.annotation(   
    crate = "protoc-gen-tonic",   
    gen_binaries = ["protoc-gen-tonic"],
)

# Other Rust dependencies ... 

crate.from_specs()
use_repo(crate, "crates")
```

### 2) Configure a custom Prost toolchain

Configuring a custom Prost toolchain is straightforward, you create a new folder with an empty BUILD.bazl file, and add
the toolchain definition.
As your Bazel setup grows over time, it is a best practice to put all custom macros, rules, and toolchains in a
dedicated folder, for example: `build/`.

Suppose you have your BUILD.bazl file in `build/prost_toolchain/BUILD.bazel`, then add the following content:

```starlark
load("@rules_rust//proto/prost:defs.bzl", "rust_prost_toolchain")
load("@rules_rust//rust:defs.bzl", "rust_library_group")

rust_library_group(
    name = "prost_runtime",
    deps = [
        "@crates//:prost",
    ],
)

rust_library_group(
    name = "tonic_runtime",
    deps = [
        ":prost_runtime",
        "@crates//:tonic",
    ],
)

rust_prost_toolchain(
    name = "prost_toolchain_impl",
    prost_plugin = "@crates//:protoc-gen-prost__protoc-gen-prost",
    prost_runtime = ":prost_runtime",
    prost_types = "@crates//:prost-types",
    proto_compiler = "@protobuf//:protoc",
    tonic_plugin = "@crates//:protoc-gen-tonic__protoc-gen-tonic",
    tonic_runtime = ":tonic_runtime",
)

toolchain(
    name = "prost_toolchain",
    toolchain = "prost_toolchain_impl",
    toolchain_type = "@rules_rust//proto/prost:toolchain_type",
)
```

The Prost and Tonic dependencies are pulled from the previously configured
crate dependencies in the MODULE file. With this custom toolchain in place, the last step is to register it.

### 3. Register custom Prost toolchain.

In your MODULE.bazel file, locate your toolchains and add the following entry right below the proto toolchain.

```starlark
# 2 Register Proto toolchain 
###############################################################################
# Proto toolchain
register_toolchains("@rules_rust//proto/protobuf:default-proto-toolchain")

# Custom Prost toolchain
register_toolchains("@//build/prost_toolchain")
```

Pay attention to the path, `build/prost_toolchain` because if your toolchain
is in a different folder, you have to update this path to make the build work.

## Usage

Once the setup has been completed, you use the proto & prost targets as you normally do. For example, to configure rust
bindings for a proto file, just add the target:

```starlark
load("@rules_proto//proto:defs.bzl", "proto_library")
load("@rules_rust//proto/prost:defs.bzl", "rust_prost_library")

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

From there, you
just [follow the target documentation](https://bazelbuild.github.io/rules_rust/rust_proto.html#rust_proto_library).
