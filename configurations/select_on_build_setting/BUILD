# We don't actually need an external repo to define Starlark settings. But
# Skylib provides convenience macros that reduce boilerplate, so we'll use that
# here.
#
# See https://docs.bazel.build/versions/master/skylark/config.html and
# https://github.com/bazelbuild/bazel-skylib) for more info.
load("@bazel_skylib//rules:common_settings.bzl", "string_flag")
load(":defs.bzl", "harvest", "fruit")

string_flag(name = "season", build_setting_default = "spring")

harvest(
    name = 'harvest',
    fruit = select({
        ":summer": ":cherries",
        ":fall": ":pumpkins",
        "//conditions:default": ":apples"
    }),
)

config_setting(
    name = "summer",
    flag_values = {
        ":season": "summer",
    }
)

config_setting(
    name = "fall",
    flag_values = {
        ":season": "fall",
    }
)

fruit(name = "cherries")
fruit(name = "apples")
fruit(name = "pumpkins")
