load("//yolo:defs.bzl", "yolo_library")
load("@bazel_skylib//rules:common_settings.bzl", "bool_flag")

bool_flag(
    name = "foo_enabled",
    build_setting_default = False,
)

config_setting(
    name = "is_foo_enabled",
    flag_values = {
        ":foo_enabled": "True",
    },
)

filegroup(
    name = "everything",
    srcs = [":a"] + select({
        ":is_foo_enabled": [":only_with_foo"],
        "//conditions:default": [],
    }),
)

yolo_library(
    name = "a",
)

yolo_library(
    name = "only_with_foo",
)
