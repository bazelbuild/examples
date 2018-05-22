"""Generate a file.

In this example, the content is passed via an attribute. If you generate
large files with a lot of static content, consider using
`ctx.actions.expand_template` instead.
"""

def _impl(ctx):
    output = ctx.outputs.out
    ctx.actions.write(output = output, content = ctx.attr.content)

file = rule(
    implementation = _impl,
    attrs = {"content": attr.string()},
    outputs = {"out": "%{name}.txt"},
)
