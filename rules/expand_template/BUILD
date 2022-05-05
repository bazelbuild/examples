load("@rules_cc//cc:defs.bzl", "cc_binary")
load(":hello.bzl", "hello")

# Generates a C++ source file that says "Hello John!".
hello(
    name = "hello_john_src",
    firstname = "John",
)

cc_binary(
    name = "hello",
    srcs = [":hello_john_src"],
)
