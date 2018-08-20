"""Rule with a mandatory provider.

In this example, rules have a number attribute. Each rule adds its number
with the numbers of its transitive dependencies, and write the result in a
file. This shows how to transfer information from a dependency to its
dependents.
"""

NumberInfo = provider("number")

def _impl(ctx):
    result = ctx.attr.number
    for dep in ctx.attr.deps:
        result += dep[NumberInfo].number
    ctx.actions.write(output = ctx.outputs.out, content = str(result))

    # Return the provider with result, visible to other rules.
    return [NumberInfo(number = result)]

sum = rule(
    implementation = _impl,
    attrs = {
        "number": attr.int(default = 1),
        # All deps must provide all listed providers.
        "deps": attr.label_list(providers = [NumberInfo]),
    },
    outputs = {"out": "%{name}.sum"},
)
