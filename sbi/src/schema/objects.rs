//! Handling of `ObjectType` Schema components

use openapiv3::*;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::{resolve_reference_or_box_schema_component, sanitize_str_for_ident};

// Resolves the `ObjectType`
//
// If `additional_properties` is set, it's an `inner` object and it's resolved as
// `field: HashMap<String, ReferredObject>`
pub(super) fn resolve_schema_component_kind_object(
    data: &SchemaData,
    object: &ObjectType,
) -> std::io::Result<ResolvedObjectType> {
    let field_tokens = if object.additional_properties.is_some() {
        let additional = object.additional_properties.as_ref().unwrap();
        if let AdditionalProperties::Schema(s) = additional {
            if let ReferenceOr::Reference { reference } = &**s {
                let referred_type = reference.split('#').last().unwrap();
                let referred_type = referred_type.split("/").last().unwrap();
                let value_ident =
                    Ident::new(&sanitize_str_for_ident(referred_type), Span::call_site());
                quote! { std::collections::HashMap<String, #value_ident> }
            } else {
                // TODO: Ideally we should not reach here, but let's keep it for now. Later make
                // this an Err Return.
                quote! { () }
            }
        } else {
            // An Empty Object can be defined with `additionalProperties: false`, so let's give
            // them one.
            // FIXME: This is an ugly hack for `EmptyObject` inside 29571_CommonData.
            quote! { _ignore : () , }
        }
    } else {
        // This is an Outer object and is resolved as a `struct`.
        let mut obj_tokens = TokenStream::new();
        for (prop_name, prop_value) in &object.properties {
            let prop_ident = Ident::new(&sanitize_str_for_ident(prop_name), Span::call_site());

            let (prop_toks, is_schema) =
                resolve_reference_or_box_schema_component(prop_name, data, prop_value)?;
            let is_required = object.required.iter().find(|&s| s == prop_name).is_some();
            let prop_toks = if !is_required {
                quote! { Option<#prop_toks> }
            } else {
                prop_toks
            };
            if !is_schema {
                obj_tokens.extend(quote! {
                    #prop_ident: #prop_toks ,
                });
            } else {
                // This happens when additional properties is true
                obj_tokens.extend(quote! { #prop_ident: #prop_toks , })
            }
        }
        obj_tokens
    };
    Ok(ResolvedObjectType { field_tokens })
}

pub(super) struct ResolvedObjectType {
    field_tokens: TokenStream,
}

impl ResolvedObjectType {
    pub(super) fn generate(self, ident: Ident, inner: bool) -> std::io::Result<TokenStream> {
        let field_tokens = self.field_tokens;
        let tokens = if inner {
            quote! {
                #field_tokens
            }
        } else {
            quote! {
                    pub struct #ident {
                        #field_tokens
                    }
            }
        };

        Ok(tokens)
    }
}
