# Cross Compilation

For cross compilation, you have to specify a custom platform to let Bazel know that you are compiling for a different platform than the default host platform.

The example code is setup to cross compile from the following hosts to the the following targets:

* {linux, x86_64} -> {linux, aarch64}
* {linux, aarch64} -> {linux, x86_64}
* {darwin, x86_64} -> {linux, x86_64}
* {darwin, x86_64} -> {linux, aarch64}
* {darwin, aarch64 (Apple Silicon)} -> {linux, x86_64}
* {darwin, aarch64 (Apple Silicon)} -> {linux, aarch64}

You cross-compile by calling the target.

`bazel build //:hello_world_x86_64`

or

`bazel build //:hello_world_aarch64`

Notice, the default target `//...` does not know about the 
many different target platform and will report an error. 
Instead, if you want to build for all platforms at once, 
call the filegroup target:

`bazel build //...`


## Setup

The setup requires three steps, first declare dependencies and toolchains in your MODULE.bazel, second configure LLVM and Rust for cross compilation, and third the configuration of the cross compilation platforms so you can use it binary targets.

### Dependencies Configuration

You add the required rules for cross compilation to your MODULE.bazel as shown below.

```Starlark
# Rules for cross compilation
# https://github.com/bazelbuild/platforms/releases
bazel_dep(name = "platforms", version = "0.0.10")
# https://github.com/bazel-contrib/toolchains_llvm
bazel_dep(name = "toolchains_llvm", version = "1.2.0", dev_dependency = True)
```

## LLVM Configuration

Next, you have to configure the LLVM toolchain because rules_rust still needs a cpp toolchain for cross compilation and
you have to add the specific platform triplets to the Rust toolchain. Suppose you want to compile a Rust binary that
supports linux on both, X86 and ARM. In that case, you have to setup three LLVM toolchains:

1) LLVM for the host
2) LLVM for X86
3) LLVM for ARM (aarch64)

For the host LLVM, you just specify a LLVM version and then register the toolchain as usual. The target LLVM toolchains,
however, have dependencies on system libraries for the target platform. Therefore, it is required to download a so-
called sysroot that contains a root file system with all those system libraries for the specific target platform.
To do so, please add the following to your MODULE.bazel

```Starlark
# https://github.com/bazelbuild/bazel/blob/master/tools/build_defs/repo/http.bzl
http_archive = use_repo_rule("@bazel_tools//:http.bzl", "http_archive")

# INTEL/AMD64 Sysroot. LastModified: 2024-04-26T19:15
# https://commondatastorage.googleapis.com/chrome-linux-sysroot/
http_archive = use_repo_rule("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
http_archive(
    name = "sysroot_linux_x64",
    build_file = "//build/sysroot:BUILD.bazel",
    sha256 = "5df5be9357b425cdd70d92d4697d07e7d55d7a923f037c22dc80a78e85842d2c",
    urls = ["https://commondatastorage.googleapis.com/chrome-linux-sysroot/toolchain/4f611ec025be98214164d4bf9fbe8843f58533f7/debian_bullseye_amd64_sysroot.tar.xz"],
)

# ARM 64 Sysroot. LastModified: 2024-04-26T18:33
# https://commondatastorage.googleapis.com/chrome-linux-sysroot/
http_archive(
    name = "sysroot_linux_aarch64",
    build_file = "//build/sysroot:BUILD.bazel",
    sha256 = "d303cf3faf7804c9dd24c9b6b167d0345d41d7fe4bfb7d34add3ab342f6a236c",
    urls = ["https://commondatastorage.googleapis.com/chrome-linux-sysroot/toolchain/906cc7c6bf47d4bd969a3221fc0602c6b3153caa/debian_bullseye_arm64_sysroot.tar.xz"],
)
```

Here, we declare to new http downloads that retrieve the sysroot for linux_x64 (Intel/AMD) and linux_aarch64 (ARM/Apple Silicon). Note, these are only
sysroots, that means you have to configure LLVM next to use these files. As mentioned earlier, three LLVM toolchains
needs to be configured and to do that, please add the following to your MODULE.bazel

```Starlark
llvm = use_extension("@toolchains_llvm//toolchain/extensions:llvm.bzl", "llvm")
llvm.toolchain(
    llvm_version = "19.1.6-1",
    sha256 = {
        # Generate checksums with shasum -a 256 filename.tar.zst
        "darwin-aarch64": "",
        "darwin-x86_64": "",
        "linux-aarch64": "",
        "linux-x86_64": "",
    },
    stdlib = {
        "linux-x86_64": "stdc++",
        "linux-aarch64": "stdc++",
    },
    urls = {
        "darwin-aarch64": ["https://github.com/MaterializeInc/toolchains/releases/download/clang-19.1.6-1/darwin_aarch64.tar.zst"],
        "darwin-x86_64": ["https://github.com/MaterializeInc/toolchains/releases/download/clang-19.1.6-1/darwin_x86_64.tar.zst"],
        "linux-aarch64": ["https://github.com/MaterializeInc/toolchains/releases/download/clang-19.1.6-1/linux_aarch64.tar.zst"],
        "linux-x86_64": ["https://github.com/MaterializeInc/toolchains/releases/download/clang-19.1.6-1/linux_x86_64.tar.zst"],
    },
)
llvm.sysroot(
    name = "llvm_toolchain",
    label = "@sysroot_linux_x64//:sysroot",
    targets = ["linux-x86_64"],
)
llvm.sysroot(
    name = "llvm_toolchain",
    label = "@sysroot_linux_aarch64//:sysroot",
    targets = ["linux-aarch64"],
)
use_repo(llvm, "llvm_toolchain")

register_toolchains("@llvm_toolchain//:all")
```

