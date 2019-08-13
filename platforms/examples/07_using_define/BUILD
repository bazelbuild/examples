load("//yolo:defs.bzl", "yolo_library")

config_setting(
    name = "is_foo_defined",
    values = {
        "define": "is_foo_defined=true",
    },
)

filegroup(
    name = "everything",
    srcs = [":a"] + select({
        ":is_foo_defined": [":only_with_foo"],
        "//conditions:default": [],
    }),
)

yolo_library(
    name = "a",
)

yolo_library(
    name = "only_with_foo",
)
