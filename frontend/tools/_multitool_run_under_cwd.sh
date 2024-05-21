#!/bin/sh
# See https://blog.aspect.build/run-tools-installed-by-bazel
bazel run "@multitool//tools/$(basename "$0"):cwd" -- "$@"
