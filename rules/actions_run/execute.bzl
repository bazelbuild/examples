def _impl(ctx):
  # The list of arguments we pass to the script.
  args = [ctx.outputs.out.path] + [f.path for f in ctx.files.chunks]
  # Action to call the script.
  ctx.actions.run(
      inputs=ctx.files.chunks,
      outputs=[ctx.outputs.out],
      arguments=args,
      progress_message="Merging into %s" % ctx.outputs.out.short_path,
      executable=ctx.executable._merge_tool)

concat = rule(
  implementation=_impl,
  attrs={
      "chunks": attr.label_list(allow_files=True),
      "out": attr.output(mandatory=True),
      "_merge_tool": attr.label(executable=True, cfg="host", allow_files=True,
                                default=Label("//actions_run:merge"))
  }
)
