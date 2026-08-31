load("@bazel_tools//tools/cpp:unix_cc_toolchain_config.bzl", "cc_toolchain_config")

cc_toolchain_config(
    name = "cc_toolchain_config",
    abi_libc_version = "unknown",
    abi_version = "unknown",
    builtin_sysroot = "external/glibc/x86_64-buildroot-linux-gnu/sysroot",
    compile_flags = [
        "--target=x86_64-unknown-linux-gnu",
        "-mcpu=x86_64",
        "-Wp,-v",
        # C++ Standard Library
        "-stdlib=libstdc++",
        # "-std=c++17",
        "-isystem",
        "external/libcxx/include",
        # C Standard Library
        "-isystem",
        "external/glibc/x86_64-buildroot-linux-gnu/sysroot/usr/include",
        "-isystem",
        "external/glibc/lib/gcc/x86_64-buildroot-linux-gnu/12.3.0/include",
        "-isystem",
        "external/glibc/x86_64-buildroot-linux-gnu/include/c++/12.3.0",
        "-isystem",
        "external/glibc/lib/gcc/x86_64-buildroot-linux-gnu/12.3.0/include-fixed",
        "-isystem",
        "external/glibc/x86_64-buildroot-linux-gnu/include/c++/12.3.0/x86_64-buildroot-linux-gnu",
        # Clang
        "-isystem",
        "external/x86_64_clang/include/c++/v1",
        "-isystem",
        "external/x86_64_clang/include",
        "-isystem",
        "external/x86_64_clang/include/x86_64-unknown-linux-gnu/c++/v1",
        "-Xlinker",
        "-rpath",
        "-Xlinker",
        "/usr/local/google/home/vinhdaitran/github/daivinhtran/examples/hermetic-cc-toolchain-with-clang/bazel-hermetic-cc-toolchain-with-clang/external/glibc/lib64/gcc/x86_64-buildroot-linux-gnu/12.3.0",
        "-Xlinker",
        "-rpath",
        "-Xlinker",
        "/usr/local/google/home/vinhdaitran/github/daivinhtran/examples/hermetic-cc-toolchain-with-clang/bazel-hermetic-cc-toolchain-with-clang/external/glibc/x86_64-buildroot-linux-gnu/lib",
        # "-Xlinker",
        # "-rpath",
        # "-Xlinker",
        # "/usr/local/google/home/vinhdaitran/github/daivinhtran/examples/bazel-hermetic-cc-toolchain-with-clang/external/glibc/x86_64-buildroot-linux-gnu/lib64",
    ],
    compiler = "clang++",
    cpu = "x86_64",
    cxx_builtin_include_directories = [
        "external/glibc/lib64/gcc/x86_64-buildroot-linux-gnu/12.3.0",
        "external/glibc/x86_64-buildroot-linux-gnu/lib64",
    ],
    host_system_name = "local",
    link_flags = [
        "--target=x86_64-unknown-linux-gnu",
        "-v",
        "-Wl,-S",
        "--rtlib=compiler-rt",
        "-fuse-ld=lld",
        # FIX ME
        "-L/usr/local/google/home/vinhdaitran/github/daivinhtran/examples/hermetic-cc-toolchain-with-clang/bazel-hermetic-cc-toolchain-with-clang/external/glibc/lib64/gcc/x86_64-buildroot-linux-gnu/12.3.0",
        "-L/usr/local/google/home/vinhdaitran/github/daivinhtran/examples/hermetic-cc-toolchain-with-clang/bazel-hermetic-cc-toolchain-with-clang/external/glibc/x86_64-buildroot-linux-gnu/sysroot/usr/lib",
        # "-L/usr/local/google/home/vinhdaitran/github/daivinhtran/examples/bazel-hermetic-cc-toolchain-with-clang/external/glibc/x86_64-buildroot-linux-gnu/sysroot/usr/lib64",
        # "-L/usr/local/google/home/vinhdaitran/github/daivinhtran/examples/bazel-hermetic-cc-toolchain-with-clang/external/glibc/x86_64-buildroot-linux-gnu/lib64",
        # "-L/usr/local/google/home/vinhdaitran/github/daivinhtran/examples/hermetic-cc-toolchain-with-clang/bazel-hermetic-cc-toolchain-with-clang/external/glibc/x86_64-buildroot-linux-gnu/sysroot/lib",
        "-L/usr/local/google/home/vinhdaitran/github/daivinhtran/examples/hermetic-cc-toolchain-with-clang/bazel-hermetic-cc-toolchain-with-clang/external/glibc/x86_64-buildroot-linux-gnu/sysroot/lib64/",
    ],
    target_libc = "unknown",
    target_system_name = "local",
    tool_paths = {
        "clang": "bin/clang",
        "clang++": "bin/clang++",
        "ar": "bin/llvm-ar",
        "nm": "bin/nm",
        "ld": "bin/clang",
        "llvm-cov": "/bin/false",
        "cpp": "/bin/false",
        "objcopy": "bin/llvm-objcopy",
        "objdump": "bin/llvm-objdump",
        "strip": "bin/llvm-strip",
        "gcc": "bin/clang++",
    },
    toolchain_identifier = "x86_64_clang",
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
    srcs = glob(["**/*"]) + [
        "@glibc//:all",
        "@libcxx//:all",
    ],
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
