Bazel with third party dependencies
===================================

Goal
----

This is a small Bazel example that uses third party dependencies ([also called external dependencies](https://bazel.build/external/overview))
but is well structured following the principles of software development. The main goal is that the principles and guidelines shown here they should scale big.
Bazel consultants with access to over 40 codebases have seen this pattern deployed successfully at scale.

Because of this, we want to keep the following requirements:

* Works with bzlmod both enabled and disabled, as a repository may need during a migration.
* We don't want all dependencies listed in the `WORKSPACE` file. It mixes order-dependent and order-independent code, and makes the file harder to manage. Thus we forbid `http_archive` and other "fetch" functions from being loaded in `WORKSPACE`
* The principles mentioned here should be applicable to any programming language.

Details (bzlmod)
----------------

The `MODULE.bazel` file lists the dependencies. When bzlmod is enabled, the `WORKSPACE.bzlmod` file
takes priority over the `WORKSPACE` file. Since `WORKSPACE.bzlmod` is empty, it means this
workspace has been fully converted to work with bzlmod.

Details (WORKSPACE)
-------------------
All dependencies need to be loaded in the workspace file but this does not prevent us on splitting this in several files. The first split comes on the [`WORKSPACE`](WORKSPACE), first we load the source code of all declared dependencies by calling `fetch_deps`, and then we finish what is left to have a fully loaded dependency.

Because WORKSPACE doesn't have the transitive dependency semantics of bzlmod, it requires us as the user to repeat the transitive dependencies. After declaring our external deps to fetch, we must
load and then call the transitive dependency fetching function from each direct dependency, and then
any initializers such as toolchain registration. The rules should indicate what WORKSPACE incantation
is required in their README or release notes.

In cases where the versions of some transitive dependency are skewed, it can be hard to tell how this
relates to the order of the WORKSPACE calls. In this case, you can add transitive dependencies to
the `fetch_deps` macro as a workaround. Since bzlmod will be the only way to specify dependencies
in some future version of Bazel, you can treat this as a short-term problem.

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

If you want to know more about the parameters of the bazel command, you can check [here](https://bazel.build/reference/command-line-reference)

To try with bzlmod, add the `--enable_bzlmod` flag to any of the bazel commands above.
