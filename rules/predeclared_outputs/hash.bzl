# A rule that takes in a series of "declarations" and produces "compiled
# output" for some of those declarations, along with a manifest. Here, the
# declarations are simply words in a dictionary text file, and the compiled
# output of a word is just its hash.
#
# The compiled outputs are specified by the user, while the manifest file is
# created automatically. Both kinds of outputs are predeclared, and can be
# referred to in the target graph or built in the command line, e.g.:
#
#     bazel build //predeclared_outputs:dog.md5
#
# If you do not need to refer to the output files as labels, it may be simpler
# to pass in the words as an attr.string_list, and declare the files in the
# implementation function instead.
#
# This rule assumes the presence of shell commands "grep", "md5sum", and "cut".

def _word_hashes_impl(ctx):
    dictionary = ctx.file.dictionary
    manifest = ctx.outputs.manifest

    # For each requested output file, validate that it's an .md5, and emit an
    # action that will generate the file from the dictionary.
    for hashfile in ctx.outputs.hashes:
        basename = hashfile.basename
        if not basename.endswith(".md5"):
            fail("Hash file '%s' must end in '.md5'".format(basename))
        word = basename[:-len(".md5")]

        ctx.actions.run_shell(
            outputs = [hashfile],
            inputs = [dictionary],
            command = "grep {} {} | md5sum | cut -f1 -d ' ' > {}".format(
                word,
                dictionary.path,
                hashfile.path,
            ),
        )

    # Produce the manifest.
    manifest_content = "".join(
        [hashfile.path + "\n" for hashfile in sorted(ctx.outputs.hashes)],
    )
    ctx.actions.write(manifest, manifest_content)

# Since we are not returning a DefaultInfo provider with a files= field,
# all the predeclared outputs will be built when the target is requested.

word_hashes = rule(
    implementation = _word_hashes_impl,
    attrs = {
        "dictionary": attr.label(
            allow_single_file = True,
            mandatory = True,
            doc = "A file containing words, one per line.",
        ),
        "hashes": attr.output_list(
            doc = "A list of files named \"<word>.md5\", where \"<word>\" " +
                  "is in the dictionary.",
        ),
    },
    outputs = {"manifest": "%{name}.manifest"},
)
