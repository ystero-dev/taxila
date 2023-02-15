//! A simple generator utility for generating specs files.

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

#[allow(unused)]
use openapiv3::*;

use sbi5g::{sanitize_str_for_ident, Generator};

// A function that generates `enum` for the schema of the following type
//
// Values:
//   anyOf:
//     - type: string
//       enum:
//       - A
//       - B
//       - C
//  - type: string
//
//  The generated enum would look like
//
//  ```rust
//  pub enum Values {
//      A,
//      B,
//      C
//  }
//  ```
fn string_string_enum_anyof(
    name: &str,
    any_of_schema: &SchemaKind,
) -> std::io::Result<TokenStream> {
    let ident = Ident::new(&sanitize_str_for_ident(name), Span::call_site());

    if let SchemaKind::AnyOf { any_of } = any_of_schema {
        if any_of.len() == 2
        /* && any_of.iter().all(|x| {
            matches!(
                x,
                ReferenceOr::Item(Schema {
                    schema_kind: SchemaKind::Type(Type::String(_)),
                    ..
                })
            )
        }) */
        {
            for item in any_of {
                if let ReferenceOr::Item(Schema {
                    schema_kind: SchemaKind::Type(Type::String(s)),
                    ..
                }) = item
                {
                    if s.enumeration.is_empty() {
                        continue;
                    }
                    let mut enum_tokens = TokenStream::new();
                    for value in &s.enumeration {
                        let value_token = Ident::new(
                            &sanitize_str_for_ident(value.as_ref().unwrap()),
                            Span::call_site(),
                        );
                        enum_tokens.extend(quote! {
                            #value_token,
                        });
                    }
                    // Now we know it's an enum, so let's make one
                    let enum_tokens = quote! {
                        pub enum #ident {
                            #enum_tokens
                        }
                    };
                    return Ok(enum_tokens);
                }
                if let ReferenceOr::Item(Schema {
                    schema_kind: SchemaKind::Any(a),
                    ..
                }) = item
                {
                    if a.enumeration.is_empty() {
                        continue;
                    }
                    let mut enum_tokens = TokenStream::new();
                    for value in &a.enumeration {
                        let value_token = Ident::new(
                            &sanitize_str_for_ident(&format!("{}", value)),
                            Span::call_site(),
                        );
                        enum_tokens.extend(quote! {
                            #value_token,
                        });
                    }
                    // Now we know it's an enum, so let's make one
                    let enum_tokens = quote! {
                        pub enum #ident {
                            #enum_tokens
                        }
                    };
                    return Ok(enum_tokens);
                }
            }
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not a supported AnyOf Schema",
        ))
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not anyof schema",
        ))
    }
}

fn main() -> std::io::Result<()> {
    let mut generator = Generator::from_path("specs")?;

    /*
    generator.generate_all("sbi", /* schema_only */ true)?;
    */

    generator.generate(
        // file_modules
        &[
            ("TS29571_CommonData.yaml", "common_data"),
            ("TS29509_Nausf_UEAuthentication.yaml", "common_data"),
            ("TS29509_Nausf_SoRProtection.yaml", "common_data"),
        ],
        // aux_files:
        &[
            "TS29510_Nnrf_AccessToken.yaml",
            "TS29514_Npcf_PolicyAuthorization.yaml",
            "TS29572_Nlmf_Location.yaml",
            "TS29503_Nudm_UEAU.yaml",
        ],
        // schema_only
        true,
        // handlers
        Some(vec![string_string_enum_anyof]),
    )?;

    //eprintln!("generator: {:#?}", generator);

    Ok(())
}
