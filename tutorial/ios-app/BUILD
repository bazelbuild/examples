load("@rules_cc//cc:defs.bzl", "objc_library")
load("@build_bazel_rules_apple//apple:ios.bzl", "ios_application")
load(
    "@rules_xcodeproj//xcodeproj:defs.bzl",
    "top_level_target",
    "xcodeproj",
)

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
    launch_storyboard = "UrlGet/UrlGetViewController.xib",
    minimum_os_version = "15.0",
    # provisioning_profile = "<your_profile_name>.mobileprovision", # Uncomment and set your own profile.
    visibility = ["//visibility:public"],
    deps = [":UrlGetClasses"],
)

xcodeproj(
    name = "xcodeproj",
    build_mode = "bazel",
    project_name = "ios-app",
    tags = ["manual"],
    top_level_targets = [
        top_level_target(
            ":ios-app",
            target_environments = ["device", "simulator"],
        ),
    ],
)
