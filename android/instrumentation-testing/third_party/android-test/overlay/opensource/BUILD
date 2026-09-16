load("@rules_android//android:rules.bzl", "android_library")
load("@rules_java//java:defs.bzl", "java_import", "java_library", "java_plugin")

# aliases that creates bazel targets that are equivalent to blaze

java_import(
    name = "androidsdk_compiletime",
    jars = [
        "@androidsdk//:platforms/android-36/android.jar",
    ],
    neverlink = 1,
    visibility = [
        "//visibility:public",
    ],
)

java_library(
    name = "auto_value",
    exported_plugins = [":auto_value_plugin"],
    visibility = ["//visibility:public"],
    exports = [
        "@maven//:com_google_auto_value_auto_value_annotations",
    ],
)

java_plugin(
    name = "auto_value_plugin",
    generates_api = True,
    processor_class = "com.google.auto.value.processor.AutoValueProcessor",
    deps = [
        "@maven//:com_google_auto_value_auto_value",
    ],
)
