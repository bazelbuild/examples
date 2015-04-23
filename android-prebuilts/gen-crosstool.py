#!/usr/bin/python
#
# Copyright 2014 Google Inc. All Rights Reserved.
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

preamble = """major_version: "local"
minor_version: ""
default_target_cpu: "same_as_host"
default_toolchain {
  cpu: "k8"
  toolchain_identifier: "x86_64-linux-glibc2.7-4.6"
}
"""

template = """

default_toolchain {
  cpu: "%(cpu)s"
  toolchain_identifier: "%(toolchain_id)s"
}
# Toolchain taken from android AOSP prebuilts, see
toolchain {
  # this should be more specific
  abi_version: "local"
  abi_libc_version: "local"

  builtin_sysroot: "android-prebuilts/gcc/%(host)s/%(toolchain_id)s/sysroot"
  compiler: "gcc"
  host_system_name: "local"
  needsPic: true
  supports_gold_linker: false
  supports_incremental_linker: false
  supports_fission: false
  supports_interface_shared_objects: false
  supports_normalizing_ar: false
  supports_start_end_lib: false
  supports_thin_archives: false
  target_libc: "local"

  target_cpu: "%(cpu)s"
  target_system_name: "%(toolchain_id)s"
  toolchain_identifier: "%(toolchain_id)s"

  tool_path {
    name: "ar"
    path: "gcc/%(host)s/%(toolchain_id)s/bin/%(bin_prefix)sar"
  }

  tool_path { name: "compat-ld"
    path: "gcc/%(host)s/%(toolchain_id)s/bin/%(bin_prefix)sld"
  }
  tool_path { name: "cpp"
    path: "gcc/%(host)s/%(toolchain_id)s/bin/%(bin_prefix)scpp"
  }
  tool_path { name: "dwp" path: "/usr/bin/dwp" }
  tool_path { name: "gcc"
    path: "gcc/%(host)s/%(toolchain_id)s/bin/%(bin_prefix)sgcc"
  }
  cxx_flag: "-std=c++0x"
  compilation_mode_flags {
    mode: FASTBUILD
  }
  compilation_mode_flags {
    mode: DBG
  }
  compilation_mode_flags {
    mode: COVERAGE
  }
  compilation_mode_flags {
    mode: OPT
  }

  tool_path {
    name: "gcov"
    path: "/usr/bin/gcov"
  }
  tool_path { name: "ld"
    path: "gcc/%(host)s/%(toolchain_id)s/bin/%(bin_prefix)sld"
  }
  linking_mode_flags {
    mode: FULLY_STATIC
  }
  tool_path { name: "nm"
    path: "gcc/%(host)s/%(toolchain_id)s/bin/%(bin_prefix)snm"
  }
  tool_path { name: "objcopy"
    path: "gcc/%(host)s/%(toolchain_id)s/bin/%(bin_prefix)sobjcopy"
  }
  objcopy_embed_flag: "-I"
  objcopy_embed_flag: "binary"
  tool_path { name: "objdump"
    path: "gcc/%(host)s/%(toolchain_id)s/bin/%(bin_prefix)sobjdump"
  }
  tool_path { name: "strip"
    path: "gcc/%(host)s/%(toolchain_id)s/bin/%(bin_prefix)sstrip"
  }
  linking_mode_flags {
    mode: MOSTLY_STATIC
  }
  linking_mode_flags {
    mode: DYNAMIC
  }

  cxx_builtin_include_directory: "gcc/%(host)s/%(toolchain_id)s/"
  cxx_builtin_include_directory: "gcc/%(host)s/%(toolchain_id)s/"
  cxx_builtin_include_directory: "gcc/linux-x86/x86_64-linux-glibc2.7-4.6/lib/gcc/x86_64-linux/4.6.x-google/include"
  cxx_builtin_include_directory: "%%sysroot%%/usr/include"


  unfiltered_cxx_flag: "-no-canonical-prefixes"

  linker_flag: "-no-canonical-prefixes"

  # add furthor system include directories using either
  #   cxx_builtin_include_directory: "DIRECTORY"
  # or
  #   unfiltered_cxx_flag: "-isystemtools/cpp/gcc/something"

  linker_flag: "-lstdc++"
  linker_flag: "-lm"
  linker_flag: "-lpthread"
}
"""

cpus = {
    "x86_64": "linux-x86/x86_64-linux-glibc2.7-4.6",
    "i686": "linux-x86/i686-linux-glibc2.7-4.6",
#    "darwin-i686": "darwin-x86/i686-apple-darwin-4.2.1",
    "mingw-x86_64": "linux-x86/x86_64-w64-mingw32-4.8",
}

out = preamble

for cpu, path in cpus.items():
  rest = path
  host = rest[:rest.index('/')]
  rest = rest[rest.index('/')+1:]
  toolchain_id = rest

  bin_prefix = {
    "x86_64": "x86_64-linux-",
    "i686": "i686-linux-",
#    "darwin-i686": "i686-apple-darwin10-",
    "mingw-x86_64": "x86_64-w64-mingw32-",
    }[cpu]

  out += template % locals()

open('CROSSTOOL', 'w').write(out)
