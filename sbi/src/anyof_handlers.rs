#[allow(unused)]
use openapiv3::*;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn default_anyof_handler(
    name: &str,
    anyof_schema: &SchemaKind,
) -> std::io::Result<TokenStream> {
    let ident = Ident::new(&crate::sanitize_str_for_ident(name), Span::call_site());

    if let SchemaKind::AnyOf { any_of } = anyof_schema {
        let mut enum_tokens = TokenStream::new();

        for item in any_of {
            let token = match item {
                ReferenceOr::Reference { reference } => {
                    quote! {
                        #reference(#reference),
                    }
                }
                ReferenceOr::Item(Schema {
                    schema_kind: SchemaKind::Type(Type::String(_)),
                    ..
                }) => {
                    quote! {
                        String(String),
                    }
                }
                ReferenceOr::Item(Schema {
                    schema_kind: SchemaKind::Type(Type::Integer(_)),
                    ..
                }) => {
                    quote! {
                        Integer(i64),
                    }
                }
                _ => {
                    eprintln!("unhandled item : {:#?}", item);
                    todo!();
                }
            };
            enum_tokens.extend(token);
        }
        Ok(quote! {
            pub enum #ident {
                #enum_tokens
            }
        })
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not anyof schema",
        ))
    }
}
