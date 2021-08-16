"""Generate a file.

In this example, the content is passed via an attribute. If you generate
large files with a lot of static content, consider using
`ctx.actions.expand_template` instead.
"""

def file(**kwargs):
    _file(out = "{name}.txt".format(**kwargs), **kwargs)

def _impl(ctx):
    output = ctx.outputs.out
    ctx.actions.write(output = output, content = ctx.attr.content)

_file = rule(
    implementation = _impl,
    attrs = {"content": attr.string(), "out": attr.output()},
)
