#!/usr/bin/env python3
import sys
import json

def eprint(*args, **kwargs):
    print(*args, flush=True, file=sys.stderr, **kwargs)

def fetch(name, edition):
    with open("./book", "w") as book:
        book.write("Book Name: %s\n" % name)
        book.write("Edition: %s\n" % edition)

def calc_edition(e):
    return [int(x) for x in e.split(".")]

def max_edition(e1, e2):
    return e1 if calc_edition(e1) > calc_edition(e2) else e2

def select(books):
    book_list = {}
    for book in books:
        book_name, edition = book.split("@")
        if book_name not in book_list:
            book_list[book_name] = edition
        else:
            book_list[book_name] = max_edition(edition, book_list[book_name])
    with open("./booklist.json", "w") as f:
        json.dump(book_list, f, indent=4, sort_keys=True)
        f.write("\n")

def main(argv=None):
    if argv is None:
        argv = sys.argv[1:]

    if argv[0] == "fetch":
        book_name, edition  = argv[1].split("@")
        fetch(book_name, edition)
    elif argv[0] == "select":
        select(argv[1:])
    else:
        eprint("Unrecognized command:", argv[0])
        return -1


if __name__ == "__main__":
    sys.exit(main())
