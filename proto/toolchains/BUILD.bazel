load("@aspect_rules_py//py:defs.bzl", "py_unpacked_wheel")
load("@rules_proto//proto:defs.bzl", "proto_lang_toolchain")

# See WORKSPACE.bazel for fetch instructions of "protobuf_4_25_3".
# Turn the downloaded .whl file into a py_library-shape (provides PyInfo)
# Avoids the need for any pip, requirements files, python interpreter for wheel unpacking, etc
py_unpacked_wheel(
    name = "protobuf_wheel",
    src = "@protobuf_4_25_3//file",
    py_package_name = "protobuf",
)

# Configure protoc to have the right arguments for generating Python stubs.
proto_lang_toolchain(
    name = "protoc_py_toolchain",
    command_line = "--python_out=%s",
    progress_message = "Generating Python proto_library %{label}",
    runtime = ":protobuf_wheel",
)

proto_lang_toolchain(
    name = "protoc_java_toolchain",
    command_line = "--java_out=%s",
    progress_message = "Generating Java proto_library %{label}",
    runtime = "@protobuf-java_3_25_3//jar",
)

# Adapters to the register_toolchains call, adding the toolchain_type to above
toolchain(
    name = "protoc_py_toolchain.registration",
    toolchain = ":protoc_py_toolchain",
    toolchain_type = "@rules_python//python/proto:toolchain_type",
)

toolchain(
    name = "protoc_java_toolchain.registration",
    toolchain = ":protoc_java_toolchain",
    toolchain_type = "@rules_java//java/proto:toolchain_type",
)
