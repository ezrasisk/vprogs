// vprogs-dsl/src/lib.rs
//! vprogs-dsl — Anchor-like high-productivity DSL for vprogs guest programs.
//!
//! Translates declarative Rust intent into the exact low-level contracts
//! required by transaction-runtime, the scheduler, deterministic codec,
//! versioned SMT, and zk backends.

pub mod attributes;
pub mod codegen;
pub mod resource;
pub mod context;

// Public re-exports — small, stable, Anchor-like surface
pub use resource::Resource;
pub use resource::derive_resource;

pub use context::TransactionContextExt;
pub use context::GuestError;

// User-friendly prelude
pub mod prelude {
    pub use super::Resource;
    pub use super::TransactionContextExt;
    pub use super::GuestError;

    pub use vprogs_core::{
        ResourceId,
        AccessMetadata,
    };

    pub use vprogs_transaction_runtime::StateDiff;

    pub use vprogs_core::codec::{self, Reader, Writer};
}

// Re-export codec at crate root
pub use vprogs_core::codec;
