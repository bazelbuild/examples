This folder contains example usages of the new external dependency system for Bazel - Bzlmod. In general, each example repository contains

- The MODULE.bazel file for defining dependencies with Bzlmod.
- The WORKSPACE file for defining equivalent dependencies with the old system.
- An empty WORKSPACE.bzlmod file to prevent fetching any dependencies from the old WORKSPACE system when Bzlmod is enabled. (Bzlmod and WORKSPACE can work at the same time, with dependencies from Bzlmod takes priority.)

To turn on Bzlmod, you'll need to pass `--experimental_enable_bzlmod` as a build flag.

As of Jun 2022, Bzlmod is still in development, it's recommended to test with Bazel built at HEAD, you can use Bazel built at HEAD via [Bazelisk](https://github.com/bazelbuild/bazelisk/releases),
```
export USE_BAZEL_VERSION=last_green
bazelisk build <targets>
```
