# Bazel with third party dependencies

## Goal

This is a small Bazel example that uses third party dependencies ([also called external dependencies](https://docs.bazel.build/versions/master/external.html))
but is well structured following the principles of software development. The main goal is that the principles and guidelines shown here they should scale big, if you think that this will not scale for a project of more than 1.000 people and 100 dependencies, please file a bug.
Because of this, we want to keep the following requirements:

* We don't want all dependencies listed in the `WORKSPACE` file.
* When we change the version of a dependency, only one file that contains exclusively that dependency is modified.
* When a new dependency is added, we want the minimum amount of changes in common files.
* The principles mentioned here should be applicable to any programming languate.

## Additional considerations

In addition to the requirements mentioned above, we need to consider:

* All direct and transitive dependencies need to be modeled in Bazel.
* A dependeny should know about its direct dependencies, not about its transitive dependencies.
* In a dependency tree we do not want different versions of the same dependency. However we should be able to have two
independent dependency trees with different versions of the same dependency.
* Multiple dependency versions should be able to coexist (with the restrictions mentioned above) especially to be able to switch to a newer version in a incremental way.
* In case that multiple versions of the same dependency appears in the same dependency tree we should be able to detect it.
* We should be able to specify dependencies without specifying the version. This brings to the poin that we need to be able to control the dependency version that will be used by the transitive dependencies.

## How it is done

**Disclaimer:** How it is done right now does not satisfy the restrictions mentioned above. This could be achieved with a proper package manager but right now there is none for Bazel (at least public). There are [some efforts going on](https://docs.google.com/document/d/1moQfNcEIttsk6vYanNKIy3ZuK53hQUFq1b1r0rmsYVg/edit) to create one but it is still in an alpha state (some examples on how it looks [can be found here](https://github.com/meteorcloudy/bzlmod-examples/tree/main/examples)). Until then this is the best that I could come up with, if you think that something can be improved please file a bug or even better create a pull request.

All dependencies need to be loaded in the workspace file but this does not prevent us on splitting this in several files. The first split comes on the [`WORKSPACE`](WORKSPACE), first we load the source code of all declared dependencies and then we finish what is left to have a fully loaded dependency.

This split is done in two functions, `load_third_party_libraries` and `load_transitive_dependencies`.
In addition we need to split the two functions into two different files ([`third_party.bzl`](third_party/third_party.bzl) and [`transitive_dependencies.bzl`](third_party/transitive_dependencies.bzl)).
This is because we cannot load the file that contains `load_transitive_dependencies` without having the source code of the dependencies.
In other words, we know nothing about the transitive dependencies without first having the code of a dependency.

If we keep going deeper we can see that in [`third_party.bzl`](third_party/third_party.bzl) we have a load for each external dependency declared in our project. On the other hand, in [`transitive_dependencies.bzl`](third_party/transitive_dependencies.bzl) we have a load for each direct dependency that we want to use.
This means that in `transitive_dependencies.bzl` we do not need to list all declared dependencies.
Now the question is, why do we want to load the source code of a dependency if we do not want to use it?
Because in this way if the dependency that we load only the source code appears as a transitive dependency in one of our direct dependencies, the version that we specified is the one that will be used. Let's see it with an example.

Imagine that we have dependency `A` and dependency `B` and dependency `A` has `B` as a requirement (`A->B`). We have two options, we could declare only `A` in our workspace or `A` and `B`. Let's see what happens in each of the situations:

* **We declare only `A`**: This means that we only load the source code of `A` in the first phase and then on the second phase when loading the transitive dependencies of `A` we will also load the source code of `B`. The implication here is that the version that we will take of `B` will be define in the source code of `A`.

* **We declare `A` and `B`**: This means that we load the source code of `A` and `B` in the first phase and then on the second phase when loading the transitive dependencies of `A` we will not load the source code of `B`. The implication here is that the version that we will take of `B` is the one that we explicitly declared. We will have more control regarding the version of `B` that will be used.

## How to try it out

In this example you can find a C++ binary, a C++ library and two C++ tests, one using [Catch2](https://github.com/catchorg/Catch2) and the other one using [Google Test](https://github.com/google/googletest).

You can run the C++ binary with the following command:

```bash
bazel run //:hello_world
```

You can run the tests with the following command:

```bash
bazel test //:all --test_output=streamed
```

For the Catch2 test, you can add additinal parameters for a nicer output:

```bash
bazel test //:catch2_test --test_arg "--reporter compact" --test_arg --success --test_output=streamed
```

If you want to know more about the parameters of the bazel command, you can check [here](https://docs.bazel.build/versions/master/command-line-reference.html)
