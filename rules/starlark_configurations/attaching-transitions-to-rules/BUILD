load("@bazel_skylib//rules:common_settings.bzl", "string_flag")
load(":defs.bzl", "shirt", "piece")

string_flag(name = "color", build_setting_default = "black")

shirt(name = "tee", sleeve = ":sleeve", back = ":back")
piece(name = "sleeve")
piece(name = "back")
