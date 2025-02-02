//! Code generator modules

use prost_types::compiler::{
    code_generator_response::{Feature, File},
    CodeGeneratorResponse,
};

use crate::ModuleRequestSet;

mod core;
mod file_descriptor_set;

pub(crate) use self::{core::CoreProstGenerator, file_descriptor_set::FileDescriptorSetGenerator};

/// A code generation result
pub type Result = std::result::Result<Vec<File>, Error>;
/// A code generation error
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Extension function to assist in converting [`Result`] into a [`CodeGeneratorResponse`]
pub trait GeneratorResultExt {
    /// Unwrap a [`Result`], producing the relevant [`CodeGeneratorResponse`]
    fn unwrap_codegen_response(self) -> CodeGeneratorResponse;
}

impl GeneratorResultExt for Result {
    fn unwrap_codegen_response(self) -> CodeGeneratorResponse {
        match self {
            Ok(file) => CodeGeneratorResponse {
                file,
                supported_features: Some(Feature::Proto3Optional as u64),
                ..Default::default()
            },
            Err(error) => error_to_codegen_response(&*error),
        }
    }
}

fn error_to_codegen_response(error: &dyn std::error::Error) -> CodeGeneratorResponse {
    CodeGeneratorResponse {
        error: Some(error.to_string()),
        supported_features: Some(Feature::Proto3Optional as u64),
        ..Default::default()
    }
}

/// A code generator
pub trait Generator {
    /// Generate one or more files based on the input request
    fn generate(&mut self, module_request_set: &ModuleRequestSet) -> Result;

    /// Chain multiple generators together, returning their composite output
    fn chain<G>(self, next: G) -> ChainedGenerator<Self, G>
    where
        Self: Sized,
    {
        ChainedGenerator {
            generator1: self,
            generator2: next,
        }
    }
}

/// A chain of generators, executed sequentially
///
/// Executes `G1` followed by `G2`, returning generated files in the same order as
/// produced. Execution short circuits in the event `G1` returns an error.
pub struct ChainedGenerator<G1, G2> {
    generator1: G1,
    generator2: G2,
}

impl<G1, G2> Generator for ChainedGenerator<G1, G2>
where
    G1: Generator,
    G2: Generator,
{
    fn generate(&mut self, module_request_set: &ModuleRequestSet) -> Result {
        let mut files = self.generator1.generate(module_request_set)?;
        files.extend(self.generator2.generate(module_request_set)?);
        Ok(files)
    }
}

impl<G> Generator for Option<G>
where
    G: Generator,
{
    fn generate(&mut self, module_request_set: &ModuleRequestSet) -> Result {
        if let Some(slf) = self {
            slf.generate(module_request_set)
        } else {
            Ok(Vec::new())
        }
    }
}
