"""
File with all 3rd party dependencies needed for the project

They are loaded but without loading the transitive dependencies, this is done in
a separated file extra_dependencies.bzl
"""

load("//third_party/bazel_skylib:direct.bzl", "load_bazel_skylib")
load("//third_party/buildtools:direct.bzl", "load_buildtools")
load("//third_party/catch2:direct.bzl", "load_catch2")
load("//third_party/gtest:direct.bzl", "load_gtest")
load("//third_party/protobuf:direct.bzl", "load_protobuf")
load("//third_party/rules_go:direct.bzl", "load_rules_go")

def load_third_party_libraries():
    """Load all third party dependencies"""
    load_bazel_skylib()
    load_buildtools()
    load_catch2()
    load_gtest()
    load_protobuf()
    load_rules_go()
