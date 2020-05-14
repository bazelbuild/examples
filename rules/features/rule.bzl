"""This example shows how to access rule features."""

def _impl(ctx):
    print("in {}, the enabled features are {}".format(ctx.label.name, ctx.features))

myrule = rule(
    implementation = _impl,
)
