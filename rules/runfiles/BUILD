load(":complex_tool.bzl", "complex_tool", "sub_tool")
load(":execute.bzl", "execute")
load(":library.bzl", "runfiles_library", "runfiles_binary")
load(":tool.bzl", "tool", "tool_user")

# Tests in this package do not currently work on Windows (as they create
# shell-script binaries not compatible with Windows).

# 1. Basic example
# `bazel build //runfiles:e` will create an executable.
# `bazel run //runfiles:e` will run it.
execute(
    name = "e",
    # The location will be expanded to "pkg/data.txt", and it will reference
    # the data.txt file in runfiles when this target is invoked as
    # "bazel run //pkg:e".
    command = "cat $(location :data.txt)",
    data = [":data.txt"],
)

# 2. Library runfiles
# A library which declares a file which is needed at runtime, and a binary
# which invokes this library without direct knowledge of its runtime requirement.
# `bazel run //runfiles:bin` to run this example.
runfiles_library(
    name = "lib",
    command = "cat $(location :data.txt)",
    data = [":data.txt"],
)
runfiles_binary(
    name = "bin",
    lib = ":lib",
)

# 3. Tool with runfiles
# An action may require use of a tool that needs files at runtime. This demonstrates
# the definition of such a tool and a rule which uses that tool.
# `bazel build //runfiles:tool_user` to build this example.
tool(
    name = "tool",
)
tool_user(
    name = "tool_user",
    tool = ":tool",
)

# 4. Tool with runfiles depending on another tool
# Same as (3), except adds the complexity of a tool which depends on another tool
# with its own runfiles. This adds complexity, as the runfiles of the "inner tool"
# is in a different location than if the inner tool were run itself.
# `bazel build //runfiles:complex_tool_user` to build this example.
sub_tool(
    name = "subtool",
)
complex_tool(
    name = "complex_tool",
)
tool_user(
    name = "complex_tool_user",
    tool = ":complex_tool",
)


