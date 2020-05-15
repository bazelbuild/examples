This is an example of how to use and read the pre-defined build
settings in the `bazel_skylib` repo.

To test it out, cd into this directory and run the following:

```
$ bazel build :ice-cream // => "flavor: strawberry"
$ bazel build :ice-cream --//starlark_configurations/use_skylib_build_settings:flavor=rocky-road => "flavor: rocky-road"
```
