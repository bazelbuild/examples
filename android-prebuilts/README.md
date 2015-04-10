
# Example: configuring Android prebuilt hermetic toolchains on Linux

Bazel ships with an example compiler configuration for /usr/bin/gcc.
While these are universally available, they are not hermetic: versions
of the compiler may vary across installations.

The following example shows how to install a prebuilt host toolchain,
using the pre-packaged compilers from the Android Open Source project.

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
