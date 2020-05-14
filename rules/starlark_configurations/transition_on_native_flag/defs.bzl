def _transition_impl(settings, attr):
    _ignore = settings, attr

    # Attaching the special prefix "//comand_line_option" to the name of a native
    # flag makes the flag available to transition on.
    return {"//command_line_option:cpu": "x86"}

# Define a transition.
cpu_transition = transition(
    implementation = _transition_impl,
    inputs = [],
    # We declare which flags the transition will be writing. The returned dict(s)
    # of flags must have keyset(s) that contains exactly this list.
    outputs = ["//command_line_option:cpu"],
)

def _impl(ctx):
    return []

# Define a rule that uses the transition.
cpu_rule = rule(
    implementation = _impl,
    # Attach the transition to the rule using the `cfg` attribute. This will transition
    # the configuration of this target, which the target's descendents will inherit.
    cfg = cpu_transition,
    attrs = {
        # This attribute is required to use starlark transitions. It allows
        # whitelisting usage of this rule. For more information, see
        # https://docs.bazel.build/versions/master/skylark/config.html#user-defined-transitions
        "_whitelist_function_transition": attr.label(
            default = "@bazel_tools//tools/whitelists/function_transition_whitelist",
        ),
    },
)