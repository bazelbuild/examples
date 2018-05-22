"""Example of using an aspect to collect information from dependencies.

For more information about aspects, see the documentation:
  https://docs.bazel.build/versions/master/skylark/aspects.html
"""

FileCollector = provider(
    fields = {"files": "collected files"},
)

def _file_collector_aspect_impl(target, ctx):
    # This function is executed for each dependency the aspect visits.

    # Collect files from the srcs
    direct = [
        f
        for f in ctx.rule.files.srcs
        if ctx.attr.extension == "*" or ctx.attr.extension == f.extension
    ]

    # Combine direct files with the files from the dependencies.
    files = depset(
        direct = direct,
        transitive = [dep[FileCollector].files for dep in ctx.rule.attr.deps],
    )

    return [FileCollector(files = files)]

file_collector_aspect = aspect(
    implementation = _file_collector_aspect_impl,
    attr_aspects = ["deps"],
    attrs = {
        "extension": attr.string(values = ["*", "h", "cc"]),
    },
)

def _file_collector_rule_impl(ctx):
    # This function is executed once per `file_collector`.

    # Write the collected information to the output file.
    content = []
    for dep in ctx.attr.deps:
        files = [f.short_path for f in dep[FileCollector].files.to_list()]
        content.append("files from {}: {}".format(dep.label, files))
    content += [""]  # trailing newline

    ctx.actions.write(
        output = ctx.outputs.out,
        content = "\n".join(content),
    )

file_collector = rule(
    implementation = _file_collector_rule_impl,
    attrs = {
        "deps": attr.label_list(aspects = [file_collector_aspect]),
        "extension": attr.string(default = "*"),
    },
    outputs = {"out": "%{name}.files"},
)
