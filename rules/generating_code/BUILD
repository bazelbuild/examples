load("//generating_code/gen:enum_maker.bzl", "enum_maker")

enum_maker(
    name = "gen_enums",
    values_file = "values.txt",
    out_py = "values.py",
    out_h = "values.h",
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
