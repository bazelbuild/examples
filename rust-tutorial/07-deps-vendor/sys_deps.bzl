
load("//basic/3rdparty/crates:defs.bzl", basic_crate_repositories = "crate_repositories")

def sys_deps():
    """
    This macro loads dependencies for the `sys` crate examples

    Commonly `*-sys` crates are built on top of some existing library and
    will have a number of dependencies. The examples here use
    [crate_universe](https://bazelbuild.github.io/rules_rust/crate_universe.html)
    to gather these dependencies and make them available in the workspace.
    """

    basic_crate_repositories()
