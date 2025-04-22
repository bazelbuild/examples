load("@rules_java//java:java_library.bzl", "java_library")

java_library(
    name = "macAndCheese",
    srcs = ["MacAndCheese.java"],
    visibility = ["//visibility:public"],
    deps = [
        "//src/main/java/com/example/ingredients:cheese",
        "//src/main/java/com/example/ingredients:macaroni",
    ],
)

java_library(
    name = "pizza",
    srcs = ["Pizza.java"],
    visibility = ["//visibility:public"],
    deps = [
        "//src/main/java/com/example/ingredients:cheese",
        "//src/main/java/com/example/ingredients:dough",
        "//src/main/java/com/example/ingredients:tomato",
    ],
)