For simplicity, all toolchains are pinned to version LLVM 19. 1.6-1.w
It is possible to pin different targets to different LLVM
versions; [see the documentation for details](https://github.com/bazel-contrib/toolchains_llvm/tree/master?tab=readme-ov-file#per-host-architecture-llvm-version).

### LLVM Troubleshooting

On older linux distributions (Ubuntu 16.04) you may encounter an error that C++ versions before C++ 14 are no longer
supported. In this case, just install gcc version 7 or newer. This is rare corner case, but there are gcc backports for
older distributions, so please upgrade your compiler if you ever see this error.

On Ubuntu 20.04 you may see an error that a shared library called libtinfo.so.5 is missing. In that case, just install
libtinfo via apt-get since its in the official 20.04 repo. To so, open a terminal and type:

`
apt update && apt install -y libtinfo5
`

The libtinfo5 library may have different package names on other distributions, but it is a well known
issue. [See this SO discussion](https://stackoverflow.com/questions/48674104/clang-error-while-loading-shared-libraries-libtinfo-so-5-cannot-open-shared-o)
for various solutions.

On MacOX, it is sufficient to have the Apple Clang compiler installed.
I don't recommend installing the full Xcode package unless you're developing software for an Apple device. Instead, the
Xcode Command Line Tools provide everything you need at a much smaller download size. In most cases, a simple:

`xcode-select --install`

From a terminal triggers the installation process. For details and alternative
options, [read this article on freebootcamp.](https://www.freecodecamp.org/news/install-xcode-command-line-tools/)

Windows is not directly supported, but you can use Linux on Windows with WSL to setup an Ubuntu environment within
Windows. Please refer to
the [official WSL documentation for details.](https://learn.microsoft.com/en-us/windows/wsl/install)

**Rust Toolchain Configuration**

The Rust toolchain only need to know the additional platform triplets to download the matching toolchains. To do so, add
or or modify your MODULE.bazel with the following entry:

```Starlark
RUST_EDITION = "2021"  

RUST_VERSION = "1.84.1"

rust = use_extension("@rules_rust//rust:extensions.bzl", "rust")
rust.toolchain(
    edition = RUST_EDITION,
    extra_target_triples = [
        "aarch64-unknown-linux-gnu",
        "x86_64-unknown-linux-gnu",
    ],
    versions = [RUST_VERSION],
)
use_repo(rust, "rust_toolchains")

register_toolchains("@rust_toolchains//:all")
```

You find the exact platform triplets in
the [Rust platform support documentation](https://doc.rust-lang.org/nightly/rustc/platform-support.html).
Next, you have to configure the target platform.

**Platform Configuration**

Once the dependencies are loaded, create an empty BUILD file to define the cross compilation toolchain targets.
As mentioned earlier, it is best practice to put all custom rules, toolchains, and platform into one folder.
Suppose you have the empty BUILD file in the following path:

`build/platforms/BUILD.bazel`

Then you add the following content to the BUILD file:

```Starlark
package(default_visibility = ["//visibility:public"])

platform(
    name = "linux-aarch64",
    constraint_values = [
        "@platforms//os:linux",
        "@platforms//cpu:aarch64",
    ],
)

platform(
    name = "linux-x86_64",
    constraint_values = [
        "@platforms//os:linux",
        "@platforms//cpu:x86_64",
    ],
)
```

The default visibility at the top of the file means that all targets in this BUILD file will be public by default, which
is sensible because cross-compilation targets are usually used across the entire project.

It is important to recognize that the platform rules use the constraint values to map those constraints to the target
triplets of the Rust toolchain. If you somehow see errors that says some crate couldn't be found with triple xyz, then
one of two things happened.

Either you forgot to add a triple to the Rust toolchain. Unfortunately, the error message
doesn't always tell you the correct triple that is missing. However, in that case you have to double check if for each
specified platform a corresponding Rust extra_target_triples has been added. If one is missing, add it and the error
goes away.

A second source of error is if the platform declaration contains a typo, for example,
cpu:arch64 instead of cpu:aarch64. You have to be meticulous in the platform declaration to make everything work
smoothly.

With the platform configuration out of the way, you are free to configure your binary targets for the specified
platforms.

## Usage

Suppose you have a simple hello world that is defined in a single main.rs file. Conventionally, you declare a minimum
binary target as shown below.

```Starlark
load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "hello_world_host",
    srcs = ["src/main.rs"],
    deps = [],
)
```

Bazel compiles this target to the same platform as the host. To cross-compile the same source file to a different
platform, you simply add one of the platforms previously declared, as shown below.

```Starlark
load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "hello_world_x86_64",
    srcs = ["src/main.rs"],
    platform = "//build/platforms:linux-x86_64",
    deps = [],
)

rust_binary(
    name = "hello_world_aarch64",
    srcs = ["src/main.rs"],
    platform = "//build/platforms:linux-aarch64",
    deps = [],
)
```

You then cross-compile by calling the target.

`bazel build //:hello_world_x86_64`

or

`bazel build //:hello_world_aarch64`

You may have to make the target public when see an access error.

However, when you build for multiple targets, it is sensible to group all of them in a filegroup.

```Starlark
filegroup(
    name = "all",
    srcs = [
        ":hello_world_host",
        ":hello_world_x86_64",
        ":hello_world_aarch64",
    ],
    visibility = ["//visibility:public"],
)
```

Then you build for all platforms by calling the filegroup target:

`bazel build //:all`
