# Stage 2

### Library

Here, we introduce the ```cc_library``` rule for building C++ libraries. We have a ```cc_library``` named ```hello-greet``` and its header and source files are defined accordingly.
```
cc_library(
    name = "hello-greet",
    srcs = ["hello-greet.cc"],
    hdrs = ["hello-greet.h"],
)
```

### Binary

The ```cc_binary``` rule we saw in stage 1 has not changed, except that we now depend on the ```cc_library``` ```hello-greet```.
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

# In Windows, note the three slashes

bazel build ///main:hello-world
```
