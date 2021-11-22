BuildSettingInfo = provider()

def _string_imp(ctx):
    value = ctx.build_setting_value
    label = ctx.label.name
    print("build setting value for label " + label + ": " + value)
    return BuildSettingInfo(value = value)

string_flag = rule(
    implementation = _string_imp,
    build_setting = config.string(flag = True),
)
