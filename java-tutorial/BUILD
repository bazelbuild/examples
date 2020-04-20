load("@rules_java//java:defs.bzl", "java_binary")

java_binary(
    name = "ProjectRunner",
    srcs = ["src/main/java/com/example/ProjectRunner.java"],
    main_class = "com.example.ProjectRunner",
    deps = [":greeter"],
)

java_library(
    name = "greeter",
    srcs = ["src/main/java/com/example/Greeting.java"],
    visibility = ["//src/main/java/com/example/cmdline:__pkg__"],
)

java_library(
    name = "tests",
    srcs = ["src/test/java/com/example/TestJava.java"],
    deps = [
        "@maven//:junit_junit",
        "@maven//:org_hamcrest_hamcrest_core",
        ":greeter",
    ],
)

java_test(
    name = "AllTests",
    size = "small",
    test_class = "com.example.TestJava",
    runtime_deps = [
        ":tests",
    ],
)

# Kotlin Test Targets
load("@io_bazel_rules_kotlin//kotlin:kotlin.bzl", "kt_jvm_library")
load("@io_bazel_rules_kotlin//kotlin:kotlin.bzl", "kt_jvm_test")

kt_jvm_test(
    name = "all_kotlin_tests",
    test_class = "com.example.TestKotlin",
    srcs = ["src/test/java/com/example/TestKotlin.kt"],
    deps = [
        "@maven//:junit_junit",
        "@maven//:org_hamcrest_hamcrest_core",
        ":tests",
    ],
)
