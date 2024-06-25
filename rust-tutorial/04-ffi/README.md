# Rust FFI

In case of an existing C++, Rust can call into the C++ function via FFI.
With Bazel, this is straightforward. However, your C++ API needs an extern "C" 
declaration to generate the C compatibility required for FFI.

## Setup

The setup is twofold, the Rust rules are declared in the MODULE file, 
but the rules_cc are not yet available in the Bazelmod format and thus are declared in 
the WORKSPACE.bzlmod file.

In your MODULE.bazel file, ensure to have the following entry:

```starlark
module(
    name = "ffi",
    version = "0.0.0"
)
###############################################################################
# B A Z E L  C E N T R A L  R E G I S T R Y # https://registry.bazel.build/
###############################################################################
# https://github.com/bazelbuild/rules_rust/releases
bazel_dep(name = "rules_rust", version = "0.46.0")

###############################################################################
# T O O L C H A I N S
###############################################################################
# Rust toolchain
RUST_EDITION = "2021"
RUST_VERSION = "1.79.0"

rust = use_extension("@rules_rust//rust:extensions.bzl", "rust")
rust.toolchain(
    edition = RUST_EDITION,
    versions = [RUST_VERSION],
)
use_repo(rust, "rust_toolchains")
register_toolchains("@rust_toolchains//:all")
```

Then, create or open the  WORKSPACE.bzlmod file and add the CC rules:

```starlark
###############################################################################
# Bzlmod and WORKSPACE can work side by side, which allows migrating dependencies
# from the WORKSPACE file to Bzlmod to be a gradual process.
# https://bazel.build/external/migration#hybrid-mode
###############################################################################
# rule http_archive
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# rules_cc
# https://github.com/bazelbuild/rules_cc/releases
http_archive(
    name = "rules_cc",
    urls = ["https://github.com/bazelbuild/rules_cc/releases/download/0.0.10-rc1/rules_cc-0.0.10-rc1.tar.gz"],
    sha256 = "d75a040c32954da0d308d3f2ea2ba735490f49b3a7aa3e4b40259ca4b814f825",
)
```


## C++ Target

Assuming you have a C++ library that defines a simple func() you declare it as a regular CC library in the BUILD file:

```starlark
load("@rules_cc//cc:defs.bzl", "cc_import", "cc_library")

cc_library(
    name = "nonstandard_name_cc_lib",
    srcs = ["c/cc_library.cc"],
)
```

In some cases, you have to deal with non standard naming. In that case you define a 
custom gen_rule to take of that and then define a cc_import.  

```starlark
load("@rules_cc//cc:defs.bzl", "cc_import", "cc_library")

genrule(
    name = "nonstandard_name_gen",
    srcs = [":nonstandard_name_cc_lib"],
    outs = ["nonstandard_name_gen.a"],
    # Copy the first member (libnonstandard_name_cc_lib.a) from the srcs to the
    # output nonstandard_name_gen.a.
    cmd = "cp $$(awk '{print $$1}' <<< '$(SRCS)') $@",
)

cc_import(
    name = "static_cclib",
    static_library = "nonstandard_name_gen.a",
)
```

## Rust Callsite 

On the Rust side, interestingly, you just declare the cc_import as a dependency of
your Rust target. 

```starlark
load("@rules_rust//rust:defs.bzl", "rust_shared_library")

# A rust_shared_library (forcing the use of pic) that depends on a native
# linker library with only a static_library member.
rust_shared_library(
    name = "rust_shared_lib_with_static_dep",
    srcs = ["src/rust_shared_lib_with_static_dep.rs"],
    deps = [":static_cclib"],
)
```

Then in your Rust source file, your create a FFI binding and wrap the call to it into unsafe. You can do that because the Rust standard library provides all the c raw types for FFI so you just import them and unsafe informs the Rust borrow checker to hold off certain checks. The public Rust function f() can then be used in regular Rust code. 

```rust
use std::os::raw::c_int;

extern "C" {
    pub fn func() -> c_int;
}

pub fn f() {
    println!("hi {}",
             unsafe {
                 func()
             }
    );
}
```

And with that, you build your FFI target as usual:

`bazel build //...`



