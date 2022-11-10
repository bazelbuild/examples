This is an example on how to generate local config repos and register toolchains with Bzlmod. The WORKSPACE file contains their equivalent definitions in the old system. It covers the following topics:

- using a module extension to generate a local config repository which contains a toolchain definition.
- registering the generated toolchain in the MODULE.bazel file.

To test it out, `cd` into this directory and run the following:

```bash
export USE_BAZEL_VERSION=last_green
export MY_SHELL_BIN_PATH=/foo/bar/sh
bazelisk build --enable_bzlmod //:get_sh_path
cat ./bazel-bin/result
```
