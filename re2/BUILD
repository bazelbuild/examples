#
# Copyright 2015 The Bazel Authors Inc. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
package(default_visibility = ["//visibility:public"])

cc_library(
    name = "util",
    srcs = [
        "util/arena.cc",
        "util/hash.cc",
        "util/rune.cc",
        "util/stringpiece.cc",
        "util/stringprintf.cc",
        "util/strutil.cc",
        "util/valgrind.cc",
    ],
    hdrs = glob(["util/*.h"]),
    includes = ["."],
)

cc_library(
    name = "re2",
    srcs = [
        "re2/bitstate.cc",
        "re2/compile.cc",
        "re2/dfa.cc",
        "re2/filtered_re2.cc",
        "re2/mimics_pcre.cc",
        "re2/nfa.cc",
        "re2/onepass.cc",
        "re2/parse.cc",
        "re2/perl_groups.cc",
        "re2/prefilter.cc",
        "re2/prefilter_tree.cc",
        "re2/prog.cc",
        "re2/re2.cc",
        "re2/regexp.cc",
        "re2/set.cc",
        "re2/simplify.cc",
        "re2/tostring.cc",
        "re2/unicode_casefold.cc",
        "re2/unicode_groups.cc",
    ],
    hdrs = [
        "re2/prog.h",
        "re2/re2.h",
        "re2/regexp.h",
        "re2/unicode_casefold.h",
        "re2/unicode_groups.h",
        "re2/walker-inl.h",
    ],
    linkopts = ["-lm"],  # C++ random pulls in math.h
    deps = [":util"],
)

cc_library(
    name = "testutil",
    srcs = glob(
        [
            "re2/testing/*.cc",
            "util/pcre.cc",
            "util/random.cc",
            "util/thread.cc",
        ],
        exclude = [
            "re2/testing/*_test.cc",
            "re2/testing/*_benchmark.cc",
        ],
    ),
    deps = [
        ":re2",
        ":util",
    ],
)

cc_library(
    name = "bench-main",
    srcs = [
        "util/benchmark.cc",
    ],
    deps = [
        ":testutil",
    ],
)

cc_library(
    name = "test-main",
    srcs = [
        "util/test.cc",
    ],
    deps = [
        ":testutil",
    ],
)

[cc_test(
    name = f[:-3],
    srcs = [
        f,
    ],
    linkopts = [
        "-lstdc++",
        "-lpthread",
    ],
    deps = [
        ":re2",
        ":test-main",
    ],
) for f in glob(["re2/testing/*_test.cc"])]

[cc_test(
    name = f[:-3],
    srcs = [
        f,
    ],
    linkopts = [
        "-lstdc++",
        "-lpthread",
    ],
    deps = [
        ":bench-main",
        ":re2",
    ],
) for f in glob(["re2/testing/*_benchmark.cc"])]
