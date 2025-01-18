cc_library(
    name = "hello-greet",
    srcs = ["hello-greet.cc"],
    hdrs = ["hello-greet.h"],
)

cc_binary(
    name = "hello-world",
    srcs = ["hello-world.cc"],
    deps = [
        ":hello-greet",
        "//lib:hello-time",
    ],
)

cc_test(
    name = "hello_test",
    size = "small",
    srcs = ["test.cpp"],
    deps = [
        "@googletest//:gtest",
        "@googletest//:gtest_main",
    ],
)