#!/bin/sh
# See https://blog.aspect.build/run-tools-installed-by-bazel
case "$(basename "$0")" in
  pnpm)
    # https://github.com/aspect-build/rules_js/blob/main/docs/faq.md#can-i-use-bazel-managed-pnpm
    target="@pnpm"
    ;;
  *)
    target="@multitool//tools/$(basename "$0")"
    ;;
esac

# NB: we don't use 'bazel run' because it may leave behind zombie processes under ibazel
# shellcheck disable=SC2046
bazel 2>/dev/null build --build_runfile_links "$target" && \
  BAZEL_BINDIR=. exec $(bazel 2>/dev/null info execution_root)/$(bazel 2>/dev/null cquery --output=files "$target") "$@"
