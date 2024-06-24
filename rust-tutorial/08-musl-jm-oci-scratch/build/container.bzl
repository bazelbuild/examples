load("@rules_pkg//pkg:tar.bzl", "pkg_tar")
load("@rules_oci//oci:defs.bzl", "oci_image", "oci_image_index")
load("//:build/transition.bzl", "multi_arch")

# Build a Bazel Macro
# https://belov.nz/posts/bazel-rules-macros/

def build_multi_arch_image(
    name,
    base,
    srcs,
    exposed_ports = [],
    platforms = [],
    visibility=None
    ):

    # https://codilime.com/blog/bazel-build-system-build-containerized-applications/
    entry_point = "bin"
    layer_name = "tar_layer"

    # Compress binary to a layer using pkg_tar
    pkg_tar(
        name = layer_name,
        srcs = srcs,
    )

    # Build container image
    oci_image(
        name = "image",
        base = base,
        tars = [layer_name],
        entrypoint = ["/{}".format(entry_point)],
        exposed_ports = exposed_ports,
    )

    multi_arch(
        name = "multi_arch_images",
        image = ":image",
        platforms = platforms,
    )

    oci_image_index(
        name = name,
        images = [
            ":multi_arch_images",
        ],
        visibility = visibility,
    )

def build_image(name, base, srcs, exposed_ports = [], visibility=None):

    # Build a Bazel Macro
    # https://belov.nz/posts/bazel-rules-macros/
    # https://codilime.com/blog/bazel-build-system-build-containerized-applications/
    entry_point = "bin"
    layer_name = "tar_layer"

    # Compress binary to a layer using pkg_tar
    pkg_tar(
        name = layer_name,
        srcs = srcs,
    )

    # Build container image
    # https://github.com/bazel-contrib/rules_oci/blob/main/docs/image.md
    oci_image(
        name = name,
        base = base,
        tars = [layer_name],
        entrypoint = ["/{}".format(entry_point)],
        exposed_ports = exposed_ports,
        visibility = visibility,
    )


def _build_sha265_tag_impl(ctx):

    # Both the input and output files are specified by the BUILD file.
    in_file = ctx.file.input
    out_file = ctx.outputs.output

    # No need to return anything telling Bazel to build `out_file` when
    # building this target -- It's implied because the output is declared
    # as an attribute rather than with `declare_file()`.
    ctx.actions.run_shell(
        inputs = [in_file],
        outputs = [out_file],
        arguments = [in_file.path, out_file.path],
        command = "sed -n 's/.*sha256:\\([[:alnum:]]\\{7\\}\\).*/\\1/p' < \"$1\" > \"$2\"",
    )


build_sha265_tag = rule(
    doc = "Extracts a 7 characters long short hash from the image digest.",
    implementation = _build_sha265_tag_impl,
    attrs = {
        "image": attr.label(
            allow_single_file = True,
            mandatory = True,
        ),
        "input": attr.label(
            allow_single_file = True,
            mandatory = True,
            doc = "The image digest file. Usually called image.json.sha256",
        ),
        "output": attr.output(
            doc = "The generated tag file. Usually named _tag.txt"
        ),
    },
)
