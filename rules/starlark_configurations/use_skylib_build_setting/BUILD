# We don't actually need an external repo to define Starlark settings. But
# Skylib provides convenience macros that reduce boilerplate, so we'll use that
# here.
#
# See https://docs.bazel.build/versions/master/skylark/config.html and
# https://github.com/bazelbuild/bazel-skylib) for more info.

load("@bazel_skylib//rules:common_settings.bzl", "string_flag")
load(":defs.bzl", "dessert")

# This can be set on the command line by setting 
# //starlark_configurations/use_skylib_build_setting:flavor=<flavor>.
# We declare here that its default value is "strawberry".
string_flag(name = "flavor", build_setting_default = "strawberry")

# A rule that depends on `:flavor`which means it can read its value.
dessert(name = "ice-cream")