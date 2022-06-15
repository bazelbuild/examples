def _write_sh_path_impl(ctx):
    # Save the shell path to the given output file.
    sh_path = ctx.toolchains["@bazel_tools//tools/sh:toolchain_type"].path
    ctx.actions.write(output = ctx.outputs.out, content = sh_path + "\n")

write_sh_path = rule(
    implementation = _write_sh_path_impl,
    attrs = {"out": attr.output(mandatory = True)},
    toolchains = ["@bazel_tools//tools/sh:toolchain_type"],
)
