This is an example of how to write and use a transition to change a native flag like
"--cpu" or "--platforms".

To test it out, cd to this directory and run the following:
```
$ bazel build :foo # => "--cpu=x86"
$ bazel build :bar # => "--cpu=arm"
$ bazel build :baz # => "--cpu=k8"
```
