// dsl/macros/src/resource_derive.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::attributes::parse_resource_attributes;
use crate::codegen::resource_codegen::{generate_encode_impl, generate_decode_impl, generate_field_names_const};
use crate::codegen::diff_codegen::generate_into_diff_impl;

#[proc_macro_derive(Resource, attributes(resource))]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let attrs = parse_resource_attributes(&ast.attrs);

    let core = quote! { vprogs_core };
    let tx = quote! { vprogs_transaction_runtime };

    let encode_body = generate_encode_impl(name, &ast.data, &attrs);
    let decode_body = generate_decode_impl(name, &ast.data, &attrs);
    let field_names = generate_field_names_const(&ast.data);
    let diff_impl = generate_into_diff_impl(name, &attrs);

    let expanded = quote! {
        pub trait Resource: Sized {
            fn resource_id(&self) -> #core::ResourceId;
            fn encode<W: #core::codec::Writer>(&self, writer: &mut W) -> Result<(), #core::codec::Error>;
            fn decode<R: #core::codec::Reader>(reader: &mut R) -> Result<Self, #core::codec::Error>;
        }

        impl #core::Resource for #name {
            fn resource_id(&self) -> #core::ResourceId {
                unimplemented!("ResourceId derivation - extend DSL if needed")
            }

            fn encode<W: #core::codec::Writer>(&self, writer: &mut W) -> Result<(), #core::codec::Error> {
                #encode_body
            }

            fn decode<R: #core::codec::Reader>(reader: &mut R) -> Result<Self, #core::codec::Error> {
                #decode_body
            }
        }

        impl #name {
            #diff_impl
        }

        impl #name {
            #field_names
            pub const IS_VERSIONED: bool = #attrs.versioned;
            pub const IS_ZK_PUBLIC: bool = #attrs.zk_public;
        }
    };

    TokenStream::from(expanded)
}
