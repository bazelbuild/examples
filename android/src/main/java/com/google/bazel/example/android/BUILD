# The final binary rule, which builds the APK and sets the application manifest,
# as well as any other resources needed by the application.
android_binary(
    name = "android",
    manifest = "AndroidManifest.xml",
    resource_files = glob(["res/**"]),
    deps = ["//android/src/main/java/com/google/bazel/example/android/activities"],
)
