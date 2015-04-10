
# Example: configuring a hermetic toolchain on Linux

Bazel ships with an example compiler configuration for /usr/bin/gcc.
While these are universally available, they are not hermetic: versions
of the compiler may vary across installations.

The following example shows how to install a prebuilt toolchain, using
the pre-packaged compilers from the Android Open Source project. The
example includes configurations for

  * Compiling on linux-x86 for linux-x86 (64-bit)
  * Compiling on linux-x86 for linux-x86 (32-bit)
  * Compiling on linux-x86 for Window 64-bit w32 (untested)

1. Create directory ~/toolchains/android-prebuilts/

2. Download the binaries:

````
get-toolchains.sh
````

3. Create the CROSSTOOL configuration

```
gen-crosstool.py
```

4. Run Bazel, eg.

````
bazel build --package_path=%workspace%:$HOME/toolchains:$HOME/bazel \
  --crosstool_top=//android-prebuilts:toolchain \
  --cpu=i686 \
  //src/google/protobuf:protoc
````
