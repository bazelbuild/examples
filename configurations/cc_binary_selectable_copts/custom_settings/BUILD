# We don't actually need an external repo to define Starlark settings. But
# Skylib provides convenience macros that reduce boilerplate, so we'll use that
# here.
#
# See https://docs.bazel.build/versions/master/skylark/config.html and
# https://github.com/bazelbuild/bazel-skylib) for more info.

load("@bazel_skylib//rules:common_settings.bzl", "string_flag")

# You can avoid the need for this entire file by using "--define" instead
# ("--define" is a native flag, built into Bazel). But we recommend this
# Starlark-based approach. This lets you model the setting you need much
# more precisely, offers typing and validation, mixes more cleanly with
# other flags, and doesn't risk namespace collisions on complicated projects
# (e.g. imgagine unrelated package reading "--define mode=1" for some
# overloaded concept of "mode").

string_flag(
    # You can set this at the command line with "$ bazel build 
    # //starlark_configurations/cc_binary_selectable_copts:all
    # --//starlark_configurations/cc_binary_selectable_copts/custom_settings:mycopts". 
    # It's also possible to disallow command line access (in this example
    # we don't need that because cc_binary sets this implicitly without the
    # end user even having to know this flag exists). See 
    # https://docs.bazel.build/versions/master/skylark/config.html#the-build_setting-rule-parameter
    # for details.
    name = "mycopts",
    build_setting_default = "unset",
    # We could also omit this attribute to allow any value:
    values = [
        "feature1",
        "feature2",
        # We'll use this to trigger an error if the binary forgets
        # to set the feature it wants:
        "unset",
    ],
)

# Predefine config_settings matching each feature. The "flag_values" attribute
# is used to match Starlark-defined flags.
#
# You could write more Starlark macros to further automate and simplify this.

config_setting(
    name = "feature1",
    flag_values = {":mycopts": "feature1"},
)

config_setting(
    name = "feature2",
    flag_values = {":mycopts": "feature2"},
)
