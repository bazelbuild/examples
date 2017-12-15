"""Create an executable with runfiles.

Runfiles are files that are needed at runtime (when the executable in run).
This example also shows a use of `ctx.expand_location`.
"""

def _impl(ctx):
  # Expand the label in the command string to a runfiles-relative path.
  # The second arg is the list of labels that may be expanded.
  command = ctx.expand_location(ctx.attr.command, ctx.attr.data)

  # Create the output executable file with command as its content.
  ctx.file_action(
      output=ctx.outputs.executable,
      content=command,
      executable=True)

  # Create runfiles from the files specified in the data attribute.
  # The shell executable - the output of this rule - can use them at
  # runtime. It is also possible to define data_runfiles and
  # default_runfiles. However if runfiles is specified it's not possible to
  # define the above ones since runfiles sets them both.
  return [DefaultInfo(
      runfiles=ctx.runfiles(files=ctx.files.data)
  )]

execute = rule(
    implementation=_impl,
    executable=True,
    attrs={
        "command": attr.string(),
        "data": attr.label_list(allow_files=True),
    },
)
