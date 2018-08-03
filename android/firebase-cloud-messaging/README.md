
### Bazel Firebase Cloud Messaging (FCM) example.

FCM requires certain information about your app (API key, app ID, project id,
etc) to be present in the `res/values/values.xml` resource file. This example
shows how to use the tools provided in the
[bazelbuild/tools_android](https://github.com/bazelbuild/tools_android) repo to
generate the `values.xml` file from the `google-services.json` file from your
Firebase console.

#### Building the Example

To build the example:

1. Make sure the `ANDROID_HOME` environment variable is set to the absolute path
   of your Android SDK.

2. Go to the Firebase console for your project, and in Settings, download
   `google-service.json`, and place it in the `app` directory.

3. Run `bazel build //app` in the project.

#### Applying the Example to Your Code

To apply this example to your code:

1. Add the following to your `WORKSPACE` file:
```
    TOOLS_ANDROID_COMMIT = "0e864ba5a86958513658250de587416d8e17c481"
    http_archive(
      name = "tools_android",
      strip_prefix = "tools_android-" + TOOLS_ANDROID_COMMIT,
      url = "https://github.com/bazelbuild/tools_android/archive/%s.tar.gz" % TOOLS_ANDROID_COMMIT,
    )
    load("@tools_android//tools/googleservices:defs.bzl", "google_services_workspace_dependencies")
    google_services_workspace_dependencies()
```

2. Add the following to your `BUILD` file:
```
  load("@tools_android//tools/googleservices:defs.bzl", "google_services_xml")

  GOOGLE_SERVICES_XML = google_services_xml(
      package_name = "com.example.myapplication",
      google_services_json = "google-services.json"
  )
```

3. Add `GOOGLE_SERVICES_XML` to the `resource_files` attribute of your
   `android_binary` rule. For example:
```
  android_binary(
      ...
      resource_files = glob(["src/main/res/**"]) + GOOGLE_SERVICES_XML,
      ...
  )
```

#### Manual Integration

It's also possible to run the Google Services values.xml generator manually and
add the results to your project:

1. Go to the Firebase console for your project, and in Settings, download
   `google-service.json`.

2. From the workspace root of the tools_android project, run the Google Services
   XML generator:
```
  bazel run //third_party/googleservices:GenerateGoogleServicesXml -- \
     com.example.myapplication \
     /absolute/path/to/google-services.json \
     /tmp/values.xml
```
   The arguments are the package name for your app, the absolute file path to  
   the `google-services.json` file, and finally the file path for `values.xml`.

3. Merge the resulting `values.xml` file into your `values.xml` file (or put the
   file into your `res/values` directory if you don't already have a
   `values.xml` file). Alternatively, the `values.xml` file can be put into a
   separate `res/values` directory and added to the `resource_files`. For the
   example here, if `values.xml` is in
   `app/src/main/google_services_xml/res/values/values.xml`, the `BUILD` file
   would have
   `resource_files = glob(["src/main/res/**"]) + ["src/main/google_services_xml/res/values/values.xml"],`.
