load("@rules_android//android:rules.bzl", "android_binary")
load("@tools_android//tools/googleservices:defs.bzl", "google_services_xml")

GOOGLE_SERVICES_XML = google_services_xml(
    package_name = "com.example.myapplication",
    google_services_json = "google-services.json",
)

android_binary(
    name = "app",
    srcs = glob(["src/main/java/**/*.java"]),
    # this sets the java package for the R class, since this android_binary
    # rule isn't under a java root (i.e., some directory named "java" for the
    # root of the java code for the app).
    custom_package = "com.example.myapplication",
    manifest = "src/main/AndroidManifest.xml",
    manifest_values = {
        "minSdkVersion": "15",
        "applicationId": "com.example.myapplication",
    },
    resource_files = glob(["src/main/res/**"]) + GOOGLE_SERVICES_XML,
    deps = [
        "@maven//:com_google_firebase_firebase_messaging",
        "@maven//:com_google_firebase_firebase_iid",
        "@maven//:com_android_support_appcompat_v7",
        # activity_main layout uses contraints
        "@maven//:com_android_support_constraint_constraint_layout",
    ],
)
