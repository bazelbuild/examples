load("@rules_java//java:defs.bzl", "java_binary")

java_binary(
    name = "ProjectRunner",
    srcs = glob(["src/main/java/com/example/*.java"]),
    deps = ["//src/main/java/com/example/cmdline:lombok_example"]
)

