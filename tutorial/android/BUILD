# The final binary rule, which builds the APK and sets the application manifest,
# as well as any other resources needed by the application.
# The package for the R class for resources is normally inferred from the
# directory containing the BUILD file, but this BUILD file is not under a java
# directory, so we specify it manually.
android_binary(
    name = "android",
    custom_package = "com.google.bazel.example.android",
    manifest = "src/main/java/com/google/bazel/example/android/AndroidManifest.xml",
    resource_files = glob(["src/main/java/com/google/bazel/example/android/res/**"]),
    visibility = ["//visibility:public"],
    deps = [":activities"],
)

# A library which compiles some Java sources and associated resources.
# Because it has resources, it requires a manifest.
android_library(
    name = "activities",
    srcs = glob(["src/main/java/com/google/bazel/example/android/activities/*.java"]),
    custom_package = "com.google.bazel.example.android.activities",
    manifest = "src/main/java/com/google/bazel/example/android/activities/AndroidManifest.xml",
    resource_files = glob(["src/main/java/com/google/bazel/example/android/activities/res/**"]),
)
