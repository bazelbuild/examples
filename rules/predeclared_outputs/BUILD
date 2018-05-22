load("//predeclared_outputs:hash.bzl", "word_hashes")

# Run any of the following:
#   bazel build //predeclared_outputs:animal_hashes
#   bazel build //predeclared_outputs:animal_hashes.manifest
#   bazel build //predeclared_outputs:dog.md5
#   bazel build //predeclared_outputs:cat.md5
#   bazel build //predeclared_outputs:elephant.md5
# Run "bazel clean" to remove files from a previous build.

word_hashes(
    name = "animal_hashes",
    dictionary = "animals.dict",
    hashes = [
        ":dog.md5",
        ":cat.md5",
        ":elephant.md5",
    ],
)
