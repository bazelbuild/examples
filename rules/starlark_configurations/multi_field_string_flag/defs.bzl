# example/buildsettings/build_settings.bzl
FlavorsProvider = provider(fields = ['flavors'])

def _flavors_impl(ctx):
    return FlavorsProvider(flavors = ctx.attr.flavors)

flavors = rule(
    implementation = _flavors_impl,
    attrs = {
      "flavors": attr.label_list(),
    }
)

FlavorProvider = provider(fields = ['flavor', 'color', 'type'])

def _flavor_impl(ctx):
    return FlavorProvider(flavor = ctx.attr.name, color = ctx.attr.color, type = ctx.attr.type)

flavor = rule(
    implementation = _flavor_impl,
    attrs = {
      "color": attr.string(),
      "type": attr.string(default = "undefined"),
    }
)

FlavorFlagProvider = provider(fields = ['flavor'])

def _flavor_flag_impl(ctx):
    # use `ctx.build_setting_value` to access the raw value 
    # of this build setting. This value is either derived from
    # the default value set on the target or from the setting
    # being set somewhere on the command line/in a transition, etc.
    raw_flavor = ctx.build_setting_value

    # Returns a provider like a normal rule
    return FlavorFlagProvider(flavor = raw_flavor)

flavor_flag = rule(
    implementation = _flavor_flag_impl,
    # This line separates a build setting from a regular target, by using
    # the `build_setting` atttribute, you mark this rule as a build setting
    # including what raw type it is and if it can be used on the command
    # line or not (if yes, you must set `flag = True`)
    build_setting = config.string(flag = True)
)

FlavorAspectProvider = provider(fields = ['flavor', 'color', 'type'])

def _aspect_impl(target, ctx):
    raw_color = "undefined"
    raw_type = "undefined"
    raw_flavor = "undefined"
    if hasattr(ctx.rule.attr, 'flavors'):
      for flavor in ctx.rule.attr.flavors:
        if flavor[FlavorProvider].flavor == ctx.attr._flavor[FlavorFlagProvider].flavor:
          raw_flavor = flavor[FlavorProvider].flavor
          raw_color = flavor[FlavorProvider].color
          raw_type = flavor[FlavorProvider].type
    return [FlavorAspectProvider(flavor = raw_flavor, color = raw_color, type = raw_type)]

flavor_aspect = aspect(
    implementation = _aspect_impl,
    attr_aspects = ["flavors"],
    # Passing the build setting flag here makes the string available to the aspect
    attrs = {
      "_flavor": attr.label(default = ":flavor_flag")
    }
)

def _impl(ctx):
    print (ctx.attr.flavors[FlavorAspectProvider].type if ctx.attr.flavors[FlavorAspectProvider].type != "undefined" else "",
           ctx.attr.flavors[FlavorAspectProvider].flavor + " is " + ctx.attr.flavors[FlavorAspectProvider].color)
    return []

ice_cream = rule(
    implementation = _impl,
    attrs = {
      "flavors": attr.label( aspects = [flavor_aspect]),
    },
)
