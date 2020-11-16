# Starlark codelab small logo reader components.

genrule(
    name = "logo_small",
    srcs = ["//starlark_codelab/java/exercise1/solution/logocommon:logos.zip"],
    outs = ["logo_small.png"],
    cmd = "unzip -p $< $$(basename $@) > $@",
)

java_binary(
    name = "logo_reader_small",
    srcs = ["LogoReaderSmall.java"],
    main_class = "exercise1.solution.logoreadersmall.LogoReaderSmall",
    resources = [":logo_small"],
)
