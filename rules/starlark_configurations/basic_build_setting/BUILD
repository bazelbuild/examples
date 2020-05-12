load(":defs.bzl", "temperature", "breakfast")

### USAGE ###
# $ bazel build //:today => will print "HOT"
# $ bazel build //:today --//:coffee-temp=ICED => will print "ICED"

breakfast(name = 'today')

temperature(name = 'coffee-temp', build_setting_default = "HOT")
