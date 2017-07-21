# Bazel Windows MSVC toolchain
A standalone Windows CROSSTOOL for Bazel
(disclaimer: This is not an official Google product.)

# How to use it

### Using [local_repository](https://docs.bazel.build/versions/master/be/workspace.html#local_repository) rule
Download this folder to your machine, add the following to your **WORKSPACE** file:
```python
local_repository(
    name = "standalone_cc_toolchain",
    path = "<path>/<to>/<your>/windows-crosstool",
)

load("@standalone_cc_toolchain//:cc_configure.bzl", "cc_configure")

cc_configure()
```

Add `build --crosstool_top=@standalone_local_config_cc//:toolchain` to your **bazelrc** file.
