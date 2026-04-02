// dsl/macros/src/attributes.rs
use syn::{Attribute, Meta, NestedMeta};

#[derive(Debug, Default)]
pub struct ResourceAttrs {
    pub versioned: bool,
    pub zk_public: bool,
    pub init: bool,
}

pub fn parse_resource_attributes(attrs: &[Attribute]) -> ResourceAttrs {
    let mut parsed = ResourceAttrs::default();

    for attr in attrs {
        if !attr.path.is_ident("resource") {
            continue;
        }

        if let Ok(meta) = attr.parse_meta() {
            if let Meta::List(meta_list) = meta {
                for nested in meta_list.nested {
                    if let NestedMeta::Meta(Meta::Path(path)) = nested {
                        if path.is_ident("versioned") {
                            parsed.versioned = true;
                        } else if path.is_ident("zk_public") {
                            parsed.zk_public = true;
                        } else if path.is_ident("init") {
                            parsed.init = true;
                        }
                    }
                }
            }
        }
    }
    parsed
}
