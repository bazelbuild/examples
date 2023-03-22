def _is_windows(repository_ctx):
    """Returns true if the host OS is Windows."""
    return repository_ctx.os.name.startswith("windows")

def _my_sh_config_impl(repository_ctx):
    """
    Detects the path of the shell interpreter via a env var
    and stores it in a sh_toolchain rule.
    """
    sh_path = repository_ctx.os.environ.get("MY_SHELL_BIN_PATH")
    if not sh_path:
        sh_path = "/shell/binary/not/found"

    if sh_path and _is_windows(repository_ctx):
        sh_path = sh_path.replace("\\", "/")

    repository_ctx.file("BUILD", """
load("@bazel_tools//tools/sh:sh_toolchain.bzl", "sh_toolchain")
sh_toolchain(
    name = "local_sh",
    path = "{sh_path}",
    visibility = ["//visibility:public"],
)
toolchain(
    name = "local_sh_toolchain",
    toolchain = ":local_sh",
    toolchain_type = "@bazel_tools//tools/sh:toolchain_type",
)
""".format(sh_path = sh_path))

my_sh_config = repository_rule(
    environ = [
        "MY_SHELL_BIN_PATH",
    ],
    local = True,
    implementation = _my_sh_config_impl,
)

# Used by WORKSPACE
def sh_configure():
    """Detect the local shell interpreter and register its toolchain."""
    my_sh_config(name = "my_local_config_sh")
    native.register_toolchains("@my_local_config_sh//:local_sh_toolchain")

# Used by MODULE.bazel
my_sh_config_extension = module_extension(
    implementation = lambda ctx: my_sh_config(name = "my_local_config_sh"),
)
