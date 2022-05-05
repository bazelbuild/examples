This is an example of how to define and use a transition on a build setting. Rules can attach transitions
in two places: (1) directly on the rule and (2) onto an attribute. It covers the following topics
- attaching a transition to a rule
- attaching a transition to an attribute
- reading the build setting value of a target
- reading the build setting value of a transitioned dependency
- reading the build setting value of a non-transitioned dependency

To test it out, cd to this directory and run the following:
```
$ bazel build :tee
```