# Count how many cities are listed in the file
genrule(
    name = "city_count",
    srcs = ["@world_cities//file"],
    outs = ["city_number"],
    cmd = "wc -l < $< > $@",
)

# Count how many emojis are listed in the file
genrule(
    name = "emoji_count",
    # To build without --expiremental_enable_bzlmod, this has to be
    # srcs = ["@emojis//file"],
    srcs = ["@com_foo_bar_emojis//file"],
    outs = ["emoji_number"],
    cmd = "wc -l < $< > $@",
)
