# Starlark codelab

In this codelab, we will use Blaze and Starlark to define and build a function, a macro, and a rule that extracts a PNG image from a .zip archive file. There are two sample programs: `logo_reader_large` and `logo_reader_small`. Each reads its PNG file and prints the image width in order to verify the correct resource was extracted. The widths should be 2000 pixels and 100 pixels respectively. The sample programs are intentionally simplified in order to keep the focus of this codelab on the build system.

See [Starlark codelab](https://docs.bazel.build/skylark/starlark_codelab.html) for a
more detailed walkthrough.
