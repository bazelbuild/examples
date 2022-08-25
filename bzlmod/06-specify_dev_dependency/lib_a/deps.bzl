load("@librarian//:librarian.bzl", "fetch_book")

def deps():
    fetch_book(
        name="the_great_gatsby",
        edition="2003.7",
    )
