"""Execute a binary.

The example below executes the binary target "//actions_run:merge" with
some arguments. The binary will be automatically built by Bazel.

The rule must declare its dependencies. To do that, we pass the target to
the attribute "merge_tool". Attributes like these are called "implicit
dependencies" because they are always there and the user does not set them.
They are often made private by prepending the attribute name with an underscore.
This means that the attribute cannot be set in a build file or macro (which
implies that it should have a meaningful default value in order to be useful to
the rule implementation). In the case of our example, because we want to use a
select() to pick different tools depending on the platform, we make the
merge_tool attribute public, but wrap all calls to the _concat rule in a macro
named concat, and set the merge_tool attribute. This effectively makes the
attribute private and allows us to use a select.
"""

def _impl(ctx):
    # The list of arguments we pass to the script.
    args = [ctx.outputs.out.path] + [f.path for f in ctx.files.chunks]

    # Action to call the script.
    ctx.actions.run(
        inputs = ctx.files.chunks,
        outputs = [ctx.outputs.out],
        arguments = args,
        progress_message = "Merging into %s" % ctx.outputs.out.short_path,
        executable = ctx.executable.merge_tool,
    )

_concat = rule(
    implementation = _impl,
    attrs = {
        "chunks": attr.label_list(allow_files = True),
        "out": attr.output(mandatory = True),
        "merge_tool": attr.label(
            executable = True,
            cfg = "host",
            allow_files = True,
            mandatory = True,
        ),
    },
)

def concat(name, chunks, out):
    _concat(
        name = name,
        chunks = chunks,
        out = out,
        merge_tool = select({
            "//conditions:default": "//actions_run:merge_on_linux",
            "//actions_run:on_linux": "//actions_run:merge_on_linux",
            "//actions_run:on_windows": "//actions_run:merge_on_windows.bat",
        }),
    )
