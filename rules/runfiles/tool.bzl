"""Create a tool with runfiles and a rule which uses it.

A rule may register an action that uses a tool which requires external files
during runtime. This demonstrates best practices for handling such a scenario.
"""

def _tool_impl(ctx):
    # Runfiles expansion for tools (executables that are to be run
    # as part of other build actions) is tricky and error-prone.

    # There is a {rulename}.runfiles directory adjacent to the tool's
    # executable file which contains all runfiles. This is not guaranteed
    # to be relative to the directory in which the executable file is run.
    runfiles_path = "$0.runfiles/"

    # Each runfile under the runfiles path resides under a directory with 
    # with the same name as its workspace.
    data_file_root = runfiles_path + ctx.workspace_name + "/"

    data_file_path = data_file_root + ctx.files._data[0].path

    # Alternatively, one can use the root_symlinks parameter of `runfiles`
    # to create a symlink rooted directly under the {rulename}.runfiles
    # directory.
    my_runfiles = ctx.runfiles(files = [ctx.files._data[0]],
        root_symlinks = {"data_dep" : ctx.files._data[0]})

    # Even root symlinks are under the runfiles path.
    data_dep_path = runfiles_path + "data_dep"

    # Thus the example directory structure is:
    # runfiles/tool     (executable)
    # runfiles/tool.runfiles/
    #     data_dep   (symlink to data.txt)
    #     <workspace_name>/
    #         runfiles/
    #        udata.txt

    # Create the output executable file with command as its content.
    ctx.actions.write(
        output = ctx.outputs.executable,
        # Simple example, effectively puts the contents of data.txt into
        # the output twice (read once via symlink, once via normal file).
        content = "cat %s %s > $1" % (data_file_path, data_dep_path),
        is_executable = True,
    )

    return [DefaultInfo(
        # The tool itself should just declare `runfiles`. The build
        # system will automatically create a `files_to_run` object
        # from the result of this declaration (used later).
        runfiles = my_runfiles,
    )]

tool = rule(
    implementation = _tool_impl,
    executable = True,
    attrs = {
        "command": attr.string(),
        "_data": attr.label(
            allow_files = True,
            default = "//runfiles:data.txt"),
    },
)

def _tool_user_impl(ctx):
    my_out = ctx.actions.declare_file(ctx.attr.name + "_out")

    # Use the tool's `files_to_run` to describe the tool's
    # required environment.
    tool_files_to_run = ctx.attr.tool[DefaultInfo].files_to_run

    ctx.actions.run(
        outputs = [my_out],
        executable = tool_files_to_run,
        arguments = [str(my_out.path)]
    )

    return [DefaultInfo(files = depset([my_out]))]

tool_user = rule(
    implementation = _tool_user_impl,
    attrs = {
        "tool": attr.label(mandatory = True),
    },
)
