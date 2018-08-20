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
        if NumberInfo in dep:
            result += dep[NumberInfo].number
    ctx.actions.write(output = ctx.outputs.out, content = str(result))

    # Return the provider with result, visible to other rules.
    return [NumberInfo(number = result)]

sum = rule(
    implementation = _impl,
    attrs = {
        "number": attr.int(default = 1),
        "deps": attr.label_list(),
    },
    outputs = {"out": "%{name}.sum"},
)
