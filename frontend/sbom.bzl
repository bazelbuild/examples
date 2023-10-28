""

load("@rules_license//rules:providers.bzl", "PackageInfo")

TransitivePkgInfo = provider(doc = "FIXME", fields = {
    # "direct": "a PackageInfo",
    #" transitive": "depset of TransitivePackageInfo, workaround for rules_license??",
    "pkgs": "TODO",
})

def _licenses_aspect_impl(target, ctx):
    direct = []
    transitive = []

    # direct
    if PackageInfo in target:
        #direct = [TransitivePkgInfo(pkgs = depset(target[PackageInfo])]
        direct = [target[PackageInfo]]

    # Propagate transitives
    for a in ["srcs", "data", "deps"]:
        for s in getattr(ctx.rule.attr, a, []):
            if TransitivePkgInfo in s:
                transitive.append(s[TransitivePkgInfo].pkgs)

    return [
        TransitivePkgInfo(pkgs = depset(direct = direct, transitive = transitive)),
    ]

gather_licenses = aspect(
    implementation = _licenses_aspect_impl,
    attr_aspects = ["deps", "data", "srcs"],
    attrs = {},
    provides = [TransitivePkgInfo],
)

# TODO: we can marshal to more formats
def _cyclonedx_impl(target, ctx):
    pass

def _spdx_impl(target, ctx):
    out = ctx.actions.declare_file("spdx")
    doc = {
        "spdxVersion": "SPDX-2.3",
        "dataLicense": "CC0-1.0",
        "SPDXID": "SPDXRef-DOCUMENT",
        "packages": [],
        #   "name": "simple@1.0.0",
        #   "documentNamespace": "http://spdx.org/spdxdocs/simple-1.0.0-bf81090e-8bbc-459d-bec9-abeb794e096a",
        #   "creationInfo": {
        #     "created": "2023-09-01T00:00:00.001Z",
        #     "creators": ["Tool: npm/cli-10.1.0"]
        #   },
        #   "documentDescribes": ["SPDXRef-Package-simple-1.0.0"],
    }

    for t in target[TransitivePkgInfo].pkgs.to_list():
        doc["packages"].append({
            "name": t.package_name,
            "versionInfo": t.package_version,
            "documentNamespace": "http://spdx.org/spdxdocs/simple-1.0.0-bf81090e-8bbc-459d-bec9-abeb794e096a",
            "creationInfo": {
                # FIXME: date
                "created": "2023-09-01T00:00:00.001Z",
                "creators": ["Tool: bazelbuild/rules_license"],
            },
            # const toSpdxID = (node) => {
            #     let name = node.packageName

            #     // Strip leading @ for scoped packages
            #     name = name.replace(/^@/, '')

            #     // Replace slashes with dots
            #     name = name.replace(/\//g, '.')

            #     return `SPDXRef-Package-${name}-${node.version}`
            # }
            "SPDXID": "SPDXRef-Package-{}-{}".format(
                (t.package_name[1:] if t.package_name.startswith("@") else t.package_name).replace("/", "."),
                t.package_version,
            ),
            # TODO:
            # "licenseDeclared": t.license
            # node.package?.license || NO_ASSERTION,
        })

    #   "packages": [
    #     {
    #
    #       "packageFileName": "",
    #       "description": "simple react app",
    #       "primaryPackagePurpose": "LIBRARY",
    #       "downloadLocation": "NOASSERTION",
    #       "filesAnalyzed": false,
    #       "homepage": "NOASSERTION",
    #       "licenseDeclared": "MIT",
    #       "externalRefs": [
    #         {
    #           "referenceCategory": "PACKAGE-MANAGER",
    #           "referenceType": "purl",
    #           "referenceLocator": "pkg:npm/simple@1.0.0"
    #         }
    #       ]
    #     },

    ctx.actions.write(out, json.encode(doc))
    return [OutputGroupInfo(spdx = depset([out]))]

# Top-level that doesn't walk the graph
spdx = aspect(
    implementation = _spdx_impl,
    attrs = {},
    required_aspect_providers = [TransitivePkgInfo],
)

def _spdx_merge_impl(ctx):
    input = ctx.attr.target[OutputGroupInfo].spdx.to_list()[0]
    output = ctx.actions.declare_file("fixed.spdx")
    ctx.actions.run(
        executable = ctx.files._merger[0],
        arguments = [input.path, output.path],
        inputs = [input],
        outputs = [output],
    )
    return [DefaultInfo(files = depset([output]))]

spdx_merge = rule(
    implementation = _spdx_merge_impl,
    attrs = {
        "_merger": attr.label(executable = True, cfg = "exec", default = "//next.js:merger"),
        "target": attr.label(
            aspects = [gather_licenses, spdx],
        ),
        "my_database": attr.label(allow_single_file = True),
    },
)
