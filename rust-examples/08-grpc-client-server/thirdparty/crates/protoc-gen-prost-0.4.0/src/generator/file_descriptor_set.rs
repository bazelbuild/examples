use std::fmt::Write;

use prost_types::compiler::code_generator_response::File;

use crate::{Generator, ModuleRequest, ModuleRequestSet, Result};

pub struct FileDescriptorSetGenerator;

impl Generator for FileDescriptorSetGenerator {
    fn generate(&mut self, module_request_set: &ModuleRequestSet) -> Result {
        let files = module_request_set
            .requests()
            .filter_map(|(_, request)| Self::generate_one(request))
            .collect();

        Ok(files)
    }
}
impl FileDescriptorSetGenerator {
    fn generate_one(request: &ModuleRequest) -> Option<File> {
        request.append_to_file(|buffer| {
            // This cannot be done with another file and `include_bytes!` because the
            // contract for a file's contents requires that they be valid UTF-8.
            //
            // So, we append them as an embedded array instead.
            append_file_descriptor_set_bytes(
                request.proto_package_name(),
                &RawProtosSet {
                    file: request.raw_files().map(|b| b.to_owned()).collect(),
                },
                buffer,
            );
        })
    }
}

/// Wire-compatible FileDescriptorSet that doesn't require fully-decoded file descriptors
#[derive(Clone, PartialEq, ::prost::Message)]
struct RawProtosSet {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub file: Vec<Vec<u8>>,
}

fn append_file_descriptor_set_bytes(
    package: &str,
    file_descriptor_set: &impl prost::Message,
    buffer: &mut String,
) {
    buffer.push_str("/// Encoded file descriptor set for the `");
    buffer.push_str(package);
    buffer.push_str("` package\n");

    buffer.push_str("pub const FILE_DESCRIPTOR_SET: &[u8] = &[\n");

    let encoded = file_descriptor_set.encode_to_vec();
    let mut chunks = encoded.chunks_exact(16);
    for chunck in chunks.by_ref() {
        writeln!(
            buffer,
            "    {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, \
             {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x},",
            chunck[0],
            chunck[1],
            chunck[2],
            chunck[3],
            chunck[4],
            chunck[5],
            chunck[6],
            chunck[7],
            chunck[8],
            chunck[9],
            chunck[10],
            chunck[11],
            chunck[12],
            chunck[13],
            chunck[14],
            chunck[15],
        )
        .unwrap();
    }

    if !chunks.remainder().is_empty() {
        buffer.push_str("    ");
        for byte in chunks.remainder() {
            write!(buffer, "{:#04x}, ", byte).unwrap();
        }
        let _ = buffer.pop();
        buffer.push('\n');
    }

    buffer.push_str("];\n");
}
