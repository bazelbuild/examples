load("@bazel_tools//tools/build_rules/appengine:appengine.bzl", "appengine_war")

appengine_war(
    name = "backend",
    data = [":webapp"],
    data_path = "/backend/webapp",
    jars = [":app_deploy.jar"],
)

filegroup(
    name = "webapp",
    srcs = glob(["webapp/**/*"]),
)

# We only need a java_library, but create a java_binary because we need a bundle
# of all of the dependencies (a deploy jar) to pass to the appengine_war. So we
# specify a non-existant class as the main_class (as the "binary" will never be
# run directly, only used as a library).
java_binary(
    name = "app",
    srcs = glob(["src/main/java/**/*.java"]),
    main_class = "does.not.exist",
    deps = [
        "//external:javax/servlet/api",
    ],
)
