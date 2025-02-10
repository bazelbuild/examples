# OCI Container

This example expands the previous [direct dependency example](../06-deps-direct) code 
and adds rules_oci to build an OCI container image. 

To build the example:

`bazel build //...`

To build the OCI container:

`bazel build //tokio_oci:image`

To publish the container, you would have to run:

`bazel run //tokio_oci:push`

However, this results in an error because no registry is defined in the example.
You have to define your target registry in the [binary BUILD](tokio_oci/BUILD.bazel) file
before you can publish your image. For details of how to configure a container registry,
please [consult the official documentation.](https://github.com/bazel-contrib/rules_oci/blob/main/docs/push.md)

## Setup

The new rules_oci build container images in Bazel without Docker. The setup is a straightforward three step process:

1) Load rules 
2) Declare base image
3) Define custom tag rule

The first two steps are self explanatory, but the third step needs a bit of background.

Conventionally, Docker images are tagged with either a version number or something generic like "latest".
However, docker image tags are mutable and therefore it is not guaranteed that the tag "latest" refers today to the 
same container image as it did last week, which violates the core principle of hermetic builds. In response, 
image registries such as Google Artifact Registry generally recommend immutable tags that ensure that, 
at any point in time, one immutable image tag refers to the same image as it did before. 
Once a tag has been assigned to an image, it cannot be changed anymore. You can add more tags as you wish,
but any of these tags cannot be changed anymore because they are immutable. 

Because there is no fixed rule how to generate container tags in Bazel, it is therefore necessary to define
a custom rule to tag a container image. 


### 1) Load rules

First, we have to load the OCI rules and the rules pkg. We
also use the LLVM toolchain to keep the build hermetic.

```Starlark
# https://github.com/bazel-contrib/toolchains_llvm
bazel_dep(name = "toolchains_llvm", version = "1.0.0")

# OCI Container rules
# https://github.com/bazel-contrib/rules_oci/releases
bazel_dep(name = "rules_oci", version = "1.7.6")
# https://github.com/bazelbuild/rules_pkg/releases
bazel_dep(name = "rules_pkg", version = "0.10.1")
```

The LLVM toolchain is configured just as in the previous example for
 the host platform. If you need to cross-compile, refer to the [cross compile
example](../02-hello-cross).

```Starlark
llvm = use_extension("@toolchains_llvm//toolchain/extensions:llvm.bzl", "llvm")
llvm.toolchain(
    llvm_version = "18.1.8",
    sha256 = {
        # Generate checksums with shasum -a 256 filename.tar.zst
        "darwin-aarch64": "41d8dea52d18c4e8b90c4fcd31965f9f297df9f40a38a33d60748dbe7f8330b8",
        "darwin-x86_64": "",
        "linux-aarch64": "",
        "linux-x86_64": "",
    },
    stdlib = {
        "linux-x86_64": "stdc++",
        "linux-aarch64": "stdc++",
    },
    urls = {
        "darwin-aarch64": ["https://github.com/MaterializeInc/toolchains/releases/download/clang-18.1.8-4/darwin_aarch64.tar.zst"],
        "darwin-x86_64": ["https://github.com/MaterializeInc/toolchains/releases/download/clang-18.1.8-4/darwin_x86_64.tar.zst"],
        "linux-aarch64": ["https://github.com/MaterializeInc/toolchains/releases/download/clang-18.1.8-4/linux_aarch64.tar.zst"],
        "linux-x86_64": ["https://github.com/MaterializeInc/toolchains/releases/download/clang-18.1.8-4/linux_x86_64.tar.zst"],
    },
)
```

### 2) Declare base image

Before you build a container, you have to declare base image. 
In this example, we use the lightweight Distroless base image.

To declare a Distroless base image, add the following to your MODULE.bazel file:

```Starlark
###############################################################################
#  O C I  B A S E  I M A G E
###############################################################################
oci = use_extension("@rules_oci//oci:extensions.bzl", "oci")
#
# https://github.com/GoogleContainerTools/distroless
oci.pull(
    name = "distroless",
    digest = "sha256:e1065a1d58800a7294f74e67c32ec4146d09d6cbe471c1fa7ed456b2d2bf06e0",
    image = "gcr.io/distroless/cc-debian12",
    platforms = ["linux/amd64", "linux/arm64/v8"],
)
use_repo(oci, "distroless")
```

### 3) Define custom tag rule

Next, you need a custom rule to tag your container. In a hermetic build, you can't rely on timestamps because these
changes regardless of whether the build has changed. Strictly speaking, timestamps as tags could be made possible in
Bazel, but it is commonly discouraged. Also, immutable container tags are increasingly encouraged to prevent accidental
pulling of a different image that has the same tag as the previous one but contains breaking changes relative to the
previous image. Instead, you want unique tags that only change when the underlying artifact has changed. Turned out,
rules_oci already generates a sha256 for each OCI image so a simple tag rule would be to extract this has and trim to,
say 7 characters and use this short hash as unique and immutable tag.

To crate this rule, crate new file, say,

`build/container.bzl`

Then add the following rule:

```Starlark
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
```

## Usage 

To build and publish a container image, several steps are required:

1) Compress the Rust binary as tar
2) Build container image
3) Build an unique and immutable image tag
4) Define a registry to publish the image

Assuming your binary target is called "bin", 
add the following to define your OCI image:

```Starlark
# Import all applicable rules. 
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_doc", "rust_doc_test")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")
load("@rules_oci//oci:defs.bzl", "oci_image", "oci_push",  "oci_image_index")
# Import the custom image tag macro
load("//:build/container.bzl", "build_sha265_tag")

# 1) Compress the Rust binary to tar
pkg_tar(
    name = "tar",
    srcs = [":bin"],
)

# 2) Build container image
# https://github.com/bazel-contrib/rules_oci/blob/main/docs/image.md
oci_image(
    name = "image",
    base = "@distroless",
    tars = [":tar"],
    entrypoint = ["/bin"],
    exposed_ports = ["4242"],
    visibility = ["//visibility:public"],
)

# 3) Build an unique and immutable image tag
build_sha265_tag(
    name = "remote_tag",
    image = ":image",
    input = "image.json.sha256",
    output = "_tag.txt",
)

# 4) Define a registry to publish the image
# https://github.com/bazel-contrib/rules_oci/blob/main/docs/push.md)
oci_push(
    name = "push",
    image = ":image",
    repository = "my.registry.com/rest-tokio",
    remote_tags = ":remote_tag",
    visibility = ["//visibility:public"],
)
```

For details of how to configure a container registry,
please [consult the official documentation.](https://github.com/bazel-contrib/rules_oci/blob/main/docs/push.md)
