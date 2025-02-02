use std::collections::HashSet;

use prost_build::Module;
use prost_types::compiler::code_generator_response::File;

use crate::{Generator, ModuleRequestSet, Result};

pub struct CoreProstGenerator {
    config: prost_build::Config,
}

impl Generator for CoreProstGenerator {
    fn generate(&mut self, module_request_set: &ModuleRequestSet) -> Result {
        let prost_requests: Vec<_> = module_request_set
            .requests()
            .flat_map(|(module, request)| {
                request
                    .files
                    .iter()
                    .map(|proto| (module.clone(), proto.clone()))
            })
            .collect();

        let modules: HashSet<_> = prost_requests
            .iter()
            .map(|(module, _)| module.clone())
            .collect();

        let mut file_contents = self.config.generate(prost_requests)?;
        let files = modules
            .into_iter()
            .filter_map(|module| {
                let content = file_contents.remove(&module).unwrap_or_default();
                Self::content_to_file(module, content, module_request_set)
            })
            .collect();

        Ok(files)
    }
}

impl CoreProstGenerator {
    pub fn new(config: prost_build::Config) -> Self {
        Self { config }
    }

    fn content_to_file(
        module: Module,
        content: String,
        module_requests: &ModuleRequestSet,
    ) -> Option<File> {
        let request = module_requests
            .for_module(&module)
            .expect("generated module that somehow wasn't in the original requests…");

        request.write_to_file(move |buffer| {
            buffer.push_str("// @generated\n");
            buffer.push_str(&content);
            buffer.push_str("// @@protoc_insertion_point(module)\n");
        })
    }
}
