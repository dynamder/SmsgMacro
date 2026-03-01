mod codegen;
mod error;
mod ir;
mod parser;

use codegen::{CodeGenerator, derive_gen::DeriveGenerator, struct_gen::StructGenerator};
use parser::parse_smsg;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{ItemMod, parse_macro_input};

#[proc_macro_attribute]
pub fn smsg(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path_str = attr.to_string().trim_matches('"').to_string();

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let full_path = std::path::Path::new(&manifest_dir).join(&path_str);

    let source_code = match std::fs::read_to_string(&full_path) {
        Ok(content) => content,
        Err(e) => {
            let err_msg = format!("Failed to read smsg file '{}': {}", full_path.display(), e);
            return TokenStream::from(quote! {
                compile_error!(#err_msg)
            });
        }
    };

    let smsg_file = match parse_smsg(&source_code) {
        Ok(file) => file,
        Err(e) => {
            let err_msg = e.to_string();
            return TokenStream::from(quote! {
                compile_error!(#err_msg)
            });
        }
    };

    let item_mod = parse_macro_input!(item as ItemMod);
    let mod_name = Ident::new(&item_mod.ident.to_string(), proc_macro2::Span::call_site());

    let struct_gen = StructGenerator::new();
    let struct_code = struct_gen.generate(&smsg_file);

    let derive_gen = DeriveGenerator::new();
    let derive_code = derive_gen.generate(&smsg_file);

    let expanded = quote! {
        pub mod #mod_name {
            use super::*;

            #struct_code
            #derive_code
        }
    };

    TokenStream::from(expanded)
}
