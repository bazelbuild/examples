"""
Transitive dependencies for protobuf
"""

load("@com_google_protobuf//:protobuf_deps.bzl", "protobuf_deps")

def load_protobuf_transitive_dependencies():
    protobuf_deps()
