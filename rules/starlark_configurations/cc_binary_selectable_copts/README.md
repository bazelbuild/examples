### Example showing how to use [Starlark configuration](https://docs.bazel.build/versions/master/skylark/config.html) so a `cc_binary` can set custom "features" for `cc_library` dependencies to include.


This example has five files:

* [custom_settings/BUILD](custom_settings/BUILD) - This defines a [custom
  Starlark
  flag](https://docs.bazel.build/versions/master/skylark/config.html#user-defined-build-settings)
  called `//custom_settings:mycopts` which defines the set of possible features
  and stores whatever features the `cc_binary` sets.
  
  `cc_library` uses a `select` to read this flag and set its `copts`
  accordingly. This file also declares the `config_setting`s the `select` uses.
  
* [defs.bzl](defs.bzl) - This defines a custom Starlark rule `transition_rule`
  which defines an attribute `set_features` that sets the desired feature and
  `actual_binary` which declares the `cc_binary` this should apply to.
  
  Because `cc_binary` is a native (not Starlark-defined) rule, we can't
  add `set_features` directly to it. For Starlark-defined rules, we could
  omit `transition_rule` and just add the functionality directly.
  
  `transition_rule` applies a [Starlark
  transition](https://docs.bazel.build/versions/master/skylark/config.html#user-defined-transitions)
  called `_copt_transition` that reads the value of `set_features` and sets
  `//custom-settings:mycopts` accordingly.
  
  Finally, this file declares a macro (also) called `cc_binary` that automates away all
  this extra abstraction: the new macro `cc_binary` simply instantiates a
  `transition_rule` with the desired `set_features` then passes all other
  attributes directly to the native `cc_binary`. To the end user this makes it
  look like `cc_binary` magically supports a new attribute.
  
* [BUILD](BUILD) - This defines three versions of a `cc_binary` all depending on
  the same `cc_library`: one uses `feature`, the other uses `feature2`, and the
  third fails to build because it "forgets" to set any feature.
  
* [main.cc](main.cc) and [lib.cc](lib.cc) for the actual C++ code.

To see it in action, cd to this directory and try the following commands:

```sh
$ bazel run :app_with_feature1
...
Running my app!
Building lib with feature 1!
```

```sh
$ bazel run :app_with_feature2
...
Running my app!
Building lib with feature 2!
```

```sh
$ bazel run :app_forgets_to_set_features 
ERROR: $(MYWORKSPACE)/cc_binary_selectable_copts/BUILD:32:13: Configurable attribute "copts"
doesn't match this configuration: You must explicitly set which features you want!
```

This example relies on `select` and involves some duplication of values
(e.g. `"feature1"` is defined in `//custom_settings:mycopts` and `//:lib`
separately sets `"-Dfeature1"`). You could write more Starlark macros to make the
`BUILD` API even simpler. For example: not requiring any changes to
`cc_library` at all.
