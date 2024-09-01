"Root BUILD file for all frontend examples"

load("@npm//:defs.bzl", "npm_link_all_packages")

package(default_visibility = ["//:__subpackages__"])

# Create the root of the "virtual store" of npm dependencies under bazel-out.
# This must be done in the package where the pnpm workspace is rooted.
npm_link_all_packages(name = "node_modules")
