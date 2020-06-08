# We don't actually need an external repo to define Starlark settings. But
# Skylib provides convenience macros that reduce boilerplate, so we'll use that
# here.
#
# See https://docs.bazel.build/versions/master/skylark/config.html and
# https://github.com/bazelbuild/bazel-skylib) for more info.
load("@bazel_skylib//rules:common_settings.bzl", "string_setting")
load(":defs.bzl", "my_rule")

string_setting(
    name = "some-string",
    build_setting_default = "abc",
)

my_rule(name = "dont-do-transition", do_transition = False)
my_rule(name = "do-transition", do_transition = True)

# This target will cause an issue because it tries to implement
# a rule transition that reads a configured attribute. 
my_rule(
    name = "will-break",
    do_transition = select({
        ":true": True,
        "//conditions:default": False,
    })
)

config_setting(
    name = "true",
    flag_values = {
        ":some-string": "abc",
    }
)

