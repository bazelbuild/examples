load("//yolo:defs.bzl", "fail_with_msg", "yolo_library")
load("@bazel_skylib//lib:selects.bzl", "selects")

package(default_visibility = ["//visibility:private"])

alias(
    name = "a",
    actual = select({
        ":yolo_lang_greater_or_equal_than_2": ":a_needs_greater_or_equal_than_2",
    }),
    visibility = ["//visibility:public"],
)

# Matching both lang 2 and 3.
selects.config_setting_group(
    name = "yolo_lang_greater_or_equal_than_2",
    match_any = [
        ":yolo_lang_2_setting",
        ":yolo_lang_3_setting",
    ],
)

config_setting(
    name = "yolo_lang_2_setting",
    constraint_values = ["//yolo:yolo_lang_2"],
)

config_setting(
    name = "yolo_lang_3_setting",
    constraint_values = ["//yolo:yolo_lang_3"],
)

yolo_library(
    name = "a_needs_greater_or_equal_than_2",
)
