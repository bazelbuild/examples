load("@gmaven_rules//:defs.bzl", "gmaven_artifact")

filegroup(
    name = "srcs",
    srcs = glob(["**"]),
    visibility = ["//examples:__pkg__"],
)

android_library(
    name = "lib",
    srcs = ["Lib.java"],
)

android_binary(
    name = "hello_world",
    srcs = glob([
        "MainActivity.java",
        "Jni.java",
    ]),
    manifest = "AndroidManifest.xml",
    resource_files = glob(["res/**"]),
    deps = [
        ":jni",
        ":lib",
        gmaven_artifact("com.android.support:appcompat-v7:aar:27.1.1"),
    ],
)

cc_library(
    name = "jni",
    srcs = ["jni.cc"],
    deps = [":jni_dep"],
)

cc_library(
    name = "jni_dep",
    srcs = ["jni_dep.cc"],
    hdrs = ["jni_dep.h"],
)
