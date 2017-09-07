# Stage 2

### Library

Below we see a new configuration option. This is a c-compiler library with a seperate header file as an interface, the sources and a name for the library to be built.
```
cc_library(
    name = "hello-greet",
    srcs = ["hello-greet.cc"],
    hdrs = ["hello-greet.h"],
)
```

### Binary

The binary configuration we saw in stage 1 has not changed except that we now depend on the library hello-great.
```
cc_binary(
    name = "hello-world",
    srcs = ["hello-world.cc"],
    deps = [
        ":hello-greet",
    ],
)
```

To build this example you use (notice that 3 slashes are required in windows)
```
bazel build //main:hello-world
```
