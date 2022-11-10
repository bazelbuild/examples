def _get_librarian_path(ctx):
    if ctx.os.name.find("windows") != -1:
        return ctx.path(Label("//:librarian.cmd"))
    else:
        return ctx.path(Label("//:librarian.py"))

def _fetch_book_impl(repository_ctx):
    librarian = _get_librarian_path(repository_ctx)
    book_name = repository_ctx.attr.name.split("~")[-1]
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
    attrs = {
        "edition": attr.string(),
    },
)

def _librarian_extension_impl(module_ctx):
    librarian = _get_librarian_path(module_ctx)
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
    tag_classes = {"book": book},
)
