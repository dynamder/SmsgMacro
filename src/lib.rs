mod codegen;
mod error;
mod hash;
mod ir;
mod parser;

use codegen::struct_gen::{ModuleGenerator, StructGenerator};
use codegen::{CodeGenerator, derive_gen::DeriveGenerator};
use parser::package_parser::{build_module_structure, parse_package_toml, walk_package_directory};
use parser::parse_smsg;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{ItemMod, parse_macro_input};

#[derive(Debug, Clone)]
enum SmsgCategory {
    File,
    Package,
}

#[derive(Debug)]
struct SmsgAttribute {
    category: SmsgCategory,
    path: String,
}

impl SmsgAttribute {
    pub fn parse(attr: &str) -> Result<Self, String> {
        let attr = attr.trim();

        if attr.starts_with('"') {
            return Ok(SmsgAttribute {
                category: SmsgCategory::File,
                path: attr.trim_matches('"').to_string(),
            });
        }

        let parts: Vec<&str> = attr.split(',').collect();
        let mut category = SmsgCategory::File;
        let mut path = String::new();

        for part in parts {
            let part = part.trim();
            if part.starts_with("category") {
                let value = part.split('=').nth(1).map(|s| s.trim()).unwrap_or("");
                category = match value {
                    "package" => SmsgCategory::Package,
                    "file" => SmsgCategory::File,
                    _ => {
                        return Err(format!(
                            "Invalid category: {}. Expected 'file' or 'package'",
                            value
                        ));
                    }
                };
            } else if part.starts_with("path") {
                path = part
                    .split('=')
                    .nth(1)
                    .map(|s| s.trim().trim_matches('"'))
                    .unwrap_or("")
                    .to_string();
            }
        }

        if path.is_empty() {
            return Err("path is required".to_string());
        }

        Ok(SmsgAttribute { category, path })
    }
}

#[proc_macro_attribute]
pub fn smsg(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_str = attr.to_string();

    let smsg_attr = match SmsgAttribute::parse(&attr_str) {
        Ok(a) => a,
        Err(e) => {
            return TokenStream::from(quote! {
                compile_error!(#e)
            });
        }
    };

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let full_path = std::path::Path::new(&manifest_dir).join(&smsg_attr.path);

    match smsg_attr.category {
        SmsgCategory::File => generate_file_type(&full_path, item),
        SmsgCategory::Package => generate_package_type(&full_path, item),
    }
}

fn generate_file_type(full_path: &std::path::Path, item: TokenStream) -> TokenStream {
    let source_code = match std::fs::read_to_string(full_path) {
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

fn generate_package_type(full_path: &std::path::Path, item: TokenStream) -> TokenStream {
    let package_toml_path = full_path.join("package.toml");

    let toml_content = match std::fs::read_to_string(&package_toml_path) {
        Ok(content) => content,
        Err(e) => {
            let err_msg = format!(
                "Failed to read package.toml '{}': {}",
                package_toml_path.display(),
                e
            );
            return TokenStream::from(quote! {
                compile_error!(#err_msg)
            });
        }
    };

    let _package = match parse_package_toml(&toml_content, &full_path.to_string_lossy()) {
        Ok(pkg) => pkg,
        Err(e) => {
            let err_msg = e.to_string();
            return TokenStream::from(quote! {
                compile_error!(#err_msg)
            });
        }
    };

    let smsg_files = match walk_package_directory(full_path) {
        Ok(files) => files,
        Err(e) => {
            let err_msg = format!(
                "Failed to read package directory '{}': {}",
                full_path.display(),
                e
            );
            return TokenStream::from(quote! {
                compile_error!(#err_msg)
            });
        }
    };

    let module_structure = build_module_structure(full_path, &smsg_files);

    let item_mod = parse_macro_input!(item as ItemMod);
    let mod_name = Ident::new(&item_mod.ident.to_string(), proc_macro2::Span::call_site());

    let module_gen = ModuleGenerator::new();
    let module_code = module_gen.generate_module_structure(&module_structure);

    let expanded = quote! {
        pub mod #mod_name {
            use super::*;

            #module_code
        }
    };

    TokenStream::from(expanded)
}
