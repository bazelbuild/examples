_MAX_OVERLAY_DIRECTORIES = 1000

def _dirname(path):
    index = path.rfind("/")
    if index == -1:
        return ""
    return path[:index]

def _ensure_parent_directory(repository_ctx, path):
    parent = _dirname(path)
    if not parent:
        return

    marker = parent + "/.android_test_overlay_dir"
    repository_ctx.file(marker, "", executable = False)
    repository_ctx.delete(marker)

def _collect_overlay_files(path):
    files = []
    directories = [(path, "")]

    for _ in range(_MAX_OVERLAY_DIRECTORIES):
        if not directories:
            return files

        directory, directory_relative_path = directories.pop()
        for entry in directory.readdir():
            child_relative_path = entry.basename if not directory_relative_path else directory_relative_path + "/" + entry.basename
            if entry.is_dir:
                directories.append((entry, child_relative_path))
            else:
                files.append((entry, child_relative_path))

    fail("AndroidX Test overlay has more than %d directories" % _MAX_OVERLAY_DIRECTORIES)

def _apply_overlay(repository_ctx, overlay_path):
    repository_ctx.watch_tree(overlay_path)

    for src, overlay_file in _collect_overlay_files(overlay_path):
        _ensure_parent_directory(repository_ctx, overlay_file)
        repository_ctx.delete(overlay_file)
        repository_ctx.symlink(src, overlay_file)

def _android_test_repository_impl(repository_ctx):
    repository_ctx.download_and_extract(
        url = repository_ctx.attr.urls,
        output = ".",
        integrity = repository_ctx.attr.integrity,
        strip_prefix = repository_ctx.attr.strip_prefix,
    )

    overlay_workspace = repository_ctx.path(repository_ctx.attr._overlay_anchor).dirname
    overlay_path = overlay_workspace.get_child("overlay")
    _apply_overlay(repository_ctx, overlay_path)

android_test_repository = repository_rule(
    implementation = _android_test_repository_impl,
    attrs = {
        "_overlay_anchor": attr.label(default = Label("//:MODULE.bazel")),
        "integrity": attr.string(default = "sha256-Jei48zc6cKQi2O8WXdYG2x6phF1cY2269neIpI0Wqf0="),
        "strip_prefix": attr.string(default = "android-test-114487817b0da655a06f0b479c3f9cb3006a5af6"),
        "urls": attr.string_list(default = [
            "https://github.com/android/android-test/archive/114487817b0da655a06f0b479c3f9cb3006a5af6.tar.gz",
        ]),
    },
    local = True,
)
