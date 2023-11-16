"Convenience macros to call the Architect program."

load("@aspect_bazel_lib//lib:copy_to_bin.bzl", "copy_to_bin")
load("@aspect_bazel_lib//lib:jq.bzl", "jq")
load("@aspect_rules_js//js:defs.bzl", "js_library")
load("@npm//angular:@angular-devkit/architect-cli/package_json.bzl", architect_cli = "bin")

# NOTE:
#  *_DEPS are propagated as deps of the final output
#  *_CONFIG are dependencies only of the architect actions and not propagated

# Global dependencies such as common config files, tools
COMMON_CONFIG = [
    "//angular:ng-config",
    "//angular:node_modules/@angular-devkit/build-angular",
    "//angular:node_modules/@angular-devkit/architect-cli",
]

# Common dependencies of Angular CLI applications
APPLICATION_CONFIG = [
    ":tsconfig.app.json",
    ":package.json",
]
APPLICATION_DEPS = [
    "//angular:node_modules/@angular/common",
    "//angular:node_modules/@angular/core",
    "//angular:node_modules/@angular/router",
    "//angular:node_modules/@angular/platform-browser",
    "//angular:node_modules/@angular/platform-browser-dynamic",
    "//angular:node_modules/@types/node",
    "//angular:node_modules/rxjs",
    "//angular:node_modules/tslib",
    "//angular:node_modules/zone.js",
]

# Common dependencies of Angular CLI libraries
LIBRARY_CONFIG = [
    ":tsconfig.lib.json",
    ":tsconfig.lib.prod.json",
    ":package.json",
]
LIBRARY_DEPS = [
    "//angular:node_modules/@angular/common",
    "//angular:node_modules/@angular/core",
    "//angular:node_modules/@angular/router",
    "//angular:node_modules/@types/node",
    "//angular:node_modules/rxjs",
    "//angular:node_modules/tslib",
]

# Common dependencies of Angular CLI test suites
TEST_CONFIG = [
    ":tsconfig.spec.json",
    ":karma.conf.js",
    "//angular:node_modules/@types/jasmine",
    "//angular:node_modules/karma-chrome-launcher",
    "//angular:node_modules/karma",
    "//angular:node_modules/karma-jasmine",
    "//angular:node_modules/karma-jasmine-html-reporter",
    "//angular:node_modules/karma-coverage",
]
TEST_DEPS = LIBRARY_DEPS + [
    "//angular:node_modules/@angular/compiler",
    "//angular:node_modules/@angular/platform-browser",
    "//angular:node_modules/@angular/platform-browser-dynamic",
    "//angular:node_modules/jasmine-core",
    "//angular:node_modules/zone.js",
]

# JQ expressions to update Angular project output paths from dist/* to projects/*/dist
JQ_DIST_REPLACE_TSCONFIG = ".compilerOptions.paths |= map_values(map(gsub(\"^dist/(?<p>.+)$\"; \"projects/\"+.p+\"/dist\")))"
JQ_DIST_REPLACE_NG_PACKAGE = ".dest = \"dist\""

def ng_config(name):
    """Create patched config files for Angular, so that the files in the source tree work with the Angular CLI

    Args:
        name: must be "ng-config"
    """
    if name != "ng-config":
        fail("NG config name must be 'ng-config'")

    # Root config files used throughout
    copy_to_bin(
        name = "angular",
        srcs = ["angular.json"],
    )

    # NOTE: project dist directories are under the project dir unlike the Angular CLI default of the root dist folder
    jq(
        name = "tsconfig",
        srcs = ["tsconfig.json"],
        filter = JQ_DIST_REPLACE_TSCONFIG,
    )

    native.filegroup(
        name = name,
        srcs = [":angular", ":tsconfig"],
    )

def ng_app(name, project_name = None, deps = [], test_deps = [], **kwargs):
    """
    Bazel macro for compiling an NG application project. Creates {name}, test, serve targets.

    Args:
      name: the rule name
      project_name: the Angular CLI project name, to the rule name
      deps: dependencies of the library
      test_deps: additional dependencies for tests
      **kwargs: extra args passed to main Angular CLI rules
    """
    srcs = native.glob(
        ["src/**/*"],
        exclude = [
            "src/**/*.spec.ts",
            "src/test.ts",
            "dist/",
        ],
    )

    test_srcs = native.glob(["src/test.ts", "src/**/*.spec.ts"])

    project_name = project_name if project_name else name

    architect_cli.architect(
        name = name,
        chdir = native.package_name(),
        args = ["%s:build" % project_name],
        out_dirs = ["dist/%s" % project_name],
        srcs = srcs + deps + APPLICATION_DEPS + APPLICATION_CONFIG + COMMON_CONFIG,
        **kwargs
    )

    architect_cli.architect_binary(
        name = "serve",
        chdir = native.package_name(),
        args = ["%s:serve" % project_name],
        data = srcs + deps + APPLICATION_DEPS + APPLICATION_CONFIG + COMMON_CONFIG,
        **kwargs
    )

    architect_cli.architect_test(
        name = "test",
        chdir = native.package_name(),
        args = ["%s:test" % project_name],
        data = srcs + test_srcs + deps + test_deps + TEST_DEPS + TEST_CONFIG + COMMON_CONFIG,
        log_level = "debug",
        **kwargs
    )

def ng_lib(name, project_name = None, deps = [], test_deps = [], **kwargs):
    """
    Bazel macro for compiling an NG library project. Creates {name}, test, targets.

    Args:
      name: the rule name
      project_name: the Angular CLI project name, defaults to current directory name
      deps: dependencies of the library
      test_deps: additional dependencies for tests
      **kwargs: extra args passed to main Angular CLI rules
    """
    srcs = native.glob(
        ["src/**/*"],
        exclude = [
            "src/**/*.spec.ts",
            "src/test.ts",
            "dist/",
        ],
    )

    test_srcs = srcs + native.glob(["src/test.ts", "src/**/*.spec.ts"])

    project_name = project_name if project_name else native.package_name().split("/").pop()

    # NOTE: dist directories are under the project dir instead of the Angular CLI default of the root dist folder
    jq(
        name = "ng-package",
        srcs = ["ng-package.json"],
        filter = JQ_DIST_REPLACE_NG_PACKAGE,
        visibility = ["//visibility:private"],
    )

    architect_cli.architect(
        name = "_%s" % name,
        chdir = native.package_name(),
        args = ["%s:build" % project_name],
        out_dirs = ["dist"],
        srcs = srcs + deps + LIBRARY_DEPS + LIBRARY_CONFIG + COMMON_CONFIG + [":ng-package"],
        visibility = ["//visibility:private"],
        **kwargs
    )

    architect_cli.architect_test(
        name = "test",
        chdir = native.package_name(),
        args = ["%s:test" % project_name, "--no-watch"],
        data = test_srcs + deps + test_deps + TEST_DEPS + TEST_CONFIG + COMMON_CONFIG + [":ng-package"],
        log_level = "debug",
        **kwargs
    )

    # Output the compiled library and its dependencies
    js_library(
        name = name,
        srcs = [":_%s" % name],
        deps = deps + LIBRARY_DEPS,
    )
