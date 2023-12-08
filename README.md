# Bazel examples and tutorials

A collection of examples of how to use the Bazel build system.

## Introductory tutorials

Some tutorials under <https://bazel.build/start> point to code in this repository:

 * [C++ basics](/cpp-tutorial)
   <br/>Working examples for the [C++ Bazel Tutorial](https://bazel.build/start/cpp)
 * [Java basics](/java-tutorial)
   <br/>Working examples for the [Java Bazel Tutorial](https://bazel.build/start/java)
 * [Using query](/query-quickstart)
   <br/>Working examples for [The Query quickstart](https://bazel.build/query/quickstart)

Note that tutorials for other languages may be found under other repositories:

 * [iOS tutorial](https://github.com/bazelbuild/rules_apple/blob/master/doc/tutorials/ios-app.md)
 * [Go tutorial](https://bazel-contrib.github.io/SIG-rules-authors/go-tutorial.html) along with
   [sources](https://github.com/bazelbuild/rules_go/tree/master/examples/basic-gazelle)

 ## Example cookbook

 ### General

 * [Using bzlmod](/bzlmod)
 * [How to generate source code from a rule and include it in a library](/rules/generating_code)

 ### [Android](/tree/main/android)

 * [Using Android Jetpack Compose](/firebase-cloud-messaging)
 * [Using Firebase Cloud Messaging](/jetpack-compose)
 * [Using the NDK](/android/ndk)
 * [Using Roboelectric tests and Kotlin](/android/robolectric-testing)

 ### Java

 * [Using Java with Maven](/java-maven)

 ### Frontend

 * [Next.js](/frontend/next.js)
 * [react](/frontend/react)
 * [react-webpack](/frontend/react-webpack)
 * [vue](/frontend/vue)

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
