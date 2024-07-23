# Examples to build Rust code

This package shows how to build Rust code for the most common use cases.

All examples use the Bazelmod configuration format, and some examples come with a Cargo configuration.
There are cases when rules have not yet been updated for the new Bazelmod format. 
In this case, you can apply the [hybrid mode](https://bazel.build/external/migration#hybrid-mode) 
by already using the new MODULE.bazel config format while also using a dedicated WORKSPACE.bazelmod 
file for those rules that have not been updated. 

Over time, you can migrate one rule at a time from the WORKSPACE.bazelmod file to the MODULE.bazel 
and eventually delete the WORKSPACE.bazelmod file when its no longer needed. 
Please read the [official migration guide for details](https://bazel.build/external/migration).

Finally, the new MODULE.bazel format is very different from the previous WORKSPACE format, so please
be mindful to never mix the two in one file. 
If unsure, please ask in the #rust channel on Bazel Slack: https://slack.bazel.build/

All code examples support Linux and MacOS. The CI builds all examples on Ubuntu Linux 18.04 and all cross compilation examples are also build for MacOS. 
The Rust examples do not officially support Windows because Bazel on Windows requires some special configuration. Please use WSL as a workaround or, alternatively, find more Windows-specific recommendations in the official [Bazel on Windows guide](https://bazel.build/configure/windows).

### Example 1: Hello World

The first example is straightforward and shows how to compile a basic binary with a single source file.

Links:
* [Readme](01-hello-world/README.md)
* [Code](01-hello-world)

### Example 2: Cross Compilation

This example shows how to cross-compile Rust code for Linux on both, aarch64 and x86_64. 

Links:
* [Readme](02-hello-cross/README.md)
* [Code](02-hello-cross)

Documentation:
* [rules_rust](https://bazelbuild.github.io/rules_rust/)
* [llvm_toolchain](https://github.com/bazel-contrib/toolchains_llvm)


### Example 3: Compiler Optimization

This example shows how to apply compiler optimization to a Rust binary.

Links:
* [Readme](03-comp-opt/README.md)
* [Code](03-comp-opt)

Documentation:
* [rules_rust](https://bazelbuild.github.io/rules_rust/)


### Example 4: FFI

This example shows how to call into a C++ function via C FFI, which comes in handy when Rust 
re-uses an existing C++ code base. 

Links:
* [Readme](04-ffi/README.md)
* [Code](04-ffi)

Documentation:
* [rules_rust](https://bazelbuild.github.io/rules_rust/)
* [rules_cc](https://github.com/bazelbuild/rules_cc)


### Example 5: Cargo Workspace Dependencies

This example shows how to manage dependencies with Cargo and use crates_repository
to generate Bazel dependencies from a (workspace) Cargo.toml. Note, whenever 
dependencies in  Cargo.toml change, you have to re-generate the Bazel dependencies.
See the Readme for details.

Links:
* [Readme](05-deps-cargo/README.md)
* [Code](05-deps-cargo)

Documentation:
* [rules_rust](https://bazelbuild.github.io/rules_rust/)


### Example 6: Direct Dependencies

In complex or larger Bazel projects, maintaining various Cargo.toml files may have diminishing returns, 
so managing Rust dependencies directly with Bazel is a more favorable option. 
This example shows how to directly declare and use Rust dependencies in Bazel to build 
an async REST API with Tokio.

Links:
* [Readme](06-deps-direct/README.md)
* [Code](06-deps-direct)

Documentation:
* [rules_rust](https://bazelbuild.github.io/rules_rust/)


### Example 7: Vendored Rust Dependencies

Some organizations require that all external dependencies are vendored, meaning downloaded 
and stored in the workspace. This helps, for example, to conduct licence scans, apply custom patches, 
or to ensure full build reproducibility since no download error could possibly occur. 
This example shows how to vendor Rust dependencies and use those vendored dependencies in a binary target. 

Links:
* [Readme](07-deps-vendor/README.md)
* [Code](07-deps-vendor)

Documentation:
* [rules_rust](https://bazelbuild.github.io/rules_rust/)
* [crates_vendor](https://bazelbuild.github.io/rules_rust/crate_universe.html#crates_vendor)


### Example 8: gRPC Client & Server

The gRPC protocol is commonly used for communication between internal microservices. 
This example shows how to build a gRPC client and server in Rust and how to set up a custom
toolchain that generates all the proto bindings for Rust. Also, the example comes with a complete 
Cargo workspace configuration that contains three different crates, 
which helps to migrate Cargo workspaces to Bazel. 

Links:
* [Readme](08-grpc-client-server/README.md)
* [Code](08-grpc-client-server)

Documentation:
* [rules_rust](https://bazelbuild.github.io/rules_rust/)
* [rust_proto](https://bazelbuild.github.io/rules_rust/rust_proto.html#rust_proto_libraryhttps://bazelbuild.github.io/rules_rust/rust_proto.html#rust_proto_library)
* [Protocol Buffers](https://protobuf.dev/)


### Example 9: Rust Container image

This example showcases how to configure Bazel to build and publish a Rust binary as OCI container image. 

Links:
* [Readme](09-oci-container/README.md)
* [Code](09-oci-container)

Documentation:
* [rules_rust](https://bazelbuild.github.io/rules_rust/)
* [rules_oci](https://github.com/bazel-contrib/rules_oci?tab=readme-ov-file#usage)

