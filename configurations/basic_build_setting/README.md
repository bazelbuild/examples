This is an example of defining and instantiating your own build setting from scratch. It covers the following topics:
- Indicating whether a build setting is a flag or not (settable on the command line or not)
- Indicating the type of the build setting
- Indicating the default value of the build setting
- Defining the build setting rule (`temperature`) and creating an instance of it (`:coffee-temp`)
- Accessing a build setting's value from inside another rule that cares about it

To test it out, cd to this directory and run the following:
```
$ bazel build :today # => "HOT"
$ bazel build :today --//starlark_configurations/basic_build_setting:coffee-temp=ICED # => "ICED"
```