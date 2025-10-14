## Auto-configured builds examples
**Requirements: Bazel 9.0 or newer.**

*Auto-configured builds* is a Bazel 9.0+ feature that lets project owners declare which build flags should apply to their project's targets.

This makes it possible to write

    $ bazel test //foo/...

and bazel automatically adds whatever flags are appropriate for `foo`'s builds. 

This is similar to [bazelrc](https://bazel.build/run/bazelrc) with the crucial difference that bazelrc files are *user-registered*, not *project-registered*. That means `bazelrc`-added flags depend on who does the build and which `bazelrcs` they've registered, not which targets they're building. 

The features described here make it easier for *anyone* to write `$ bazel test //foo/...` - a project dev, external contributor, library maintainer, IDE, AI agent, or CI system - and consistently get the same results with the same flags regardless of how they've set up their workspace or if they've ever built this project before. Project owners can then ensure everyone builds with project-approved correct flags.

More info at https://github.com/bazelbuild/bazel/issues/24839.

### `PROJECT.scl`
Flag settings are declared in a file called `PROJECT.scl` which lives in your source repository next to your `BUILD` files. 

`$ bazel test //foo/bar/baz:all` looks for a `PROJECT.scl` in `foo/bar/baz/` to find the project's flag settings. If that file doesn't exist, it looks in `foo/bar/`, then `foo/`, and so on until it either finds a match or determines the project has no flag settings.

This also applies to `bazel build` and `bazel cquery`.

### Project-wide settings with warnings for unexpected flags
[warn](warn) is an example that sets two sets of flags for a project. The first set - `default_config` triggers by default. The second - `debug_config` can be set with `--scl_config=debug_config`. If the user sets any other flags, bazel emits a warning that the build is non-canonical:

*Default flags:*
```sh
$ bazel build //warn:all
INFO: Reading project settings from //warn:PROJECT.scl.
INFO: Applying flags from the config 'default_config' defined in //warn:PROJECT.scl: [--platforms=//:myplatform,
 --compilation_mode=opt, --@custom_flags//:project_flag="custom flag value"]
INFO: Found 2 targets...
INFO: Build completed successfully, 3 total actions
```

*Non-default supported flags:*
```sh
$ bazel build //warn:all --scl_config=debug_config
INFO: Reading project settings from //warn:PROJECT.scl.
INFO: Applying flags from the config 'debug_config' defined in //warn:PROJECT.scl: [--platforms=//:myplatform,
 --compilation_mode=dbg, --@custom_flags//:project_flag="debug value"]
INFO: Found 2 targets...
INFO: Build completed successfully, 3 total actions
```

*Unexpected flags:*
```sh
$ bazel build //warn:all --copt=abc
INFO: Reading project settings from //warn:PROJECT.scl.
WARNING: This build uses a project file (//warn:PROJECT.scl), but also sets output-affecting flags in the command
 line or user bazelrc: ['--copt=abc']. Please consider removing these flags.
INFO: Applying flags from the config 'default_config' defined in //warn:PROJECT.scl: [--platforms=//:myplatform, 
 --compilation_mode=opt, --@custom_flags//:project_flag="custom flag value"
INFO: Found 2 targets...
INFO: Build completed successfully, 3 total actions
```

### Project-wide settings with incompatible flag checks
[compatible](compatible) is an example that sets two sets of flags for a project. It lets the user set other, unrelated flags with a warning but fails the build if the user sets flags that contradict the project's flags.

*Default flags:*
```sh
$ bazel build //compatible:all
INFO: Reading project settings from //compatible:PROJECT.scl.
INFO: Applying flags from the config 'default_config' defined in //compatible:PROJECT.scl:
 [--platforms=//:myplatform, --compilation_mode=opt, --@custom_flags//:project_flag="custom flag value"]
INFO: Found 2 targets...
INFO: Build completed successfully, 3 total actions
```

*Unexpected non-conflicting flag:*
```sh
$ bazel build //compatible:all --copt=abc
INFO: Reading project settings from //compatible:PROJECT.scl.
WARNING: This build uses a project file (//compatible:PROJECT.scl), but also sets output-affecting flags in
 the command line or user bazelrc: ['--copt=abc']. Please consider removing these flags.
INFO: Applying flags from the config 'default_config' defined in //compatible:PROJECT.scl:
 [--platforms=//:myplatform, --compilation_mode=opt, --@custom_flags//:project_flag="custom flag value"]
INFO: Build completed successfully, 3 total actions
```

*Unexpected conflicting flag:*
```sh
$ bazel build //compatible:all --compilation_mode=fastbuild
INFO: Reading project settings from //compatible:PROJECT.scl.
ERROR: Cannot parse options: This build uses a project file (//compatible:PROJECT.scl) that does not allow
 conflicting flags in the command line or user bazelrc. Found ['--compilation_mode=fastbuild']. Please remove
 these flags or disable project file resolution via --noenforce_project_configs.
ERROR: Build did NOT complete successfully
```

### Project-wide settings with strict checks
[strict](strict) is an example that sets two sets of flags for a project. It fails the build if the user sets any flags to any different values than those the project specifies. This is the strictest form of flag checking that ensures all builds use pre-approved, canonical settings.

*Default flags:*
```sh
$ bazel build //strict:all
INFO: Reading project settings from //strict:PROJECT.scl.
INFO: Applying flags from the config 'default_config' defined in //strict:PROJECT.scl: [--platforms=//:myplatform,
 --compilation_mode=opt, --@custom_flags//:project_flag="custom flag value"]
INFO: Found 2 targets...
INFO: Build completed successfully, 3 total actions
```

*Unexpected non-conflicting flag:*
```sh
$ bazel build //strict:all --copt=abc
INFO: Reading project settings from //strict:PROJECT.scl.
ERROR: Cannot parse options: This build uses a project file (//strict:PROJECT.scl) that does not allow
 output-affeccting flags in the command line or user bazelrc. Found ['--copt=abc']. Please remove these flags or
 disable project file resolution via --noenforce_project_configs.
ERROR: Build did NOT complete successfully
```

### Per-target flag settings
[target_specific](target_specific) sets different default flags for different targets in a project. This example applies the [warn](warn) enforcement policy.

*`//target_specific:one`:*
```sh
$ bazel build //target_specific:one
INFO: Reading project settings from //target_specific:PROJECT.scl.
INFO: Applying flags from the config 'default_config_for_target_one' defined in //target_specific:PROJECT.scl:
 [--platforms=//:myplatform, --@custom_flags//:project_flag="settings for target one"]
INFO: Found 1 target...
INFO: Build completed successfully, 1 total action
```

*`//target_specific:two`:*
```sh
$ bazel build //target_specific:two
INFO: Reading project settings from //target_specific:PROJECT.scl.
INFO: Applying flags from the config 'default_config_for_target_two' defined in //target_specific:PROJECT.scl:
 [--platforms=//:myplatform, --@custom_flags//:project_flag="settings for target two"]
INFO: Found 1 target...
INFO: Build completed successfully, 1 total action
```

**Note:** `$ bazel build //target_specific:all` fails because no common set of flags applies to both targets. You can resolve this either by explicitly setting `--scl_config` or disabling project flags with `--noenforce_project_configs`. Comment on https://github.com/bazelbuild/bazel/issues/24839 if you're interested in more automatic behavior.

### Aliases
[alias](alias) is a project with different disjoint directories. You can refer them all to the same souce of flag truth with `PROJECT.scl` aliases.

*Main project definition in `alias/project_main`:*
```sh
$ bazel build //alias/project_main:main
INFO: Reading project settings from //alias/project_main:PROJECT.scl.
INFO: Applying flags from the config 'default_config' defined in //alias/project_main:PROJECT.scl:
 [--platforms=//:myplatform, --compilation_mode=opt, --@custom_flags//:project_flag="custom flag value"]
INFO: Found 1 target...
INFO: Build completed successfully, 1 total action
```

*`alias/project_lib` is a different directory but part of the same project:*
 ```sh
$ cat alias/project_lib/PROJECT.scl
project = {
    "actual": "//alias/project_main:PROJECT.scl",
}

bazel build //alias/project_lib:lib
INFO: Reading project settings from //alias/project_main:PROJECT.scl.
INFO: Applying flags from the config 'default_config' defined in //alias/project_main:PROJECT.scl:
 [--platforms=//:myplatform, --compilation_mode=opt, --@custom_flags//:project_flag="custom flag value"]
INFO: Found 1 target...
INFO: Build completed successfully, 1 total action
```  

**Note:** In this example both directories have the same parent directory. You can alternatively move the project definition to `alias/PROJECT.scl` for the same effect. This also requires an `alias/BUILD` file, which may be empty.

### Questions?
This is an evolving feature. These examples don't cover everything.

Comment at https://github.com/bazelbuild/bazel/issues/24839 with more questions and requests. 

