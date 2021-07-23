load(":defs.bzl", "foo_binary", "simple")

# Defines a rule where its `tool` dependency is built
# for multiple cpus
foo_binary(
    name = "foo",
    tool = ":blah",
)

simple(name = "blah")
