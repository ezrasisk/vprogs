//! Code generation for StateDiff construction.

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::attributes::ResourceAttrs;

pub fn generate_into_diff_impl(struct_name: &Ident, attrs: &ResourceAttrs) -> TokenStream {
    let core = quote! { vprogs_core };
    let tx = quote! { vprogs_transaction_runtime };

    let versioned_comment = if attrs.versioned {
        quote! { /* Participates in versioned SMT state root */ }
    } else {
        quote! {}
    };

    quote! {
        pub fn into_diff(self, id: #core::ResourceId) -> #tx::StateDiff {
            let mut diff = #tx::StateDiff::new(id);
            diff.set_access(#core::AccessMetadata::write());

            let mut buffer = Vec::new();
            {
                let mut writer = #core::codec::VecWriter::new(&mut buffer);
                self.encode(&mut writer).expect("encode should not fail");
            }
            diff.set_data(buffer);

            #versioned_comment
            diff
        }
    }
}
