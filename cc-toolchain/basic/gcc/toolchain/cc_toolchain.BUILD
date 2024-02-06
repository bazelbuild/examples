load("@bazel_tools//tools/cpp:unix_cc_toolchain_config.bzl", "cc_toolchain_config")

cc_toolchain_config(
    name = "cc_toolchain_config",
    abi_libc_version = "unknown",
    abi_version = "unknown",
    # TODO Remove absolute path
    builtin_sysroot = "external/gcc_toolchain/x86_64-buildroot-linux-gnu/sysroot",
    compile_flags = [
        # "-no-canonical-prefixes",
        "-v",
        "-fno-canonical-system-headers",
        "-isystem",
        "external/gcc_toolchain/x86_64-buildroot-linux-gnu/include/c++/12.3.0",
        "-isystem",
        "external/gcc_toolchain/x86_64-buildroot-linux-gnu/include/c++/12.3.0/x86_64-buildroot-linux-gnu",
        "-isystem",
        "external/gcc_toolchain/x86_64-buildroot-linux-gnu/include",
        "-isystem",
        "external/gcc_toolchain/x86_64-buildroot-linux-gnu/sysroot/usr/include",
        "-isystem",
        "external/gcc_toolchain/lib/gcc/x86_64-buildroot-linux-gnu/12.3.0/include",
    ],
    compiler = "gcc",
    cpu = "x86_64",
    cxx_builtin_include_directories = [
        "%sysroot%/include/c++/12.3.0",
        "%sysroot%/include/c++/12.3.0/x86_64-linux",
        "%sysroot%/lib/gcc/x86_64-linux/12.3.0/include-fixed",
        "%sysroot%/lib/gcc/x86_64-linux/12.3.0/include",
        "%sysroot%/usr/include",
    ],
    host_system_name = "local",
    link_flags = [
        "-v",
        # "-fuse-ld=lld",
        "-print-search-dirs",
        "-Bexternal/gcc_toolchain/x86_64-buildroot-linux-gnu/sysroot/bin",
        "-Bexternal/gcc_toolchain/x86_64-buildroot-linux-gnu/sysroot/usr/lib64",
        "-Bexternal/gcc_toolchain/x86_64-buildroot-linux-gnu/sysroot/lib64",
        "-Lexternal/gcc_toolchain/x86_64-buildroot-linux-gnu/sysroot/bin",
        "-Lexternal/gcc_toolchain/x86_64-buildroot-linux-gnu/sysroot/usr/lib64",
        "-Lexternal/gcc_toolchain/x86_64-buildroot-linux-gnu/sysroot/lib64",
    ],
    # link_flags = ["--specs=nosys.specs"],
    target_libc = "unknown",
    target_system_name = "local",
    tool_paths = {
        "gcc": "bin/x86_64-buildroot-linux-gnu-gcc",
        "cpp": "bin/x86_64-buildroot-linux-gnu-cpp",
        "ar": "bin/x86_64-buildroot-linux-gnu-ar",
        "nm": "bin/x86_64-buildroot-linux-gnu-nm",
        "ld": "bin/x86_64-buildroot-linux-gnu-ld",
        "as": "bin/x86_64-buildroot-linux-gnu-as",
        "objcopy": "bin/x86_64-buildroot-linux-gnu-objcopy",
        "objdump": "bin/x86_64-buildroot-linux-gnu-objdump",
        "gcov": "bin/x86_64-buildroot-linux-gnu-gcov",
        "strip": "bin/x86_64-buildroot-linux-gnu-strip",
        "llvm-cov": "/bin/false",
    },
    toolchain_identifier = "arm_gcc",
)

toolchain(
    name = "toolchain",
    exec_compatible_with = [
        "@platforms//os:linux",
        "@platforms//cpu:x86_64",
    ],
    target_compatible_with = [
        "@platforms//os:linux",
        "@platforms//cpu:x86_64",
    ],
    toolchain = ":cc_toolchain",
    toolchain_type = "@bazel_tools//tools/cpp:toolchain_type",
)

filegroup(
    name = "include",
    srcs = glob([
        "lib/gcc/x86_64-buildroot-linux-gnu/*/include/**",
        "lib/gcc/x86_64-buildroot-linux-gnu/*/include-fixed/**",
        "x86_64-buildroot-linux-gnu/include/**",
        "x86_64-buildroot-linux-gnu/sysroot/usr/include/**",
        "x86_64-buildroot-linux-gnu/include/c++/*/**",
        "x86_64-buildroot-linux-gnu/include/c++/*/x86_64-buildroot-linux-gnu/**",
        "x86_64-buildroot-linux-gnu/include/c++/*/backward/**",
    ]),
    visibility = ["//visibility:public"],
)

filegroup(
    name = "lib",
    srcs = glob(
        include = [
            "lib64/**",
            "x86_64-buildroot-linux-gnu/sysroot/lib64/libc.so.6",
            "x86_64-buildroot-linux-gnu/sysroot/usr/lib64/**",
            # "x86_64-buildroot-linux-gnu/sysroot/usr/lib64/crti.o",
            "lib64/gcc/x86_64-buildroot-linux-gnu/12.3.0/**",
        ],
        exclude = [
            "lib*/**/*python*/**",
            "lib/gawk/**",
        ],
    ),
)

filegroup(
    name = "compiler_files",
    srcs = [
        ":gcc",
        ":include",
    ],
)

filegroup(
    name = "linker_files",
    srcs = [
        ":gcc",
        ":lib",
    ],
)

filegroup(
    name = "all_files",
    srcs = [
        ":compiler_files",
        ":include",
        ":linker_files",
    ],
)

filegroup(
    name = "gcc",
    srcs = [
        "bin/x86_64-buildroot-linux-gnu-cpp",
        "bin/x86_64-buildroot-linux-gnu-cpp.br_real",
        "bin/x86_64-buildroot-linux-gnu-g++",
        "bin/x86_64-buildroot-linux-gnu-g++.br_real",
        "bin/x86_64-buildroot-linux-gnu-gcc",
        "bin/x86_64-buildroot-linux-gnu-gcc.br_real",
    ] + glob([
        "**/cc1plus",
        "**/cc1",
        "lib64/libgmp.so*",
        "lib64/libmpc.so*",
        "lib64/libmpfr.so*",
    ]),
    visibility = ["//visibility:public"],
)

cc_toolchain(
    name = "cc_toolchain",
    all_files = ":all_files",
    ar_files = ":all_files",
    as_files = ":all_files",
    compiler_files = ":compiler_files",
    dwp_files = ":all_files",
    dynamic_runtime_lib = ":all_files",
    linker_files = ":linker_files",
    objcopy_files = ":all_files",
    static_runtime_lib = ":all_files",
    strip_files = ":all_files",
    toolchain_config = ":cc_toolchain_config",
)
