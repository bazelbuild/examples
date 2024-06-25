# Vendored Rust Dependencies

This example shows how to vendor Rust dependencies and use those vendored dependencies in a binary target.
You can run the example vendoring target:

`bazel run //basic/3rdparty:crates_vendor`

And the build target:

`bazel build //...`

## Setup

For the setup,
you need to add the skylib in addition to the rust rules to your MODUE.bazel.

```starlark
module(
    name = "deps_vendored",
    version = "0.0.0"
)
###############################################################################
# B A Z E L  C E N T R A L  R E G I S T R Y # https://registry.bazel.build/
###############################################################################
# https://github.com/bazelbuild/bazel-skylib/releases/
bazel_dep(name = "bazel_skylib", version = "1.7.1")

# https://github.com/bazelbuild/rules_rust/releases
bazel_dep(name = "rules_rust", version = "0.46.0")

###############################################################################
# T O O L C H A I N S
###############################################################################

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

###############################################################################
# R U S T  C R A T E S
###############################################################################
crate = use_extension("@rules_rust//crate_universe:extension.bzl", "crate")
```

Note, it is important to load the crate_universe rules otherwise you will get an error
as the rule set is needed in the vendored target.

Assuming you have a package called `basic` in which you want to vendor dependencies, 
then you create a folder `basic/3rdparty`. The folder name can be arbitrary, 
but by convention, its either thirdparty or 3rdparty to indicate vendored dependencies. 
In the 3rdparty folder, you add a target crates_vendor to declare your dependencies to vendor. In the example, we vendor a specific version of bzip2. 

```starlark
load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_vendor")

crates_vendor(
    name = "crates_vendor",
    annotations = {
        "bzip2-sys": [crate.annotation(
            gen_build_script = True,
        )],
    },
    cargo_lockfile = "Cargo.Bazel.lock",
    generate_build_scripts = False,
    mode = "remote",
    packages = {
        "bzip2": crate.spec(
            version = "=0.3.3",
        ),
    },
    repository_name = "basic",
    tags = ["manual"],
)
```

Next, you have to run `Cargo build` to generate a Cargo.lock file with all resolved dependencies.
Then, you rename Cargo.lock to Cargo.Bazel.lock and place it inside the `basic/3rdparty` folder. 

At this point, you have the following folder and files:

```
basic
    ├── 3rdparty
    │   ├── BUILD.bazel   
    │   ├── Cargo.Bazel.lock   
``` 

Now you can run the `crates_vendor` target:

`bazel run //basic/3rdparty:crates_vendor`

This generates a crate folders with all configurations for the vendored dependencies.

```
basic
    ├── 3rdparty
    │   ├── cratea    
    │   ├── BUILD.bazel   
    │   ├── Cargo.Bazel.lock   
``` 

## Usage

Suppose you have an application in `basic/src` that is defined in `basic/BUILD.bazel` and 
that depends on a vendored dependency. You find a list of all available vendored dependencies
in the BUILD file of the generated folder: `basic/3rdparty/crates/BUILD.bazel`
You declare a vendored dependency in you target as following:

```starlark
load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "hello_sys",
    srcs = ["src/main.rs"],
    # Note the `crate_unvierse` dependencies here need to have been loaded
    # in the WORKSPACE file. See `//:sys_deps.bzl` for more details.
    deps = ["//basic/3rdparty/crates:bzip2"],
    visibility = ["//visibility:public"],
)
```
Note, the vendored dependency is not yet accessible.

Before you can build, you have to define how to load the vendored dependencies. For that, 
you first create a file `sys_deps.bzl` and add the following content:

```starlark
# rename the default name "crate_repositories" in case you import multiple vendored folders.
load("//basic/3rdparty/crates:defs.bzl", basic_crate_repositories = "crate_repositories")

def sys_deps():
    """
    This macro loads dependencies for the `basic` crate examples

    Commonly `*-sys` crates are built on top of some existing library and
    will have a number of dependencies. The examples here use
    [crate_universe](https://bazelbuild.github.io/rules_rust/crate_universe.html)
    to gather these dependencies and make them available in the workspace.
    """

    # Load the vendored dependencies
    basic_crate_repositories()
```

This is straightforward, you import the generated crate_repositories from the crates folder,
rename it to avoid name clashes in case you import from multiple vendored folders, and then
just load the vendored dependencies.

In a WORKSPACE configuration, you would just load and call sys_deps(), but in a MODULE configuration, you cannot do that. Instead, you create a new file `WORKSPACE.bzlmod` and add the following content.

```starlark
load("//:sys_deps.bzl", "sys_deps")
sys_deps()
```

Now, you can build the project as usual:

`bazel build //...`

If you ever see an error referring to some cyclical dependencies in a WORKSPACE, it
is caused because you did not load the bazel_skylib at the top of the MODULE.bazel file.
To fix this error, make sure to have the following entry in your MODULE.bazel file:

```starlark
# ...
# https://github.com/bazelbuild/bazel-skylib/releases/
bazel_dep(name = "bazel_skylib", version = "1.7.1")
# ....
```

Your build will complete once skylib loads. 