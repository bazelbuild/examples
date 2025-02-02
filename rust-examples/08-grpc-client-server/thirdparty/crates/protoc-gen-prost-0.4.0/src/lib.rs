#![doc = include_str!("../README.md")]

use std::{
    borrow::Cow,
    collections::{BTreeMap, HashSet},
    fmt, str,
};

use once_cell::sync::Lazy;
use prost::Message;
use prost_build::Module;
use prost_types::{
    compiler::{code_generator_response::File, CodeGeneratorRequest},
    FileDescriptorProto,
};

use self::generator::{CoreProstGenerator, FileDescriptorSetGenerator};

mod generator;

pub use self::generator::{Error, Generator, GeneratorResultExt, Result};

/// Execute the core _Prost!_ generator from an encoded [`CodeGeneratorRequest`]
pub fn execute(raw_request: &[u8]) -> generator::Result {
    let request = CodeGeneratorRequest::decode(raw_request)?;
    let params = request.parameter().parse::<Parameters>()?;

    let module_request_set = ModuleRequestSet::new(
        request.file_to_generate,
        request.proto_file,
        raw_request,
        params.prost.default_package_filename(),
    )?;
    let file_descriptor_set_generator = params
        .file_descriptor_set
        .then_some(FileDescriptorSetGenerator);

    let files = CoreProstGenerator::new(params.prost.to_prost_config())
        .chain(file_descriptor_set_generator)
        .generate(&module_request_set)?;

    Ok(files)
}

/// A set of requests to generate code for a series of modules
pub struct ModuleRequestSet {
    requests: BTreeMap<Module, ModuleRequest>,
}

impl ModuleRequestSet {
    /// Construct a new module request set from an encoded [`CodeGeneratorRequest`]
    ///
    /// [`CodeGeneratorRequest`]: prost_types::compiler::CodeGeneratorRequest
    pub fn new<I>(
        input_protos: I,
        proto_file: Vec<FileDescriptorProto>,
        raw_request: &[u8],
        default_package_filename: Option<&str>,
    ) -> std::result::Result<Self, prost::DecodeError>
    where
        I: IntoIterator<Item = String>,
    {
        let raw_protos = RawProtos::decode(raw_request)?;

        Ok(Self::new_decoded(
            input_protos,
            proto_file,
            raw_protos,
            default_package_filename.unwrap_or("_"),
        ))
    }

    fn new_decoded<I>(
        input_protos: I,
        proto_file: Vec<FileDescriptorProto>,
        raw_protos: RawProtos,
        default_package_filename: &str,
    ) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        let input_protos: HashSet<_> = input_protos.into_iter().collect();

        let requests = proto_file.into_iter().zip(raw_protos.proto_file).fold(
            BTreeMap::new(),
            |mut acc, (proto, raw)| {
                let module = Module::from_protobuf_package_name(proto.package());
                let proto_filename = proto.name();
                let entry = acc
                    .entry(module)
                    .or_insert_with(|| ModuleRequest::new(proto.package().to_owned()));

                if entry.output_filename().is_none() && input_protos.contains(proto_filename) {
                    let filename = match proto.package() {
                        "" => default_package_filename.to_owned(),
                        package => format!("{package}.rs"),
                    };
                    entry.with_output_filename(filename);
                }

                entry.push_file_descriptor_proto(proto, raw);
                acc
            },
        );

        Self { requests }
    }

    /// An ordered iterator of all requests
    pub fn requests(&self) -> impl Iterator<Item = (&Module, &ModuleRequest)> {
        self.requests.iter()
    }

    /// Retrieve the request for the given module
    pub fn for_module(&self, module: &Module) -> Option<&ModuleRequest> {
        self.requests.get(module)
    }
}

/// A code generation request for a specific module
pub struct ModuleRequest {
    proto_package_name: String,
    output_filename: Option<String>,
    files: Vec<FileDescriptorProto>,
    raw: Vec<Vec<u8>>,
}

impl ModuleRequest {
    fn new(proto_package_name: String) -> Self {
        Self {
            proto_package_name,
            output_filename: None,
            files: Vec::new(),
            raw: Vec::new(),
        }
    }

