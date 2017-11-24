# Stage 3

In this stage we step it up and showcase how to integrate multiple cc_library targets from different packages.

Below we see a similar configurtion from stage 2. The differance is that this BUILD file is in the lib package / directory. And the new property below visibility will tell bazel which other packages that should be able to see / use this package. So in this case the main package can use this ```cc_library```. 
```
cc_library(
    name = "hello-time",
    srcs = ["hello-time.cc"],
    hdrs = ["hello-time.h"],
    visibility = ["//main:__pkg__"],
)
```

Then to use the ```cc_library``` above we add an extra dependency for //[package-path]:[dependency-name].
```
cc_binary(
    name = "hello-world",
    srcs = ["hello-world.cc"],
    deps = [
        ":hello-greet",
        "//lib:hello-time",
    ],
)
```

To build this example you use
```
bazel build //main:hello-world
```
