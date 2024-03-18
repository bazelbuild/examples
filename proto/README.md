# #NeverCompileProtocAgain

`protoc` has always been distributed as pre-built binaries on https://github.com/protocolbuffers/protobuf/releases

Now rules_proto can fetch that binary rather than re-build it!

## Sadness

```
$ time bazel --output_base=$(mktemp -d) build :foo
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
INFO: Elapsed time: 17.336s, Critical Path: 7.23s
INFO: 192 processes: 5 internal, 187 linux-sandbox.
INFO: Build completed successfully, 192 total actions

real    0m17.797s
user    0m0.015s
sys     0m0.031s
```

## With `incompatible_enable_proto_toolchain_resolution`

```
$ time bazel --output_base=$(mktemp -d) build --incompatible_enable_proto_toolchain_resolution :foo
Starting local Bazel server and connecting to it...
Target //:foo up-to-date:
  bazel-bin/foo-descriptor-set.proto.bin
INFO: Elapsed time: 2.469s, Critical Path: 0.05s
INFO: 2 processes: 1 internal, 1 linux-sandbox.
INFO: Build completed successfully, 2 total actions

real    0m2.973s
user    0m0.018s
sys     0m0.026s
```
