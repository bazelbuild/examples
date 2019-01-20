package(default_visibility = ["//visibility:public"])

java_binary(
    name = "ProjectRunner",
    srcs = glob(["src/main/java/com/example/ProjectRunner.java"]),
    deps = [":greeter"],
)

java_library(
    name = "greeter",
    srcs = glob(["src/main/java/com/example/Greeting.java"]),
)
