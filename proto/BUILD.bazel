load("@rules_proto//proto:defs.bzl", "proto_library")
load("@rules_python//python:proto.bzl", "py_proto_library")

package(default_visibility = ["//visibility:public"])

proto_library(
    name = "foo_proto",
    srcs = ["foo.proto"],
)

py_proto_library(
    name = "foo_py_proto",
    deps = [":foo_proto"],
)

java_proto_library(
    name = "foo_java_proto",
    deps = [":foo_proto"],
)
