//! Schema Resolution for oneOf components

use openapiv3::*;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::sanitize_str_for_ident;

pub(super) struct ResolvedOneOfType {
    tokens: TokenStream,
}

impl ResolvedOneOfType {
    pub(super) fn generate(&self, ident: Ident, inner: bool) -> std::io::Result<TokenStream> {
        let enum_tokens = &self.tokens;
        if inner {
            Ok(quote! { #ident })
        } else {
            Ok(quote! {
                pub enum #ident {
                    #enum_tokens
                }
            })
        }
    }
}
pub(super) fn resolve_schema_component_kind_oneof(
    data: &SchemaData,
    kind: &SchemaKind,
) -> std::io::Result<ResolvedOneOfType> {
    if let SchemaKind::OneOf { one_of } = kind {
        let mut tokens = TokenStream::new();
        for component in one_of {
            match component {
                ReferenceOr::Reference { reference } => {
                    let referred_type = reference.split('#').last().unwrap();
                    let referred_type = referred_type.split("/").last().unwrap();
                    let field_ty_ident =
                        Ident::new(&sanitize_str_for_ident(referred_type), Span::call_site());
                    let enum_token = quote! {
                        #field_ty_ident,
                    };
                    tokens.extend(enum_token);
                }
                _ => {}
            }
        }
        Ok(ResolvedOneOfType { tokens })
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to Resolve OneOf Schema Kind.",
        ))
    }
}
