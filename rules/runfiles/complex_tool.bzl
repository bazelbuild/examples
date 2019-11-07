"""Create a complex tool with runfiles and a rule which uses it.

A tool (executable used for action registration) may depend on another
tool with its own runfiles. This example demonstrates this scenario."""

def _sub_tool_impl(ctx):
    # Since this tool may be used by another tool, it must support accepting
    # a different runfiles directory root. The runfiles directory is always
    # adjacent to the *root* tool being run, which may not be this tool.
    # (In this case, this is done by environment variable RUNFILES_DIR.)
    command = """
if [[ -z "${RUNFILES_DIR}" ]]; then
  RUNFILES_DIR=${0}.runfiles
fi

cat ${RUNFILES_DIR}/examples/runfiles/data.txt > $1"""

    # Using root_symlinks or symlinks for a tool is very brittle if the
    # tool may be used by another tool; there will be a collision when merging
    # runfiles if the other tool defines a symlink of the same name as one
    # defined by this rule.
    ctx.actions.write(
        output = ctx.outputs.executable,
        content = command,
        is_executable = True,
    )

    # Subtool depends on RUNFILES_DIR/<workspace_name>/runfiles/data.txt.
    return [DefaultInfo(
        runfiles = ctx.runfiles(files = [ctx.files._data[0]]),
    )]

sub_tool = rule(
    implementation = _sub_tool_impl,
    executable = True,
    attrs = {
        "command": attr.string(),
        "_data": attr.label(
            allow_files = True,
            default = "//runfiles:data.txt"),
    },
)

def _complex_tool_impl(ctx):
    my_runfiles = ctx.runfiles(files = [ctx.files._data[0]])
    # Use runfiles.merge to merge the runfiles of both tools. All runfiles will
    # be rooted under the runfiles directory owned by this rule, however.
    my_runfiles = my_runfiles.merge(ctx.attr._subtool[DefaultInfo].default_runfiles)

    # Thus the example directory structure is:
    # runfiles/complex_tool     (executable)
    # runfiles/complex_tool.runfiles/
    #     <workspace_name>/
    #         runfiles/
    #             complex_tool_data.txt
    #             data.txt
    #             subtool

    runfiles_relative_tool_path = ctx.workspace_name + "/" + ctx.attr._subtool[DefaultInfo].files_to_run.executable.short_path

    # This tool forwards its runfiles directory via the RUNFILES_DIR to the
    # subtool, otherwise the subtool would be looking to $0.runfiles, which does
    # not exist.
    command = ("#!/bin/bash\nexport RUNFILES_DIR=\"$0.runfiles\" && "
             + "${RUNFILES_DIR}/%s $1 && cat ${RUNFILES_DIR}/examples/%s >> $1") % (
                   runfiles_relative_tool_path, ctx.files._data[0].short_path)

    ctx.actions.write(
        output = ctx.outputs.executable,
        content = command,
        is_executable = True,
    )

    return [DefaultInfo(
        runfiles = my_runfiles,
    )]

complex_tool = rule(
    implementation = _complex_tool_impl,
    executable = True,
    attrs = {
        "command": attr.string(),
        "_subtool": attr.label(
            allow_files = True,
            default = "//runfiles:subtool"),
        "_data": attr.label(
            allow_files = True,
            default = "//runfiles:complex_tool_data.txt"),
    },
)
