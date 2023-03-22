This is an example of how to integrate a third party package manager with module extension. The WORKSPACE file contains the equivalent definitions in the old system.

In this example, we try to integrate a pseudo package manager called ["librarian"](../utils/librarian/README.md).

Our goal is to integrate it with module extension to select and fetch books, the actual implementation is in the [librarian directory](../utils/librarian).

It covers the following topics:

- Defining module extension tags.
- Using module extension tags in MODULE.bazel files.
- Collecting transitive dependency info from module extension tags.
- Invoking the package manager to resolve dependencies.
- Generating repositories for resolved dependencies.

To test it out, `cd` into this directory and run the following:

```bash
export USE_BAZEL_VERSION=last_green
bazelisk build --enable_bzlmod //:check_books
cat ./bazel-bin/books
```

The newest editions of all required books should be fetched, expected output:

```bash
$ cat bazel-bin/books
Book Name: hamlet
Edition: 2005.1
Book Name: the_great_gatsby
Edition: 2020.5
```
