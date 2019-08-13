load("//yolo:defs.bzl", "yolo_library")

package(default_visibility = ["//visibility:private"])

alias(
    name = "a",
    actual = select({
        ":is_windows": ":a_windows",
        ":is_linux": ":a_linux",
    }),
    visibility = ["//visibility:public"],
)

config_setting(
    name = "is_windows",
    constraint_values = ["@platforms//os:windows"],
)

config_setting(
    name = "is_linux",
    constraint_values = ["@platforms//os:linux"],
)

yolo_library(
    name = "a_windows",
)

yolo_library(
    name = "a_linux",
)
