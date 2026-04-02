// dsl/macros/src/lib.rs
pub mod attributes;
pub mod codegen;

pub use crate::resource_derive::derive_resource;
pub use crate::resource_derive::Resource;

pub use crate::context::TransactionContextExt;
pub use crate::context::GuestError;

// Re-export for convenience
pub use vprogs_core::codec;
