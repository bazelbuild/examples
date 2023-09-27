"""Defines the string_flag setting.

Note that the same exists in bazel-skylib:
https://github.com/bazelbuild/bazel-skylib/blob/652c8f0b2817daaa2570b7a3b2147643210f7dc7/docs/common_settings_doc.md#string_flag
"""

BuildSettingInfo = provider(
    doc = "A singleton provider that contains the raw value of a build setting",
    fields = {
        "value": "The value of the build setting in the current configuration. " +
                 "This value may come from the command line or an upstream transition, " +
                 "or else it will be the build setting's default.",
    },
)

def _string_imp(ctx):
    value = ctx.build_setting_value
    label = ctx.label.name

    # buildifier: disable=print
    print("evaluated value for " + label + ": " + value)
    return BuildSettingInfo(value = value)

string_flag = rule(
    implementation = _string_imp,
    # https://docs.bazel.build/versions/main/skylark/config.html#the-build_setting-rule-parameter
    build_setting = config.string(flag = True),
)
