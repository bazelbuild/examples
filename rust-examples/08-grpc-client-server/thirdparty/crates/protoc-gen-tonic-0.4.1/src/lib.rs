#![doc = include_str!("../README.md")]

use std::str;

use prost::Message;
use prost_types::compiler::CodeGeneratorRequest;
use protoc_gen_prost::{Generator, InvalidParameter, ModuleRequestSet, Param, Params};
use tonic_build::Attributes;

use self::{generator::TonicGenerator, resolver::Resolver};

mod generator;
mod resolver;
mod util;

/// Execute the core _Prost!_ generator from a raw [`CodeGeneratorRequest`]
pub fn execute(raw_request: &[u8]) -> protoc_gen_prost::Result {
    let request = CodeGeneratorRequest::decode(raw_request)?;
    let params = request.parameter().parse::<Parameters>()?;

    let module_request_set = ModuleRequestSet::new(
        request.file_to_generate,
        request.proto_file,
        raw_request,
        params.default_package_filename.as_deref(),
    )?;

    let resolver = Resolver::new(params.extern_path, params.compile_well_known_types);
    let mut generator = TonicGenerator {
        resolver,
        generate_server: !params.no_server,
        generate_client: !params.no_client,
        generate_transport: !params.no_transport,
        server_attributes: params.server_attributes,
        client_attributes: params.client_attributes,
        emit_package: !params.disable_package_emission,
        insert_include: !params.no_include,
    };

    let files = generator.generate(&module_request_set)?;

    Ok(files)
}

/// Parameters use to configure [`Generator`]s built into `protoc-gen-prost-serde`
///
/// [`Generator`]: protoc_gen_prost::generators::Generator
#[derive(Debug, Default)]
struct Parameters {
    default_package_filename: Option<String>,
    extern_path: Vec<(String, String)>,
    server_attributes: Attributes,
    client_attributes: Attributes,
    compile_well_known_types: bool,
    disable_package_emission: bool,
    no_server: bool,
    no_client: bool,
    no_transport: bool,
    no_include: bool,
}

impl str::FromStr for Parameters {
    type Err = InvalidParameter;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret_val = Self::default();
        for param in Params::from_protoc_plugin_opts(s)? {
            match param {
                Param::Parameter {
                    param: "default_package_filename",
                }
                | Param::Value {
                    param: "default_package_filename",
                    ..
                } => ret_val.default_package_filename = param.value().map(|s| s.to_string()),
                Param::KeyValue {
                    param: "extern_path",
                    key: prefix,
                    value: module,
                } => ret_val.extern_path.push((prefix.to_string(), module)),
                Param::Parameter {
                    param: "compile_well_known_types",
                }
                | Param::Value {
                    param: "compile_well_known_types",
                    value: "true",
                } => ret_val.compile_well_known_types = true,
                Param::Value {
                    param: "compile_well_known_types",
                    value: "false",
                } => (),
                Param::Parameter {
                    param: "disable_package_emission",
                }
                | Param::Value {
                    param: "disable_package_emission",
                    value: "true",
                } => ret_val.disable_package_emission = true,
                Param::Value {
                    param: "disable_package_emission",
                    value: "false",
                } => (),
                Param::Parameter { param: "no_server" }
                | Param::Value {
                    param: "no_server",
                    value: "true",
                } => ret_val.no_server = true,
                Param::Value {
                    param: "no_server",
                    value: "false",
                } => (),
                Param::Parameter { param: "no_client" }
                | Param::Value {
                    param: "no_client",
                    value: "true",
                } => ret_val.no_client = true,
                Param::Value {
                    param: "no_client",
                    value: "false",
                } => (),
                Param::Parameter {
                    param: "no_transport",
                }
                | Param::Value {
                    param: "no_transport",
                    value: "true",
                } => ret_val.no_transport = true,
                Param::Value {
                    param: "no_transport",
                    value: "false",
                } => (),
                Param::Parameter {
                    param: "no_include",
                }
                | Param::Value {
                    param: "no_include",
                    value: "true",
                } => ret_val.no_include = true,
                Param::Value {
                    param: "no_include",
                    value: "false",
                } => (),
                Param::KeyValue {
                    param: "client_mod_attribute",
                    key: prefix,
                    value: attribute,
                } => ret_val
                    .client_attributes
                    .push_mod(prefix, attribute.replace(r"\,", ",").replace(r"\\", r"\")),
                Param::KeyValue {
                    param: "client_attribute",
                    key: prefix,
                    value: attribute,
                } => ret_val
                    .client_attributes
                    .push_struct(prefix, attribute.replace(r"\,", ",").replace(r"\\", r"\")),
                Param::KeyValue {
                    param: "server_mod_attribute",
                    key: prefix,
                    value: attribute,
                } => ret_val
                    .server_attributes
                    .push_mod(prefix, attribute.replace(r"\,", ",").replace(r"\\", r"\")),
                Param::KeyValue {
                    param: "server_attribute",
                    key: prefix,
                    value: attribute,
                } => ret_val
                    .server_attributes
                    .push_struct(prefix, attribute.replace(r"\,", ",").replace(r"\\", r"\")),
                _ => return Err(InvalidParameter::from(param)),
            }
        }

        Ok(ret_val)
    }
}
