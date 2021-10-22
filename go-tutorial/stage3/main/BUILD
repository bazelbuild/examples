load("@io_bazel_rules_go//go:def.bzl", "go_binary")

go_binary(
    name = "hello-world",
    srcs = [
        "hello-world.go",
        "hello-greet.go",
    ],
    deps = [
        "//lib:hello-time",
    ],
    visibility = ["//visibility:public"],
)
