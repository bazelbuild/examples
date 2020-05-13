load(":defs.bzl", "temperature", "breakfast")

### USAGE ###
# $ bazel build //:today => will print "HOT"
# $ bazel build //:today --//:coffee-temp=ICED => will print "ICED"

breakfast(name = 'today')

# This is a string-typed build settings. Here we set its
# default value to "HOT".
temperature(name = 'coffee-temp', build_setting_default = "HOT")
