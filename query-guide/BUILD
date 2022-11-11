load("@rules_java//java:defs.bzl", "java_binary")

java_binary(
    name = "runner",
    srcs = glob(["src/main/java/com/example/*.java"]),
    main_class = "com.example.Runner",
    deps = [
        "//src/main/java/com/example/restaurant:cafe"
    ],
)
