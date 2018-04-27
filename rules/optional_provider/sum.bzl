"""Rule with an optional provider.

In this example, rules have a number attribute. Each rule adds its number
with the numbers of its transitive dependencies, and write the result in a
file. This shows how to transfer information from a dependency to its
dependents. Dependencies are not required to provide a number.
"""

NumberInfo = provider("number")

def _impl(ctx):
  result = ctx.attr.number
  for dep in ctx.attr.deps:
    result += dep[NumberInfo].number

  ctx.file_action(output=ctx.outputs.out, content=str(result))

  # Return the provider with result, visible to other rules.
  return [NumberInfo(number=result)]

sum = rule(
    attrs = {
        "number": attr.label(
            cfg = "host",
            allow_files = True,
        ),
        "deps": attr.label_list(),
    },
    outputs = {"out": "%{name}.sum"},
    implementation = _impl,
)
