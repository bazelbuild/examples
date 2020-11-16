# Starlark codelab large logo reader components.

load("//starlark_codelab/java/exercise3/solution/logocommon:logocommon.bzl", "get_logo_file_via_rule")

get_logo_file_via_rule(
    name = "logo_large",
    logo_filename = "logo_large.png",
)

java_binary(
    name = "logo_reader_large",
    srcs = ["LogoReaderLarge.java"],
    main_class = "exercise3.solution.logoreaderlarge.LogoReaderLarge",
    resources = [":logo_large"],
)
