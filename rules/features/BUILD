load(":rule.bzl", "myrule")

package(features = ["foo"])

# features have no specific meaning in Bazel. Each rule can decide how to use
# the features.
# See the documentation:
#   https://docs.bazel.build/versions/master/be/common-definitions.html#common.features

# in target1, the enabled features are ["foo"]
myrule(
    name = "target1",
)

# in target2, the enabled features are ["bar", "foo"]
myrule(
    name = "target2",
    features = ["bar"],
)

# in target3, the enabled features are ["bar", "baz"]
myrule(
    name = "target3",
    features = [
        "-foo",
        "bar",
        "baz",
    ],
)
