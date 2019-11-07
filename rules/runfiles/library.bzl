"""Create a library with runfiles and a rule which uses it.

A library might need some external files during runtime, and every dependent
binary should know about them. This demonstrates best practices for handling
such a scenario.
"""

# When possible, use custom providers to manage propagating information
# between dependencies and their dependers.
RuntimeRequiredFiles = provider(fields = ["file", "data_files"])

def _library_impl(ctx):
    # Expand the label in the command string to a runfiles-relative path.
    # The second arg is the list of labels that may be expanded.
    command = ctx.expand_location(ctx.attr.command, ctx.attr.data)

    my_out = ctx.actions.declare_file(ctx.attr.name + "_out")
    ctx.actions.write(
        output = my_out,
        content = command,
        is_executable = True,
    )

    return [
        RuntimeRequiredFiles(file = my_out, data_files = depset(ctx.files.data)),
    ]

runfiles_library = rule(
    implementation = _library_impl,
    attrs = {
        "command": attr.string(),
        "data": attr.label_list(allow_files = True),
    },
    provides = [RuntimeRequiredFiles],
)

def _binary_impl(ctx):
    # Create the output executable file, which simply runs the library's
    # primary output file (obtained from RuntimeRequiredFiles.file).
    ctx.actions.write(
        output = ctx.outputs.executable,
        content = "$(cat " + ctx.attr.lib[RuntimeRequiredFiles].file.short_path + ")",
        is_executable = True,
    )

    my_runfiles = ctx.runfiles(
        files = [ctx.attr.lib[RuntimeRequiredFiles].file],
        transitive_files = ctx.attr.lib[RuntimeRequiredFiles].data_files)

    return [DefaultInfo(
        runfiles = my_runfiles
    )]

runfiles_binary = rule(
    implementation = _binary_impl,
    executable = True,
    attrs = {
        "lib": attr.label(mandatory = True, providers = [RuntimeRequiredFiles]),
    },
)
