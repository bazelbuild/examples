genrule(
    name = "check_books",
    srcs = [
        "@hamlet//:book_file",
        "@the_great_gatsby//:book_file",
    ],
    outs = ["books"],
    cmd = "cat $(SRCS) > $@",
)
