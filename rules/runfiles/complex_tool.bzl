"""Create a complex tool with runfiles and a rule which uses it.

A tool (executable used for action registration) may depend on another
tool with its own runfiles. This example demonstrates this scenario."""

# Bash helper function for looking up runfiles.
# Vendored from
# https://github.com/bazelbuild/bazel/blob/master/tools/bash/runfiles/runfiles.bash
BASH_RLOCATION_FUNCTION = r"""\
# --- begin runfiles.bash initialization v3 ---
set -uo pipefail; set +e; f=bazel_tools/tools/bash/runfiles/runfiles.bash
source "${RUNFILES_DIR:-/dev/null}/$f" 2>/dev/null || \
  source "$(grep -sm1 "^$f " "${RUNFILES_MANIFEST_FILE:-/dev/null}" | cut -f2- -d' ')" 2>/dev/null || \
  source "$0.runfiles/$f" 2>/dev/null || \
  source "$(grep -sm1 "^$f " "$0.runfiles_manifest" | cut -f2- -d' ')" 2>/dev/null || \
  source "$(grep -sm1 "^$f " "$0.exe.runfiles_manifest" | cut -f2- -d' ')" 2>/dev/null || \
  { echo>&2 "ERROR: cannot find $f"; exit 1; }; f=; set -e
# --- end runfiles.bash initialization v3 ---
"""

def _sub_tool_impl(ctx):
    # Since this tool may be used by another tool, it must support accepting
    # a different runfiles directory root. The runfiles helper library does this correctly.
    command = BASH_RLOCATION_FUNCTION + """

cat $(rlocation examples/runfiles/data.txt) > $1"""

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
            default = ":data.txt",
        ),
    },
)

def _complex_tool_impl(ctx):
    my_runfiles = ctx.runfiles(files = [ctx.files._data[0], ctx.files._runfiles_lib[0]])

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

    # This tool expects that the subtool will correctly resolve its runfiles.
    command = ("#!/bin/bash\n{}" +
               "$(rlocation {}) $1 && cat $(rlocation examples/{}) >> $1").format(
        BASH_RLOCATION_FUNCTION,
        runfiles_relative_tool_path,
        ctx.files._data[0].short_path,
    )

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
            default = ":subtool",
        ),
        "_data": attr.label(
            allow_files = True,
            default = ":complex_tool_data.txt",
        ),
        "_runfiles_lib": attr.label(
            allow_single_file = True,
            default = "@bazel_tools//tools/bash/runfiles",
        ),
    },
)
