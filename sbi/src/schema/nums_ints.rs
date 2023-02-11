//! Handling of `NumberType` and `IntegerType`

use openapiv3::{IntegerType, NumberType, SchemaData};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

// Resolves the `NumberType`
pub(super) fn resolve_schema_component_kind_number(
    _data: &SchemaData,
    _num: &NumberType,
) -> std::io::Result<ResolvedNumber> {
    Ok(ResolvedNumber)
}

// TODO: It's an empty struct for now.
pub(super) struct ResolvedNumber;

impl ResolvedNumber {
    // This function is an internal API to generate code for the NumberType

    pub(super) fn generate(&self, ident: Ident, inner: bool) -> std::io::Result<TokenStream> {
        if inner {
            Ok(quote! { #ident: f64 , })
        } else {
            Ok(quote! { pub struct #ident(f64); })
        }
    }
}

pub(super) fn resolve_schema_component_kind_integer(
    _data: &SchemaData,
    _num: &IntegerType,
) -> std::io::Result<ResolvedInteger> {
    Ok(ResolvedInteger)
}

// TODO : It's an empty struct for now but later on this should get all the further details like
// handling `min/max` etc.
pub(super) struct ResolvedInteger;

impl ResolvedInteger {
    // This function is an internal API to generate code for the NumberType

    pub(super) fn generate(&self, ident: Ident, inner: bool) -> std::io::Result<TokenStream> {
        if inner {
            Ok(quote! { #ident: i64 , })
        } else {
            Ok(quote! { pub struct #ident(i64); })
        }
    }
}
