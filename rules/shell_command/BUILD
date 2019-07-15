# A small example that takes a text file and generates modified versions of it,
# and metadata about it. These rules depend on unix shell commands like `tr`
# and `stat`.
#
# Build with:
#     bazel build //shell_command:*

load(":rules.bzl", "emit_size", "convert_to_uppercase")

emit_size(
    name = "foo",
    file = "foo.txt",
    macos_syntax = select({
        "@bazel_tools//src/conditions:darwin": True,
        "@bazel_tools//src/conditions:darwin_x86_64": True,
        "//conditions:default": False,
    }),
)

convert_to_uppercase(
    name = "make_uppercase",
    input = "foo.txt",
    output = "upper_foo.txt",
)
