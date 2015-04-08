#
# Copyright 2015 Google Inc. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
package(default_visibility = ["//visibility:public"])

filegroup(
    name = "toolchain",
    srcs = [
        ":cc-compiler-armeabi-v7a",
        ":empty",
        "//android-ndk/platforms/android-19/arch-arm:everything",
    ],
)

cc_library(
    name = "malloc",
    srcs = [],
)

filegroup(
    name = "gcc-arm-android-4.8-toolchain",
    srcs = glob([
        "toolchains/arm-linux-androideabi-4.8/**",
    ]),
    output_licenses = ["unencumbered"],
)

filegroup(
    name = "android-armeabi-v7a-files",
    srcs = [
        ":gcc-arm-android-4.8-toolchain",
        "//android-ndk/platforms/android-19/arch-arm:everything",
    ],
)

cc_toolchain(
    name = "cc-compiler-armeabi-v7a",
    all_files = ":android-armeabi-v7a-files",
    compiler_files = ":gcc-arm-android-4.8-toolchain",
    cpu = "armeabi-v7a",
    dwp_files = ":gcc-arm-android-4.8-toolchain",
    dynamic_runtime_libs = [":gcc-arm-android-4.8-toolchain"],
    linker_files = ":gcc-arm-android-4.8-toolchain",
    objcopy_files = ":gcc-arm-android-4.8-toolchain",
    static_runtime_libs = [":gcc-arm-android-4.8-toolchain"],
    strip_files = ":gcc-arm-android-4.8-toolchain",
    supports_param_files = 0,
)
