# Copyright 2016 The Bazel Authors. All rights reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
"""Rules for configuring the C++ toolchain (experimental)."""


load("//:windows_cc_configure.bzl", "configure_windows_toolchain")

def _impl(repository_ctx):
  configure_windows_toolchain(repository_ctx)


cc_autoconf = repository_rule(
    implementation=_impl,
    environ = [
        "BAZEL_PYTHON",
        "BAZEL_SH",
        "BAZEL_VC",
        "BAZEL_VS",
        "CC_CONFIGURE_DEBUG",
        "CUDA_COMPUTE_CAPABILITIES",
        "CUDA_PATH",
        "NO_WHOLE_ARCHIVE_OPTION",
        "USE_DYNAMIC_CRT",
        "USE_MSVC_WRAPPER",
        "SYSTEMROOT",
        "VS90COMNTOOLS",
        "VS100COMNTOOLS",
        "VS110COMNTOOLS",
        "VS120COMNTOOLS",
        "VS140COMNTOOLS"])


def cc_configure():
  """A C++ configuration rules that generate the crosstool file."""
  cc_autoconf(name="standalone_local_config_cc")
