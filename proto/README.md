# #NeverCompileProtocAgain

`protoc` has always been distributed as pre-built binaries on https://github.com/protocolbuffers/protobuf/releases

Now rules_proto can fetch that binary rather than re-build it!

## Sadness

You're probably used to watching `protoc` compile. FREQUENTLY...

`--incompatible_enable_proto_toolchain_resolution=false` is the default for this new flag
which was introduced in Bazel 7:

```
$ OUTPUT_BASE=$(mktemp -d)
$ time bazel --output_base=$OUTPUT_BASE build --incompatible_enable_proto_toolchain_resolution=false :foo_proto
Starting local Bazel server and connecting to it...
In file included from /usr/include/string.h:535,
                 from external/protobuf~3.19.6/src/google/protobuf/stubs/port.h:39,
                 from external/protobuf~3.19.6/src/google/protobuf/stubs/common.h:48,
                 from external/protobuf~3.19.6/src/google/protobuf/message_lite.h:45,
                 from external/protobuf~3.19.6/src/google/protobuf/message_lite.cc:36:
In function 'void* memcpy(void*, const void*, size_t)',
    inlined from 'uint8_t* google::protobuf::io::EpsCopyOutputStream::WriteRaw(const void*, int, uint8_t*)' at external/protobuf~3.19.6/src/google/protobuf/io/coded_stream.h:706:16,
    inlined from 'virtual uint8_t* google::protobuf::internal::ImplicitWeakMessage::_InternalSerialize(uint8_t*, google::protobuf::io::EpsCopyOutputStream*) const' at external/protobuf~3.19.6/src/google/protobuf/implicit_weak_message.h:84:28,
    inlined from 'bool google::protobuf::MessageLite::SerializePartialToZeroCopyStream(google::protobuf::io::ZeroCopyOutputStream*) const' at external/protobuf~3.19.6/src/google/protobuf/message_lite.cc:412:30:
/usr/include/x86_64-linux-gnu/bits/string_fortified.h:29:33: warning: 'void* __builtin___memcpy_chk(void*, const void*, long unsigned int, long unsigned int)' specified size between 18446744071562067968 and 18446744073709551615 exceeds maximum object size 9223372036854775807 [-Wstringop-overflow=]
   29 |   return __builtin___memcpy_chk (__dest, __src, __len,
      |          ~~~~~~~~~~~~~~~~~~~~~~~^~~~~~~~~~~~~~~~~~~~~~
   30 |                                  __glibc_objsize0 (__dest));
      |                                  ~~~~~~~~~~~~~~~~~~~~~~~~~~
INFO: Found 1 target...
Target //:foo up-to-date:
  bazel-bin/foo-descriptor-set.proto.bin
INFO: Elapsed time: 19.616s, Critical Path: 7.67s
INFO: 191 processes: 5 internal, 186 linux-sandbox.
INFO: Build completed successfully, 191 total actions

real    0m19.682s
user    0m0.008s
sys     0m0.019s

$ du --max-depth=0 --human $OUTPUT_BASE
118M    /tmp/tmp.AgrSRIJxUn
```

## With `incompatible_enable_proto_toolchain_resolution`

```
$ OUTPUT_BASE=$(mktemp -d)
$ time bazel --output_base=$OUTPUT_BASE build --incompatible_enable_proto_toolchain_resolution :foo_proto
Starting local Bazel server and connecting to it...
Target //:foo up-to-date:
  bazel-bin/foo-descriptor-set.proto.bin
INFO: Elapsed time: 2.469s, Critical Path: 0.05s
INFO: 2 processes: 1 internal, 1 linux-sandbox.
INFO: Build completed successfully, 2 total actions

real    0m2.711s
user    0m0.001s
sys     0m0.014s

$ du --max-depth=0 --human $OUTPUT_BASE
24M     /tmp/tmp.FJBFOfpOIO
```

We ran two actions instead of 191, giving us a 7x speedup and using 20% as much disk space.

## Python

Let's see how `py_proto_library` works. In the current world, you end up building `protoc`
again because this line:
https://github.com/bazelbuild/rules_python/blob/0.31.0/python/private/proto/BUILD.bazel#L42

triggers this dependency path
```
bazel query --output=label_kind 'somepath(@com_google_protobuf//:protobuf_python,
 @com_google_protobuf//:protoc)'
py_library rule @com_google_protobuf//:protobuf_python
py_library rule @com_google_protobuf//:well_known_types_py_pb2
generated file @com_google_protobuf//:python/google/protobuf/any_pb2.py
proto_gen rule @com_google_protobuf//:well_known_types_py_pb2_genproto
cc_binary rule @com_google_protobuf//:protoc
```

Gross! The fix here is more interesting: what do protobuf users do outside of Bazel?
They get the runtime from their package manager of course!
It's nicely published at https://pypi.org/project/protobuf/#history

So, this example fetches that package (as a wheel file) and hand that to the `runtime`
attribute of our `proto_lang_toolchain`.

> Note, we use aspect_rules_py to do that, so that we don't need any requirements files, no calls to `pip` or repository rule like `pip_parse`, and no Python interpreter needed during the build. You could use the standard rules_python incantations if you prefer...

Looking at timing again:

```
$ OUTPUT_BASE=$(mktemp -d); time RULES_PYTHON_ENABLE_PYSTAR=0 bazel --output_base=$OUTPU
T_BASE build --incompatible_enable_proto_toolchain_resolution :foo_py_proto
INFO: Analyzed target //:foo_py_proto (83 packages loaded, 5573 targets configured).
Target //:foo_py_proto up-to-date:
  bazel-bin/foo_pb2.py
INFO: Elapsed time: 3.912s, Critical Path: 0.14s
INFO: 3 processes: 1 internal, 2 linux-sandbox.
INFO: Build completed successfully, 3 total actions

real    0m3.948s
user    0m0.003s
sys     0m0.012s
```

Nice, still fast and only one more action required to generate the Python proto stubs.

## Java

This follows the same methodology as for Python, above.

We fetch the protobuf-java package from Maven central in the WORKSPACE.bazel file.
It's passed as the runtime to the proto_lang_toolchain, and also as a dep of the Java code.

```
% bazel run java
INFO: Running command line: bazel-bin/java/java
msg: "Hello World!"
```
