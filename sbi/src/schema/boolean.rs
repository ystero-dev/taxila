//! Handling of `Boolean` Schema objects

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use super::ResolvedSchemaComponent;

pub(super) fn resolve_schema_component_kind_boolean() -> std::io::Result<ResolvedBoolean> {
    Ok(ResolvedBoolean)
}

pub(super) struct ResolvedBoolean;

impl ResolvedBoolean {
    pub(super) fn generate(
        self,
        ident: Ident,
        inner: bool,
    ) -> std::io::Result<ResolvedSchemaComponent> {
        let tokens = if !inner {
            quote! { pub struct #ident(bool); }
        } else {
            quote! { bool }
        };

        Ok(ResolvedSchemaComponent {
            tokens,
            aux_tokens: TokenStream::new(),
        })
    }
}
