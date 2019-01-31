Bazel end-to-end example
========================

This is a "hello world" example that uses a variety of Bazel features. There is
a backend server and a couple of front-end applications that use this backend,
including:

* an [Android application](#android-application)
* an [iOS application](#ios-application)

To build these examples, you will need to
[install Bazel](http://bazel.io/docs/install.html).

The backend
-----------

The backend server listens for requests to localhost:8080 and replies with a
JSON string: `{'requested' : '/my-page'}` (or whatever the URI was that you
visited).

You can build and run this server by running:

```
$ bazel build //backend
$ bazel-bin/backend/backend
```

The backend server uses the AppEngine SDK to run a local webserver. The first
time you build this target, it will download the entire SDK (~160MB), which
may take a little while. (After that first build, it should be cached.)

Once the `bazel-bin/backend/backend` prints
`INFO: Dev App Server is now running`, you can visit any URI relative to
localhost:8080 and get a response from the server.

Android application
-------------------

The Android application has one activity, which retrieves the `'requested'` key
of the JSON object retrieved from the URL http://10.0.2.2:8080/boop and displays
it on the screen.

In order to build the Android application, you will need to download the SDK at
https://developer.android.com/sdk/installing/index.html?pkg=tools and install
the Android SDK and build tools as described in 1. Get the latest SDK tools in
https://developer.android.com/sdk/installing/adding-packages.html.
Once these are done, set up the android_sdk_repository rule in the WORKSPACE file,
following the instructions in the comments there.

You can build and install this application by connecting an Android emulator or
device via adb and running:

```
$ bazel mobile-install //android
```

iOS application
---------------

The iOS application is a single view which retrives the contents of the HTTP
response received for a given URL (such as localhost:8080). It displays the
response in a text view.

You can build the application by running:

```
$ bazel build //ios-app
```

Bazel will generate some output files, most notably `bazel-bin/ios-app/ios-app.xcodeproj`

Open this file in xcode and run the application on your target device
(or device simulator).

Continuous integration
----------------------
The script in ci/build.sh is used by [http://ci.bazel.io] to test that this workspace
still build against Bazel at head.
