# Examples to build C++ code

This folder is part of the C++ Bazel Tutorial, found at <https://bazel.build/start/cpp>

This package will showcase how to build C++ code in stages.

### Stage 0
The zeroth stage shows how to setup the toolchain so that Bazel doesn't use the system's C++ gcc toolchain.
This provides hermeticity, making the build more portable and reproducible on other computers.

> Note, this is for Linux only. MacOS and Windows still use the system gcc.

### Stage 1
The first stage is really simple and shows you how to compile a binary with a single source file.

### Stage 2
The second stage will showcase how to build an application with multiple source and header files, separated in a library and a binary.

### Stage 3
The third stage showcases how to link multiple build directories by building multiple libraries in different packages and then connecting it up with the main application.
