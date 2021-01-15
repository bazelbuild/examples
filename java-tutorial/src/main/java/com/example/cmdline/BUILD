load("@rules_java//java:defs.bzl", "java_binary", "java_library")

java_binary(
    name = "runner",
    srcs = ["Runner.java"],
    main_class = "com.example.cmdline.Runner",
    # deps = ["//:greeter"],
    deps = ["@lombok//:lombok"]
)

java_library(
    name = "lombok_example",
    srcs = ["LombokExample.java"],
    deps = ["@lombok//:lombok"],
    visibility = ["//visibility:public"]
)
