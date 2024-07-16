# Direct Dependencies

In cases where Rust targets have heavy interactions with other Bazel targets ([Cc](https://docs.bazel.build/versions/main/be/c-cpp.html), [Proto](https://rules-proto-grpc.com/en/4.5.0/lang/rust.html),
etc.), maintaining Cargo.toml files may have diminishing returns as things like rust-analyzer 
begin to be confused about missing targets or environment variables defined only in Bazel.
In situations like this, it may be desirable to have a “Cargo free” setup. 
crates_repository supports this through the packages attribute.

To build the example code, run:

`bazel build //...`

To declare Rust crate dependencies directly, add them to your MODULE file as shown below:

```starlark
 
crate = use_extension("@rules_rust//crate_universe:extension.bzl", "crate")
#
# External crates
crate.spec(package = "arc-swap", version = "1.7")
crate.spec(package = "serde", features = ["derive"], version = "1.0")
crate.spec(package = "serde_json", version = "1.0")
crate.spec(package = "tokio", default_features=False, features = ["macros", "net", "rt-multi-thread", "signal"], version = "1.38")
crate.spec(package = "tokio-cron-scheduler", features = ["signal"], version = "0.10")
crate.spec(package = "warp", version = "0.3")

crate.from_specs()
use_repo(crate, "crates")
```

Consuming dependencies may be more ergonomic in this case through the aliases defined in the new repository. In your BUILD files, you use direct dependencies as shown below:


```starlark
rust_binary(
    name = "bin",
    crate_root = "src/main.rs",
    srcs = glob([
        "src/*.rs",
    ]),
    deps = [
        # External crates
        # External crates
        "@crates//:arc-swap",
        "@crates//:serde",
        "@crates//:serde_json",
        "@crates//:tokio",
        "@crates//:tokio-cron-scheduler",
        "@crates//:warp",
    ],
    visibility = ["//visibility:public"],
)
```

Notice, direct dependencies do not need repining. Only a cargo workspace needs updating whenever the underlying Cargo.toml file changed.

Build with debug symbols:

`
bazel build //...
`

Build with optimization:

`
bazel build -c opt //...
`

And run the optimized binary:

`
bazel run -c opt //rest_tokio:bin
`
