BuildSettingInfo = provider()

def _string_imp(ctx):
    value = ctx.build_setting_value
    label = ctx.label.name
    print("evaluated value for " + label + ": " + value)
    return BuildSettingInfo(value = value)

string_flag = rule(
    implementation = _string_imp,
    # https://docs.bazel.build/versions/main/skylark/config.html#the-build_setting-rule-parameter
    build_setting = config.string(flag = True),
)
