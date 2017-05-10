#!/bin/bash

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

function setup_android_repositories() {
  if [ ! -f WORKSPACE.bak ] && [ -n "${ANDROID_SDK_PATH-}" ]; then
    cp WORKSPACE WORKSPACE.bak
    trap '[ -f WORKSPACE.bak ] && rm WORKSPACE && mv WORKSPACE.bak WORKSPACE' \
      EXIT
    # Make sure that WORKSPACE ends with a newline, otherwise we'll end up with
    # a syntax error.
    echo >>WORKSPACE
    cat >>WORKSPACE <<EOF
android_sdk_repository(
    name = "androidsdk",
    path = "${ANDROID_SDK_PATH}",
)

bind(
    name = "android_sdk_for_testing",
    actual = "@androidsdk//:files",
)
EOF
    if [ -n "${ANDROID_NDK_PATH-}" ]; then
      cat >>WORKSPACE <<EOF
android_ndk_repository(
    name = "androidndk",
    path = "${ANDROID_NDK_PATH}",
)

bind(
    name = "android_ndk_for_testing",
    actual = "@androidndk//:files",
)
EOF
    fi
  fi
}

setup_android_repositories
