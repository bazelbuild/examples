load("@rules_android//android:rules.bzl", "android_library")

package(
    default_visibility = ["//visibility:public"],
)

android_library(
    name = "greeter_activity",
    srcs = [
        "Greeter.java",
        "MainActivity.java",
    ],
    manifest = "AndroidManifest.xml",
    resource_files = glob(["res/**"]),
)