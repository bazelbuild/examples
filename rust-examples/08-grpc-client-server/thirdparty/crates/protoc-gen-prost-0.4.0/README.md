# protoc-gen-prost

A `protoc` plugin that generates code using the _[Prost!]_ code generation engine.

[Prost!]: https://github.com/tokio-rs/prost

When used in projects that use only Rust code, the preferred mechanism for
generating protobuf definitions with _Prost!_ is to use [`prost-build`] from
within a `build.rs` file. However, when working in polyglot environments,
it can be advantageous to utilize common tooling in the Protocol Buffers
ecosystem. One common tool used for this purpose is _[buf]_, which simplifies
the code generation process and includes several useful features, including
linting, package management, and breaking change detection.

[`prost-build`]: https://docs.rs/prost-build
[buf]: https://buf.build

## Usage

Ensure that `protoc-gen-prost` has been installed within a directory on your
`$PATH`. Then invoke `protoc` from the command line as follows:

```shell
protoc --prost_out=proto/gen -I proto proto/greeter/v1/greeter.proto
```

### Options

This tool supports all the same options from `prost-build`. For more
information on the effects of these settings, see the related documentation
from that crate:

* `btree_map=<proto_path>`: [btree_map](https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.btree_map)
* `bytes=<proto_path>`: [bytes](https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.bytes)
* `default_package_filename=<value>`: [default_package_filename](https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.default_package_filename)
* `disable_comments=<proto_path>`: [disable_comments](https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.disable_comments)
* `extern_path=<proto_path>=<rust_path>`: [extern_path](https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.extern_path)
* `compile_well_known_types(=<boolean>)`: [compile_well_known_types](https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.compile_well_known_types)
* `retain_enum_prefix(=<boolean>)`: [retain_enum_prefix](https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.retain_enum_prefix)
* `field_attribute=<proto_path>=<attribute>`: [field_attribute](https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.field_attribute)
* `type_attribute=<proto_path>=<attribute>`: [type_attribute](https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.type_attribute)
* `enable_type_names(=<boolean>)`: [enable_type_names](https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.enable_type_names)

In addition, the following options can also be specified:

* `file_descriptor_set(=<boolean>)`: Includes the encoded `FileDescriptorSet`
  in the generated output for each module. Note that this behavior is
  different from `prost-build` in that each module only includes the
  file descriptors for that module. This allows for better scoping of
  information when passing to a protobuf reflection API, and less
  exposure of useless or excess information. In addition, this module
  embeds the raw file descriptors without having first decoded them with
  _Prost!_, ensuring that extensions and unexpected tags are preserved.

A note on parameter values:

* `<attribute>`: All `,`s appearing in the value must be `\` escaped
  (i.e. `\,`) This is due to the fact that internally, `protoc` joins all
  passed parameters with a `,` before sending it as a single string to the
  underlying plugin.
* `<proto_path>`: Protobuf paths beginning with `.` will be matched from the
  global root (prefix matches). All other paths will be matched as suffix
  matches.
* `(=<boolean>)`: Boolean values may be specified after a parameter, but if
  not, the value is assumed to be `true` by virtue of having listed the
  parameter.

### Usage with _buf_

When used with _buf_, options can be specified in the `buf.gen.yaml` file:

```yaml
version: v1
plugins:
  - plugin: prost
    out: gen
    opt:
      - bytes=.
      - compile_well_known_types
      - extern_path=.google.protobuf=::pbjson_types
      - file_descriptor_set
      - type_attribute=.helloworld.v1.HelloWorld=#[derive(Eq\, Hash)]
```

The `protoc-gen-prost` plugin is also published on the Buf Schema Registry as
a plugin which you can execute remotely, without needing to explicitly install
this tool. See the [plugin listing][1] to identify the latest published version
for use. The plugin is referenced as follows:

[1]: https://buf.build/community/neoeinstein-prost

```yaml
version: v1
plugins:
  - plugin: buf.build/community/neoeinstein-prost:v0.2.3
    out: gen
```

If an include file or generated crate is desired, then that should be run
as a distinct step, as in the following example. For more information, see
the `protoc-gen-prost-crate` plugin.

```yaml
version: v1
plugins:
  - plugin: prost
    out: gen/src
    opt:
      - bytes=.
      - file_descriptor_set
  - plugin: prost-crate
    out: gen
    strategy: all
    opt:
      - gen_crate=Cargo.toml.tpl
```

## Extensions

When building output, `protoc-gen-prost` adds insertion points inside modules
to make it easy to add more trait implementations. These insertion points
are placed in each module and in the include file, if one was generated.
Output module files are named based on the untransformed protobuf package
name. Thus a package named `helloworld.abstract.v1` will have an output
filename of `helloworld.abstract.v1.rs`.

Within module files (`<proto_package>.rs`):

* `module`: Appends to the end of the module file

Within the include file:

* `<proto_package>`: Appends to the module defined for this package

Here is an example for _buf_ using the `protoc-gen-prost-serde` plugin:

```yaml
version: v1
plugins:
  - plugin: prost
    out: gen/src
    opt:
      - bytes=.
      - file_descriptor_set
  - plugin: prost-serde
    out: gen/src
  - plugin: prost-crate
    out: gen
    strategy: all
    opt:
      - gen_crate=Cargo.toml.tpl
```
