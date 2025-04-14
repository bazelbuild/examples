load("@bazel_skylib//:version.bzl", "hello_msg")
load("@rules_cc//cc:cc_binary.bzl", "cc_binary")

# buildifier: disable=print
print(hello_msg)

cc_binary(
    name = "main",
    srcs = ["main.cc"],
    deps = [
        "@com_github_google_glog//:glog",
        "@lib_a",
    ],
)
