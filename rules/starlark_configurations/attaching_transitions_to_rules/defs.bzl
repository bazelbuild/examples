# Load the provider of the pre-made settings defined in bazel_skylib.
load("@bazel_skylib//rules:common_settings.bzl", "BuildSettingInfo")

# Define two different transitions that both transition the `color` build setting
# we defined in the BUILD.
def _blue_impl(settings, attr):
    _ignore = settings, attr

    return {"//starlark_configurations/attaching_transitions_to_rules:color": "blue"}

blue_transition = transition(
    implementation = _blue_impl,
    inputs = [],
    outputs = ["//starlark_configurations/attaching_transitions_to_rules:color"],
)

def _red_impl(settings, attr):
    _ignore = settings, attr

    return {"//starlark_configurations/attaching_transitions_to_rules:color": "red"}

red_transition = transition(
    implementation = _red_impl,
    inputs = [],
    outputs = ["//starlark_configurations/attaching_transitions_to_rules:color"],
)

def _impl(ctx):
    # Access the value of //starlark_configurations/attaching_transitions_to_rules:color for the target (blue).
    print("shirt color: " + ctx.attr._color[BuildSettingInfo].value)

    # Access the value of //starlark_configurations/attaching_transitions_to_rules:color for the transitioned dep (red).
    # Note that you have to index by [0] here for the transitioned dep and you don't need to
    # do so below - this is because attribute-attached transitions can transition to multiple
    # new configurations so you must specify which one you want.
    print("sleeve color: " + ctx.attr.sleeve[0][BuildSettingInfo].value)

    # Access the value of //starlark_configurations/attaching_transitions_to_rules:color for the non-transitioned dep (blue).
    print("back color: " + ctx.attr.back[BuildSettingInfo].value)
    return []

shirt = rule(
    implementation = _impl,
    # Attaching at rule transitions the configuration of this target and all its dependencies
    # (until it gets overwritten again, for example...)
    cfg = blue_transition,
    attrs = {
        # Attaching to an attribute transitions the configuration of this dependency (and
        # all its dependencies)
        "sleeve": attr.label(cfg = red_transition),
        # Here is an attribute with no transition so it will inherit its parent's --//:color
        "back": attr.label(),
        # Depend on the build setting so that we can access it in the rule implementation.
        # Use a private attribute (one that is prefixed with "_") so that target writers
        # can't override the value.
        "_color": attr.label(default = ":color"),
        # This attribute is required to use starlark transitions. It allows
        # allowlisting usage of this rule. For more information, see
        # https://docs.bazel.build/versions/master/skylark/config.html#user-defined-transitions
        "_allowlist_function_transition": attr.label(
            default = "@bazel_tools//tools/allowlists/function_transition_allowlist",
        ),
    },
)

def _piece_impl(ctx):
    return ctx.attr._color[BuildSettingInfo]

piece = rule(
    implementation = _piece_impl,
    attrs = {
        # Depend on the build setting so that we can access it in the rule implementation.
        # Use a private attribute (one that is prefixed with "_") so that target writers
        # can't override the value.
        "_color": attr.label(default = ":color"),
    },
)
