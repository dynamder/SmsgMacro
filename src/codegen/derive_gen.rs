use crate::codegen::CodeGenerator;
use crate::hash::compute_message_hash;
use crate::ir::{MessageDef, SmsgFile};
use proc_macro2::Ident;
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
    fn generate(&self, smsg_file: &SmsgFile) -> proc_macro2::TokenStream {
        let message_impls: Vec<proc_macro2::TokenStream> = smsg_file
            .messages
            .iter()
            .map(generate_message_meta)
            .collect();

        let envelope_impls: Vec<proc_macro2::TokenStream> = smsg_file
            .messages
            .iter()
            .map(generate_smsg_envelope)
            .collect();

        let trait_def = quote! {
            pub trait MessageMeta {
                fn version_hash() -> [u8; 32];
                fn message_name() -> &'static str;
            }
        };

        quote! {
            #trait_def
            #(#message_impls)*
            #(#envelope_impls)*
        }
    }
}

fn generate_message_meta(message: &MessageDef) -> proc_macro2::TokenStream {
    let struct_name = Ident::new(&message.name, proc_macro2::Span::call_site());
    let hash = compute_message_hash(message);
    let message_name = message.name.as_str();

    quote! {
        impl MessageMeta for #struct_name {
            fn version_hash() -> [u8; 32] {
                [#(#hash),*]
            }

            fn message_name() -> &'static str {
                #message_name
            }
        }
    }
}

fn generate_smsg_envelope(message: &MessageDef) -> proc_macro2::TokenStream {
    let struct_name = Ident::new(&message.name, proc_macro2::Span::call_site());
    let envelope_name = Ident::new(
        &format!("{}Envelope", message.name),
        proc_macro2::Span::call_site(),
    );

    quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub struct #envelope_name {
            pub version_hash: [u8; 32],
            pub payload: #struct_name,
        }

        impl #envelope_name {
            pub fn new(payload: #struct_name) -> Self {
                Self {
                    version_hash: #struct_name::version_hash(),
                    payload,
                }
            }

            pub fn into_parts(self) -> ([u8; 32], #struct_name) {
                (self.version_hash, self.payload)
            }
        }
    }
}
