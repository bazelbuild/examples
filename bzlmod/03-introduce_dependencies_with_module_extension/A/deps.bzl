load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_file")

def emojis():
    http_file(
        name = "emojis",
        urls = ["https://raw.githubusercontent.com/datasets/emojis/4b470b8f873e47e7443b38ab1a1d8875d6f7253f/data/emojis.csv"],
        sha256 = "9768ed212d23668749c74c6a64d068ed818fda1ecb7aa6561655d58192db6382",
    )

data_deps_ext = module_extension(
    implementation = lambda ctx: emojis(),
)
