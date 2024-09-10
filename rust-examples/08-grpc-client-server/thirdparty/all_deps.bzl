"""This module loads dependencies for the `basic` crate examples"""

# rename the default name "crate_repositories" in case you import multiple vendored folders.
load("//thirdparty/crates:defs.bzl", all_crate_repositories = "crate_repositories")

def all_deps():
    """
    This macro loads all vendored dependencies for the repo
    """

    # Load the vendored dependencies
    all_crate_repositories()