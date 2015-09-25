# The whole of crosstool for all configurations.

#
# Copyright 2014 The Bazel Authors. All Rights Reserved.
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

cpus = {
    "x86_64": "linux-x86/host/x86_64-linux-glibc2.7-4.6",
    "i686": "linux-x86/host/i686-linux-glibc2.7-4.6",
    "mingw-x86_64": "linux-x86/host/x86_64-w64-mingw32-4.8",
}

[filegroup(
    name = "gcc-toolchain-%s" % cpu,
    srcs = glob([
        "gcc/%s/**" % p,
    ]),
    output_licenses = ["unencumbered"],
) for cpu, p in cpus.items()]

filegroup(
    name = "empty",
    srcs = [],
)

filegroup(
    name = "toolchain",
    srcs = [
        ":empty",
    ] + [":cc-compiler-%s" % cpu for cpu in cpus],
)

[cc_toolchain(
    name = "cc-compiler-%s" % cpu,
    all_files = ":gcc-toolchain-%s" % cpu,
    compiler_files = ":gcc-toolchain-%s" % cpu,
    cpu = cpu,
    dwp_files = ":gcc-toolchain-%s" % cpu,
    dynamic_runtime_libs = [":gcc-toolchain-%s" % cpu],
    linker_files = ":gcc-toolchain-%s" % cpu,
    objcopy_files = ":gcc-toolchain-%s" % cpu,
    static_runtime_libs = [":gcc-toolchain-%s" % cpu],
    strip_files = ":gcc-toolchain-%s" % cpu,
    supports_param_files = 0,
) for cpu in cpus.keys()]
