load("@rules_cc//cc:defs.bzl", "cc_binary")

cc_binary(
    name = "hello-world",
    srcs = ["hello-world.cc"],
    features = ["full_static_link"],
    linkstatic = True,
)

cc_binary(
    name = "mybin_full_static_linux_x86_64",
    srcs = [
        "bin.c",
        "lib.h",
    ],
    features = ["full_static_link"],
    linkopts = [
        "-v",
    ],
    linkstatic = True,
    deps = [":mylib_linux_x86_64"],
)

cc_library(
    name = "mylib_linux_x86_64",
    srcs = [
        "lib.c",
        "lib.h",
    ],
    copts = [
        "-v",
    ],
)
