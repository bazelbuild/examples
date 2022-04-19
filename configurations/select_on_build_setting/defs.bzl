FruitInfo = provider(fields = ["type"])

def _impl(ctx):
    print("We're harvesting " + ctx.attr.fruit[FruitInfo].type + "!")

harvest = rule(
    implementation = _impl,
    attrs = {
        "fruit": attr.label(),
    },
)

def _fruit_impl(ctx):
    return FruitInfo(type = ctx.label.name)

fruit = rule(
    implementation = _fruit_impl,
)
