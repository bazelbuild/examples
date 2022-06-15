This is an example on how to specify dev dependencies with Bzlmod. It covers the following topics:

- Specifying a Bazel module as a dev dependency.
- Specifying a module extension usage as a dev dependency.

Dev dependencies won't propagate downstream.

To test it out, `cd` into this directory and run the following:
```
$ export USE_BAZEL_VERSION=last_green
$ export LIBRARIAN_BIN_PATH=$PWD/../utils/librarian/librarian.py
$ bazelisk build --experimental_enable_bzlmod //:check_books
$ cat ./bazel-bin/books
```
