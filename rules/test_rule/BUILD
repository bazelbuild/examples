load(":line_length.bzl", "line_length_test")

# Run 'bazel test :80columns' to check that all files in the directory
# have at most 80 columns.
line_length_test(
    name = "80columns",
    srcs = glob(["*"]),
    columns = 80,
)
