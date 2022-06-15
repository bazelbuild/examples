# Count how many cities are listed in the file
genrule(
    name = "city_count",
    cmd = "wc -l < $< > $@",
    outs = ["city_number"],
    srcs = ["@world_cities//file"],
)

# Count how many emojis are listed in the file
genrule(
    name = "emoji_count",
    cmd = "wc -l < $< > $@",
    outs = ["emoji_number"],
    # To build without --expiremental_enable_bzlmod, this has to be
    # srcs = ["@emojis//file"],
    srcs = ["@com_foo_bar_emojis//file"],
)
