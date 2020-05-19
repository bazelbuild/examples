ToolInfo = provider(fields = ["type"])

def _toolbox_impl(ctx):
    # Access the label flag attribute and you can expect the providers
    # of the value of the label flag.
    print("Using a " + ctx.attr._tool[ToolInfo].type + ".")
    return []

toolbox = rule(
    implementation = _toolbox_impl,
    attrs = {
        # Depend on the label flag.
        # Optionally use a private variable (one prefixed with "_" to prevent
        # target writers from changing what flag is read.
        "_tool": attr.label(default = "//starlark_configurations/label_typed_build_setting:tool"),
    },
)

def _tool_impl(ctx):
    return ToolInfo(type = ctx.label.name)

tool = rule(implementation = _tool_impl)
