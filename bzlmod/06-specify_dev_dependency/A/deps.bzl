load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@librarian//:librarian.bzl", "fetch_book")

def deps():
    fetch_book(
        name="the_great_gatsby",
        edition="2003.7",
    )

def dev_deps():
    http_archive(
        name = "bazel_skylib",
        urls = ["https://github.com/bazelbuild/bazel-skylib/releases/download/1.2.0/bazel-skylib-1.2.0.tar.gz"],
        sha256 = "af87959afe497dc8dfd4c6cb66e1279cb98ccc84284619ebfec27d9c09a903de",
    )
    fetch_book(
        name="hamlet",
        edition="2005.1",
    )
