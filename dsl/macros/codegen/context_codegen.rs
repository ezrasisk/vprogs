//! Code generation for TransactionContextExt methods.

use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_context_ext_impl() -> TokenStream {
    let core = quote! { vprogs_core };
    let tx = quote! { vprogs_transaction_runtime };

    quote! {
        impl<T: #tx::TransactionContext> #tx::TransactionContextExt for T {
            fn load<R: #core::Resource>(
                &self,
                id: #core::ResourceId,
            ) -> Result<R, super::GuestError> {
                self.register_access(id, #core::AccessMetadata::read_write());

                let data = match self.get_resource_data(id) {
                    Some(d) => d,
                    None => return Err(super::GuestError::ResourceNotFound(id)),
                };

                let mut reader = #core::codec::SliceReader::new(&data);
                R::decode(&mut reader)
                    .map_err(|e| super::GuestError::DecodeError(format!("{:?}", e)))
            }

            fn load_or_init<R: #core::Resource + Default>(
                &self,
                id: #core::ResourceId,
            ) -> Result<R, super::GuestError> {
                if let Some(data) = self.get_resource_data(id) {
                    let mut reader = #core::codec::SliceReader::new(&data);
                    R::decode(&mut reader)
                        .map_err(|e| super::GuestError::DecodeError(format!("{:?}", e)))
                } else {
                    Ok(R::default())
                }
            }

            fn read<R: #core::Resource>(
                &self,
                id: #core::ResourceId,
            ) -> Result<R, super::GuestError> {
                self.register_access(id, #core::AccessMetadata::read_only());

                let data = match self.get_resource_data(id) {
                    Some(d) => d,
                    None => return Err(super::GuestError::ResourceNotFound(id)),
                };

                let mut reader = #core::codec::SliceReader::new(&data);
                R::decode(&mut reader)
                    .map_err(|e| super::GuestError::DecodeError(format!("{:?}", e)))
            }

            fn current_slot(&self) -> u64 {
                self.get_current_slot()
            }
        }
    }
}

pub fn generate_guest_error_impl() -> TokenStream {
    quote! {
        impl std::fmt::Display for super::GuestError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    super::GuestError::ResourceNotFound(id) => write!(f, "Resource not found: {:?}", id),
                    super::GuestError::DecodeError(msg) => write!(f, "Decode error: {}", msg),
                    super::GuestError::InsufficientBalance => write!(f, "Insufficient balance"),
                }
            }
        }
        impl std::error::Error for super::GuestError {}
    }
}
