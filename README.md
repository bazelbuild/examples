# Bazel examples and tutorials

A collection of examples of how to use the Bazel build system.

## Introductory tutorials

* [C++ basics](/tree/main/cpp-tutorial)
* [Java basics](/tree/main/java-tutorial)
* [End to End](/tree/main/tutorial)
  <br/>A full end-to-end system with a backend, an Android app, and an iOS app.
* [iOS basics](/tree/main/tutorial/ios-app)
* [Using query](/tree/main/query-quickstart)
  <br/>Working examples for [The Query quickstart](https://bazel.build/query/quickstart)

## Example cookbook

### General

* [Using bzlmod](/tree/main/bzlmod)
* [How to generate source code from a rule and include it in a library](/tree/main/rules/generating_code)

### [Android](/tree/main/android)

* [Using Android Jetpack Compose](/tree/main/firebase-cloud-messaging)
* [Using Firebase Cloud Messaging](/tree/main/jetpack-compose)
* [Using the NDK](/tree/main/android/ndk)
* [Using Roboelectic tests and Kotlin](/tree/main/android/robolectric-testing)

### Java

* [Using Java with Maven](/tree/main/java-maven)

### [Rule writing](/tree/main/rules)

* [Accessing attributes of a rule](/tree/main/rules/attributes)
* [A rule with both explict and implicit outputs](/tree/main/rules/implicit_output)
* [Creating a simple `*_test` rule](/tree/main/rules/test_rule)
* [Rules that change the build flags](/tree/main/configurations)
* [Rules with implicit dependencies](/tree/main/rules/computed_dependencies)
* [Using a macro wrapper to compute an output file name](/tree/main/rules/optional_provider)
* [Using aspects](/tree/main/rules/aspect)
* [Using ctx.actions.expand_template](/tree/main/rules/expand_template)
* [Using ctx.actions.run_shell to wrap simple commands](/tree/main/rules/shell_command)
* [Using ctx.actions.run to run a tool](/tree/main/rules/actions_run)
* [Using ctx.actions.write to create a file at analysis time](/tree/main/rules/actions_write)
* [Using depsets](/tree/main/rules/depsets)
* [Using "Make" variables in your rules](/tree/main/make-variables)
* [Using mandatory providers to ensure your dependencies are of the right type](/tree/main/rules/mandatory_provider)
* [Using runfiles](/tree/main/rules/runfiles)


CI:
[![Build status](https://badge.buildkite.com/260bbace6a4067a3c60539a31fed1191d341a24cb0bfeb0e23.svg)](https://buildkite.com/bazel/bazel-bazel-examples)
