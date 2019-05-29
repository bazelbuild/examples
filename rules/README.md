# Bazel Rules

This directory contains examples of Bazel rules. For additional information,
please refer to [the documentation](https://docs.bazel.build/versions/master/skylark/concepts.html).

## Getting started

These examples cover the most basic features: creating a rule, creating actions,
passing information from a dependency to a target.

* [empty](empty/): Minimal example to show the creation of a rule.

* [attributes](attributes/): Example of a rule with attributes.

* [actions.run](actions_run/): Example where a binary target is used as implicit
  dependency and executed by the rule.

* [actions.write](actions_write/): Example where a file is generated.

* [expand_template](expand_template/): Example where a file is generated based
  on a template.

* [shell command](shell_command/): Example where a shell command is executed to
  do simple text file processing.

* [mandatory provider](mandatory_provider/): Example with a mandatory provider,
  to access information from a dependency.

* [optional provider](optional_provider/): Example with an optional provider,
  to access information from a dependency.

* [depsets](depsets/): Example of a using a depset to gather transitive
  information. Each target collects data from its dependencies.

## Additional examples

These examples explore more specific scenarios. They allow you to have more
control on the behavior of the rules.

* [executable](executable/): Example of an executable rule.

* [test rule](test_rule/): Example of a test rule.

* [runfiles](runfiles/): Example of an executable rule with runfiles (files
  required at runtime).

* [computed dependencies](computed_dependencies/): Example with computed
  dependencies. The set of implicit dependencies depends on the rule attributes.

* [predeclared outputs](predeclared_outputs/): Example with `attr.output_list`
  and `outputs`.

* [implicit output](implicit_output/): Example with implicit outputs. Some
  outputs are available only when explicitly requested.

* [aspect](aspect/): Example of using an aspect to collect information from
  other rules.
