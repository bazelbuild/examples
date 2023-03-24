"""Turn a list of words into enums in various languages.

"""

def _impl(ctx):
    args = ctx.actions.args()
    args.add("--values", ctx.file.values_file.path)

    outputs = []
    if ctx.outputs.out_py:
        args.add("--out_py", ctx.outputs.out_py.path)
        outputs.append(ctx.outputs.out_py)
    if ctx.outputs.out_h:
        args.add("--out_h", ctx.outputs.out_h.path)
        outputs.append(ctx.outputs.out_h)

    # Action to call the script.
    ctx.actions.run(
        inputs = ctx.files.values_file,
        outputs = outputs,
        arguments = [args],
        progress_message = "Generating enums for %s" % ctx.file.values_file.short_path,
        executable = ctx.executable.gen_tool,
    )

enum_maker = rule(
    implementation = _impl,
    attrs = {
        "values_file": attr.label(
           allow_single_file = True,
           mandatory = True,
        ),
        "out_py": attr.output(),
        "out_h": attr.output(),
        "gen_tool": attr.label(
            default = Label(":enum_maker"),
            executable = True,
            allow_files = True,
            cfg = "exec",
        ),
    },
)
