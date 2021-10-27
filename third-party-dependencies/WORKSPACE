# We cannot fully load a dependency without loading the source code of the transitive dependencies.
# In addition we want that if a dependency appears as a direct dependency and also as a transitive dependency
# the direct dependency is loaded first, in this way we have better control on which version is used in
# case that it is defined with the same name but different version.
# Because of that, we load the 3rd party dependencies in two steps. First we load only the source code of all
# dependencies explicitly declared in our project. Then on a second step we do everything that is missing to
# fully load the dependency (load transitive dependencies, registar rules or toolchains, etc.)

load("//third_party:third_party.bzl", "load_third_party_libraries")

load_third_party_libraries()

load("//third_party:transitive_dependencies.bzl", "load_transitive_dependencies")

load_transitive_dependencies()
