# Starlark codelab large logo reader components.

genrule(
    name = "logo_large",
    srcs = ["//starlark_codelab/java/exercise1/logocommon:logos.zip"],
    outs = ["logo_large.png"],
    cmd = "unzip -p $< $$(basename $@) > $@",
)

java_binary(
    name = "logo_reader_large",
    srcs = ["LogoReaderLarge.java"],
    main_class = "exercise1.logoreaderlarge.LogoReaderLarge",
    resources = [":logo_large"],
)
