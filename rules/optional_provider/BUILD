load(":sum.bzl", "sum")

sum(
    name = "n",
    deps = [
        ":n2",
        ":n5",
    ],
)

sum(
    name = "n2",
    number = 2,
)

sum(
    name = "n5",
    number = 5,
)
