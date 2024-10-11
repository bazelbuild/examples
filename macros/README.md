# Macros

This workspace demonstrates the features of
[Symbolic Macros](https://bazel.build/extending/macros), new to Bazel 8.0.

It's recommended to start by reading the `//main` package's BUILD file, then
`//word_counter`, `//fancy_word_counter`, and `//letter_metrics`. Each builds
on the features of the previous, and the `//main` package ties them all
together.

To avoid additional dependencies, this example uses Python scripts as tools
without encapsulating them in a `py_binary`. A `python3` interpreter must be
available in the genrule environment to build the example.
