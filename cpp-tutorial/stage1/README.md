# Stage 1

This showcases how to build a single file to create a runnable application.

In the build file for this stage you can read that this will use the c-compiler to build a binary file. (cc_binary)
The name of the binary will be hello-world
And we also name the required source files to build the current application.
```
cc_binary(
    name = "hello-world",
    srcs = ["hello-world.cc"],
)
```

To build this example you use (notice that 3 slashes are required in windows)
```
bazel build //main:hello-world
```

The build will output some information for example

```
____Loading complete.  Analyzing...
____Found 1 target...
____Building...
Target //main:hello-world up-to-date:
  C:/tools/msys64/tmp/_bazel_woden/vqeu6v3v/execroot/__main__/bazel-out/msvc_x64-fastbuild/bin/main/hello-world.exe
____Elapsed time: 0,400s, Critical Path: 0,01s
```

In the run log above you can see where the executable was built so you can locate it and use it.
