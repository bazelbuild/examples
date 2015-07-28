Bazel end-to-end example
========================

This is a "hello world" example that uses a variety of Bazel features. There is
a backend server and a couple of front-end applications that use this backend,
including:

* an [Android application](#android-app)
* an [iOS application](#ios-app)
* a [web application](#web-app)

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

<a name="android-app" />
Android application
-------------------

TODO

<a name="ios-app" />
iOS application
---------------

TODO

<a name="web-app" />
Web application
---------------

TODO
