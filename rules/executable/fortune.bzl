# A rule that generates a "fortune"-style executable for haikus. It takes haiku
# files as inputs, and produces a data file consisting of their concatenation,
# along with a shell script.
#
# To run, use
#     bazel run //executable:bazel_haikus

# The script content, with placeholders for the data file name and number of
# haikus.
script_template = """\
#!/bin/bash
R=$(($RANDOM%{num_fortunes}*3+1))
cat {fortunes_file} | sed -n "$R,$(($R+2))p"
"""

def _haiku_fortune_impl(ctx):
    # Generate a datafile of concatenated fortunes.
    datafile = ctx.actions.declare_file(ctx.label.name + ".fortunes")
    ctx.actions.run_shell(
        outputs = [datafile],
        inputs = ctx.files.srcs,
        command = "cat {} > {}".format(
            " ".join([f.path for f in ctx.files.srcs]),
            datafile.path))

    # Emit the executable shell script.
    script = script_template.format(
        fortunes_file = datafile.short_path,
        num_fortunes = len(ctx.attr.srcs))
    ctx.actions.write(ctx.outputs.exe, script, is_executable=True)

    # The datafile must be in the runfiles for the executable to see it.
    runfiles = ctx.runfiles(files=[datafile])
    return [DefaultInfo(executable=ctx.outputs.exe, runfiles=runfiles)]

haiku_fortune = rule(
    implementation = _haiku_fortune_impl,
    attrs = {
        "srcs": attr.label_list(
            allow_files = True,
            doc = "Input haiku files. Each file must have exactly three lines. "
                + "The last line must be terminated by a newline character."
        ),
    },
    outputs = {
        "exe": "%{name}-fortune",
    },
    executable = True,
)
