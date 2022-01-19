load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_library", "cc_test")
load("@com_github_bazelbuild_buildtools//buildifier:def.bzl", "buildifier")

package(default_visibility = ["//visibility:public"])

buildifier(
    name = "buildifier",
    lint_mode = "warn",
    mode = "check",
    verbose = True,
)

cc_binary(
    name = "hello_world",
    srcs = ["hello_world.cpp"],
    deps = [":my_lib"],
)

cc_library(
    name = "my_lib",
    srcs = ["my_lib.cpp"],
    hdrs = ["my_lib.h"],
)

cc_test(
    name = "catch2_test",
    srcs = ["catch2_test.cpp"],
    deps = [
        ":my_lib",
        "@catch2",
    ],
)

cc_test(
    name = "gtest_test",
    srcs = ["gtest_test.cpp"],
    deps = [
        ":my_lib",
        "@gtest",
        "@gtest//:gtest_main",
    ],
)
