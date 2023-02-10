//! Handling of the `SchemaKind::String(StringType)` schemas

use openapiv3::*;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::sanitize_str_for_ident;

// Resolves the `Type::String(StringType)` component
//
// TODO: Handling `schema_data`
pub(super) fn resolve_schema_component_kind_string(
    _data: &SchemaData,
    s: &StringType,
) -> std::io::Result<ResolvedStringType> {
    let resolved = if s.enumeration.is_empty() {
        ResolvedStringType {
            enum_variants: None,
        }
    } else {
        let enum_variants = s
            .enumeration
            .iter()
            .map(|s| s.as_ref().unwrap())
            .collect::<Vec<_>>();
        let mut enum_variant_tokens = TokenStream::new();
        for var in enum_variants {
            let var_ident = Ident::new(&sanitize_str_for_ident(&var), Span::call_site());
            enum_variant_tokens.extend(quote! { #var_ident, });
        }

        ResolvedStringType {
            enum_variants: Some(enum_variant_tokens),
        }
    };

    Ok(resolved)
}

// A structure returned by 'processing' the `StringType`.
//
// This structure is implements a `generate` method.
pub(super) struct ResolvedStringType {
    enum_variants: Option<TokenStream>,
}

impl ResolvedStringType {
    // This is an internal API used by the code generator.
    //
    // Returns a `TokenStream` for the `struct`/`enum`. May also later generate an `impl` block for
    // the structure.
    pub(super) fn generate(&self, ident: Ident, inner: bool) -> std::io::Result<TokenStream> {
        let toks = if inner {
            if self.enum_variants.is_none() {
                quote! { #ident: String , }
            } else {
                // TODO : Make an Enum Type Like #ident + Enum
                quote! { #ident: String , }
            }
        } else {
            if self.enum_variants.is_none() {
                quote! { pub struct #ident(String); }
            } else {
                let enum_variants = self.enum_variants.as_ref().unwrap();
                quote! {
                    pub enum #ident {
                        #enum_variants
                    }
                }
            }
        };
        Ok(toks)
    }
}
