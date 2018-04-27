"""Rule with an optional provider.

In this example, rules have a number attribute. Each rule adds its number
with the numbers of its transitive dependencies, and write the result in a
file. This shows how to transfer information from a dependency to its
dependents. Dependencies are not required to provide a number.
"""

NumberInfo = provider("number")

def _sum_impl(ctx):
  inputs = []
  if ctx.attr.number:
    inputs.append(ctx.file.number)
  for dep in ctx.attr.deps:
    # What do I do here to detect that the provided number is not a single
    # file, but instead a directory of files?
    # How do I look at the list of files that exist in this directory to filter
    # out the things that are not appropriate for this rule to digest?
    inputs.append(dep[NumberInfo].number)

  input_paths = [i.path for i in inputs]

  ctx.actions.run_shell(
      command = "cat {input_paths} | paste -s -d+ - | bc > {out}".format(
          input_paths=" ".join(input_paths),
          out=ctx.outputs.out.path
      ),
      inputs=inputs,
      outputs=[ctx.outputs.out],
  )

  # Return the provider with result, visible to other rules.
  return [NumberInfo(number=ctx.outputs.out)]

def _generate_sums_impl(ctx):
  d = ctx.actions.declare_directory("dir")

  ctx.actions.run_shell(
      command = """
      mkdir {path}
      echo 1 > {path}/1.txt
      echo 2 > {path}/2.txt
      """.format(path=d.path),
      outputs=[d],
  )

  return [NumberInfo(number=d)]

sum = rule(
    attrs = {
        "number": attr.label(
            allow_single_file = True,
        ),
        "deps": attr.label_list(),
    },
    outputs = {"out": "%{name}.sum"},
    implementation = _sum_impl,
)

generate_sums = rule(
    attrs = {},
    implementation = _generate_sums_impl,
)
