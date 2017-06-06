load("@build_bazel_rules_apple//apple:ios.bzl", "ios_application")

objc_library(
    name = "UrlGetClasses",
    srcs = [
	"UrlGet/main.m",
        "UrlGet/AppDelegate.m",
        "UrlGet/UrlGetViewController.m",
    ],
    hdrs = glob(["UrlGet/*.h"]),
    xibs = ["UrlGet/UrlGetViewController.xib"],
)

ios_application(
    name = "ios-app",
    deps = [":UrlGetClasses"],
    bundle_id = "Google.UrlGet",
    families = ["iphone", "ipad"],
    infoplists = [":UrlGet/UrlGet-Info.plist"],
)
