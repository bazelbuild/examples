load("@rules_cc//cc:cc_library.bzl", "cc_library")

# This load statement replaces the native cc_binary (which has no "set_features"
# attribute) with a macro that strings together the logic to make that work,
# then passes everything else back to the native cc_binary.
load(":defs.bzl", "cc_binary")

# To see what this does, try "$ bazel run //:app_with_feature1".
cc_binary(
    name = "app_with_feature1",
    srcs = ["main.cc"],
    set_features = "feature1",
    deps = [":lib"],
)

# To see what this does, try "$ bazel run //:app_with_feature2".
cc_binary(
    name = "app_with_feature2",
    srcs = ["main.cc"],
    set_features = "feature2",
    deps = [":lib"],
)

# This binary "forgets" to set any features, so we have it fail with
# a descriptive message.
cc_binary(
    name = "app_forgets_to_set_features",
    srcs = ["main.cc"],
    deps = [":lib"],
)

# The library only builds if some feature is requested.
cc_library(
    name = "lib",
    srcs = ["lib.cc"],
    copts = select(
        {
            "//cc_binary_selectable_copts/custom_settings:feature1": ["-Dfeature1"],
            "//cc_binary_selectable_copts/custom_settings:feature2": ["-Dfeature2"],
        },
        no_match_error = "You must explicitly set which features you want!",
    ),
)
