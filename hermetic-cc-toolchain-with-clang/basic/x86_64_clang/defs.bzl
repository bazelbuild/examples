load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def instal_clang_toolchain_x86_64():
    http_archive(
        name = "glibc",
        urls = ["https://toolchains.bootlin.com/downloads/releases/toolchains/x86-64/tarballs/x86-64--glibc--stable-2023.11-1.tar.bz2"],
        strip_prefix = "x86-64--glibc--stable-2023.11-1",
        sha256 = "e3c0ef1618df3a3100a8a167066e7b19fdd25ee2c4285cf2cfe3ef34f0456867",
        build_file = "//basic/x86_64_clang:glibc.BUILD",
    )

    http_archive(
        name = "libcxx",
        urls = ["https://github.com/llvm/llvm-project/releases/download/llvmorg-14.0.6/libcxx-14.0.6.src.tar.xz"],
        strip_prefix = "libcxx-14.0.6.src",
        sha256 = "f7a9865e25a6c5175549e31609605767bf3478e8cdf8428be2c911838e7b683d",
        build_file = "//basic/x86_64_clang:libcxx.BUILD",
    )

    http_archive(
        name = "x86_64_clang",
        urls = ["https://github.com/llvm/llvm-project/releases/download/llvmorg-14.0.6/clang+llvm-14.0.6-x86_64-linux-gnu-rhel-8.4.tar.xz"],
        strip_prefix = "clang+llvm-14.0.6-x86_64-linux-gnu-rhel-8.4",
        sha256 = "7412026be8bb8f6b4c25ef58c7a1f78ed5ea039d94f0fa633a386de9c60a6942",
        build_file = "//basic/x86_64_clang:x86_64_clang.BUILD",
    )

    native.register_toolchains(
        "@x86_64_clang//:toolchain",
    )

def flags():
    # FIXME: avoid using prefix
    path_prefix = "/usr/local/google/home/vinhdaitran/github/daivinhtran/examples/hermetic-cc-toolchain-with-clang/bazel-hermetic-cc-toolchain-with-clang/"

    libraries_to_link_paths = [
        "external/glibc/x86_64-buildroot-linux-gnu/sysroot/usr/lib",
    ]

    libraries_to_link_absolute_paths = [prefix + path for path in libraries_to_link_paths]

    compiler_flags = []
    linker_flags = []

    for path in libraries_to_link_absolute_paths:
        compile_flags.extend([
            "-Xlinker",
            "-rpath",
            "-Xlinker",
            path,
        ])
        linker_flags.append(
            "-L" + path,
        )

    return compiler
