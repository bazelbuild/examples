config_setting(
    name = "darwin",
    values = {"cpu": "darwin"},
    visibility = ["//visibility:public"],
)

config_setting(
    name = "darwin_x86_64",
    values = {"cpu": "darwin_x86_64"},
    visibility = ["//visibility:public"],
)

filegroup(
    name = "all",
    srcs = [
        "//backend",
        "//android",
    ] + select({
        ":darwin": ["//ios-app"],
        ":darwin_x86_64": ["//ios-app"],
        "//conditions:default": [],
    }),
)
