# Vendored Rust Dependencies

This example shows how to vendor Rust dependencies and use those vendored dependencies 
in a binary target. You can run the example vendoring target:

`bazel run //thirdparty:crates_vendor`

This downloads all the dependencies and creates the folder `thirdparty/crates`. 

And then build the binary target:

`bazel build //...`

## Setup

For the setup, you just add rules_rust as usual. Note, you do not declare any dependencies
or crate_universe at this stage. 

```starlark
module(
    name = "deps_vendored",
    version = "0.0.0",
)

# https://github.com/bazelbuild/rules_rust/releases
bazel_dep(name = "rules_rust", version = "0.57.1")

# Rust toolchain
RUST_EDITION = "2021"  # NOTE: 2024 edition will be released with Rust 1.85.0

RUST_VERSION = "1.84.1"

rust = use_extension("@rules_rust//rust:extensions.bzl", "rust")
rust.toolchain(
    edition = RUST_EDITION,
    versions = [RUST_VERSION],
)
use_repo(rust, "rust_toolchains")

register_toolchains("@rust_toolchains//:all")

# Rust dependencies; see thirdparty/BUILD.bazel
```


The vendor folder name can be arbitrary, but by convention, its either thirdparty or 3rdparty to indicate vendored dependencies. Also note, you can structure any number of sub-folders in the vendor folder for example. Note, in that case, each sub-folder must have a `BUILD.bazel` file that declares its vendored dependencies. 

```starlark
basic
thirdparty
    ├── common
    │   ├── tokio
    │   ├── warp     
    ├── sys
    │   ├── bzip2
    ├── macros
    │   ├── sys  
```
  


In this example, the vendor folder is named thirdparty and you add a `BUILD.bazel` to declare your dependencies, for example:  

```starlark
load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_vendor")

crates_vendor(
    name = "crates_vendor",
    annotations = {}, # For crate annotations
    mode = "local", # Store crates locally in the crates folder
    packages = {
        "tokio": crate.spec(
            default_features = False,
            features = [
                "macros",
                "net",
                "rt-multi-thread",
                "time",
            ],
            package = "tokio",
            version = "1.43.0",
        ),
    },
    repository_name = "vendored",
    tags = ["manual"],
)
```

Then you run `bazel run //thirdparty:crates_vendor` which then downloads all the dependencies and creates the folder `thirdparty/crates`. 

At this point, you have the following folder and files:
```starlark
basic
thirdparty
    ├── crates/ 
    ├── BUILD.bazel
```

Bazel generated a bunch of files and folder in the crates folder. For the most part, you just run
a build and when it completes, you then just check these vendored dependencies into git to ensure
all subsequent and CI build use the exact same dependencies. 

## Usage

Suppose you have an application in `basic/src` that is defined in `basic/BUILD.bazel` and 
that depends on a vendored dependency. You find a list of all available vendored dependencies
in the BUILD file of the generated folder: `basic/3rdparty/crates/BUILD.bazel`
You declare a vendored dependency in you target as following:

```starlark
load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "hello_vendored",
    srcs = ["src/main.rs"],
    visibility = ["//visibility:public"],
    deps = [
        "//thirdparty/crates:tokio-1.43.0",
    ],
)
```

Now, you can build the project as usual:

`bazel build //...`

And run the binary:

`bazel run //basic:hello_vendored`

You should see the expected output.

```text
Starting the tokio example program
Task 1 started
Task 2 started
Task 3 started
Task 1 finished after 1 second(s)
Task 2 finished after 2 second(s)
Task 3 finished after 3 second(s)
All tasks completed
```