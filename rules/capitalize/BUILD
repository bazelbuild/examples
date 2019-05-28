# A simple example of defining a rule to perform a simple transformation on
# text files. Uses the unix `tr` command.
#
# Build with:
#     bazel build //capitalize:upper_output.txt //capitalize:lower_output.txt

load(":capitalize.bzl", "generate_uppercase", "generate_lowercase")

generate_uppercase(
    name = "upper_text",
    input = "input.txt",
    output = "upper_output.txt",
)

generate_lowercase(
    name = "lower_text",
    input = "input.txt",
    output = "lower_output.txt",
)
