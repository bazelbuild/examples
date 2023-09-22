This folder contains example usages of the new external dependency system for Bazel - Bzlmod. In general, each example repository contains

- The MODULE.bazel file for defining dependencies with Bzlmod.
- The WORKSPACE file for defining equivalent dependencies with the old system.
- An empty WORKSPACE.bzlmod file to prevent fetching any dependencies from the old WORKSPACE system when Bzlmod is enabled. (Bzlmod and WORKSPACE can work at the same time, with dependencies from Bzlmod takes priority.)

With Bazel 6, you'll need to pass `--enable_bzlmod` as a build flag to turn
this feature on. Consider adding this to `.bazelrc`:

```
common --enable_bzlmod
```
