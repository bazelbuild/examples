ios_application(
    name = "ios-app",
    binary = "ios-app-binary",
    infoplist = "UrlGet/UrlGet-Info.plist",
    visibility = ["//visibility:public"],
)

objc_binary(
    name = "ios-app-binary",
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
