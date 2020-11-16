"""Common build definitions for starlark codelab."""

def get_logo_file_via_macro(name, archive_file, logo_filename, **kwargs):
    """Extract the file named logo_filename from archive_file.
    """
    native.genrule(
        name = name,
        srcs = [archive_file],
        outs = [logo_filename],
        cmd = "unzip -p $< $$(basename $@) > $@",
        **kwargs
    )
