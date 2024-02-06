This folder contains examples of setting up cc toolchain in Bazel. The examples are broken down into several repositories to show different layers of complexity and flexibity that Bazel provides to take control of your cc builds.


* 01-minimal-gcc-toolchain: A gcc toolchain for single-platform build

Work-in-progress:
* 00-minimal-gcc-toolchain: A gcc toolchain for single-platform build using system tools
* 02-minimal-clang-toolchain: A clang toolchain for single-compilation build
* 03-crosstool-clang-toolchain: A clang toolchain for cross-compilation build. The example has implementation of a `CcToolchainConfigInfo` provider to use advanced concepts such as `feature` and `action_config` to have more control over

> All of the examples assume your host os is `linux` and host cpu is `x86_64`. If you have different host configuration, please tweak the setup to match with yours.

