# Copyright 2018 The Bazel Authors. All rights reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    http:#www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

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
