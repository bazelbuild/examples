def _generate_uppercase_impl(ctx):
    in_file = ctx.file.input
    out_file = ctx.outputs.output
    ctx.actions.run_shell(
        outputs = [out_file],
        inputs = [in_file],
        arguments = [in_file.path, out_file.path],
        command = "tr '[:lower:]' '[:upper:]' < \"$1\" > \"$2\"",
    )

generate_uppercase = rule(
    implementation = _generate_uppercase_impl,
    attrs = {
        "input": attr.label(allow_single_file=True, doc = "The file to transform"),
        "output": attr.output(doc = "The generated file"),
    },
    doc = "Transforms a text file by changing its characters to uppercase.",
)

def _generate_lowercase_impl(ctx):
    in_file = ctx.file.input
    out_file = ctx.outputs.output
    ctx.actions.run_shell(
        outputs = [out_file],
        inputs = [in_file],
        arguments = [in_file.path, out_file.path],
        command = "tr '[:upper:]' '[:lower:]' < \"$1\" > \"$2\"",
    )

generate_lowercase = rule(
    implementation = _generate_lowercase_impl,
    attrs = {
        "input": attr.label(allow_single_file=True, doc = "The file to transform"),
        "output": attr.output(doc = "The generated file"),
    },
    doc = "Transforms a text file by changing its characters to lowercase.",
)
