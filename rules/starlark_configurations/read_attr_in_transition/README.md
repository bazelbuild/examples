This is an example of a transition that reads both the current configuration and
an attribute. Transitions have access to both those pieces of information in their
implementation functions.

To try it out, cd to this directory and run the following:
```
$ bazel build :dont-do-transition # "value of some-string: abc"
$ bazel build :do-transition # "value of some-string: abc-transitioned"
```

Caveat: <b>You cannot read a
[configured attribute](https://docs.bazel.build/versions/master/configurable-attributes.html)
in a rule transition.</b> This can create a dependency cycle between attribute
values and configuration. To see an example of this cycle, run the following:
```
$ bazel build :will-break # error message
```
