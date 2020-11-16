# Starlark codelab large logo reader components.

load("//starlark_codelab/java/exercise3/logocommon:logocommon.bzl", "get_logo_file_via_rule")

get_logo_file_via_rule(
    name = "logo_large",
    # TODO: Fill in any necessary parameters.
)

java_binary(
    name = "logo_reader_large",
    srcs = ["LogoReaderLarge.java"],
    main_class = "exercise3.logoreaderlarge.LogoReaderLarge",
    resources = [":logo_large"],
)
