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
