load("@aspect_rules_swc//swc:defs.bzl", "swc")

swc(
    name = "transpile",
    srcs = [
        "app.jsx",
        "index.jsx",
    ],
    swcrc = "//react-webpack:.swcrc",
    visibility = ["//visibility:public"],
)
