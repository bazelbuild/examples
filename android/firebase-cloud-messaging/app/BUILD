load("@tools_android//tools/googleservices:defs.bzl", "google_services_xml")
load("@gmaven_rules//:defs.bzl", "gmaven_artifact")

GOOGLE_SERVICES_XML = google_services_xml(
    package_name = "com.example.myapplication",
    google_services_json = "google-services.json"
)

android_binary(
    name = "app",
    srcs = glob(["src/main/java/**/*.java"]),
    manifest = "src/main/AndroidManifest.xml",
    resource_files = glob(["src/main/res/**"]) + GOOGLE_SERVICES_XML,
    # this sets the java package for the R class, since this android_binary
    # rule isn't under a java root.
    custom_package = "com.example.myapplication",
    deps = [
        gmaven_artifact("com.google.firebase:firebase_messaging:aar:17.0.0"),
        # firebase messaging 17.0.0 depends on firebase iid 16.0.0, so make sure
        # to use the same version when depending on iid directly from the app.
        gmaven_artifact("com.google.firebase:firebase_iid:aar:16.0.0"),
        gmaven_artifact("com.android.support:appcompat_v7:aar:26.1.0"),
        # activity_main layout uses contraints
        gmaven_artifact("com.android.support.constraint:constraint_layout:aar:1.0.2"),
    ],
    manifest_values = {
      "minSdkVersion": "15",
      "applicationId": "com.example.myapplication",
    }
)
