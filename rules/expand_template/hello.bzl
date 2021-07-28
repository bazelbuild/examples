"""Generate a file using a template.

It is much more memory-efficient to use a template file than creating the whole
content during the analysis phase.
"""

# Label of the template file to use.
_TEMPLATE = "//expand_template:hello.cc"

def hello(**kwargs):
    _hello(
        source_file = "{name}.cc".format(**kwargs),
        **kwargs
    )

def _hello_impl(ctx):
    ctx.actions.expand_template(
        template = ctx.file._template,
        output = ctx.outputs.source_file,
        substitutions = {
            "{FIRSTNAME}": ctx.attr.firstname,
        },
    )

_hello = rule(
    implementation = _hello_impl,
    attrs = {
        "firstname": attr.string(mandatory = True),
        "_template": attr.label(
            default = Label(_TEMPLATE),
            allow_single_file = True,
        ),
        "source_file": attr.output(mandatory = True),
    },
)
