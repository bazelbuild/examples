# An example of the common pattern for loading dependencies.
#
# We cannot fully load a dependency without loading its transitive dependencies.
# This set of dependencies could be large and change over time, so we do not
# want to manage it in our own WORKSPACE.  The pattern we use is:
#
# - Rule sets declare a method to load their dependencies.
# - We load that method and call it.
# - This pattern, of course, is repeated at each level.
#
# We also want some fine grained control of diamond dependencies. That is,
# when we have a direct dependency on a rule set, but it is also a transitive
# dependency of one of our dependencies, we would like to make sure we always
# get the version we specify, rather than the one another rule set might.
# To ensure that, we load the 3rd party dependencies in several phases.
#
# First, we import the all rules we depend on directly.
# Then, for each of our dependencies
#   - load and run their dependency loader method.
#   - register any toolchains they might provide which we intend to use.

load("//third_party:third_party.bzl", "load_third_party_libraries")

load_third_party_libraries()

load("//third_party:transitive_dependencies.bzl", "load_transitive_dependencies")

load_transitive_dependencies()
