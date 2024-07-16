# Cargo Workspace Dependencies 

One of the simpler ways to wire up dependencies would be to first structure your project into a Cargo workspace.

To build the example code, run:

`bazel build //...`

The crates_repository rule can ingest a root Cargo.toml file and generate Bazel dependencies from there. 

```starlark
 
crate = use_extension("@rules_rust//crate_universe:extension.bzl", "crate")
#
# External crates
crate.from_cargo(
    name = "crates",
    cargo_lockfile = "//:Cargo.lock",
    manifests = ["//:Cargo.toml"],
)
use_repo(crate, "crates")
```

The generated crates_repository contains helper macros which make collecting dependencies for Bazel targets simpler.
Notably, the all_crate_deps and aliases macros (
see [Dependencies API](https://bazelbuild.github.io/rules_rust/crate_universe.html#dependencies-api)) commonly allow the
Cargo.toml files to be the single source of truth for dependencies.
Since these macros come from the generated repository, the dependencies and alias definitions 
they return will automatically update BUILD targets. In your BUILD files, 
you use these macros for a Rust library as shown below:

```starlark
load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

rust_library(
    name = "lib",
    aliases = aliases(),
    deps = all_crate_deps(
        normal = True,
    ),
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
)

rust_test(
    name = "unit_test",
    crate = ":lib",
    aliases = aliases(
        normal_dev = True,
        proc_macro_dev = True,
    ),
    deps = all_crate_deps(
        normal_dev = True,
    ),
    proc_macro_deps = all_crate_deps(
        proc_macro_dev = True,
    ),
)
```

For a Rust binary that does not depend on any macro, use the following configuration 
in your build file:

```starlark
rust_binary(
    name = "bin",
    srcs = ["src/main.rs"],
    deps = all_crate_deps(normal = True),
)
```

You have to repin before your first build to ensure all Bazel targets for the macros 
are generated. 

## Repining / Updating Dependencies

Dependency syncing and updating is done in the repository rule which means it's done during the
analysis phase of builds. As mentioned in the environments variable table above, the `CARGO_BAZEL_REPIN`
(or `REPIN`) environment variables can be used to force the rule to update dependencies and potentially
render a new lockfile. Given an instance of this repository rule named `crates`, the easiest way to
repin dependencies is to run:

```shell
CARGO_BAZEL_REPIN=1 bazel sync --only=crates
```

This will result in all dependencies being updated for a project. The `CARGO_BAZEL_REPIN` 
environment variable can also be used to customize how dependencies are updated. 
For more details about repin, [please refer to the documentation](https://bazelbuild.github.io/rules_rust/crate_universe.html#crates_vendor).