load("@librarian//:librarian.bzl", "fetch_book")

def fetch_book_for_lib_a():
    fetch_book(
        name="the_great_gatsby",
        edition="2003.7",
    )
    fetch_book(
        name="hamlet",
        edition="1800.1",
    )
