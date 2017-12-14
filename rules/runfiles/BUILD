load(":execute.bzl", "execute")

# `bazel build //runfiles:all` will create an executable.
# `bazel run //runfiles:all` will run it.
execute(
    name = "e",
    # The location will be expanded to "pkg/data.txt", and it will reference
    # the data.txt file in runfiles when this target is invoked as
    # "bazel run //pkg:e".
    command = "cat $(location :data.txt)",
    data = [":data.txt"],
)
