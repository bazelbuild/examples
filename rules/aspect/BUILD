load(":file_collector.bzl", "file_collector")

cc_library(
    name = "lib",
    srcs = [
        "lib.cc",
        "lib.h",
    ],
)

cc_binary(
    name = "app",
    srcs = [
        "app.cc",
    ],
    deps = ["lib"],
)

file_collector(
    name = "h_file_count",
    extension = "h",
    deps = ["app"],
)

file_collector(
    name = "file_count",
    deps = [
        "app",
        "lib",
    ],
)
