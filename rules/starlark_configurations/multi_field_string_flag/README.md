This is an example of encoding multiple fields into a single string flag. It makes use of starlark aspects to lookup several
fields based on the value of the flag.

This is useful if you have several settings that are interdependent and you don't want the user to need to define them all on command line.

To test it out, cd to this directory and run the following:
```
$ bazel build :ice_cream # => "vanilla is white"
$ bazel build :ice_cream --//starlark_configurations/multi_field_string_flag:flavor_flag=grape # => "sugar-free grape is purple"
```