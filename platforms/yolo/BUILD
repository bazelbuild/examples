# This package contains rule definitions for Yolo-lang.

load("//yolo:defs.bzl", "yolo_library", "yolo_toolchain")

# @local_config_platform is autodetecting host platform and generating relevant
# constraints.
load("@local_config_platform//:constraints.bzl", "HOST_CONSTRAINTS")

package(default_visibility = ["//visibility:public"])

# Toolchain type is used to distinguish toolchains per languages.
#
# By convention, toolchain_type targets are named "toolchain_type" and
# distinguished by their package path. So the full path for this would be
# //yolo:toolchain_type.
toolchain_type(name = "toolchain_type")

# Toolchain that targets Linux, and executes on Linux.
toolchain(
    name = "linux_toolchain",
    exec_compatible_with = [
        "@platforms//os:linux",
        ":yolo_lang_1",
    ],
    target_compatible_with = [
        "@platforms//os:linux",
        ":yolo_lang_1",
    ],
    toolchain = ":linux_yolo_toolchain",
    toolchain_type = ":toolchain_type",
)

yolo_toolchain(
    name = "linux_yolo_toolchain",
    executing_on_cpu = "-",
    executing_on_os = "linux",
    targetting_cpu = "-",
    targetting_os = "linux",
)

# Toolchain that targets Windows, and executes on Linux.
toolchain(
    name = "windows_toolchain",
    exec_compatible_with = ["@platforms//os:linux"],
    target_compatible_with = ["@platforms//os:windows"],
    toolchain = ":windows_yolo_toolchain",
    toolchain_type = ":toolchain_type",
)

yolo_toolchain(
    name = "windows_yolo_toolchain",
    executing_on_cpu = "-",
    executing_on_os = "linux",
    targetting_cpu = "-",
    targetting_os = "windows",
)

# Naive toolchain that targets and executes on the current host platforms as
# autodetected by Bazel.
toolchain(
    name = "host_toolchain",
    exec_compatible_with = HOST_CONSTRAINTS,
    target_compatible_with = HOST_CONSTRAINTS,
    toolchain = ":host_yolo_toolchain",
    toolchain_type = ":toolchain_type",
)

yolo_toolchain(
    name = "host_yolo_toolchain",
    executing_on_cpu = "host",
    executing_on_os = "host",
    targetting_cpu = "host",
    targetting_os = "host",
)

# Toolchain that targets Linux, and executes on Linux, and requires yolo_lang_3
# version.
toolchain(
    name = "linux_lang_3_toolchain",
    exec_compatible_with = [
        "@platforms//os:linux",
        ":yolo_lang_3",
    ],
    target_compatible_with = [
        "@platforms//os:linux",
        ":yolo_lang_3",
    ],
    toolchain = ":linux_lang_3_yolo_toolchain",
    toolchain_type = ":toolchain_type",
)

yolo_toolchain(
    name = "linux_lang_3_yolo_toolchain",
    executing_on_cpu = "-",
    executing_on_os = "linux_with_yolo_lang_3",
    targetting_cpu = "-",
    targetting_os = "linux_with_yolo_lang_3",
)

constraint_setting(name = "yolo_lang_version")

constraint_value(
    name = "yolo_lang_1",
    constraint_setting = ":yolo_lang_version",
)

constraint_value(
    name = "yolo_lang_2",
    constraint_setting = ":yolo_lang_version",
)

constraint_value(
    name = "yolo_lang_3",
    constraint_setting = ":yolo_lang_version",
)
