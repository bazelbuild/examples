load("@librarian//:librarian.bzl", "fetch_book")

def fetch_book_for_lib_b():
    fetch_book(
        name="the_great_gatsby",
        edition="2020.5",
    )
    fetch_book(
        name="hamlet",
        edition="1603.1",
    )
