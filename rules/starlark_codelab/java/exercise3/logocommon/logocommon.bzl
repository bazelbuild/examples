"""Common build definitions for starlark codelab."""

_ARCHIVE_FILE = "//starlark_codelab/java/exercise3/logocommon:logos.zip"

def _get_logo_file_via_rule_impl(ctx):
    input = ctx.file._archive_file

    # TODO: Replace "TODO" to specify the correct output logo file.
    output = ctx.actions.declare_file("TODO")
    ctx.actions.run_shell(
        inputs = [input],
        outputs = [output],

        # TODO: Replace the following command with a shell command that extracts the
        # logo file from the archive file.
        command = "touch %s" % output.path,
    )
    return [DefaultInfo(
        files = depset(
            [output],
        ),
    )]

get_logo_file_via_rule = rule(
    implementation = _get_logo_file_via_rule_impl,
    attrs = {
        "_archive_file": attr.label(
            default = Label(_ARCHIVE_FILE),
            allow_single_file = True,
        ),
        # TODO: Handle any additional parameters.
    },
)
