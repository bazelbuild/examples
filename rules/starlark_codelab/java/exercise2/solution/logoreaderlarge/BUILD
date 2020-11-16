# Starlark codelab large logo reader components.

load("//starlark_codelab/java/exercise2/solution/logocommon:logocommon.bzl", "get_logo_file_via_macro")

get_logo_file_via_macro(
    name = "logo_large",
    archive_file = "//starlark_codelab/java/exercise2/solution/logocommon:logos.zip",
    logo_filename = "logo_large.png",
)

java_binary(
    name = "logo_reader_large",
    srcs = ["LogoReaderLarge.java"],
    main_class = "exercise2.solution.logoreaderlarge.LogoReaderLarge",
    resources = [":logo_large"],
)
