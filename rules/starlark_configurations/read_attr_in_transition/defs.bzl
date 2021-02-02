# Load the provider of the pre-made settings defined in bazel_skylib.
load("@bazel_skylib//rules:common_settings.bzl", "BuildSettingInfo")

def _transition_impl(settings, attr):
    new_val = settings["//starlark_configurations/read_attr_in_transition:some-string"]

    if attr.do_transition:
        new_val = new_val + "-transitioned"

    return {"//starlark_configurations/read_attr_in_transition:some-string": new_val}

my_transition = transition(
    implementation = _transition_impl,
    inputs = ["//starlark_configurations/read_attr_in_transition:some-string"],
    outputs = ["//starlark_configurations/read_attr_in_transition:some-string"],
)

def _impl(ctx):
    print(ctx.attr.do_transition)
    print("value of some-string: " + ctx.attr._some_string[BuildSettingInfo].value)
    return []

my_rule = rule(
    implementation = _impl,
    cfg = my_transition,
    attrs = {
        "do_transition": attr.bool(),
        "_some_string": attr.label(default = Label("//starlark_configurations/read_attr_in_transition:some-string")),
        "_allowlist_function_transition": attr.label(
            default = "@bazel_tools//tools/allowlists/function_transition_allowlist",
        ),
    },
)
