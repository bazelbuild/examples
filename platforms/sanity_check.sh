#!/bin/bash

set -euo pipefail

_BAZEL_BINARY="${BAZEL_BINARY:-"bazel"}"

function b() {
  $_BAZEL_BINARY build $@
}

b //examples/01_hello_world:a
b //examples/02_using_different_platforms:a --platforms=//:linux_platform
b //examples/02_using_different_platforms:a --platforms=//:windows_platform
b //examples/03_target_not_compatible_with_constraint:a --platforms=//:linux_platform
b //examples/03_target_not_compatible_with_constraint:b --platforms=//:linux_platform && (exit 15) || true
b //examples/04_select_on_constraint:a --platforms=//:linux_platform
b //examples/04_select_on_constraint:a --platforms=//:windows_platform
b //examples/04_select_on_constraint:a --platforms=//:android_platform && (exit 18) || true
b //examples/05_select_on_platform:a --platforms=//:linux_platform
b //examples/05_select_on_platform:a --platforms=//:windows_platform
b //examples/06_integer_constraint:a --platforms=//:linux_platform && (exit 21) || true
b //examples/06_integer_constraint:a --platforms=//:linux_yolo_3_platform --host_platform=//:linux_yolo_3_platform
b //examples/07_using_define:everything
b //examples/07_using_define:everything --define is_foo_defined=true
b //examples/08_using_build_setting:everything
b //examples/08_using_build_setting:everything --//examples/08_using_build_setting:foo_enabled
b //examples/08_using_build_setting:everything --//examples/08_using_build_setting:foo_enabled=False
b //examples/08_using_build_setting:everything --//examples/08_using_build_setting:foo_enabled=True
