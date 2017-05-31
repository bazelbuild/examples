load("@build_bazel_rules_apple//apple:ios.bzl", "ios_application")

ios_application(
    name = "ios-app",
    deps = [":UrlGetMain"],
    infoplists = ["UrlGet/UrlGet-Info.plist"],
    bundle_id = "examples.UrlGet",
    families = ["iphone", "ipad"],
    visibility = ["//visibility:public"],
)

objc_library(
    name = "UrlGetMain",
    srcs = [
       "UrlGet/main.m",
    ],
    deps = [
       ":UrlGetClasses",
    ],
)

objc_library(
    name = "UrlGetClasses",
    srcs = [
        "UrlGet/AppDelegate.m",
        "UrlGet/UrlGetViewController.m",
    ],
    hdrs = glob(["UrlGet/*.h"]),
    xibs = ["UrlGet/UrlGetViewController.xib"],
)
