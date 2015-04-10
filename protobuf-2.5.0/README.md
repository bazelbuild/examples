# Building protoc with Bazel

Instructions

1. Put BUILD into the root source directory
2. Put BUILD.src.google.protobuf into src/google/protobuf/BUILD
3. Run configure
4. Run bazel:

````
touch WORKSPACE
bazel build //src/google/protobuf:protoc
````
