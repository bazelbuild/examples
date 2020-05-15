This is an example of instantiating and using a label-typed build setting. Label-typed build settings are special
because they're built-in to Bazel so they don't need to be defined as rules or imported from skylib. 

Label-typed build settings are, like all build settings, targets, but their values are also other targets. 
They automatically forward the providers of the targets to which they point. Let's say you have:

Target A ------------------->Target B
                 <-
             Provider B


where Target A consumes provider FooInfo from Target B. 

If you have a situation where you'd like the value of Target B to change depending on the configuration, e.g. by
setting the value of Target B on the command line, you can use a `label_flag` to make that happen:


Target A --------------> Label Flag X --------------> Target B
                <-                          <-
            Provider B                  Provider B


To test it out, cd to this directory and run the following:
```
$ bazel build :my-toolbox # "Using a hammer."
$ bazel build :my-toolbox --//starlark_configurations/label_typed_build_setting:tool= \
	//starlark_configurations/label_typed_build_setting:screwdriver # "Using a screwdriver."
```
