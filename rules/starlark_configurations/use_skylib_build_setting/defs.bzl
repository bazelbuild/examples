# Load the provider of the pre-made settings defined in bazel_skylib
load("@bazel_skylib//rules:common_settings.bzl", "BuildSettingInfo")

def _impl(ctx):
    # Access the value of --//starlark_configurations/use_skylib_build_setting:flavor for the target.
    print("flavor: " + ctx.attr._flavor[BuildSettingInfo].value)
    return []

dessert = rule(
    implementation = _impl,
    attrs = {
        # Depend on the build setting so that we can access it in the rule implementation.
        # Use a private attribute (one that is prefixed with "_") so that target writers
        # can't override the value.
        "_flavor": attr.label(default = ":flavor"),
    },
)
