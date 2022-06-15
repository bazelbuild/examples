This is an example of how to integrate a third party package manager with module extension. The WORKSPACE file contains the equivalent definitons in the old system.

In this example, we try to integrate a pseudo package manager called "librarian", which can do two things:
- Genreates a `book` file for a given book name and edition, which contains the name and edition of the book.
- Given a list of required books, selects the newest required edition for the same book name and generates a `booklist.json` file which contains the list of selected books.

Our goal is to integrate it with module extension to select and fetch books.

It covers the following topics:

 - Defining module extension tags.
 - Using module extension tags in MODULE.bazel files.
 - Collecting transitive dependency info from module extension tags.
 - Invoking the package manager to resolve dependencies.
 - Generating repositories for resolved dependencies.

To test it out, `cd` into this directory and run the following:
```
$ export USE_BAZEL_VERSION=last_green
$ export LIBRARIAN_BIN_PATH=$PWD/../utils/librarian/librarian.py
$ bazelisk build --experimental_enable_bzlmod //:check_books
$ cat ./bazel-bin/books
```
