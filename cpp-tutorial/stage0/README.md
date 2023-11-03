
# Stage 0

**Linux-only**

In this initial stage, we'll focus on setting up a hermetic GCC toolchain for
our Bazel build environment, utilizing the hermetic GCC toolchain provided here.
A hermetic toolchain is crucial for achieving deterministic builds, ensuring
that our build outputs are solely dependent on our input sources and build
instructions, unaffected by the external system environment.

Note that there are several options for hermetic toolchains, depending on which
compiler you use.
See <https://github.com/bazelbuild/rules_cc#using-the-rules_cc-toolchain>.

## Load the hermetic GCC toolchain

Add to your WORKSPACE file:

```python
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "aspect_gcc_toolchain",
    sha256 = "<sha256>", # Replace with the SHA-256 hash of the release tarball.
    strip_prefix = "gcc-toolchain-<version>", # Replace with the release version.
    urls = [
        # Replace with the release version.
        "https://github.com/aspect-build/gcc-toolchain/archive/<version>.tar.gz",
    ],
)

load("@aspect_gcc_toolchain//toolchain:repositories.bzl", "gcc_toolchain_dependencies")

gcc_toolchain_dependencies()

load("@aspect_gcc_toolchain//toolchain:defs.bzl", "gcc_register_toolchain", "ARCHS")

gcc_register_toolchain(
    name = "gcc_toolchain_x86_64",
    target_arch = ARCHS.x86_64,
)
```

### The X11 sysroot variant

The hermetic GCC toolchain is available in two variants: the X11 sysroot variant
and the non-X11 sysroot variant. The non-X11 sysroot variant is the default
variant, and is suitable for building command-line applications. The X11 sysroot
variant is suitable for building GUI applications that require X11 libraries.

To use the X11 sysroot variant, replace the `gcc_toolchain_x86_64` target in
the above snippet with the following:

```python
gcc_register_toolchain(
    name = "gcc_toolchain_x86_64",
    sysroot_variant = "x86_64-X11",
    target_arch = ARCHS.x86_64,
)
```

## Cross-compiling for ARM

This approach uses `--platforms` to cross-compile for ARM. For more information
on how to use `platforms`, see the documentation here:
https://bazel.build/extending/platforms.

### aarch64 (a.k.a arm64 or armv8)

Add to your WORKSPACE file:

```python
gcc_register_toolchain(
    name = "gcc_toolchain_aarch64",
    target_arch = ARCHS.aarch64,
)
```

### armv7-hf (32-bit)

Add to your WORKSPACE file:

```python
gcc_register_toolchain(
    name = "gcc_toolchain_armv7",
    target_arch = ARCHS.armv7,
)
```

## Setting up the required flags in your .bazelrc

Add to your .bazelrc file:

```
# Prevent Bazel from detecting the system's C++ toolchain.
build --action_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=1
build --incompatible_strict_action_env=true
# Enable the CC toolchain resolution based on platforms.
build --incompatible_enable_cc_toolchain_resolution
```