    fn with_output_filename(&mut self, filename: String) {
        self.output_filename = Some(filename);
    }

    fn push_file_descriptor_proto(&mut self, encoded: FileDescriptorProto, raw: Vec<u8>) {
        self.files.push(encoded);
        self.raw.push(raw);
    }

    /// The protobuf package name for this module
    pub fn proto_package_name(&self) -> &str {
        &self.proto_package_name
    }

    /// The output filename for this module
    pub fn output_filename(&self) -> Option<&str> {
        self.output_filename.as_deref()
    }

    /// An iterator of the file descriptors
    pub fn files(&self) -> impl Iterator<Item = &FileDescriptorProto> {
        self.files.iter()
    }

    /// An iterator of the encoded [`FileDescriptorProto`]s from [`files()`][Self::files()]
    pub fn raw_files(&self) -> impl Iterator<Item = &[u8]> {
        self.raw.iter().map(|b| b.as_slice())
    }

    /// Creates a code generation file from the output
    pub(crate) fn write_to_file<F: FnOnce(&mut String)>(&self, f: F) -> Option<File> {
        self.output_filename.as_deref().map(|name| {
            let mut content = String::with_capacity(8_192);
            f(&mut content);

            File {
                name: Some(name.to_owned()),
                content: Some(content),
                ..Default::default()
            }
        })
    }

    /// Appends generated code to the end of the main file for this module
    ///
    /// This is generally a good way to add includes referencing the output
    /// of other plugins or to directly append to the main file.
    pub fn append_to_file<F: FnOnce(&mut String)>(&self, f: F) -> Option<File> {
        self.output_filename.as_deref().map(|name| {
            let mut content = String::new();
            f(&mut content);

            File {
                name: Some(name.to_owned()),
                content: Some(content),
                insertion_point: Some("module".to_owned()),
                ..Default::default()
            }
        })
    }
}

/// Parameters use to configure [`Generator`]s built into `protoc-gen-prost`
///
/// [`Generator`]: crate::Generator
#[derive(Debug, Default)]
struct Parameters {
    /// Prost parameters, used to generate [`prost_build::Config`]
    prost: ProstParameters,

    /// Whether a file descriptor set has been requested in each module
    file_descriptor_set: bool,
}

/// Parameters used to configure the underlying Prost generator
#[derive(Debug, Default)]
struct ProstParameters {
    btree_map: Vec<String>,
    bytes: Vec<String>,
    disable_comments: Vec<String>,
    default_package_filename: Option<String>,
    extern_path: Vec<(String, String)>,
    type_attribute: Vec<(String, String)>,
    field_attribute: Vec<(String, String)>,
    enum_attribute: Vec<(String, String)>,
    message_attribute: Vec<(String, String)>,
    compile_well_known_types: bool,
    retain_enum_prefix: bool,
    enable_type_names: bool,
}

impl ProstParameters {
    /// Builds a [`prost_build::Config`] from the parameters
    fn to_prost_config(&self) -> prost_build::Config {
        let mut config = prost_build::Config::new();
        config.btree_map(self.btree_map.iter());
        config.bytes(self.bytes.iter());
        config.disable_comments(self.disable_comments.iter());

        if let Some(filename) = self.default_package_filename.as_deref() {
            config.default_package_filename(filename);
        }

        for (proto_path, rust_path) in &self.extern_path {
            config.extern_path(proto_path, rust_path);
        }
        for (proto_path, attribute) in &self.type_attribute {
            config.type_attribute(proto_path, attribute);
        }
        for (proto_path, attribute) in &self.field_attribute {
            config.field_attribute(proto_path, attribute);
        }
        for (proto_path, attribute) in &self.enum_attribute {
            config.enum_attribute(proto_path, attribute);
        }
        for (proto_path, attribute) in &self.message_attribute {
            config.message_attribute(proto_path, attribute);
        }

        if self.compile_well_known_types {
            config.compile_well_known_types();
        }
        if self.retain_enum_prefix {
            config.retain_enum_prefix();
        }
        if self.enable_type_names {
            config.enable_type_names();
        }

        config
    }

    fn default_package_filename(&self) -> Option<&str> {
        self.default_package_filename.as_deref()
    }

