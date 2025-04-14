load("@rules_cc//cc:cc_library.bzl", "cc_library")

package(default_visibility = ["//visibility:public"])

cc_library(
    name = "lib_a",
    srcs = ["lib_a.cc"],
    hdrs = ["lib_a.h"],
    deps = ["@com_github_google_glog//:glog"],
)
