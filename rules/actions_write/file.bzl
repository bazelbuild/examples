def _impl(ctx):
  output = ctx.outputs.out
  ctx.actions.write(output=output, content=ctx.attr.content)

file = rule(
    implementation=_impl,
    attrs={"content": attr.string()},
    outputs={"out": "%{name}.txt"},
)
