# Bazel examples and tutorials

A collection of examples of how to use the Bazel build system.

## Introductory tutorials

 * [C++ basics](/cpp-tutorial)
 * [Java basics](/java-tutorial)
 * [End to End](/tutorial)
   <br/>A full end-to-end system with a backend, an Android app, and an iOS app.
 * [iOS basics](/tutorial/ios-app)
 * [Using query](/query-quickstart)
   <br/>Working examples for [The Query quickstart](https://bazel.build/query/quickstart)

 ## Example cookbook

 ### General

 * [Using bzlmod](/bzlmod)
 * [How to generate source code from a rule and include it in a library](/rules/generating_code)

 ### [Android](/tree/main/android)

 * [Using Android Jetpack Compose](/firebase-cloud-messaging)
 * [Using Firebase Cloud Messaging](/jetpack-compose)
 * [Using the NDK](/android/ndk)
 * [Using Roboelectic tests and Kotlin](/android/robolectric-testing)

 ### Java

 * [Using Java with Maven](/java-maven)

 ### [Rule writing](/rules)

 * [Accessing attributes of a rule](/rules/attributes)
 * [A rule with both explict and implicit outputs](/rules/implicit_output)
 * [Creating a simple `*_test` rule](/rules/test_rule)
 * [Rules that change the build flags](/configurations)
 * [Rules with implicit dependencies](/rules/computed_dependencies)
 * [Using a macro wrapper to compute an output file name](/rules/optional_provider)
 * [Using aspects](/rules/aspect)
 * [Using ctx.actions.expand_template](/rules/expand_template)
 * [Using ctx.actions.run_shell to wrap simple commands](/rules/shell_command)
 * [Using ctx.actions.run to run a tool](/rules/actions_run)
 * [Using ctx.actions.write to create a file at analysis time](/rules/actions_write)
 * [Using depsets](/rules/depsets)
 * [Using "Make" variables in your rules](/make-variables)
 * [Using mandatory providers to ensure your dependencies are of the right type](/rules/mandatory_provider)
 * [Using runfiles](/rules/runfiles)


CI:
[![Build status](https://badge.buildkite.com/260bbace6a4067a3c60539a31fed1191d341a24cb0bfeb0e23.svg)](https://buildkite.com/bazel/bazel-bazel-examples)
