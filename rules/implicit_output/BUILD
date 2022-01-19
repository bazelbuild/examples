load(":hash.bzl", "hashes")

# `bazel build //implicit_output:*`
# You will see 3 outputs: build_hash.md5, build_hash.sha1, and build_hash.sha256
hashes(
    name = "build_hash",
    src = "BUILD",
)
