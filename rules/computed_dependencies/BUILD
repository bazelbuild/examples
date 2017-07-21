load("//computed_dependencies:hash.bzl", "md5_sum")

# Run 'bazel build :hash_no_space' to compute the md5 sum of 'hello.txt'
# ignoring all spaces.
md5_sum(
    name = "hash_no_space",
    src = "hello.txt",
    filter = "spaces",
)

# Run 'bazel build :hash_no_comment' to compute the md5 sum of 'hello.txt',
# ignoring lines that start with '#'.
md5_sum(
    name = "hash_no_comments",
    src = "hello.txt",
    filter = "comments",
)

# Run 'bazel build :hash' to compute the md5 sum of 'hello.txt'.
md5_sum(
    name = "hash",
    src = "hello.txt",
)

## spaces and comments are used by the rule 'md5_sum'.
sh_binary(
    name = "comments",
    srcs = ["comments.sh"],
)

sh_binary(
    name = "spaces",
    srcs = ["spaces.sh"],
)
