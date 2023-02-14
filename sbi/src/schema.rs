//! Functions related to resolving schema components as Rust 'struct's or 'enum's

#[allow(unused)]
use openapiv3::*;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

mod strings;
use strings::resolve_schema_component_kind_string;

mod nums_ints;
use nums_ints::{resolve_schema_component_kind_integer, resolve_schema_component_kind_number};

mod objects;
use objects::resolve_schema_component_kind_object;

mod arrays;
use arrays::resolve_schema_component_kind_array;

mod boolean;
use boolean::resolve_schema_component_kind_boolean;

// Returns a TokenStream corresponding to the schema component.
//
// Typically this function will be called by `Generator`.
pub(crate) fn resolve_schema_component(
    name: &str,
    schema: &Schema,
) -> std::io::Result<TokenStream> {
    match &schema.schema_kind {
        SchemaKind::Type(_) => resolve_schema_type_component(name, schema, false),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not implemented yet!",
        )),
    }
}

// Resolves the `Type(Type)` variant of `SchemaKind`
//
// Basically calls the resolver for each of the variant of the `Type`
//
// TODO: AnyOf, AllOf, OneOf
fn resolve_schema_type_component(
    name: &str,
    schema: &Schema,
    inner: bool,
) -> std::io::Result<TokenStream> {
    let ident = Ident::new(&sanitize_str_for_ident(&name), Span::call_site());
    if let SchemaKind::Type(ref t) = schema.schema_kind {
        match t {
            Type::String(ref s) => {
                let string_tokens = resolve_schema_component_kind_string(&schema.schema_data, s)?;
                string_tokens.generate(ident, inner)
            }
            Type::Object(ref o) => {
                let object_tokens = resolve_schema_component_kind_object(&schema.schema_data, o)?;
                object_tokens.generate(ident, inner)
            }
            Type::Array(ref a) => {
                let array_tokens = resolve_schema_component_kind_array(&schema.schema_data, a)?;
                array_tokens.generate(ident, inner)
            }
            Type::Number(ref n) => {
                let nums = resolve_schema_component_kind_number(&schema.schema_data, n)?;
                nums.generate(ident, inner)
            }
            Type::Integer(ref i) => {
                let ints = resolve_schema_component_kind_integer(&schema.schema_data, i)?;
                ints.generate(ident, inner)
            }
            Type::Boolean { .. } => {
                let bool_tokens = resolve_schema_component_kind_boolean()?;
                bool_tokens.generate(ident, inner)
            }
        }
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Schema: {:#?} is not of Kind Type", schema),
        ))
    }
}

fn resolve_reference_or_box_schema_component(
    name: &str,
    _data: &SchemaData,
    ref_or_schema: &ReferenceOr<Box<Schema>>,
) -> std::io::Result<(TokenStream, bool)> {
    match ref_or_schema {
        ReferenceOr::Reference { reference } => {
            let referred_type = reference.split('#').last().unwrap();
            let referred_type = referred_type.split("/").last().unwrap();
            let field_ty_ident =
                Ident::new(&sanitize_str_for_ident(referred_type), Span::call_site());
            Ok((quote! { #field_ty_ident }, false))
        }
        ReferenceOr::Item(ref s) => Ok((resolve_schema_type_component(name, s, true)?, true)),
    }
}

fn sanitize_str_for_ident(name: &str) -> String {
    if name.starts_with("5") {
        name.replace("5", "Five")
    } else if name.starts_with("3GPP") {
        name.replace("3GPP", "THREEGPP")
    } else {
        sanitize_keywords(name)
    }
}

fn sanitize_keywords(name: &str) -> String {
    let keywords = vec!["type", "self"];
    let mut name = name.to_string();
    if keywords.iter().find(|&s| s == &name).is_some() {
        name += "_";
    }
    name
}
