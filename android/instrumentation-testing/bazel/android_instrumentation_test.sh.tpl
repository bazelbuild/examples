#!/usr/bin/env bash
set -euo pipefail

if [[ -z "${TEST_SRCDIR:-}" && -z "${RUNFILES_DIR:-}" ]]; then
  echo "TEST_SRCDIR or RUNFILES_DIR must be set by Bazel test." >&2
  exit 1
fi

workspace="%workspace%"

function resolve_runfile() {
  local path="$1"
  local base
  for base in "${RUNFILES_DIR:-}" "${TEST_SRCDIR:-}"; do
    if [[ -z "${base}" ]]; then
      continue
    fi
    if [[ -e "${base}/${path}" ]]; then
      echo "${base}/${path}"
      return
    fi
    if [[ -e "${base}/${workspace}/${path}" ]]; then
      echo "${base}/${workspace}/${path}"
      return
    fi
  done
  echo "Could not resolve runfile: ${path}" >&2
  exit 1
}

function join_runfiles() {
  local separator="$1"
  shift
  local result=""
  local path
  for path in "$@"; do
    if [[ -z "${path}" ]]; then
      continue
    fi
    if [[ -n "${result}" ]]; then
      result+="${separator}"
    fi
    result+="$(resolve_runfile "${path}")"
  done
  echo "${result}"
}

if [[ -z "${TESTBRIDGE_TEST_ONLY+set}" ]]; then
  android_testbridge_test_only=""
else
  android_testbridge_test_only="${TESTBRIDGE_TEST_ONLY}"
  unset TESTBRIDGE_TEST_ONLY
fi

test_entry_point="$(resolve_runfile "%test_entry_point%")"
adb="$(resolve_runfile "%adb%")"
aapt="$(resolve_runfile "%aapt%")"
dexdump="$(resolve_runfile "%dexdump%")"
target_apk="$(resolve_runfile "%target_apk%")"
instrumentation_apk="$(resolve_runfile "%instrumentation_apk%")"
support_apks="$(join_runfiles "," %support_apks%)"

if [[ -n "${support_apks}" ]]; then
  apks_to_install="${support_apks},${target_apk},${instrumentation_apk}"
else
  apks_to_install="${target_apk},${instrumentation_apk}"
fi

argv=$(cat <<END
--aapt=${aapt} \
--adb=${adb} \
--dexdump_path=${dexdump} \
--device_broker_type=%device_broker_type% \
--apks_to_install=${apks_to_install} \
--bootstrap_instrumentation_package=%bootstrap_instrumentation_package% \
--install_basic_services=%install_basic_services% \
--install_test_services=%install_test_services% \
--scan_target_package_for_tests=%scan_target_package_for_tests% \
%test_packages% \
--test_label=%test_label% \
--test_filter=${android_testbridge_test_only} \
$@
END
)

"${test_entry_point}" \
  --wrapper_script_flag=--jvm_flag=--add-opens=java.base/java.lang=ALL-UNNAMED \
  --wrapper_script_flag=--jvm_flag=--add-opens=java.base/java.util.function=ALL-UNNAMED \
  --wrapper_script_flag=--jvm_flag=-Dbazel.test_suite=com.google.android.apps.common.testing.suite.AndroidDeviceTestSuite \
  --wrapper_script_flag=--jvm_flag=-Dargv="${argv}" \
  ${argv}
