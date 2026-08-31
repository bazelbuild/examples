# Android Instrumentation Testing with Bazel

This sample builds a small Java Android app and runs an AndroidX/JUnit4
instrumentation test on a connected local device.

## Run

Point Bazel at an Android SDK:

```shell
export ANDROID_HOME=/path/to/android/sdk
```

Alternatively, keep the SDK path in `.bazelrc.user`:

```text
common --repo_env=ANDROID_HOME=/path/to/android/sdk
```

Build the app and instrumentation APK:

```shell
bazel build //app/src/main:greeter_test_app
```

Run the device test:

```shell
bazel test //app/src/main:greeter_instrumentation_test \
  --config=local_device \
  --nocache_test_results
```

`--config=local_device` uses the local adb server and runs the test
exclusively. `adb devices` should show one usable device; for multiple devices,
pass an explicit serial with `--test_arg=--device_serial_number=<serial>`.
Use either an emulator or a real device that is already running; this sample
does not create or manage an emulator.

## Approach

The sample uses a small Starlark rule in `bazel/android_instrumentation_test.bzl`
to connect `rules_android` APK providers to AndroidX Test's host-side runner.
The rule adds the target APK, instrumentation APK, and runner to runfiles, and
leaves device selection to the `local_device` Bazel config.

AndroidX Test support is vendored as a separate module in
`third_party/android-test`. The root module reaches it through
`local_path_override` and a module extension that creates
`@android_test_support`. Keeping this as a separate module avoids putting the
AndroidX Test dependency graph and overrides directly in the sample module.

The AndroidX Test repository is generated from an upstream source archive plus
the local `third_party/android-test/overlay` tree. The overlay replaces selected
BUILD files with Bzlmod-compatible labels and carries the small local-device
patches needed for retail devices. It also patches AndroidX Test's host-side
test discovery to use the Android SDK `dexdump` from `@androidsdk//:dexdump`
with modern SDK arguments, instead of falling back to AndroidX Test's embedded
Linux `dexdump_annotations` binary.

`.bazelignore` excludes the overlay from normal package discovery because those
BUILD files are intended for the generated `@android_test_support` repository,
not the root sample workspace.
