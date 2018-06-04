
### Bazel Firebase Cloud Messaging (FCM) example.

FCM requires certain information about your app (API key, app ID, project id,
etc) to be present in the `res/values/values.xml` resource file. This example
project provides a tool and a build macro that automatically generates this
`values.xml` file from the `google-services.json` file that's available from
your Firebase console.


To build the example:

1. Make sure the `ANDROID_HOME` environment variable is set to the absolute path
   of your Android SDK.

2. Go to the Firebase console for your project, and in Settings, download
   `google-service.json`, and place it in the `app` directory.

3. Run `bazel build //app` in the project.


To apply this example to your code:

1. Copy the `tools` directory to the workspace root of your project.

2. Add the following to your `BUILD` file:
```
  load("//tools/googleservices:defs.bzl", "google_services_xml")

  GOOGLE_SERVICES_XML = google_services_xml(
      packageName = "com.example.myapplication",
      google_services_json = "google-services.json")
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

It's also possible to run the Firebase values.xml generator manually and add
the results to your project:

1. Go to the Firebase console for your project, and in Settings, download
   `google-service.json`.

2. From the workspace root of the example project, run the Firebase XML
   generator:
```
  bazel run //tools/googleservices:GenerateFirebaseXml -- \
     com.example.myapplication \
     /absolute/path/to/google-services.json \
     /tmp/values.xml
```
   The arguments are the package name for your app, the absolute file path to  
   the `google-services.json` file, and finally the file path for `values.xml`.

3. Merge the resulting `values.xml` file into your `values.xml` file (or put the
   file into your `res/values` directory if you don't already have a
   `values.xml` file).
