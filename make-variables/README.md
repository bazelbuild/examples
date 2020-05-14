# Examples demonstrating "Make" variables

For the complete guide, see the page about ["Make"
variables](https://docs.bazel.build/versions/master/be/make-variables.html) in
the BUILD Encyclopedia.

The `//testapp:show_app_output` target demonstrates predefined source/output
path variables:
```
$ bazel build //testapp:show_app_output && cat bazel-bin/testapp/app_output
Target //testapp:show_app_output up-to-date:
  bazel-bin/testapp/app_output
INFO: Build completed successfully, 7 total actions
:app output paths
 execpath: bazel-out/host/bin/testapp/app
 runfiles: testapp/app
 location: bazel-out/host/bin/testapp/app

source file paths
 execpath: testapp/empty.source
 runfiles: testapp/empty.source
 location: testapp/empty.source
```

The `//testapp:show_custom_var` target demonstrates custom variables:
```
$ bazel build //testapp:show_custom_var && cat bazel-bin/testapp/custom_var
Target //testapp:show_custom_var up-to-date:
  bazel-bin/testapp/custom_var
INFO: Build completed successfully, 2 total actions
FOO is equal to bar!
```

