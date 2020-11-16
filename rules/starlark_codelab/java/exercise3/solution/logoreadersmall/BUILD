# Starlark codelab small logo reader components.

load("//starlark_codelab/java/exercise3/solution/logocommon:logocommon.bzl", "get_logo_file_via_rule")

get_logo_file_via_rule(
    name = "logo_small",
    logo_filename = "logo_small.png",
)

java_binary(
    name = "logo_reader_small",
    srcs = ["LogoReaderSmall.java"],
    main_class = "exercise3.solution.logoreadersmall.LogoReaderSmall",
    resources = [":logo_small"],
)
