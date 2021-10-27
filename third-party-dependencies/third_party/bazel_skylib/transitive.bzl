"""
Transitive dependencies for Bazel standard library Skylib
"""

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")

def load_bazel_skylib_transitive_dependencies():
    bazel_skylib_workspace()
