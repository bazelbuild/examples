load("//custom_outputs:extension.bzl", "rule_with_outputs")

# Run 'bazel build :my_rule' to generate both 'my_output.txt' and
# 'other_output.txt'.
rule_with_outputs(
    name = "my_rule",
    outs = [
        "my_output.txt",
        "other_output.txt",
    ],
)
