# Copyright 2015 The Bazel Authors. All rights reserved.
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

# Scripts to compile the tutorial on the CI system
# This script expect the following environment variable:
#   $BAZEL_INSTALLER set to the path to the bazel installer
#   $ANDROID_SDK_PATH, $ANDROID_SDK_API_LEVEL,
#      $ANDROID_SDK_BUILD_TOOLS_VERSION are respectively the
#      path to the Android SDK, the API Level of the SDK and
#      the version of the build tools. If the path isn't defined
#      the android build won't be tested. The 2 other one are defaulted
#      to, respectively, "22" and "22.0.1".

IOS_SDK_VERSION=$(xcrun --sdk iphonesimulator --show-sdk-version)

# Go to the workspace root
cd "$(dirname "$(dirname "${BASH_SOURCE[0]}")")"

echo "Workspace directory: ${PWD}"

if [ -z "${BAZEL_INSTALLER}" ]; then
  echo "BAZEL_INSTALLER environment variable not provided," >&2
  echo "Please set it to the path of the Bazel's installer." >&2
  exit 1
fi

set -eux
# Set-up android tooling if provided
if [ -n "${ANDROID_SDK_PATH-}" ]; then
  cat >>WORKSPACE <<EOF
android_sdk_repository(
    name = "androidsdk",
    path = "${ANDROID_SDK_PATH}",
    api_level = ${ANDROID_SDK_API_LEVEL:-22},
    build_tools_version = "${ANDROID_SDK_BUILD_TOOLS_VERSION:-22.0.1}",
)
EOF
  # Revert change to workspace
  trap 'git checkout HEAD WORKSPACE' EXIT
fi

# Install bazel
BASE="$(dirname "${PWD}")/bazel-install"
bash "${BAZEL_INSTALLER}" \
  --base="${BASE}" \
  --bazelrc="$PWD/.bazelrc" \
  --bin="${BASE}/binary"

BAZEL="${BASE}/binary/bazel"
 
# Cleanup
"${BAZEL}" clean --expunge

# bazel info
"${BAZEL}" info

# Test building the backend
"${BAZEL}" build //backend

# Test the android application
if [ -n "${ANDROID_SDK_PATH-}" ]; then
  "${BAZEL}" build //android
fi

# Under darwin, test the ios application
if [ "$(uname -s | tr 'A-Z' 'a-z')" = "darwin" ]; then
  "${BAZEL}" build //ios-app --ios_sdk_version=$IOS_SDK_VERSION
fi

echo "Yay!"
