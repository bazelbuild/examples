use heck::ToUpperCamelCase;
use prost_build::{Comments, Module};
use prost_types::FileDescriptorProto;

pub fn to_snake(s: &str) -> String {
    let as_module = Module::from_protobuf_package_name(s);
    assert_eq!(as_module.len(), 1, "unexpected `.` in name part");
    let mut parts = as_module.parts();
    parts.next().unwrap().to_owned()
}

/// Converts a `snake_case` identifier to an `UpperCamel` case Rust type identifier.
pub fn to_upper_camel(s: &str) -> String {
    let mut ident = s.to_upper_camel_case();

    // Suffix an underscore for the `Self` Rust keyword as it is not allowed as raw identifier.
    if ident == "Self" {
        ident += "_";
    }
    ident
}

pub fn get_service_comments(file: &FileDescriptorProto, service_index: usize) -> Comments {
    let path = [
        6,
        i32::try_from(service_index).expect("unexpectedly large service index"),
    ];
    get_comments(file, &path).unwrap_or_else(default_comments)
}

pub fn get_method_comments(
    file: &FileDescriptorProto,
    service_index: usize,
    method_index: usize,
) -> Comments {
    let path = [
        6,
        i32::try_from(service_index).expect("unexpectedly large service index"),
        2,
        i32::try_from(method_index).expect("unexpectedly large method index"),
    ];
    get_comments(file, &path).unwrap_or_else(default_comments)
}

fn get_comments(file: &FileDescriptorProto, path: &[i32]) -> Option<Comments> {
    let source_code_info = file.source_code_info.as_ref()?;
    let idx = source_code_info
        .location
        .binary_search_by(|probe| probe.path.as_slice().cmp(path))
        .ok()?;
    let location = &source_code_info.location[idx];
    Some(Comments {
        leading: vec![location.leading_comments().to_owned()],
        leading_detached: vec![location.leading_detached_comments.clone()],
        trailing: vec![location.trailing_comments().to_owned()],
    })
}

fn default_comments() -> Comments {
    Comments {
        trailing: Vec::new(),
        leading: Vec::new(),
        leading_detached: Vec::new(),
    }
}
