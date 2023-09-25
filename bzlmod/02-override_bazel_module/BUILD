load("@bazel_skylib//:version.bzl", "hello_msg")

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
