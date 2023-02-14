//! Handling of `Boolean` Schema objects

use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub(super) fn resolve_schema_component_kind_boolean() -> std::io::Result<ResolvedBoolean> {
    Ok(ResolvedBoolean)
}

pub(super) struct ResolvedBoolean;

impl ResolvedBoolean {
    pub(super) fn generate(self, ident: Ident, inner: bool) -> std::io::Result<TokenStream> {
        if !inner {
            Ok(quote! { pub struct #ident(bool); })
        } else {
            Ok(quote! { bool , })
        }
    }
}
