load("@bazel_tools//tools/cpp:unix_cc_toolchain_config.bzl", "cc_toolchain_config")

cc_toolchain_config(
    name = "cc_toolchain_config",
    abi_libc_version = "unknown",
    abi_version = "unknown",
    builtin_sysroot = "external/gcc_toolchain/x86_64-buildroot-linux-gnu/sysroot",
    compile_flags = [
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
    host_system_name = "local",
    link_flags = ["--specs=nosys.specs"],
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
    toolchain_type = "@rules_cc//cc:toolchain_type",
)

filegroup(
    name = "all",
    srcs = glob(
        [
            "bin/**",
            "**/*.h",
        ],
        exclude = [
            # "lib/gawk/**",
        ],
    ),
)

cc_toolchain(
    name = "cc_toolchain",
    all_files = ":all",
    ar_files = ":all",
    as_files = ":all",
    compiler_files = ":all",
    dwp_files = ":all",
    dynamic_runtime_lib = ":all",
    linker_files = ":all",
    objcopy_files = ":all",
    static_runtime_lib = ":all",
    strip_files = ":all",
    toolchain_config = ":cc_toolchain_config",
)
