exports_files(
    srcs = [
        "hamlet.txt",
        "macbeth.txt",
    ],
    # Visible to the //main package, but consumed by targets created in macros
    # that themselves have no intrinsic permission to view these files.
    visibility = ["//main:__pkg__"],
)
