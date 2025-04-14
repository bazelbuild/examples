load("@rules_java//java:java_binary.bzl", "java_binary")

java_binary(
    name = "review",
    srcs = [
        "Review.java",
    ],
    main_class = "com.example.reviews.Review",
    tags = ["pizza"],
    visibility = ["//visibility:public"],
    deps = [
        "//src/main/java/com/example/customers:amir",
    ],
)
