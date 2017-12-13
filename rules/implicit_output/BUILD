load(":hash.bzl", "hashes")

# To get the md5 sum, run:
# `bazel build //implicit_output:build_hash.md5sum`
# If you use `//implicit_output:build_hash`, you will get only the sha256.
hashes(
    name = "build_hash",
    src = "BUILD",
)
