load("@bazel_skylib//:version.bzl", "hello_msg")
print(hello_msg)

cc_binary(
  name = "main",
  srcs = ["main.cc"],
  deps = [
    "@lib_a//:lib_a",
    "@com_github_google_glog//:glog",
  ],
)
