load(":execute.bzl", "concat")

concat(
    name = "sh",
    out = "page.html",
    chunks = [
        "header.html",
        "body.html",
        "footer.html",
    ],
)

# This target is used by the shell rule.
sh_binary(
    name = "merge",
    srcs = ["merge.sh"],
)
