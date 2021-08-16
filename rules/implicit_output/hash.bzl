def _impl(ctx):
    # Create actions to generate the three output files.
    # Actions are run only when the corresponding file is requested.

    ctx.actions.run_shell(
        outputs = [ctx.outputs.md5],
        inputs = [ctx.file.src],
        command = "md5sum {} > {}".format(ctx.file.src.path, ctx.outputs.md5.path),
    )

    ctx.actions.run_shell(
        outputs = [ctx.outputs.sha1],
        inputs = [ctx.file.src],
        command = "sha1sum {} > {}".format(ctx.file.src.path, ctx.outputs.sha1.path),
    )

    ctx.actions.run_shell(
        outputs = [ctx.outputs.sha256],
        inputs = [ctx.file.src],
        command = "sha256sum {} > {}".format(ctx.file.src.path, ctx.outputs.sha256.path),
    )

    # By default (if you run `bazel build` on this target, or if you use it as a
    # source of another target), only the sha256 is computed.
    return DefaultInfo(files = depset([ctx.outputs.sha256]))

_hashes = rule(
    implementation = _impl,
    attrs = {
        "src": attr.label(mandatory = True, allow_single_file = True),
        "md5": attr.output(),
        "sha1": attr.output(),
        "sha256": attr.output(),
    },
)

def hashes(**kwargs):
    name = kwargs["name"]
    _hashes(
        md5 = "%s.md5" % name,
        sha1 = "%s.sha1" % name,
        sha256 = "%s.sha256" % name,
        **kwargs
    )
