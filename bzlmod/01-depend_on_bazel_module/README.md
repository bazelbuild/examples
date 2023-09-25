This is an example on how to introduce dependencies on Bazel modules in the MODULE.bazel file. The WORKSPACE file contains the equivalent definitions in the old system. It covers the following topics:

- defining a Bazel module dependency
- referencing the dependency with a given repository name instead of the module name

To test it out, `cd` into this directory and run the following:

```bash
GLOG_logtostderr=1 bazel run main
```