    fn try_handle_parameter<'a>(&mut self, param: Param<'a>) -> std::result::Result<(), Param<'a>> {
        match param {
            Param::Value {
                param: "btree_map",
                value,
            } => self.btree_map.push(value.to_string()),
            Param::Value {
                param: "bytes",
                value,
            } => self.bytes.push(value.to_string()),
            Param::Parameter {
                param: "default_package_filename",
            }
            | Param::Value {
                param: "default_package_filename",
                ..
            } => self.default_package_filename = param.value().map(|s| s.into_owned()),
            Param::Parameter {
                param: "compile_well_known_types",
            }
            | Param::Value {
                param: "compile_well_known_types",
                value: "true",
            } => self.compile_well_known_types = true,
            Param::Value {
                param: "compile_well_known_types",
                value: "false",
            } => (),
            Param::Value {
                param: "disable_comments",
                value,
            } => self.disable_comments.push(value.to_string()),
            Param::Parameter {
                param: "retain_enum_prefix",
            }
            | Param::Value {
                param: "retain_enum_prefix",
                value: "true",
            } => self.retain_enum_prefix = true,
            Param::Value {
                param: "retain_enum_prefix",
                value: "false",
            } => (),
            Param::KeyValue {
                param: "extern_path",
                key: prefix,
                value: module,
            } => self.extern_path.push((prefix.to_string(), module)),
            Param::KeyValue {
                param: "type_attribute",
                key: prefix,
                value: module,
            } => self.type_attribute.push((
                prefix.to_string(),
                module.replace(r"\,", ",").replace(r"\\", r"\"),
            )),
            Param::KeyValue {
                param: "field_attribute",
                key: prefix,
                value: module,
            } => self.field_attribute.push((
                prefix.to_string(),
                module.replace(r"\,", ",").replace(r"\\", r"\"),
            )),
            Param::KeyValue {
                param: "enum_attribute",
                key: prefix,
                value: module,
            } => self.enum_attribute.push((
                prefix.to_string(),
                module.replace(r"\,", ",").replace(r"\\", r"\"),
            )),
            Param::KeyValue {
                param: "message_attribute",
                key: prefix,
                value: module,
            } => self.message_attribute.push((
                prefix.to_string(),
                module.replace(r"\,", ",").replace(r"\\", r"\"),
            )),
            Param::Parameter {
                param: "enable_type_names",
            }
            | Param::Value {
                param: "enable_type_names",
                value: "true",
            } => self.enable_type_names = true,
            Param::Value {
                param: "enable_type_names",
                value: "false",
            } => (),
            _ => return Err(param),
        }

        Ok(())
    }
}

/// Standard parameter regular expression
///
/// Supports the following forms:
///
/// ```text
/// parameter
/// parameter=key
/// parameter=key=value
/// ```
///
/// * `parameter` is terminated on the first `=` or `,`
/// * If `parameter` is terminated with `=`, then `key` follows, terminated by the first `=` or `,`.
/// * If `key` is terminated with `=`, then `value` follows. It is terminated only by `,`. However,
///   if that `,` is prefixed by `\` but not `\\`, then it will not terminate.
static PARAMETER: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(
        r"(?:(?P<param>[^,=]+)(?:=(?P<key>[^,=]+)(?:=(?P<value>(?:[^,\\]|\\,|\\\\)+))?)?)",
    )
    .unwrap()
});

pub struct Params<'a> {
    params: Vec<Param<'a>>,
}

impl<'a> IntoIterator for Params<'a> {
    type IntoIter = <Vec<Param<'a>> as IntoIterator>::IntoIter;
    type Item = <Vec<Param<'a>> as IntoIterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.params.into_iter()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Param<'a> {
    Parameter {
        param: &'a str,
    },
    Value {
        param: &'a str,
        value: &'a str,
    },
    KeyValue {
        param: &'a str,
        key: &'a str,
        value: String,
    },
}

impl<'a> Param<'a> {
    pub fn value(self) -> Option<Cow<'a, str>> {
        match self {
            Self::Parameter { .. } => None,
            Self::Value { value, .. } => Some(Cow::Borrowed(value)),
            Self::KeyValue { value, .. } => Some(Cow::Owned(value)),
        }
    }
}

