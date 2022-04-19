TemperatureProvider = provider(fields = ['type'])

temperatures = ["HOT", "LUKEWARM", "ICED"]

def _impl(ctx):
    # use `ctx.build_setting_value` to access the raw value 
    # of this build setting. This value is either derived from
    # the default value set on the target or from the setting
    # being set somewhere on the command line/in a transition, etc.
    raw_temperature = ctx.build_setting_value

    # Things you can do in a build setting implementation function
    # include more advanced type checking
    if raw_temperature not in temperatures:
        fail(str(ctx.label) + " build setting allowed to take values {"
             + ", ".join(temperatures) + "} but was set to unallowed value "
             + raw_temperature)

    # Returns a provider like a normal rule
    return TemperatureProvider(type = raw_temperature)

temperature = rule(
    implementation = _impl,
    # This line separates a build setting from a regular target, by using
    # the `build_setting` atttribute, you mark this rule as a build setting
    # including what raw type it is and if it can be used on the command
    # line or not (if yes, you must set `flag = True`)
    build_setting = config.string(flag = True)
)

def _day_impl(ctx):
    # This accesses the value of the build setting at the time this
    # rule implementation is evaluated. This means if there was
    # a command-line change or a transition change in the ancestors
    # of the target this is running for, those will be reflected here.
    print(ctx.attr._temperature[TemperatureProvider].type)
    return []


breakfast = rule(
    implementation = _day_impl,
    # Depending on a build setting will give you access to its
    # configuration value inside your rule implementation. You
    # might want to make these private attributes so rule
    # users can't change what piece of configuration you are
    # reading (https://docs.bazel.build/versions/master/skylark/rules.html#private-attributes-and-implicit-dependencies)
    attrs = {
        "_temperature": attr.label(default = ":coffee-temp")
    },
)
