"""Example definition of a macro that wraps a submacro."""

load("//word_counter:defs.bzl", "count_words")

def _impl(name, visibility, srcs, out, make_it_loud, _shout_tool, **kwargs):
    # If shouting is requested, make a genrule to create an intermediary file
    # holding a capitalized version of the text, and feed that into the
    # submacro in place of the original file.

    # It's important that make_it_loud is non-configurable. Otherwise it would
    # be a select() value that always evaluate to boolean True (#14506).
    if make_it_loud:
        native.genrule(
            name = name + "_gen_shouted",
            # No visibility passed in, so this genrule and the output file it
            # creates are private to (macros defined in) this package.
            # (We could've also written `visibility = ["//visibility:private"]`
            # to be explicit.)
            outs = [name + "_shouted"],
            tools = [_shout_tool],
            srcs = srcs,
            cmd = "$(location %s) $(SRCS) > $@" % str(_shout_tool),
        )

    count_words(
        name = name,
        visibility = visibility,
        # If we're forwarding our internal <name>_shouted target, count_words
        # is able to instantiate dependencies on it because we passed the label
        # in here as an attribute, even though package //word_counter is not in
        # <name>_shouted's visibility.
        srcs = [name + "_shouted"] if make_it_loud else srcs,
        out = out,
        # It's good practice to accept **kwargs and thread it through to the
        # rule or submacro being wrapped. This helps mitigate friction when the
        # schema of the wrapped object changes in ways that don't otherwise
        # interact with the wrapping macro's logic.
        #
        # (`out` is an example of a parameter that could've been omitted from
        # the implementation function and passed via **kwargs. We left it in
        # for better readability.)
        **kwargs
    )

fancy_count_words = macro(
    implementation = _impl,
    attrs = {
        "srcs": attr.label_list(mandatory=True),
        "out": attr.output(mandatory=True),
        # `configurable=False` means it can't be set to a select() by the user,
        # and won't be promoted to a select() when passed to the implementation
        # function. Morally, this should be used whenever the value is consumed
        # by the macro logic itself as opposed to being passed through to the
        # underlying targets.
        "make_it_loud": attr.bool(default=False, configurable=False),
        "_shout_tool": attr.label(default="//shout:shout.py", configurable=False),
    },
    doc = """
    Wraps the count_words macro, adding a shouting (capitalization) feature.
    """,
)

# A word about select() promotion: An initial version of this macro had it
# accepting a "src" attr.label() attribute, and instantiating count_words with
# `srcs = [src]`. This failed because src was promoted to a select(), which is
# not permitted as a list element. Had this been a legacy macro, we would not
# discover this problem until a client attempted to pass in a select() for src.
# Thanks to select() promotion, we found the problem immediately and could
# address it by changing the schema to be a attr.label_list(). We could have
# also kept it an attr.label() and set configurable=False.
