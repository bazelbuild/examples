"""Example definition of a simple symbolic macro."""

def _impl(name, visibility, srcs, out, _count_tool):
    native.genrule(
        # Main target of the macro, so name = name.
        name = name,
        # To declare that this target is exported to our caller and not just an
        # implementation detail of our macro, we thread the macro's visibility
        # attribute into the target.
        #
        # The value of our visibility parameter is whatever was passed in by
        # our caller, plus the location of the call site (so that our exported
        # targets are guaranteed to always be visible to the caller without
        # them having to explicitly say so).
        #
        # We do *not* write visibility = ["//visibility:public"], which would
        # make this target exposed to the entire build, ignoring the visibility
        # passed in from the caller.
        visibility = visibility,
        outs = [out],
        tools = [_count_tool],
        srcs = srcs,
        cmd = "$(location %s) -n 5 $(SRCS) > $@" % str(_count_tool),
    )

    # Hidden feature: Also emit a count of letters, to be aggregated by a
    # separate reporting macro, generate_letter_frequencies.
    native.genrule(
        # Not the main target, so we append a suffix to distinguish it.
        # Conventionally the suffix begins with an underscore.
        name = name + "_gen_letter_freq",
        # This target and the output file it creates are visible to a friend
        # package, but are not exported to the caller of this macro.
        visibility = ["//letter_metrics:__pkg__"],
        # (If we wanted to both export it *and* make it visible to the friend,
        # we'd write `visibility = visibility + ["//letter_metrics:__pkg__"]`.)
        outs = [name + "_letter_freq"],
        tools = [_count_tool],
        srcs = srcs,
        cmd = "$(location %s) -l $(SRCS) > $@" % str(_count_tool),
    )

count_words = macro(
    implementation = _impl,
    attrs = {
        "srcs": attr.label_list(mandatory=True),
        "out": attr.output(mandatory=True),
        # An alternative to having _count_tool be an implicit dependency attribute
        # would be to just reference the label directly in the implementation
        # function. There, we could create a local variable
        #
        #     _count_tool = Label(":counter.py")
        #
        # Note that we wouldn't write
        #
        #     _count_tool = ":counter.py"
        #
        # since that would resolve relative to the package this macro is
        # instantiated in, rather than relative to this .bzl file.
        "_count_tool": attr.label(default=":counter.py", configurable=False),
    },
    doc = """
    Emits a file containing a few of the most frequently occurring words in the
    given srcs.
    """,
)
