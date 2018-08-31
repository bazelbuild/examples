# Simple Android Example

## Setup Android SDK and NDK

In order to build these examples, add the following two rules to the top-level
`WORKSPACE` file (two directories above this file):

```python
android_sdk_repository(
    name = "androidsdk",
    path = "<full path to your Android SDK>",
    api_level = "<api level>",
)

android_ndk_repository(
    name = "androidndk",
    path = "<path to your Android NDK>",
    api_level = "<api_level>",
)
```

For the `android_sdk_repository` rule, the value of `api_level` corresponds to a
directory in the SDK containing the specific version of `android.jar` to compile
against. For example, if `path = "/Users/<username>/Library/Android/sdk"` and
`api_level = 27`, then the directory
`/Users/<username>/Library/Android/sdk/platforms/android-27` must exist.

Similarly, for the `android_ndk_repository` rule, the value of the `api_level`
attribute corresponds to a directory containing the NDK libraries for that API
level. For example, if `path=/Users/<username>/Library/Android/android-ndk-r16b`
and `api_level=27`, then you your NDK must contain the directory
`/Users/<username>/Library/Android/android-ndk-r16b/platforms/android-27`.

The example `android_binary` depends on the
`com.android.support:appcompat-v7:27.1.0` AAR hosted on [Google Maven
Repository](https://maven.google.com), so you will need to add the
[`gmaven_rules`](https://github.com/bazelbuild/gmaven_rules) dependency to the
WORKSPACE file:

```python
# Google Maven Repository
# Get the tag from https://github.com/bazelbuild/gmaven_rules/releases
GMAVEN_TAG = "YYYYMMDD-<SNAPSHOT>"

http_archive(
    name = "gmaven_rules",
    strip_prefix = "gmaven_rules-%s" % GMAVEN_TAG,
    url = "https://github.com/bazelbuild/gmaven_rules/archive/%s.tar.gz" % GMAVEN_TAG,
)

load("@gmaven_rules//:gmaven.bzl", "gmaven_rules")

gmaven_rules()
```

## Build the app

The following command can be used to build the example app:

```
bazel build //java/bazel:hello_world
```

## Faster iterative development with mobile-install

Bazel has a nice way to speed up the edit-compile-install development cycle for
physical Android devices and emulators: Bazel knows what code changed since the
last build, and can use this knowledge to install only the changed code to the
device. This currently works with L devices and changes to Java code and Android
resources. To try this out, take an `android_binary` rule and:

 * Set the `proguard_specs` attribute to `[]` (the empty list) or just omit it
   altogether
 * Set the `multidex` attribute to `native`
 * Set the `dex_shards` attribute to a number between 2 and 200. This controls
   the size of chunks the code is split into. As this number is increased,
   compilation and installation becomes faster but app startup becomes slower. A
   good initial guess is 10.
 * Connect your device over USB to your workstation and enable USB debugging on
   it
 * Run `bazel mobile-install //java/bazel:hello_world`
 * Edit Java code or Android resources
 * Run `bazel mobile-install --incremental //java/bazel:hello_world`

Note that if you change anything other than Java code or Android resources (C++
code or something on the device), you must omit the `--incremental` command line
option. Yes, we know that this is also clunky and we are working on improving
it.
