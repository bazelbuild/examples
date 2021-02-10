def _transition_impl(settings, attr):
    _ignore = settings

    # Attaching the special prefix "//comand_line_option" to the name of a native
    # flag makes the flag available to transition on. The result of this transition
    # is to set --cpu
    # We read the value from the attr also named `cpu` which allows target writers
    # to modify how the transition works. This could also just be a hardcoded
    # string like "x86" if you didn't want to give target writers that power.
    return {"//command_line_option:cpu": attr.cpu}

# Define a transition.
cpu_transition = transition(
    implementation = _transition_impl,
    inputs = [],
    # We declare which flags the transition will be writing. The returned dict(s)
    # of flags must have keyset(s) that contains exactly this list.
    outputs = ["//command_line_option:cpu"],
)

def _impl(ctx):
    # Print the current cpu using `ctx.var` which contains a
    # dict of configuration variable
    # https://docs.bazel.build/versions/master/skylark/lib/ctx.html#var
    print("--cpu=" + ctx.var["TARGET_CPU"])
    return []

# Define a rule that uses the transition.
cpu_rule = rule(
    implementation = _impl,
    # Attach the transition to the rule using the `cfg` attribute. This will transition
    # the configuration of this target, which the target's descendents will inherit.
    cfg = cpu_transition,
    attrs = {
        # This attribute is required to use starlark transitions. It allows
        # allowlisting usage of this rule. For more information, see
        # https://docs.bazel.build/versions/master/skylark/config.html#user-defined-transitions
        "_allowlist_function_transition": attr.label(
            default = "@bazel_tools//tools/allowlists/function_transition_allowlist",
        ),
        "cpu": attr.string(default = "x86"),
    },
)
