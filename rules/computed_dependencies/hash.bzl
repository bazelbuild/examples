"""This example shows how a rule can compute implicit dependencies.

Bazel needs to know about all dependencies before doing the analysis phase
and calling the implementation function. Dependencies can be computed
based on the rule attributes: to do so, use a function as the default
value of an attribute (the attribute must be private and have type 'label'
or 'list of labels'). The parameters of this function must correspond to
the attributes that are accessed in the function body.

The example below computes the md5 sum of a file. The file can be
preprocessed using a filter. The exact dependencies depend on the filter
chosen by the user.
"""

_filters = {
    "comments": Label("//computed_dependencies:comments"),
    "spaces": Label("//computed_dependencies:spaces"),
    "none": None,
}

def _get_filter(filter):  # requires attribute "filter"
    # Return the value for the attribute "_filter_bin"
    # It can be a label or None.
    return _filters[filter]

def _impl(ctx):
    src = ctx.file.src

    if not ctx.attr._filter_bin:
        # Skip the processing
        processed = src
    else:
        # The temporary file is based on 'ctx.label.name' to avoid conflicts.
        processed = ctx.actions.declare_file(ctx.label.name + "_processed")

        # Run the selected binary
        ctx.actions.run(
            outputs = [processed],
            inputs = [ctx.file.src],
            progress_message = "Apply filter '%s'" % ctx.attr.filter,
            arguments = [ctx.file.src.path, processed.path],
            executable = ctx.executable._filter_bin,
        )

    # Compute the hash
    out = ctx.outputs.text
    ctx.actions.run_shell(
        outputs = [out],
        inputs = [processed],
        command = "md5sum < %s > %s" % (processed.path, out.path),
    )

md5_sum = rule(
    implementation = _impl,
    attrs = {
        "filter": attr.string(values = _filters.keys(), default = "none"),
        "src": attr.label(mandatory = True, allow_single_file = True),
        "_filter_bin": attr.label(default = _get_filter, executable = True, cfg = "host"),
    },
    outputs = {"text": "%{name}.txt"},
)
