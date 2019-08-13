load("//yolo:defs.bzl", "fail_with_msg", "yolo_library")

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
    values = {
        "platforms": "//:windows_platform",
    },
)

config_setting(
    name = "is_linux",
    values = {
        "platforms": "//:linux_platform",
    },
)

yolo_library(
    name = "a_windows",
)

yolo_library(
    name = "a_linux",
)
