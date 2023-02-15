//! handling of `AnyOf` Schemas

use openapiv3::{ReferenceOr, Schema, SchemaKind};
use proc_macro2::TokenStream;

use crate::AnyOfHandler;

pub(super) fn resolve_schema_component_anyof(
    name: &str,
    any_of: &SchemaKind,
    handlers: &Vec<AnyOfHandler>,
) -> std::io::Result<TokenStream> {
    for handler in handlers {
        eprintln!("resolving name: {}", name);
        let result = handler(name, any_of);
        if result.is_ok() {
            return result;
        }
    }
    eprintln!("name: {}, not resolved yet.", name);
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "No handlers could resolve anyof",
    ))
}
