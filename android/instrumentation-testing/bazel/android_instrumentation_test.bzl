load("@rules_android//providers:providers.bzl", "AndroidInstrumentationInfo", "ApkInfo")

def _apk_from_target(target, attr_name):
    if ApkInfo in target:
        apk = target[ApkInfo].signed_apk
        if not apk:
            fail("%s must provide a signed APK" % attr_name)
        return apk

    apks = [f for f in target[DefaultInfo].files.to_list() if f.basename.endswith(".apk")]
    if len(apks) != 1:
        fail("%s must provide exactly one .apk file, got %d" % (attr_name, len(apks)))
    return apks[0]

def _runfiles_path(file):
    path = file.short_path
    if path.startswith("../"):
        return path[3:]
    return path

def _apparent_label(label):
    return "//%s:%s" % (label.package, label.name)

def _android_instrumentation_test_impl(ctx):
    test_app = ctx.attr.test_app
    test_apk = test_app[ApkInfo].signed_apk
    target_apk = test_app[AndroidInstrumentationInfo].target.signed_apk
    if not target_apk:
        fail("test_app must set instruments to an android_binary that produces a signed APK")

    support_apks = [_apk_from_target(apk, "support_apks") for apk in ctx.attr.support_apks]
    executable = ctx.actions.declare_file(ctx.label.name)

    substitutions = {
        "%workspace%": ctx.workspace_name,
        "%test_label%": _apparent_label(ctx.label),
        "%test_entry_point%": _runfiles_path(ctx.executable._test_entry_point),
        "%adb%": _runfiles_path(ctx.file._adb),
        "%aapt%": _runfiles_path(ctx.executable._aapt),
        "%dexdump%": _runfiles_path(ctx.file._dexdump),
        "%target_apk%": _runfiles_path(target_apk),
        "%instrumentation_apk%": _runfiles_path(test_apk),
        "%support_apks%": " ".join([_runfiles_path(apk) for apk in support_apks]),
        "%test_packages%": " ".join([
            "additional_test_packages=%s" % package
            for package in ctx.attr.test_packages
        ]),
        "%device_broker_type%": ctx.attr.device_broker_type,
        "%bootstrap_instrumentation_package%": ctx.attr.bootstrap_instrumentation_package,
        "%install_basic_services%": str(ctx.attr.install_basic_services).lower(),
        "%install_test_services%": str(ctx.attr.install_test_services).lower(),
        "%scan_target_package_for_tests%": str(ctx.attr.scan_target_package_for_tests).lower(),
    }

    ctx.actions.expand_template(
        template = ctx.file._template,
        output = executable,
        substitutions = substitutions,
        is_executable = True,
    )

    runfiles = ctx.runfiles(files = [
        executable,
        ctx.executable._test_entry_point,
        ctx.file._adb,
        ctx.executable._aapt,
        ctx.file._dexdump,
        target_apk,
        test_apk,
    ] + support_apks)
    runfiles = runfiles.merge(ctx.attr._test_entry_point[DefaultInfo].default_runfiles)
    runfiles = runfiles.merge(ctx.attr._aapt[DefaultInfo].default_runfiles)

    return [DefaultInfo(
        executable = executable,
        runfiles = runfiles,
    )]

android_instrumentation_test = rule(
    implementation = _android_instrumentation_test_impl,
    attrs = {
        "bootstrap_instrumentation_package": attr.string(),
        "device_broker_type": attr.string(default = "LOCAL_ADB_SERVER"),
        "install_basic_services": attr.bool(default = False),
        "install_test_services": attr.bool(default = True),
        "scan_target_package_for_tests": attr.bool(default = False),
        "support_apks": attr.label_list(allow_files = [".apk"]),
        "test_app": attr.label(
            mandatory = True,
            providers = [[ApkInfo, AndroidInstrumentationInfo]],
        ),
        "test_packages": attr.string_list(),
        "_aapt": attr.label(
            default = Label("@androidsdk//:aapt_binary"),
            executable = True,
            cfg = "exec",
        ),
        "_adb": attr.label(
            default = Label("@androidsdk//:adb"),
            allow_single_file = True,
        ),
        "_dexdump": attr.label(
            default = Label("@androidsdk//:dexdump"),
            allow_single_file = True,
        ),
        "_template": attr.label(
            default = Label("//bazel:android_instrumentation_test.sh.tpl"),
            allow_single_file = True,
        ),
        "_test_entry_point": attr.label(
            default = Label("@android_test_support//:instrumentation_test_runner"),
            executable = True,
            cfg = "exec",
        ),
    },
    test = True,
)
