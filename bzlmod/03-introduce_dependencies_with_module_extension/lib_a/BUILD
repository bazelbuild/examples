genrule(
    name = "emoji_count",
    srcs = ["@emojis//file"],
    outs = ["emoji_number"],
    cmd = "wc -l < $< > $@",
)
