load(":defs.bzl", "temperature", "breakfast")

# A rule that can read `:coffee-temp`. See its implementation
# for details.
breakfast(name = 'today')

# This is a string-typed build setting. Here we set its
# default value to "HOT".
temperature(name = 'coffee-temp', build_setting_default = "HOT")
