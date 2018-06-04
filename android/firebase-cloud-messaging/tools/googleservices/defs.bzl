
def google_services_xml(packageName, google_services_json):
  """Creates Android resource XML for Google services (e.g. FCM) from a
  google-services.json file.

  Args:
    packageName: The package name (or application ID) of the Android app.
    google_services_json: The google-services.json file.

  Returns:
    A list of the generated resource files which can be used with
    android_binary.resource_files or android_library.resource_files.
  """
  # Adding the package name and google-services.json file to the outs and name
  # of the rule is necessary in case there are multiple calls to
  # google_services_xml() with different package names or different json files.
  outs = ["google_services_xml/%s/%s/res/values/values.xml" %
      (packageName, google_services_json.replace("/", "_"))]
  name = "gen_google_services_xml_%s_%s" % (
      packageName.replace(".", "_"),
      google_services_json.replace(".", "_").replace("/", "_"))
  if not native.existing_rule(name):
    native.genrule(
      name = name,
      srcs = [google_services_json],
      outs = outs,
      tools = ["//tools/googleservices:GenerateFirebaseXml"],
      cmd = "$(location //tools/googleservices:GenerateFirebaseXml) %s $< $@" % packageName,
    )
  return outs
