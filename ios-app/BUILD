ios_application(
    name = "UrlGet",
    binary = ":UrlGetBinary",
    infoplist = "UrlGet/UrlGet-Info.plist",
)

objc_binary(
    name = "UrlGetBinary",
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
