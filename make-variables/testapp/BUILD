load("//testapp:defs.bzl", "var_providing_rule")

cc_binary(
    name = "app",
    srcs = ["app.cc"],
)

genrule(
    name = "show_app_output",
    srcs = ["empty.source"],
    outs = ["app_output"],
    cmd = """cat <<EOF > $@
:app output paths
 execpath: $(execpath :app)
 runfiles: $(rootpath :app)
 location: $(location :app)

source file paths
 execpath: $(execpath empty.source)
 runfiles: $(rootpath empty.source)
 location: $(location empty.source)
EOF""",
    tools = [":app"],
)

var_providing_rule(
    name = "set_foo_to_bar",
    var_value = "bar",
)

genrule(
    name = "show_custom_var",
    srcs = [],
    outs = ["custom_var"],
    cmd = "echo FOO is equal to $(FOO)! > $@",
    toolchains = [":set_foo_to_bar"],
)

