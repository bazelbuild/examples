load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

java_binary(
    name = "ProjectRunner",
    srcs = ["src/main/java/com/example/ProjectRunner.java"],
    main_class = "com.example.ProjectRunner",
    deps = [":greeter",":calculator"],
)

java_library(
    name = "java-maven-lib",
    srcs = glob(["src/main/java/com/example/*.java"]),
)

java_test(
    name = "runner_tests",
    srcs = glob(["src/test/java/com/example/*.java"]),
    test_class = "com.example.CalculatorTest",
    deps = [
        ":java-maven-lib",
        "@maven//:junit_junit",
    ],
)

java_library(
    name = "greeter",
    srcs = ["src/main/java/com/example/Greeting.java"],
    visibility = ["//src/main/java/com/example/cmdline:__pkg__"],
)

java_library(
    name = "calculator",
    srcs = ["src/main/java/com/example/Calculator.java"],
    visibility = ["//src/main/java/com/example/cmdline:__pkg__"],
)