"""
Dependency to catch2, a unit test framework for C++
"""

load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def load_catch2():
    # It is not clear in Bazel what is the best practice for using http_archive.
    # If you call http_archive without any kind of check, you could call it two times
    # with the same name and different parameters and you would not get any warning/error.
    #
    # One option is to check if it is already available in the existing_rules and only
    # call http_archive if it is not present. In the else you could display a message in
    # case that was already present but in reality you would only want a warning/error if was
    # already called with different parameters (different library version for example).
    #
    # Another option is to wrap the http_archive in a maybe call but this will also not display
    # a warning. It behaves like the if check with the advantage that the name has not to be
    # repeated
    maybe(
        http_archive,
        name = "catch2",
        url = "https://github.com/catchorg/Catch2/archive/refs/tags/v2.13.9.zip",
        sha256 = "860e3917f07d7ee75654f86900d50a03acf0047f6fe5ba31d437e1e9cda5b456",
        strip_prefix = "Catch2-2.13.9",
    )
