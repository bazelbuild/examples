genrule(
    name = "emoji_count",
    cmd = "wc -l < $< > $@",
    outs = ["emoji_number"],
    srcs = ["@emojis//file"],
)
