# Starlark codelab small logo reader components.

load("//starlark_codelab/java/exercise3/logocommon:logocommon.bzl", "get_logo_file_via_rule")

get_logo_file_via_rule(
    name = "logo_small",
    # TODO: Fill in any necessary parameters.
)

java_binary(
    name = "logo_reader_small",
    srcs = ["LogoReaderSmall.java"],
    main_class = "exercise3.logoreadersmall.LogoReaderSmall",
    resources = [":logo_small"],
)
