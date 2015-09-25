#!/bin/sh

# Copyright 2014 The Bazel Authors Inc. All Rights Reserved.
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

set -eux
for p in gcc/linux-x86/host/i686-linux-glibc2.7-4.6 \
  gcc/linux-x86/host/x86_64-linux-glibc2.7-4.6 \
  gcc/linux-x86/host/x86_64-w64-mingw32-4.8 \
  gcc/darwin-x86/host/headers \
  gcc/darwin-x86/host/i686-apple-darwin-4.2.1
do
  prefix=$(echo $p | sed 's|\(gcc/[^/]*\)/.*|\1|g')
  mkdir -p ${prefix}
  (cd ${prefix} &&
    git clone https://android.googlesource.com/platform/prebuilts/$p )
done
