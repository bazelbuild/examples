def _fetch_book_impl(repository_ctx):
    librarian = repository_ctx.os.environ.get("LIBRARIAN_BIN_PATH")
    if not librarian:
        fail("LIBRARIAN_BIN_PATH is not set!")
    if not repository_ctx.path(librarian).exists:
        fail("Path %s doesn't exist!" % librarian)
    book_name = repository_ctx.attr.name.split(".")[-1]
    edition = repository_ctx.attr.edition
    result = repository_ctx.execute([librarian, "fetch", "%s@%s" % (book_name, edition)])
    if result.return_code != 0:
        fail(result.stderr)
    repository_ctx.file("BUILD", """
filegroup(
    name = "book_file",
    srcs = ["book"],
    visibility = ["//visibility:public"],
)
""")

fetch_book = repository_rule(
    implementation = _fetch_book_impl,
    environ = ["LIBRARIAN_BIN_PATH"],
    attrs = {
        "edition": attr.string(),
    },
)

def _librarian_extension_impl(module_ctx):
    librarian = module_ctx.os.environ.get("LIBRARIAN_BIN_PATH")
    if not librarian:
        fail("LIBRARIAN_BIN_PATH is not set!")
    books = []
    for mod in module_ctx.modules:
        for book in mod.tags.book:
            books.append("%s@%s" % (book.name, book.edition))
    result = module_ctx.execute([librarian, "select"] + books)
    if result.return_code != 0:
        fail(result.stderr)
    resolved_book_list = json.decode(module_ctx.read("./booklist.json"))
    for name in resolved_book_list:
        edition = resolved_book_list[name]
        fetch_book(name=name, edition=edition)


book = tag_class(attrs={
    "name": attr.string(),
    "edition": attr.string(),
})

librarian_extension = module_extension(
    implementation = _librarian_extension_impl,
    # TODO: module_extension should also support the "environ" attribute.
    # environ = ["LIBRARIAN_BIN_PATH"],
    tag_classes = {"book": book},
)
