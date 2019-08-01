load("@build_bazel_rules_apple//apple:ios.bzl", "ios_application")

objc_library(
    name = "UrlGetClasses",
    srcs = [
        "UrlGet/AppDelegate.m",
        "UrlGet/UrlGetViewController.m",
        "UrlGet/main.m",
    ],
    hdrs = glob(["UrlGet/*.h"]),
)

ios_application(
    name = "ios-app",
    bundle_id = "Google.UrlGet",
    families = [
        "iphone",
        "ipad",
    ],
    infoplists = [":UrlGet/UrlGet-Info.plist"],
    visibility = ["//visibility:public"],
    deps = [":UrlGetClasses"],
    minimum_os_version = "8.0",
    launch_storyboard = "UrlGet/UrlGetViewController.xib",
)
