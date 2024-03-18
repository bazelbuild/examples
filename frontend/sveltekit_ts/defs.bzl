"Implementation details for sveltekit rule"

load("@aspect_rules_js//js:defs.bzl", "js_library", "js_run_binary", "js_run_devserver")
load("@npm//sveltekit_ts:@sveltejs/kit/package_json.bzl", svelte_kit_bin = "bin")
load("@npm//sveltekit_ts:vite/package_json.bzl", vite_bin = "bin")
load("@npm//sveltekit_ts:tsconfig-to-swcconfig/package_json.bzl", tsconfig_to_swcconfig = "bin")
load("@aspect_rules_ts//ts:defs.bzl", "ts_config")

def sveltekit_ts(
        name,
        srcs = [],
        data = [],
        **kwargs):
    """SvelteKit TypeScript rule.

    Args:
      name: Name of the rule.
      srcs: List of source files.
      deps: List of dependencies.
      **kwargs: Additional arguments to pass to the TypeScript compiler.
    """

    # Configuration.
    ts_config(
        name = "tsconfig",
        src = "tsconfig.json",
        visibility = ["//visibility:public"],
    )

    #tsconfig_to_swcconfig.t2s(
    #    name = "write_swcrc",
    #    srcs = ["tsconfig.json"],
    #    args = [
    #        "--filename",
    #        "$(location tsconfig.json)",
    #    ],
    #    stdout = ".swcrc",
    #    visibility = ["//sveltekit_ts:__subpackages__"],
    #)

    js_library(
        name = "package_json",
        srcs = ["package.json"],
        visibility = ["//visibility:public"],
    )

    js_library(
        name = "svelte_config",
        srcs = ["svelte.config.js"],
        data = [
            ":node_modules/@sveltejs/adapter-auto",
            ":node_modules/@sveltejs/kit",
            ":node_modules/@sveltejs/vite-plugin-svelte",
            #":node_modules/vitest",
        ],
        visibility = ["//sveltekit_ts:__subpackages__"],
    )

    js_library(
        name = "vite_config",
        srcs = ["vite.config.ts"],
        data = [
            ":node_modules/vitest",
        ],
        visibility = ["//sveltekit_ts:__subpackages__"],
    )

    js_library(
        name = "playwright_config",
        srcs = ["playwright.config.ts"],
        data = [
            ":node_modules/vitest",
        ],
        visibility = ["//sveltekit_ts:__subpackages__"],
    )

    # This target produces the .svelte-kit directory which is a required
    # dependency for other targets.
    svelte_kit_bin.svelte_kit(
        name = "sveltekit_sync",
        srcs = [
            ":svelte_config",
            ":vite_config",
            ":tsconfig",
            ":package_json",
            ":playwright_config",
        ],
        args = ["sync"],
        chdir = native.package_name(),
        out_dirs = [".svelte-kit"],
    )

    vite_bin.vite_binary(
        name = "vite",
        chdir = native.package_name(),
        data = ["vite.config.ts"],
    )

    js_run_devserver(
        name = "{name}_dev".format(name = name),
        data = srcs + data + [
            ":vite",
            ":sveltekit_sync",
        ],
        command = ":vite",
        args = ["dev"],
        chdir = native.package_name(),
        **kwargs
    )

    # TODO: production build
    #js_run_binary(
    #    name = name,
    #    tool = vite_bin,
    #    srcs = srcs + data,
    #    out_dirs = [
    #        ".svelte-kit",
    #    ],
    #    args = ["build"],
    #    chdir = native.package_name(),
    #    **kwargs
    #)
