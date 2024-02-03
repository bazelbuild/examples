load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def instal_basic_gcc_toolchain():
    http_archive(
        name = "gcc_toolchain",
        urls = ["https://toolchains.bootlin.com/downloads/releases/toolchains/x86-64/tarballs/x86-64--glibc--stable-2023.11-1.tar.bz2"],
        strip_prefix = "x86-64--glibc--stable-2023.11-1",
        sha256 = "e3c0ef1618df3a3100a8a167066e7b19fdd25ee2c4285cf2cfe3ef34f0456867",
        build_file = "//basic/gcc/toolchain:cc_toolchain.BUILD",
    )

    native.register_toolchains(
        "@gcc_toolchain//:toolchain",
    )
