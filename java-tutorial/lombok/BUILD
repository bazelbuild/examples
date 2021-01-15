load("@rules_java//java:defs.bzl", "java_library", "java_plugin")

java_plugin(
    name = "lombok_plugin",
    generates_api = True,
    processor_class = "lombok.launch.AnnotationProcessorHider$AnnotationProcessor",
    deps = ["@lombok_jar//jar"],
)

java_library(
  name = "lombok",
  exports = ["@lombok_jar//jar"],
  exported_plugins = [":lombok_plugin"],
  visibility = ["//visibility:public"]
)

