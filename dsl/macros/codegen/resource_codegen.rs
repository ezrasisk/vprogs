//! Code generation for Resource encode/decode methods.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, Fields, Ident};

use crate::attributes::ResourceAttrs;

pub fn generate_encode_impl(name: &Ident, data: &Data, _attrs: &ResourceAttrs) -> TokenStream {
    let fields = match data {
        Data::Struct(ds) => match &ds.fields {
            Fields::Named(f) => &f.named,
            _ => return quote! { compile_error!("Resource must use named fields"); },
        },
        _ => return quote! { compile_error!("Resource can only be derived on structs"); },
    };

    let encode_stmts = fields.iter().map(|f| {
        let fname = &f.ident;
        quote! { self.#fname.encode(writer)?; }
    });

    quote! {
        #(#encode_stmts)*
        Ok(())
    }
}

pub fn generate_decode_impl(name: &Ident, data: &Data, _attrs: &ResourceAttrs) -> TokenStream {
    let fields = match data {
        Data::Struct(ds) => match &ds.fields {
            Fields::Named(f) => &f.named,
            _ => return quote! { compile_error!("Resource must use named fields"); },
        },
        _ => return quote! { compile_error!("Resource can only be derived on structs"); },
    };

    let decode_stmts = fields.iter().map(|f| {
        let fname = &f.ident;
        let fty = &f.ty;
        quote! {
            let #fname = <#fty as vprogs_core::codec::Decode>::decode(reader)
                .map_err(|e| e.with_context(concat!("field '", stringify!(#fname), "'")))?;
        }
    });

    let field_names = fields.iter().map(|f| &f.ident);

    quote! {
        #(#decode_stmts)*
        Ok(#name { #(#field_names),* })
    }
}

pub fn generate_field_names_const(data: &Data) -> TokenStream {
    let fields = match data {
        Data::Struct(ds) => match &ds.fields {
            Fields::Named(f) => f.named.iter().filter_map(|fld| fld.ident.as_ref()),
            _ => return quote! {},
        },
        _ => return quote! {},
    };
    let names: Vec<String> = fields.map(|id| id.to_string()).collect();

    quote! {
        pub const FIELD_NAMES: &'static [&'static str] = &[ #(#names),* ];
    }
}
