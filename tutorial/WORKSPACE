new_http_archive(
    name = "appengine-java",
    url = "http://central.maven.org/maven2/com/google/appengine/appengine-java-sdk/1.9.23/appengine-java-sdk-1.9.23.zip",
    sha256 = "05e667036e9ef4f999b829fc08f8e5395b33a5a3c30afa9919213088db2b2e89",
    build_file = "appengine.BUILD",
)

bind(
    name = "appengine/java/sdk",
    actual = "@appengine-java//:sdk",
)

bind(
    name = "appengine/java/api",
    actual = "@appengine-java//:api",
)

bind(
    name = "appengine/java/jars",
    actual = "@appengine-java//:jars",
)

maven_jar(
    name = "commons-lang",
    artifact = "commons-lang:commons-lang:2.6",
)

maven_jar(
    name = "javax-servlet-api",
    artifact = "javax.servlet:servlet-api:2.5",
)

bind(
    name = "javax/servlet/api",
    actual = "//tools/build_rules/appengine:javax.servlet.api",
)

maven_jar(
    name = "json",
    artifact = "org.json:json:20141113",
)

# To build the example Android app, uncomment this rule and set the three
# parameters: path, api_level, build_tools_version.

# android_sdk_repository(
#     name = "androidsdk",
#     # Set the path to the directory the Android SDK was unzipped into.
#     path = "/path/to/android-sdk",
#     # Set the API level of the installed SDK Platform.
#     api_level = 22,
#     # Set the version of the build tools (a directory inside build-tools)
#     build_tools_version="22.0.1"
# )
