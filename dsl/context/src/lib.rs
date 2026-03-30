use proc_macro2::TokenStream;
use quote::quote;

use crate::codegen::context_codegen::{generate_context_ext_impl, generate_guest_error_impl};

pub fn generate_context_extensions() -> TokenStream {
    let trait_def = generate_context_ext_trait();
    let impl_block = generate_context_ext_impl();
    let error_impl = generate_guest_error_impl();

    quote! {
        #trait_def
        #impl_block
        #error_impl
    }
}

fn generate_context_ext_trait() -> TokenStream {
    let core = quote! { vprogs_core };
    let tx = quote! { vprogs_transaction_runtime };

    quote! {
        pub trait TransactionContextExt {
            fn load<R: #core::Resource>(&self, id: #core::ResourceId) -> Result<R, GuestError>;
            fn load_or_init<R: #core::Resource + Default>(&self, id: #core::ResourceId) -> Result<R, GuestError>;
            fn read<R: #core::Resource>(&self, id: #core::ResourceId) -> Result<R, GuestError>;
            fn current_slot(&self) -> u64;
        }

        #[derive(Debug)]
        pub enum GuestError {
            ResourceNotFound(#core::ResourceId),
            DecodeError(String),
            InsufficientBalance,
        }
    }
}
