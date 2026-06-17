load(":repository.bzl", "android_test_repository")

def _android_test_impl(_module_ctx):
    android_test_repository(name = "android_test_support")

android_test = module_extension(
    implementation = _android_test_impl,
)
