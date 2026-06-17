This is an example of how to set up a hermetic single-platform gcc toolchain using the `cc_toolchain_config` rule provided from [`@bazel_tools//tools/cpp:unix_cc_toolchain_config.bzl`](https://github.com/bazelbuild/bazel/blob/master/tools/cpp/unix_cc_toolchain_config.bzl).

The example contains several pieces:

1. Download `glibc` from [toolchains.bootlin.com](toolchains.bootlin.com) which includes gcc and C standard library (aka `libc`)

2. Write the BUILD file for the glibc repository. The BUILD file

    a. A `cc_toolchain_config` target to specify compiler flags, linker flags, a map of gcc tools, and the sysroot. To ensures `gcc` uses the downloaded headers instead of system headers, compiler flags include `-fno-canonical-system-headers`.

    b. A `cc_toolchain` target to specify all the inputs that cc actions need in addition to the toolchain config.
    
    c. A `toolchain` target to specify the exec and target configuration, and the corresponding cc toolchain.

## Testing
To test the toolchain, run

```
$ bazel build //main:hello-world
INFO: Analyzed target //main:hello-world (0 packages loaded, 0 targets configured).
INFO: Found 1 target...
Target //main:hello-world up-to-date:
  bazel-bin/main/hello-world

$ bazel-bin/main/hello-world
Hello, World!
```

## Debugging

To confirm the build is hermetic, add `-v` to compiler flags and linker flags to ensure the header and library search paths and tools are in the toolchain (i.e. begining with `external/gcc_toolchain`) instead of system.

## Limitation

In Bazel sandbox, the linker fails finding `libc.so.6` since `($SYSROOT)/usr/lib/libc.so` refers to `libc.so.6` as

```
/* GNU ld script
   Use the shared library, but some functions are only in
   the static library, so try that secondarily.  */
OUTPUT_FORMAT(elf64-x86-64)
GROUP ( /lib64/libc.so.6 /usr/lib64/libc_nonshared.a  AS_NEEDED ( /lib64/ld-linux-x86-64.so.2 ) )
```

This is because linker fails to find resolve `($SYSROOT)/lib64/libc.so.6` correctly in the sandbox. See https://stackoverflow.com/questions/52386530/linker-fails-in-sandbox-when-running-through-bazel-but-works-when-sandboxed-comm for more context. We can confirm this is an issue in sandbox in two ways

1. Running the build with `--spawn_strategy=standalone`
2. Running the link command directly outside of Bazel

There are several ways to fix this:

1. If you have control over `libc.so`, change it to
```
/* GNU ld script
   Use the shared library, but some functions are only in
   the static library, so try that secondarily.  */
OUTPUT_FORMAT(elf64-x86-64)
GROUP ( /lib/x86_64-linux-gnu/libc.so.6 /lib/x86_64-linux-gnu/libc_nonshared.a AS_NEEDED ( /lib/x86_64-linux-gnu/ld-linux-x86-64.so.2 ) )
```

where the paths exist relative to the sysroot root.

2. Setting symbolic links
```
$ ln -s /lib/x86_64-linux-gnu/libc_nonshared.a /lib/x86_64-linux-gnu/libc_nonshared.a 
$ ln -s /lib/x86_64-linux-gnu/libc.so.6 /lib64/libc.so.6
```
