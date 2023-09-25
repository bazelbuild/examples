load("//generating_code/gen:enum_maker.bzl", "enum_maker")

enum_maker(
    name = "gen_enums",
    out_h = "values.h",
    out_py = "values.py",
    values_file = "values.txt",
)

py_library(
    name = "p_values",
    srcs = [":values.py"],
)

py_binary(
    name = "p_maybe",
    srcs = ["p_maybe.py"],
    deps = [":p_values"],
)

cc_library(
    name = "c_values",
    hdrs = [":values.h"],
)

cc_binary(
    name = "c_maybe",
    srcs = ["c_maybe.cc"],
    deps = [":c_values"],
)
