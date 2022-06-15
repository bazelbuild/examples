load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_file")

def world_cities():
    http_file(
        name = "world_cities",
        urls = ["https://raw.githubusercontent.com/datasets/world-cities/048e93f02a0d34f788e91e1bf2f2f59c7a4c9687/data/world-cities.csv"],
        sha256 = "4d2469729be61b55fcc758ab16bf590196733ff99f1c80e361623decb34ac35d",
    )

def _data_deps_extension_impl(ctx):
    world_cities()

data_deps_ext = module_extension(
    implementation = _data_deps_extension_impl,
)
