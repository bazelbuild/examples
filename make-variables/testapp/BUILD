load("//testapp:defs.bzl", "var_providing_rule")

# Targets demonstrating predefined variables available to all rules:
genrule(
    name = "show_predefined_variables",
    srcs = [],
    outs = ["show_predefined_variables.out"],
    cmd = """cat <<EOF > $@
COMPILATION_MODE: $(COMPILATION_MODE)
BINDIR: $(BINDIR)
GENDIR: $(GENDIR)
TARGET_CPU: $(TARGET_CPU)
EOF""",
)

# Targets demonstrating predefined variables only available to genrules:
genrule(
    name = "show_genrule_variables",
    srcs = [
        "show_genrule_variables1.src",
        "show_genrule_variables2.src",
    ],
    outs = [
        "subdir/show_genrule_variables1.out",
        "subdir/show_genrule_variables2.out",
    ],
    cmd = """cat <<EOF > $(execpath subdir/show_genrule_variables1.out)
SRCS: $(SRCS)
OUTS: $(OUTS)
RULEDIR: $(RULEDIR)
@D (prefer RULEDIR to this): $(@D)
 * Because this genrule has multiple outputs, @D is the same as RULEDIR.
EOF
touch $(execpath subdir/show_genrule_variables2.out)
""",
)

genrule(
    name = "single_file_genrule",
    srcs = ["show_genrule_variables1.src"],
    outs = ["subdir/single_file_genrule.out"],
    cmd = """cat <<EOF > $@
<: $<
@: $@
RULEDIR: $(RULEDIR)
@D: $(@D)
 * Because this genrule has one input, < is a valid variable.
 * Because this genrule has one output, @ is a valid variable.
 * Because this genrule has one output, @D is different than RULEDIR.
EOF
    """,
)

# Targets demonstrating predefined input/output path variables:
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

# Targets demonstrating Starlrk-defined custom variables:
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
