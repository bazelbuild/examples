load(":execute.bzl", "concat")

concat(
    name = "sh",
    out = "page.html",
    chunks = [
        "header.html",
        "body.html",
        "footer.html",
    ],
    merge_tool = select({
        "//conditions:default": "//actions_run:merge_on_linux",
        "on_linux": "//actions_run:merge_on_linux",
        "on_windows": "//actions_run:merge_on_windows.bat",
    }),
)

# This target is used by the shell rule.
sh_binary(
    name = "merge_on_linux",
    srcs = ["merge.sh"],
)

sh_binary(
    name = "merge_on_windows.bat",
    srcs = ["merge.bat"],
)

config_setting(
    name = "on_linux",
    constraint_values = [
        "@platforms//os:linux",
    ],
)

config_setting(
    name = "on_windows",
    constraint_values = [
        "@platforms//os:windows",
    ],
)
