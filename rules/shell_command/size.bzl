def _impl(ctx):
  output = ctx.outputs.out
  input = ctx.file.file
  # The command may only access files declared in inputs.
  ctx.actions.run_shell(
      inputs=[input],
      outputs=[output],
      progress_message="Getting size of %s" % input.short_path,
      command="stat -L -c%%s %s > %s" % (input.path, output.path))

size = rule(
    implementation=_impl,
    attrs={"file": attr.label(mandatory=True, allow_files=True, single_file=True)},
    outputs={"out": "%{name}.size"},
)
