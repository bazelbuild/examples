# We don't actually need an external repo to define Starlark settings. But
# Skylib provides convenience macros that reduce boilerplate, so we'll use that
# here.
#
# See https://docs.bazel.build/versions/master/skylark/config.html and
# https://github.com/bazelbuild/bazel-skylib) for more info.
load("@bazel_skylib//rules:common_settings.bzl", "string_flag")
load(":defs.bzl", "shirt", "piece")

# Define a build setting using the skylib pre-defined settings
string_flag(name = "color", build_setting_default = "black")

# Define a rule with two dependencies
shirt(name = "tee", sleeve = ":sleeve", back = ":back")
piece(name = "sleeve")
piece(name = "back")
