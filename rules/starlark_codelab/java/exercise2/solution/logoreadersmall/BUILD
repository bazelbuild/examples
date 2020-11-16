# Starlark codelab small logo reader components.

load("//starlark_codelab/java/exercise2/solution/logocommon:logocommon.bzl", "get_logo_file_via_macro")

get_logo_file_via_macro(
    name = "logo_small",
    archive_file = "//starlark_codelab/java/exercise2/solution/logocommon:logos.zip",
    logo_filename = "logo_small.png",
)

java_binary(
    name = "logo_reader_small",
    srcs = ["LogoReaderSmall.java"],
    main_class = "exercise2.solution.logoreadersmall.LogoReaderSmall",
    resources = [":logo_small"],
)
