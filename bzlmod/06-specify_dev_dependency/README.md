This is an example on how to specify dev dependencies with Bzlmod. It covers the following topics:

- Specifying a Bazel module as a dev dependency.
- Specifying a module extension usage as a dev dependency.

Dev dependencies only take effect when the current module is the root module, and are ignored if the current module is used as a dependency.

To test it out, `cd` into this directory and run the following:

```bash
export USE_BAZEL_VERSION=last_green
bazelisk build --enable_bzlmod //:check_books
cat ./bazel-bin/books
```

Bazel skylib version 1.1.1 should be used and the newest editions of all required books that're not dev dependencies should be fetched, expected output:

```bash
$ bazelisk build --enable_bzlmod //:check_books
...
DEBUG: <path to examples dir>/examples/bzlmod/06-specify_dev_dependency/BUILD:3:6: Bazel Skylib version: 1.1.1
...

$ cat bazel-bin/books
Book Name: hamlet
Edition: 1800.1
Book Name: the_great_gatsby
Edition: 2003.7
```
