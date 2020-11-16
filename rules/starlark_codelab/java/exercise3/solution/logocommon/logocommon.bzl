"""Common build definitions for starlark codelab."""

_ARCHIVE_FILE = "//starlark_codelab/java/exercise3/solution/logocommon:logos.zip"

def _get_logo_file_via_rule_impl(ctx):
    """Extract the file named logo_filename from _archive_file.
    """
    input = ctx.file._archive_file
    output = ctx.actions.declare_file(ctx.attr.logo_filename)
    ctx.actions.run_shell(
        inputs = [input],
        outputs = [output],
        command = "unzip -p '%s' '%s' > '%s'" % (input.path, output.basename, output.path),
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
        "logo_filename": attr.string(),
    },
)
