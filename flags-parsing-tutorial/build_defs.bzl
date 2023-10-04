# buildifier: disable=module-docstring
BuildSettingInfo = provider(doc = "", fields = ["value"])

def _string_imp(ctx):
    value = ctx.build_setting_value
    label = ctx.label.name

    # buildifier: disable=print
    print("evaluated value for " + label + ": " + value)
    return BuildSettingInfo(value = value)

string_flag = rule(
    implementation = _string_imp,
    # https://bazel.build/extending/config#the-build_setting-rule-parameter
    build_setting = config.string(flag = True),
)
