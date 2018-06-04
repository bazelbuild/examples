load("//tools/googleservices:defs.bzl", "google_services_xml")

GOOGLE_SERVICES_XML = google_services_xml(
    packageName = "com.example.myapplication",
    google_services_json = "google-services.json")

android_binary(
    name = "app",
    srcs = glob(["src/main/java/**/*.java"]),
    manifest = "src/main/AndroidManifest.xml",
    resource_files = glob(["src/main/res/**"]) + GOOGLE_SERVICES_XML,
    # this sets the java package for the R class, since this android_binary
    # rule isn't under a java root.
    custom_package = "com.example.myapplication",
    deps = [
        "@com_google_firebase_firebase_messaging_17_0_0//aar",
        # firebase messaging 17.0.0 depends on firebase iid 16.0.0.
        "@com_google_firebase_firebase_iid_16_0_0//aar",
        "@com_android_support_appcompat_v7_26_1_0//aar",
        # activity_main layout uses contraints
        "@com_android_support_constraint_constraint_layout_1_0_2//aar",
    ],
    manifest_values = {
      "minSdkVersion": "15",
      "applicationId": "com.example.myapplication",
    }
)
