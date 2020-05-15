ToolInfo = provider(fields = ["type"])

def _toolbox_impl(ctx):
	# Both `label_flag` and `label_setting` automatically forward the
	# providers of the target to which they point.
    print("Using a " + ctx.attr._tool[ToolInfo].type + ".")
    return []

toolbox = rule(
    implementation = _toolbox_impl,
    attrs = {
    	# Depend on the label flag to access the providers of whatever value
    	# the label flag is set to at the time the rule implementation is run.
        "_tool": attr.label(default = "//experimental/users/juliexxia:tool"),
    },
)

def _tool_impl(ctx):
    return ToolInfo(type = ctx.label.name)

tool = rule(
    implementation = _tool_impl,
)
