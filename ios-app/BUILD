objc_binary(
    name = "UrlGet",
    srcs = [
        "UrlGet/main.m",
    ],
    deps = [
        ":UrlGetClasses",
    ],
    infoplist = "UrlGet/UrlGet-Info.plist",
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
