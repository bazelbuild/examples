"""External repositories, fetched by Bazel during analysis, when bzlmod is disabled.

Unlike the rest of /WORKSPACE, these calls are guaranteed to be order-independent.
We also put all fetches in this file to make WORKSPACE shorter.
"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", _http_archive = "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")

# Wrap http_archive with maybe so we don't try to declare a dependency twice
def http_archive(**kwargs):
    maybe(_http_archive, **kwargs)

def fetch_deps():
    """Definitions of external fetches.

    Bazel will only lazy-download the ones needed for requested targets.
    """
    http_archive(
        name = "bazel_skylib",
        sha256 = "66ffd9315665bfaafc96b52278f57c7e2dd09f5ede279ea6d39b2be471e7e3aa",
        urls = [
            "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.4.2/bazel-skylib-1.4.2.tar.gz",
            "https://github.com/bazelbuild/bazel-skylib/releases/download/1.4.2/bazel-skylib-1.4.2.tar.gz",
        ],
    )

    http_archive(
        name = "buildifier_prebuilt",
        sha256 = "72b5bb0853aac597cce6482ee6c62513318e7f2c0050bc7c319d75d03d8a3875",
        strip_prefix = "buildifier-prebuilt-6.3.3",
        urls = [
            "http://github.com/keith/buildifier-prebuilt/archive/6.3.3.tar.gz",
        ],
    )

    http_archive(
        name = "catch2",
        url = "https://github.com/catchorg/Catch2/archive/refs/tags/v3.4.0.zip",
        sha256 = "cd175f5b7e62c29558d4c17d2b94325ee0ab6d0bf1a4b3d61bc8dbcc688ea3c2",
        strip_prefix = "Catch2-3.4.0",
    )

    http_archive(
        name = "gtest",
        url = "https://github.com/google/googletest/archive/release-1.8.1.zip",
        sha256 = "927827c183d01734cc5cfef85e0ff3f5a92ffe6188e0d18e909c5efebf28a0c7",
        strip_prefix = "googletest-release-1.8.1",
    )

    http_archive(
        name = "com_google_protobuf",
        url = "https://github.com/protocolbuffers/protobuf/archive/v3.13.0.tar.gz",
        sha256 = "9b4ee22c250fe31b16f1a24d61467e40780a3fbb9b91c3b65be2a376ed913a1a",
        strip_prefix = "protobuf-3.13.0",
    )
