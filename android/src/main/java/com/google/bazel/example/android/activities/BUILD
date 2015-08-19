package(default_visibility=["//android/src/main/java/com/google/bazel/example/android:__pkg__"])

# A library which compiles some Java sources and associated resources.
# Because it has resources, it requires a manifest.
android_library(
    name = "activities",
    srcs = glob(["*.java"]),
    manifest = "AndroidManifest.xml",
    resource_files = glob(["res/**"]),
)
