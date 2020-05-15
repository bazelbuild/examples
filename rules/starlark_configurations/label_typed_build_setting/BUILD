load(":defs.bzl", "tool", "toolbox")

# Define a label_flag (or setting) using the built in bazel rules.
# It's default value it the default target this flag should point to. 
label_flag(name = 'tool', build_setting_default = ":hammer")

# A target that depends on the label flag.
toolbox(name = "my-toolbox")

# Some targets that are potential values of the label flag.
tool(name = "hammer")
tool(name = "screwdriver")
