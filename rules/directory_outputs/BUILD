load(":sum.bzl", "sum", "generate_sums")

sum(
    name = "n",
    deps = [
        ":n2",
        ":n5",
        ":n_dir",
    ],
)

sum(
    name = "n2",
    number = "n2.txt",
)

sum(
    name = "n5",
    number = "n5.txt",
)

generate_sums(
    name = "n_dir",
)
