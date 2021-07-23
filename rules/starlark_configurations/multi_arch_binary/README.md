This is an example of how to write a rule that builds a dependency for more than one
architecture. It covers the following topics:
- defining a
  [split](https://docs.bazel.build/skylark/config.html#defining-12-transitions)
  (also know as 1:2+) transition
- attaching a transition to a rule
- reading the transitioned deps

To test it out, cd to this directory and run the following:
```
$ bazel build :foo
```
