# rename the default name "crate_repositories" in case you import multiple vendored folders.
load("//basic/3rdparty/crates:defs.bzl", basic_crate_repositories = "crate_repositories")

def sys_deps():
    """
    This macro loads dependencies for the `basic` crate examples

    Commonly `*-sys` crates are built on top of some existing library and
    will have a number of dependencies. The examples here use
    [crate_universe](https://bazelbuild.github.io/rules_rust/crate_universe.html)
    to gather these dependencies and make them available in the workspace.
    """

    # Load the vendored dependencies
    basic_crate_repositories()
