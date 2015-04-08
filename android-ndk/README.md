# An example of configuring a compiler.

This gives an example of how to setup a cross-compiler for embedded
development.  It is using the cross compiler from the Android NDK, but
other GCC flavors should be similar.

*This is not an example of how build android NDK apps*.

## Instructions

The following instructions assume Linux 64-bit machine.

* Download the Android NDK from
  https://developer.android.com/tools/sdk/ndk/index.html

* Unpack it into $HOME/tmp/android such that
  `$HOME/tmp/android/android-ndk` contains `README.txt`

* Put `BUILD` and `CROSSTOOL` into `$HOME/tmp/android/android-ndk`

* Put `BUILD.sysroot` into
  `$HOME/tmp/android/android-ndk/platforms/android-19/arch-arm/BUILD`
  (don't forget to rename it.)

* Now create a separate workspace with a BUILD for a `cc_binary`

* Run

```
bazel build --dynamic_mode=off \
    --package_path=%workspace%:$HOME/tmp/android:$HOME/bazel-src \
    --crosstool_top=//android-ndk:toolchain \
    --host_crosstool_top=//tools/cpp:toolchain \
    --cpu=armeabi-v7a \
    --custom_malloc=//android-ndk:malloc \
    //my/cc:binary
```

* To deploy and test the resulting to your Android phone, using

```
adb bazel-bin/my/cc/binary data/local/tmp/my-binary
adb shell ./data/local/tmp/my-binary
```

For Darwin, the steps will be similar, but all paths referring to
"linux-x86" should the equivalent darwin-x86 path instead.
