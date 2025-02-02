use std::{collections::HashMap, iter};

use prost_build::Module;

use crate::util;

pub(crate) struct Resolver {
    extern_root: Node,
    compile_well_known_types: bool,
}

#[derive(Default, Debug)]
struct Node {
    extern_path: Option<String>,
    children: HashMap<String, Node>,
}

impl Node {
    fn insert(&mut self, fq_proto_path: &str, extern_path: String) {
        assert_eq!(".", &fq_proto_path[..1]);
        let mut path = &fq_proto_path[1..];
        let mut current = &mut *self;
        while !path.is_empty() {
            let (next, remaining) = path.split_once('.').unwrap_or((path, ""));
            path = remaining;
            current = current.children.entry(next.to_owned()).or_default();
        }

        current.extern_path = Some(extern_path);
    }

    fn resolve_extern(&self, fq_proto_path: &str) -> Option<String> {
        assert_eq!(".", &fq_proto_path[..1]);
        let mut path = &fq_proto_path[1..];
        let mut current = self;
        while !path.is_empty() {
            let (next, remaining) = path.split_once('.').unwrap_or((path, ""));
            if let Some(node) = current.children.get(next) {
                current = node;
                path = remaining;
            } else {
                let prefix = current.extern_path.as_deref()?;
                let (package, type_name) = path.rsplit_once('.').unwrap_or(("", path));
                let module = Module::from_protobuf_package_name(package);
                let ret_val = module
                    .parts()
                    .chain(iter::once(util::to_upper_camel(type_name).as_str()))
                    .fold(prefix.to_owned(), |mut acc, next| {
                        acc.push_str("::");
                        acc.push_str(next);
                        acc
                    });

                return Some(ret_val);
            }
        }

        current.extern_path.clone()
    }
}

impl Resolver {
    pub(crate) fn new(extern_path: Vec<(String, String)>, compile_well_known_types: bool) -> Self {
        let mut extern_root = Node::default();

        if !compile_well_known_types {
            extern_root.insert(".google.protobuf", "::prost_types".to_string());
            extern_root.insert(".google.protobuf.BoolValue", "bool".to_string());
            extern_root.insert(
                ".google.protobuf.BytesValue",
                "::prost::alloc::vec::Vec<u8>".to_string(),
            );
            extern_root.insert(".google.protobuf.DoubleValue", "f64".to_string());
            extern_root.insert(".google.protobuf.Empty", "()".to_string());
            extern_root.insert(".google.protobuf.FloatValue", "f32".to_string());
            extern_root.insert(".google.protobuf.Int32Value", "i32".to_string());
            extern_root.insert(".google.protobuf.Int64Value", "i64".to_string());
            extern_root.insert(
                ".google.protobuf.StringValue",
                "::prost::alloc::string::String".to_string(),
            );
            extern_root.insert(".google.protobuf.UInt32Value", "u32".to_string());
            extern_root.insert(".google.protobuf.UInt64Value", "u64".to_string());
        }

        for (fq_proto_path, extern_path) in extern_path {
            extern_root.insert(&fq_proto_path, extern_path);
        }

        Self {
            extern_root,
            compile_well_known_types,
        }
    }

    pub(crate) fn compile_well_known_types(&self) -> bool {
        self.compile_well_known_types
    }

    pub(crate) fn resolve_ident(&self, from: &Module, to_fq: &str) -> String {
        if let Some(proto_ident) = self.extern_root.resolve_extern(to_fq) {
            return proto_ident;
        }

        let (package, type_name) = to_fq.rsplit_once('.').unwrap();
        let to = Module::from_protobuf_package_name(package);

        let (down, prefix) = difference(from, &to);

        iter::repeat("super".to_owned())
            .take(down)
            .chain(to.parts().skip(prefix).map(|s| s.to_owned()))
            .chain(iter::once(util::to_upper_camel(type_name)))
            .reduce(|mut l, r| {
                l.push_str("::");
                l.push_str(&r);
                l
            })
            .unwrap_or_default()
    }
}

fn difference(left: &Module, right: &Module) -> (usize, usize) {
    let mut left_parts = left.parts();
    let mut right_parts = right.parts();

    let mut prefix = 0;

    loop {
        match (left_parts.next(), right_parts.next()) {
            (Some(left), Some(right)) if left == right => prefix += 1,
            (Some(_), Some(_)) => return (left_parts.count() + 1, prefix),
            (Some(_), None) => return (left_parts.count() + 1, prefix),
            (None, Some(_)) => return (0, prefix),
            (None, None) => return (0, prefix),
        }
    }
}
