use crate::codegen::CodeGenerator;
use crate::ir::SmsgFile;
use quote::quote;

pub struct DeriveGenerator;

impl DeriveGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DeriveGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeGenerator for DeriveGenerator {
    fn generate(&self, _smsg_file: &SmsgFile) -> proc_macro2::TokenStream {
        quote! {}
    }
}
