# Generating code

This example shows how you can write a rule that generates code that 
gets consumed by another rule.

The overall concept is that:
- your code generator has one or more inputs
- which produce one or more outputs in some source language
- which are consumed by a `cc_library` (or other language) rule

This example will take a text file as input to create some
sources in C++ and Python. Those are, in turn, used to create
`cc_library` and `py_library` targets that others can consume.

Try it:
```
bazel run :p_maybe
bazel run :c_maybe
```
It should print 'MAYBE is 3'.
