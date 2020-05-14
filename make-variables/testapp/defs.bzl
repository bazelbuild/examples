def _var_providing_rule_impl(ctx):
  return [
      platform_common.TemplateVariableInfo({
          "FOO": ctx.attr.var_value,
      }),
  ]

var_providing_rule = rule(
    implementation = _var_providing_rule_impl,
    attrs = { "var_value": attr.string() }
)

