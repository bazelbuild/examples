This folder is a Bazel module that hosts a module extension which integrates a pseudo package manager called `librarian`.

The `librarian` package manager can do two things:
- `librarian fetch <book>@<edition>`: Generates a `book` file for a given book name and edition, which contains the name and edition of the book.
- `librarian select <book_1>@<edition_1> <book_2>@<edition_2> ...`: Given a list of required books, selects only the newest required edition for the same book name and generates a `booklist.json` file containing the list of selected books.

The module extension `librarian_extension` does the following:

- Invoking `librarian select` to resolve book editions for all required books in the whole dependency graph.
- Calling the repository rule (`fetch_book`) that invokes `librarian fetch` to define repositories for all selected books.

To use this extension:
```
bazel_dep(name = "librarian", version = "")
local_path_override(module_name = "librarian", path = "<this directory>")

librarian_extension = use_extension("@librarian//:librarian.bzl", "librarian_extension")

librarian_extension.book(name="the_great_gatsby", edition="2020.5")
use_repo(librarian_extension, "the_great_gatsby")
```
