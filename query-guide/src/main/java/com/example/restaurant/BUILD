java_library(
    name = "chef",
    visibility = ["//visibility:public"],
    srcs = [
        "Chef.java",
    ],
    deps = [
        "//src/main/java/com/example/dishes:macAndCheese",
        "//src/main/java/com/example/dishes:pizza",
    ],

)

java_library(
    name = "cafe",
    visibility = ["//visibility:public"],
    srcs = [
        "Cafe.java",
    ],
    deps = [
        ":chef",
    ],
)