impl From<Param<'_>> for InvalidParameter {
    fn from(param: Param<'_>) -> Self {
        let message = match param {
            Param::Parameter { param } => param.to_owned(),
            Param::Value { param, value } => format!("{param}={value}"),
            Param::KeyValue { param, key, value } => {
                let value = value.replace('\\', r"\\").replace(',', r"\,");
                format!("{param}={key}={value}")
            }
        };
        InvalidParameter(message)
    }
}

impl<'a> Params<'a> {
    pub fn from_protoc_plugin_opts(s: &'a str) -> std::result::Result<Self, InvalidParameter> {
        let params = PARAMETER
            .captures_iter(s)
            .map(|capture| {
                let param = capture
                    .get(1)
                    .expect("any captured group will at least have the param name")
                    .as_str()
                    .trim();

                let key = capture.get(2).map(|m| m.as_str());
                let value = capture.get(3).map(|m| m.as_str());

                match (key, value) {
                    (None, None) => Ok(Param::Parameter { param }),
                    (Some(value), None) => Ok(Param::Value { param, value }),
                    (Some(key), Some(value)) => Ok(Param::KeyValue {
                        param,
                        key,
                        value: value.replace(r"\,", ",").replace(r"\\", r"\"),
                    }),
                    _ => Err(InvalidParameter(
                        capture.get(0).unwrap().as_str().to_string(),
                    )),
                }
            })
            .collect::<std::result::Result<_, _>>()?;
        Ok(Self { params })
    }
}

impl str::FromStr for Parameters {
    type Err = InvalidParameter;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ret_val = Self::default();
        for param in Params::from_protoc_plugin_opts(s)? {
            if let Err(param) = ret_val.prost.try_handle_parameter(param) {
                match param {
                    Param::Parameter {
                        param: "file_descriptor_set",
                    }
                    | Param::Value {
                        param: "file_descriptor_set",
                        value: "true",
                    } => ret_val.file_descriptor_set = true,
                    Param::Value {
                        param: "file_descriptor_set",
                        value: "false",
                    } => (),
                    _ => return Err(InvalidParameter::from(param)),
                }
            }
        }

        Ok(ret_val)
    }
}

/// An invalid parameter
#[derive(Debug)]
pub struct InvalidParameter(String);

impl InvalidParameter {
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

impl fmt::Display for InvalidParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("invalid parameter: ")?;
        f.write_str(&self.0)
    }
}

impl std::error::Error for InvalidParameter {}

/// A wire-compatible reader of a [`CodeGeneratorRequest`]
///
/// This type treats the proto files contained in the request as raw byte
/// arrays so that we can round-trip those bytes into the generated files
/// as an encoded [`FileDescriptorSet`].
///
/// [`CodeGeneratorRequest`]: prost_types::compiler::CodeGeneratorRequest
/// [`FileDescriptorSet`]: prost_types::FileDescriptorSet
#[derive(Clone, PartialEq, ::prost::Message)]
struct RawProtos {
    #[prost(bytes = "vec", repeated, tag = "15")]
    proto_file: Vec<Vec<u8>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiler_option_string_with_three_plus_equals_parses_correctly() {
        const INPUT: &str = r#"enable_type_names,compile_well_known_types,disable_comments=.,extern_path=.google.protobuf=::pbjson_types,type_attribute=.=#[cfg(all(feature = "test"\, feature = "orange"))]"#;

        let expected: &[Param] = &[
            Param::Parameter {
                param: "enable_type_names",
            },
            Param::Parameter {
                param: "compile_well_known_types",
            },
            Param::Value {
                param: "disable_comments",
                value: ".",
            },
            Param::KeyValue {
                param: "extern_path",
                key: ".google.protobuf",
                value: "::pbjson_types".into(),
            },
            Param::KeyValue {
                param: "type_attribute",
                key: ".",
                value: r#"#[cfg(all(feature = "test", feature = "orange"))]"#.into(),
            },
        ];

        let actual = Params::from_protoc_plugin_opts(INPUT).unwrap();
        assert_eq!(actual.params, expected);
    }
}
