# buildifier: disable=module-docstring
# We can transition on native options using this
# //command_line_option:<option-name> syntax
_BUILD_SETTING = "//command_line_option:test_arg"

def _test_arg_transition_impl(_, __):
    return {_BUILD_SETTING: ["new arg"]}

_test_arg_transition = transition(
    implementation = _test_arg_transition_impl,
    inputs = [],
    outputs = [_BUILD_SETTING],
)

def _test_transition_rule_impl(ctx):
    # We need to copy the executable because starlark doesn't allow
    # providing an executable not created by the rule
    executable_src = ctx.executable.actual_test
    executable_dst = ctx.actions.declare_file(ctx.label.name)
    ctx.actions.run_shell(
        tools = [executable_src],
        outputs = [executable_dst],
        command = "cp %s %s" % (executable_src.path, executable_dst.path),
    )
    runfiles = ctx.attr.actual_test[DefaultInfo].default_runfiles
    return [DefaultInfo(runfiles = runfiles, executable = executable_dst)]

transition_rule_test = rule(
    cfg = _test_arg_transition,
    implementation = _test_transition_rule_impl,
    attrs = {
        "actual_test": attr.label(cfg = "target", executable = True),
        "_allowlist_function_transition": attr.label(
            default = "@bazel_tools//tools/allowlists/function_transition_allowlist",
        ),
    },
    test = True,
)

def test_arg_cc_test(name, **kwargs):
    # Prepend leading underscore (_) to mark the native test as internal.
    cc_test_name = "_" + name + "_native_test"
    transition_rule_test(
        name = name,
        # bazel test picks up the args from the transitioned test.
        args = kwargs.pop("args", None),
        actual_test = ":%s" % cc_test_name,
    )

    # The native test is built as usual, but mark as "manual" so that blaze test :all
    # does not run it.
    native.cc_test(name = cc_test_name, tags = kwargs.pop("tags", []) + ["manual"], **kwargs)
