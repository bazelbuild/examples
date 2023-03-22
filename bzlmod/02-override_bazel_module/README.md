This is an example on how to override Bazel module dependencies in the MODULE.bazel file. The WORKSPACE file contains the equivalent definitions in the old system. It covers the following topics:

- overriding a Bazel module with an archive URL.
- overriding a Bazel module with a git repository.
- overriding a Bazel module with a local path.
- overriding a Bazel module with a local patch file.

Note that the final source tree after overriding must contain a MODULE.bazel file at the root.

To test it out, `cd` into this directory and run the following:

```bash
export USE_BAZEL_VERSION=last_green
bazelisk build --enable_bzlmod //:main
GLOG_logtostderr=1 ./bazel-bin/main
```
